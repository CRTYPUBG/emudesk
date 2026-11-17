//! Persistent device registry.
//!
//! Keeps the list of registered devices in a JSON file on disk and tracks a
//! small "online" flag derived from the last seen heartbeat. The server only
//! stores metadata needed for rendezvous (`label`, `endpoint`, timestamps) —
//! never passwords, keys, or screen data.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Context;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::ONLINE_WINDOW_SECS;

/// One registered device's persistent metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceRecord {
    pub device_id: String,
    pub label: Option<String>,
    /// Publicly reachable endpoint for the RDP session, if the device
    /// advertised one (e.g. `host:port` or a NAT-friendly hint).
    pub endpoint: Option<String>,
    pub created_at: u64,
    /// Unix seconds of the most recent WebSocket heartbeat (or hello).
    pub last_seen: Option<u64>,
}

impl DeviceRecord {
    /// Whether the device is considered online given the heartbeat window.
    pub fn is_online(&self) -> bool {
        self.last_seen
            .map(|ts| now_epoch_secs().saturating_sub(ts) < ONLINE_WINDOW_SECS)
            .unwrap_or(false)
    }
}

/// The on-disk shape of the registry.
#[derive(Debug, Default, Serialize, Deserialize)]
struct StoreFile {
    devices: HashMap<String, DeviceRecord>,
}

/// A cloneable handle to the shared device registry. Underneath it is an
/// async-friendly `Mutex<RegistryInner>`; callers drive the blocking/persist
/// work via the helper functions below.
#[derive(Clone)]
pub struct Registry {
    inner: Arc<Mutex<RegistryInner>>,
}

struct RegistryInner {
    devices: HashMap<String, DeviceRecord>,
    store_path: PathBuf,
}

impl Registry {
    /// Load a registry from `store_path`, creating an empty one if absent.
    pub fn load(store_path: PathBuf) -> anyhow::Result<Self> {
        let devices = if store_path.exists() {
            let data = std::fs::read(&store_path)?;
            let file: StoreFile = serde_json::from_slice(&data)?;
            file.devices
        } else {
            HashMap::new()
        };
        Ok(Self {
            inner: Arc::new(Mutex::new(RegistryInner {
                devices,
                store_path,
            })),
        })
    }

    /// Insert or update a device and persist. Returns an error on I/O failure.
    pub async fn upsert(&self, rec: DeviceRecord) -> anyhow::Result<()> {
        let mut inner = self.inner.lock().await;
        inner.devices.insert(rec.device_id.clone(), rec);
        persist(&inner).context("persisting registry")
    }

    /// Mark a device as seen (heartbeat). Returns `false` if the device is not
    /// registered, `true` when the timestamp was updated and persisted.
    pub async fn touch(&self, device_id: &str) -> anyhow::Result<bool> {
        let mut inner = self.inner.lock().await;
        let Some(rec) = inner.devices.get_mut(device_id) else {
            return Ok(false);
        };
        rec.last_seen = Some(now_epoch_secs());
        persist(&inner).context("persisting heartbeat")?;
        Ok(true)
    }

    /// Look up a single device record by id (cloned; `None` when absent).
    pub async fn by_id(&self, device_id: &str) -> Option<DeviceRecord> {
        self.inner.lock().await.devices.get(device_id).cloned()
    }

    /// Snapshot of every registered device.
    pub async fn all(&self) -> Vec<DeviceRecord> {
        let inner = self.inner.lock().await;
        inner.devices.values().cloned().collect()
    }
}

/// Write the in-memory map to disk. Uses a temp-file + rename so a crash
/// mid-write cannot leave a truncated registry.
fn persist(inner: &RegistryInner) -> std::io::Result<()> {
    if let Some(parent) = inner.store_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let file = StoreFile {
        devices: inner.devices.clone(),
    };
    let json = serde_json::to_vec_pretty(&file).map_err(std::io::Error::other)?;

    let tmp = temp_path(&inner.store_path);
    std::fs::write(&tmp, json)?;
    std::fs::rename(&tmp, &inner.store_path)?;
    Ok(())
}

fn temp_path(store_path: &Path) -> PathBuf {
    let mut name = store_path
        .file_name()
        .map(|s| s.to_os_string())
        .unwrap_or_else(|| "devices.json".into());
    name.push(".tmp");
    store_path.with_file_name(name)
}

fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn upsert_then_by_id() {
        let dir = tempdir().unwrap();
        let reg = Registry::load(dir.path().join("devices.json")).unwrap();
        let rec = DeviceRecord {
            device_id: "dev-1".to_string(),
            label: Some("One".to_string()),
            endpoint: None,
            created_at: 1,
            last_seen: None,
        };
        reg.upsert(rec.clone()).await.unwrap();
        let got = reg.by_id("dev-1").await.unwrap();
        assert_eq!(got.label.as_deref(), Some("One"));
    }

    #[tokio::test]
    async fn reload_from_disk() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("devices.json");

        {
            let reg = Registry::load(path.clone()).unwrap();
            reg.upsert(DeviceRecord {
                device_id: "persist".to_string(),
                label: None,
                endpoint: Some("10.0.0.5:3389".to_string()),
                created_at: 7,
                last_seen: Some(99),
            })
            .await
            .unwrap();
        }

        let reg = Registry::load(path).unwrap();
        let got = reg.by_id("persist").await.unwrap();
        assert_eq!(got.endpoint.as_deref(), Some("10.0.0.5:3389"));
        assert_eq!(got.last_seen, Some(99));
    }

    #[tokio::test]
    async fn touch_marks_unknown_false() {
        let dir = tempdir().unwrap();
        let reg = Registry::load(dir.path().join("devices.json")).unwrap();
        assert!(!reg.touch("ghost").await.unwrap());
    }

    #[test]
    fn online_window_logic() {
        let rec = DeviceRecord {
            device_id: "x".to_string(),
            label: None,
            endpoint: None,
            created_at: 0,
            last_seen: Some(now_epoch_secs()),
        };
        assert!(rec.is_online());

        let stale = DeviceRecord {
            last_seen: Some(now_epoch_secs() - ONLINE_WINDOW_SECS - 1),
            ..rec.clone()
        };
        assert!(!stale.is_online());
    }
}
