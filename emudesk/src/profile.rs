//! Connection profile model and JSON persistence.
//!
//! A profile captures everything needed to launch an RDP session against a
//! host: target address, port, optional username, and display preferences.
//! Profiles are stored as a single JSON file on disk. Passwords are NEVER
//! stored here — Windows' own credential UI handles authentication.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

/// Default RDP port.
pub const DEFAULT_PORT: u16 = 3389;

/// How the RDP window should be sized.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Resolution {
    /// Windowed with a fixed desktop resolution (stored as a string like "1280x800").
    Windowed,
    /// Full screen on a single monitor.
    Fullscreen,
    /// Full screen spanning multiple monitors (sets multi_monitor = true implicitly).
    MultiMonitor,
}

impl fmt::Display for Resolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resolution::Windowed => write!(f, "Pencere"),
            Resolution::Fullscreen => write!(f, "Tam ekran"),
            Resolution::MultiMonitor => write!(f, "Çoklu monitör"),
        }
    }
}

/// A saved connection target.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    /// Human-friendly name for the saved device.
    pub name: String,
    /// IP address, computer name, or EmuDesk ID.
    pub target: String,
    /// RDP port (default 3389).
    pub port: u16,
    /// Optional account name (passwords are never stored).
    pub username: Option<String>,
    /// Windowed desktop size, e.g. "1280x800".
    pub desktop_size: String,
    /// Display mode.
    pub resolution: Resolution,
    /// Span desktop across monitors when possible.
    pub multi_monitor: bool,
    /// Launch fullscreen.
    pub fullscreen: bool,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            name: String::new(),
            target: String::new(),
            port: DEFAULT_PORT,
            username: None,
            desktop_size: "1280x800".to_string(),
            resolution: Resolution::Windowed,
            multi_monitor: false,
            fullscreen: false,
        }
    }
}

impl Profile {
    /// Validate required fields. Returns a Turkish error message on failure.
    pub fn validate(&self) -> Result<(), String> {
        if self.target.trim().is_empty() {
            return Err("Hedef (IP / bilgisayar adı / EmuDesk ID) boş olamaz.".to_string());
        }
        if self.port == 0 {
            return Err("Port 0 olamaz.".to_string());
        }
        if self.name.trim().is_empty() {
            return Err("Profil adı boş olamaz.".to_string());
        }
        Ok(())
    }

    /// Effective desktop size for the RDP file (fullscreen-aware).
    pub fn effective_size(&self) -> String {
        if self.fullscreen {
            "fullscreen".to_string()
        } else if self.desktop_size.trim().is_empty() {
            "1280x800".to_string()
        } else {
            self.desktop_size.trim().to_string()
        }
    }
}

/// Persistence for a list of profiles backed by a single JSON file.
pub struct ProfileStore {
    path: PathBuf,
}

impl ProfileStore {
    /// Create a store at `path`, creating parent dirs if missing.
    pub fn new(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        Ok(Self { path })
    }

    /// Load all saved profiles (empty list if none exist yet).
    pub fn load_all(&self) -> anyhow::Result<Vec<Profile>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let data = std::fs::read(&self.path)?;
        let profiles: Vec<Profile> = serde_json::from_slice(&data)?;
        Ok(profiles)
    }

    /// Persist the full list of profiles.
    pub fn save_all(&self, profiles: &[Profile]) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(profiles)?;
        std::fs::write(&self.path, json)?;
        Ok(())
    }

    /// Add a profile if its name does not already exist; returns the stored list.
    pub fn upsert(&self, profiles: &mut Vec<Profile>, profile: Profile) -> anyhow::Result<()> {
        if let Some(existing) = profiles.iter_mut().find(|p| p.name == profile.name) {
            *existing = profile;
        } else {
            profiles.push(profile);
        }
        self.save_all(profiles)
    }

    /// Remove a profile by name.
    pub fn remove(&self, profiles: &mut Vec<Profile>, name: &str) -> anyhow::Result<()> {
        profiles.retain(|p| p.name != name);
        self.save_all(profiles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn sample() -> Profile {
        Profile {
            name: "Is Sunucusu".to_string(),
            target: "192.168.1.50".to_string(),
            port: 3389,
            username: Some("admin".to_string()),
            desktop_size: "1920x1080".to_string(),
            resolution: Resolution::Windowed,
            multi_monitor: false,
            fullscreen: false,
        }
    }

    #[test]
    fn profile_roundtrip_serde() {
        let p = sample();
        let json = serde_json::to_string(&p).unwrap();
        let back: Profile = serde_json::from_str(&json).unwrap();
        assert_eq!(p, back);
        assert_eq!(back.effective_size(), "1920x1080");
    }

    #[test]
    fn profile_persistence_with_tempfile() {
        let dir = tempdir().unwrap();
        let store = ProfileStore::new(dir.path().join("profiles.json")).unwrap();
        let mut list = Vec::new();
        store.upsert(&mut list, sample()).unwrap();
        assert_eq!(list.len(), 1);

        let reload = store.load_all().unwrap();
        assert_eq!(reload, list);

        store.remove(&mut list, "Is Sunucusu").unwrap();
        assert!(store.load_all().unwrap().is_empty());
    }

    #[test]
    fn validate_catches_bad_input() {
        let mut p = sample();
        p.target = "   ".to_string();
        assert!(p.validate().is_err());

        let mut p = sample();
        p.port = 0;
        assert!(p.validate().is_err());

        assert!(sample().validate().is_ok());
    }

    #[test]
    fn fullscreen_size() {
        let mut p = sample();
        p.fullscreen = true;
        assert_eq!(p.effective_size(), "fullscreen");
    }
}
