//! Platform-agnostic RDP launcher.
//!
//! This module dispatches to the active RDP engine based on the host OS:
//! - Windows: the built-in MSTSC client (`mstsc.exe`).
//! - Linux: FreeRDP (`xfreerdp3` / `xfreerdp`).
//!
//! Any other target is unsupported.

use crate::profile::Profile;

#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

/// Error produced when launching an RDP session fails.
#[derive(Debug, thiserror::Error)]
pub enum LaunchError {
    /// The platform's RDP client binary could not be located.
    #[error("RDP istemcisi bulunamadi: {0}")]
    NotFound(String),
    /// A temporary RDP file could not be written.
    #[error("RDP dosyasi olusturulamadi: {0}")]
    WriteFile(#[from] std::io::Error),
    /// The RDP client process could not be started.
    #[error("Baslatma hatasi: {0}")]
    Spawn(String),
    /// This operating system has no supported RDP engine.
    #[error("Bu platformda desteklenen RDP motoru yok: {0}")]
    #[allow(dead_code)] // Only constructed on non-Windows/non-Linux targets.
    Unsupported(String),
}

/// Launch an RDP session for the given profile, dispatched by platform.
#[cfg(windows)]
pub fn launch(profile: &Profile) -> Result<(), LaunchError> {
    windows::launch(profile)
}

/// Launch an RDP session for the given profile, dispatched by platform.
#[cfg(target_os = "linux")]
pub fn launch(profile: &Profile) -> Result<(), LaunchError> {
    linux::launch(profile)
}

/// Launch an RDP session for the given profile, dispatched by platform.
#[cfg(not(any(windows, target_os = "linux")))]
pub fn launch(_profile: &Profile) -> Result<(), LaunchError> {
    Err(LaunchError::Unsupported(
        "yalnizca Windows ve Linux desteklenmektedir.".to_string(),
    ))
}

/// Build the `target:port` host argument used by RDP clients.
pub fn host_arg(target: &str, port: u16) -> String {
    format!("{}:{}", target.trim(), port)
}

/// Describe the active RDP engine for display in the GUI.
#[cfg(windows)]
pub fn describe_backend() -> &'static str {
    "Windows MSTSC"
}

/// Describe the active RDP engine for display in the GUI.
#[cfg(target_os = "linux")]
pub fn describe_backend() -> &'static str {
    "FreeRDP (xfreerdp3)"
}

/// Describe the active RDP engine for display in the GUI.
#[cfg(not(any(windows, target_os = "linux")))]
pub fn describe_backend() -> &'static str {
    "Desteklenmeyen platform"
}
