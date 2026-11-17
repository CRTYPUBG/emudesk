//! EmuDesk core crate.
//!
//! Shared primitives used by both the desktop client (`emudesk`) and the
//! coordination server (`emudesk-server`):
//!   * persistent device identity generation & local storage
//!   * shared configuration model
//!
//! Remote-desktop transport is provided by Windows' built-in `mstsc.exe`
//! (launched by the desktop app), so this crate intentionally carries no
//! screen/input protocol.

pub mod config;
pub mod identity;

pub use config::{AppConfig, AuthConfig};
pub use identity::{DeviceIdentity, IdentityStore};
