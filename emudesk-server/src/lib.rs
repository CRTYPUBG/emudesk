//! EmuDesk coordination (rendezvous) server.
//!
//! A lightweight HTTP + WebSocket service that lets EmuDesk clients resolve
//! each other's online status and public endpoint. The server only maps a
//! public `device_id` to a reachable endpoint — it never stores or relays RDP
//! passwords, session keys, or screen data. The `device_id` is a rendezvous
//! address, **not** an access credential; pairing happens peer-to-peer.
//!
//! Layout:
//!   * [`store`] — persistent registry (JSON file) plus online presence.
//!   * [`routes`] — axum router factory (HTTP API + WebSocket endpoint).
//!   * [`ws`]     — WebSocket hello/ping/pong heartbeat handling.
//!   * [`server`] — thin bootstrap used by the binary.

use std::net::SocketAddr;
use std::path::PathBuf;

use anyhow::Context;
use axum::extract::{ConnectInfo, Path, State, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

pub mod store;
pub mod ws;

pub use store::{DeviceRecord, Registry};

/// Default server listen port.
pub const DEFAULT_PORT: u16 = 5876;

/// Number of seconds a device is considered "online" after its last message.
pub const ONLINE_WINDOW_SECS: u64 = 30;

/// Default relative path of the persisted device registry (override via
/// `SERVER_DATA_DIR`).
pub const DEFAULT_DATA_DIR: &str = "./emudesk-server-data";
pub const DEVICES_FILE: &str = "devices.json";

/// Server-wide shared state.
#[derive(Clone)]
pub struct AppState {
    /// The API token required to register devices (empty == disabled).
    pub api_token: String,
    /// The `device_id` this server advertises; the desktop app can use it to
    /// self-identify to the rendezvous layer.
    pub server_id: String,
    /// Shared registry (in-memory online presence + persistent file).
    pub registry: Registry,
}

impl AppState {
    pub fn new(api_token: String, server_id: String, store_path: PathBuf) -> anyhow::Result<Self> {
        let registry = Registry::load(store_path).context("loading device registry")?;
        Ok(Self {
            api_token,
            server_id,
            registry,
        })
    }
}

/// Resolve `SERVER_DATA_DIR` (or a fixed override) to the registry file path.
pub fn data_dir_from_env() -> PathBuf {
    std::env::var("SERVER_DATA_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_DATA_DIR))
}

// ---------------------------------------------------------------------------
// HTTP API
// ---------------------------------------------------------------------------

/// Build the full axum [`Router`] for the server. Exposed separately so unit
/// tests can call handlers via `Router::oneshot`/`Router::into_service`.
pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/register", post(register))
        .route("/api/lookup/{device_id}", get(lookup))
        .route("/api/devices", get(devices))
        .route("/ws", get(ws_endpoint))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok" }))
}

/// Request body for `POST /api/register`.
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub device_id: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub public_endpoint: Option<String>,
}

/// Response body for `POST /api/register`.
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub device_id: String,
}

async fn register(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(body): Json<RegisterRequest>,
) -> impl IntoResponse {
    if let Err(resp) = check_api_token(&state, &headers) {
        return resp.into_response();
    }
    let device_id = body.device_id.trim().to_string();
    if device_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": "device_id is required" })),
        )
            .into_response();
    }

    let record = DeviceRecord {
        device_id: device_id.clone(),
        label: body.label,
        endpoint: body.public_endpoint,
        created_at: now_epoch_secs(),
        last_seen: None,
    };
    match state.registry.upsert(record).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({ "device_id": device_id })),
        )
            .into_response(),
        Err(e) => {
            eprintln!("register persist error: {e:#}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "storage_failed" })),
            )
                .into_response()
        }
    }
}

/// Response body for `GET /api/lookup/{device_id}`.
#[derive(Debug, Serialize)]
pub struct LookupResponse {
    pub registered: bool,
    pub label: Option<String>,
    pub online: bool,
    pub endpoint: Option<String>,
}

async fn lookup(State(state): State<AppState>, Path(device_id): Path<String>) -> impl IntoResponse {
    let device_id = device_id.trim().to_string();
    match state.registry.by_id(&device_id).await {
        Some(rec) => {
            let online = rec.is_online();
            (
                StatusCode::OK,
                Json(LookupResponse {
                    registered: true,
                    label: rec.label.clone(),
                    online,
                    endpoint: rec.endpoint.clone(),
                }),
            )
                .into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "not_found" })),
        )
            .into_response(),
    }
}

/// Response body for `GET /api/devices` — a compact summary of every device.
#[derive(Debug, Serialize)]
pub struct DeviceSummary {
    pub device_id: String,
    pub label: Option<String>,
    pub online: bool,
}

async fn devices(State(state): State<AppState>) -> impl IntoResponse {
    let all = state.registry.all().await;
    let list: Vec<DeviceSummary> = all
        .into_iter()
        .map(|rec| {
            let online = rec.is_online();
            DeviceSummary {
                device_id: rec.device_id,
                label: rec.label,
                online,
            }
        })
        .collect();
    Json(serde_json::json!({ "devices": list }))
}

// ---------------------------------------------------------------------------
// WebSocket endpoint
// ---------------------------------------------------------------------------

/// Upgrades a client to a WebSocket and runs the heartbeat loop.
async fn ws_endpoint(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws::handle_socket(socket, addr, state))
}

/// Guard used by mutating HTTP handlers: compare `Authorization: Bearer <tok>`
/// with the configured API token. An empty server token disables the check.
fn check_api_token(
    state: &AppState,
    headers: &axum::http::HeaderMap,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if state.api_token.is_empty() {
        return Ok(());
    }
    let got = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string());
    if got.as_deref() == Some(state.api_token.as_str()) {
        Ok(())
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": "unauthorized" })),
        ))
    }
}

fn now_epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Bootstrap used by the binary
// ---------------------------------------------------------------------------

/// Server configuration resolved from the environment.
pub struct ServerSettings {
    pub bind: String,
    pub port: u16,
    pub api_token: String,
    pub data_dir: PathBuf,
}

impl ServerSettings {
    pub fn from_env() -> Self {
        let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(DEFAULT_PORT);
        let api_token = std::env::var("API_TOKEN").unwrap_or_default();
        let data_dir = data_dir_from_env();
        Self {
            bind,
            port,
            api_token,
            data_dir,
        }
    }

    /// Where the registry file lives under `data_dir`.
    pub fn store_path(&self) -> PathBuf {
        self.data_dir.join(DEVICES_FILE)
    }
}

/// Build the listener and run the axum server until the process is killed.
pub async fn run(settings: ServerSettings) -> anyhow::Result<()> {
    if settings.api_token.is_empty() {
        println!("WARNING: API_TOKEN is not set; device registration is OPEN.");
    }

    let server_id = emudesk_core::identity::DeviceIdentity::generate().id;
    let addr: SocketAddr = format!("{}:{}", settings.bind, settings.port)
        .parse()
        .with_context(|| format!("invalid bind address {}", settings.bind))?;

    let state = AppState::new(
        settings.api_token.clone(),
        server_id.clone(),
        settings.store_path(),
    )
    .context("initializing server state")?;

    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("could not bind {addr}"))?;
    let router = app(state).into_make_service_with_connect_info::<SocketAddr>();

    println!(
        "EmuDesk coordination server listening on {addr} (data: {})",
        settings.data_dir.display()
    );
    println!("server identity: {server_id}");

    axum::serve(listener, router).await.context("server error")
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn test_state(dir: &tempfile::TempDir) -> AppState {
        AppState::new(
            "".to_string(),
            "test-server".to_string(),
            dir.path().join(DEVICES_FILE),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let dir = tempfile::tempdir().unwrap();
        let router = app(test_state(&dir));
        let resp = router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn register_then_lookup() {
        let dir = tempfile::tempdir().unwrap();
        let state = AppState::new(
            "".to_string(),
            "test-server".to_string(),
            dir.path().join(DEVICES_FILE),
        )
        .unwrap();

        // Register a device directly against the registry (as the handler does).
        let rec = DeviceRecord {
            device_id: "abc-123".to_string(),
            label: Some("Living Room".to_string()),
            endpoint: Some("192.0.2.10:3389".to_string()),
            created_at: now_epoch_secs(),
            last_seen: None,
        };
        state.registry.upsert(rec.clone()).await.unwrap();

        // Look it up on the same shared registry.
        let got = state.registry.by_id("abc-123").await.unwrap();
        assert_eq!(got.label.as_deref(), Some("Living Room"));
        assert_eq!(got.endpoint.as_deref(), Some("192.0.2.10:3389"));
        // Not yet connected over WS, so offline.
        assert!(!got.is_online());
    }

    #[tokio::test]
    async fn lookup_unknown_returns_404() {
        let dir = tempfile::tempdir().unwrap();
        let router = app(test_state(&dir));
        let resp = router
            .oneshot(
                Request::builder()
                    .uri("/api/lookup/nope")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn registry_persists_across_reloads() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(DEVICES_FILE);

        // Write once.
        {
            let state = test_state(&dir);
            state
                .registry
                .upsert(DeviceRecord {
                    device_id: "dev-persist".to_string(),
                    label: Some("Persistent".to_string()),
                    endpoint: None,
                    created_at: 1,
                    last_seen: Some(2),
                })
                .await
                .unwrap();
        }

        // Reload from disk in a fresh registry.
        let reloaded = Registry::load(path.clone()).unwrap();
        let rec = reloaded.by_id("dev-persist").await.unwrap();
        assert_eq!(rec.label.as_deref(), Some("Persistent"));
        assert_eq!(rec.last_seen, Some(2));
    }
}
