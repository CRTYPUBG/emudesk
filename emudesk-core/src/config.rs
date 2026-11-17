//! Shared configuration model.
//!
//! Kept dependency-light so both the desktop app and the signal server can
//! load the same on-disk layout. Sensitive values (tokens, secrets) are never
//! embedded here — they come from environment variables or interactive input.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Default TLS listener port for the signalling / relay server.
pub const DEFAULT_SIGNAL_PORT: u16 = 5876;

/// Where the desktop app keeps its local state.
pub fn default_data_dir() -> PathBuf {
    #[cfg(windows)]
    {
        std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".").join("emudesk"))
            .join("EmuDesk")
    }
    #[cfg(not(windows))]
    {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
            .unwrap_or_else(|| PathBuf::from("."))
            .join("emudesk")
    }
}

/// Authentication / pairing-code policy. These bounds protect against
/// brute-force and long-lived codes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// How long a pairing code stays valid.
    pub code_ttl_secs: u64,
    /// Maximum failed attempts before a code is invalidated.
    pub max_attempts: u32,
    /// PBKDF2 iterations when hashing codes.
    pub pbkdf2_iterations: u32,
    /// If true, a previously paired (pinned fingerprint) device may reconnect
    /// without a fresh pairing code for a bounded window.
    pub allow_trusted_reconnect: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            code_ttl_secs: 120, // 2 minutes
            max_attempts: 5,
            pbkdf2_iterations: 600_000,
            allow_trusted_reconnect: true,
        }
    }
}

/// Desktop application configuration (Client & Server modes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Signalling server to reach (defaults to the local one).
    pub signal_server: String,
    /// Port of the signalling server.
    pub signal_port: u16,
    /// Path of the identity file.
    pub identity_path: PathBuf,
    /// Authentication policy.
    pub auth: AuthConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            signal_server: "127.0.0.1".to_string(),
            signal_port: DEFAULT_SIGNAL_PORT,
            identity_path: default_data_dir().join("identity.json"),
            auth: AuthConfig::default(),
        }
    }
}

/// Signal-server configuration (hosted side).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Address to bind, e.g. "0.0.0.0".
    pub bind: String,
    /// TLS port.
    pub port: u16,
    /// Directory for persisted peer registrations (optional).
    pub data_dir: PathBuf,
    /// Authentication policy for pairing codes.
    pub auth: AuthConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: "0.0.0.0".to_string(),
            port: DEFAULT_SIGNAL_PORT,
            data_dir: default_data_dir().join("server"),
            auth: AuthConfig::default(),
        }
    }
}
