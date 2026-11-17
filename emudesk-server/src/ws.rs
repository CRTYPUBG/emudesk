//! WebSocket heartbeat handling for the coordination server.
//!
//! A connected client sends `{"type":"hello","device_id":...}` as its first
//! frame; the server marks that device "online" (by refreshing `last_seen` in
//! the registry) and answers with an ack. Every subsequent frame refreshes the
//! timestamp, so a device that stops talking for [`crate::ONLINE_WINDOW_SECS`]
//! seconds is treated as offline by the lookup API. The server also sends its
//! own periodic ping frames to keep NAT mappings / proxies alive.

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use axum::extract::ws::{Message, WebSocket};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::time::interval;

use crate::AppState;

/// Server->client liveness probe interval.
const PING_INTERVAL: Duration = Duration::from_secs(15);

/// Incoming client frame.
#[derive(Debug, Deserialize)]
struct ClientMsg {
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    device_id: Option<String>,
}

/// Outgoing server frame.
#[derive(Debug, Serialize)]
struct ServerMsg {
    #[serde(rename = "type")]
    kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
}

/// Run the heartbeat loop for one upgraded connection until it closes.
pub async fn handle_socket(socket: WebSocket, addr: SocketAddr, state: AppState) {
    if let Err(e) = run(socket, &state).await {
        eprintln!("ws {addr}: {e:#}");
    }
}

async fn run(mut socket: WebSocket, state: &AppState) -> anyhow::Result<()> {
    // The client must introduce itself with a `hello` before anything else.
    let first = socket
        .next()
        .await
        .context("waiting for client hello")?
        .context("read error before hello")?;

    let device_id = match hello_device_id(&first) {
        Some(id) => id,
        None => {
            socket
                .send(Message::Text(json_string(ServerMsg {
                    kind: "error",
                    device_id: None,
                })))
                .await
                .ok();
            anyhow::bail!("client did not send a valid hello");
        }
    };

    if !state
        .registry
        .touch(&device_id)
        .await
        .context("marking online")?
    {
        // Unknown device: it must register over HTTP first. Reply and close.
        socket
            .send(Message::Text(json_string(ServerMsg {
                kind: "unknown_device",
                device_id: None,
            })))
            .await
            .ok();
        anyhow::bail!("device {device_id} is not registered");
    }
    socket
        .send(Message::Text(json_string(ServerMsg {
            kind: "hello_ack",
            device_id: Some(device_id.clone()),
        })))
        .await
        .context("sending hello_ack")?;

    let mut ping = interval(PING_INTERVAL);
    loop {
        tokio::select! {
            frame = socket.next() => {
                let Some(frame) = frame else {
                    // Clean close by the peer.
                    break;
                };
                let frame = frame.context("read frame")?;
                if handle_frame(frame, &mut socket, state, &device_id).await? {
                    // Connection should close.
                    break;
                }
            }
            _ = ping.tick() => {
                socket.send(Message::Text(json_string(ServerMsg {
                    kind: "ping",
                    device_id: None,
                }))).await.context("sending server ping")?;
            }
        }
    }
    Ok(())
}

/// Extract and validate the `device_id` from a `hello` text frame.
fn hello_device_id(frame: &Message) -> Option<String> {
    let Message::Text(text) = frame else {
        return None;
    };
    let msg: ClientMsg = serde_json::from_str(text).ok()?;
    if msg.kind != "hello" {
        return None;
    }
    let id = msg.device_id?.trim().to_string();
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

/// Process one incoming frame. Returns `true` when the connection should end.
async fn handle_frame(
    frame: Message,
    socket: &mut WebSocket,
    state: &AppState,
    device_id: &str,
) -> anyhow::Result<bool> {
    match frame {
        Message::Text(text) => {
            // Refresh online presence for any traffic from this device.
            state
                .registry
                .touch(device_id)
                .await
                .context("refreshing online presence")?;

            if let Ok(msg) = serde_json::from_str::<ClientMsg>(&text) {
                match msg.kind.as_str() {
                    "ping" => {
                        socket
                            .send(Message::Text(json_string(ServerMsg {
                                kind: "pong",
                                device_id: None,
                            })))
                            .await
                            .context("sending pong")?;
                    }
                    "bye" => return Ok(true),
                    _ => {}
                }
            }
            Ok(false)
        }
        Message::Close(_) => Ok(true),
        Message::Ping(bytes) => {
            socket
                .send(Message::Pong(bytes))
                .await
                .context("sending pong")?;
            Ok(false)
        }
        Message::Pong(_) | Message::Binary(_) => Ok(false),
    }
}

/// Serialize a server frame to a JSON string.
fn json_string(msg: ServerMsg) -> String {
    serde_json::to_string(&msg).expect("serializing ws message")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_hello() {
        let frame = Message::Text(json!({"type": "hello", "device_id": "  dev-1  "}).to_string());
        assert_eq!(hello_device_id(&frame).as_deref(), Some("dev-1"));
    }

    #[test]
    fn rejects_non_hello() {
        let frame = Message::Text(json!({"type": "ping"}).to_string());
        assert_eq!(hello_device_id(&frame), None);
    }
}
