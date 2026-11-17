//! Device identity generation and persistent local storage.
//!
//! Each machine gets a stable, unpredictable 128-bit device ID plus a
//! long-lived secret used to prove ownership of that ID. The ID alone must
//! never grant access — it is only a rendezvous address; authentication goes
//! through the pairing code / session keys layer.

use base64::engine::general_purpose::URL_SAFE_NO_PAD as B64;
use base64::Engine;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::path::{Path, PathBuf};

/// A persistent device identity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceIdentity {
    /// Human-displayable stable device id, e.g. "123-456-789".
    pub id: String,
    /// 32 random bytes: proof-of-ownership secret (kept on disk, never sent).
    pub secret: Vec<u8>,
    /// Human-friendly label the user gave this machine (optional).
    pub label: Option<String>,
}

impl DeviceIdentity {
    /// Generate a fresh random identity. `id` is derived from the random
    /// secret so an attacker cannot predict it from the public id.
    pub fn generate() -> Self {
        let mut secret = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut secret);
        let id = make_id(&secret);
        Self {
            id,
            secret: secret.to_vec(),
            label: None,
        }
    }
}

/// Derive a stable, short, collision-resistant public id from the secret.
fn make_id(secret: &[u8]) -> String {
    // 15 hex-ish chars -> ~60 bits of entropy, formatted in 3 groups of 5.
    let mut hasher = sha2::Sha256::new();
    hasher.update(secret);
    let digest = hasher.finalize();
    let b64 = B64.encode(&digest[..11]); // 11 bytes -> 15 chars (URL_SAFE_NO_PAD)
    let b64 = &b64[..15];
    format!("{}-{}-{}", &b64[0..5], &b64[5..10], &b64[10..15])
}

/// Persists `DeviceIdentity` on local disk in a user-owned location.
pub struct IdentityStore {
    path: PathBuf,
}

impl IdentityStore {
    /// Create a store at `path`. Parent directory is created if missing.
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(Self { path })
    }

    /// Load an existing identity or generate and persist a fresh one.
    pub fn load_or_create(&self) -> anyhow::Result<DeviceIdentity> {
        if self.path.exists() {
            let data = std::fs::read(&self.path)?;
            let ident: DeviceIdentity = serde_json::from_slice(&data)?;
            return Ok(ident);
        }
        let ident = DeviceIdentity::generate();
        self.save(&ident)?;
        Ok(ident)
    }

    /// Overwrite the identity on disk.
    pub fn save(&self, ident: &DeviceIdentity) -> anyhow::Result<()> {
        let json = serde_json::to_vec_pretty(ident)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn id_is_stable_and_well_formed() {
        let ident = DeviceIdentity::generate();
        assert_eq!(ident.id.len(), 17); // 5+1+5+1+5
        assert_eq!(ident.secret.len(), 32);
        assert!(ident
            .id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
    }

    #[test]
    fn store_persists_and_reloads() {
        let dir = tempdir().unwrap();
        let store = IdentityStore::new(dir.path().join("id.json")).unwrap();
        let a = store.load_or_create().unwrap();
        let b = store.load_or_create().unwrap();
        assert_eq!(a.id, b.id);
        assert_eq!(a.secret, b.secret);
    }

    #[test]
    fn two_devices_differ() {
        let a = DeviceIdentity::generate();
        let b = DeviceIdentity::generate();
        assert_ne!(a.id, b.id);
        assert_ne!(a.secret, b.secret);
    }
}
