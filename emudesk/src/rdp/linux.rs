//! Linux RDP engine: launches the FreeRDP client (`xfreerdp3` / `xfreerdp`).
//!
//! FreeRDP ships its own graphical authentication prompt, so passwords are
//! never passed on the command line or written to any file. The default
//! certificate verification behavior is preserved so users are warned about
//! unverified servers instead of being silently exposed.

use std::path::PathBuf;
use std::process::Command;

use super::LaunchError;
use crate::profile::Profile;

/// Candidate executable names for FreeRDP, in priority order.
const FREERDP_NAMES: [&str; 2] = ["xfreerdp3", "xfreerdp"];

/// Extra directories to probe in addition to `$PATH`.
const EXTRA_DIRS: [&str; 2] = ["/usr/bin", "/usr/local/bin"];

/// Locate a FreeRDP client executable in `$PATH` and common install dirs.
pub fn find_freerdp() -> Option<PathBuf> {
    for dir in std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
        .unwrap_or_default()
    {
        for name in FREERDP_NAMES {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    // Fall back to common absolute install locations.
    for base in EXTRA_DIRS {
        for name in FREERDP_NAMES {
            let candidate = PathBuf::from(base).join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }

    None
}

/// Build the argument vector passed to FreeRDP for the given profile.
///
/// Separated from `launch` so it can be unit-tested without spawning a
/// process. The password is never part of these arguments.
pub fn args_for(profile: &Profile) -> Vec<String> {
    let mut args = Vec::new();
    args.push(format!(
        "/v:{}",
        crate::rdp::host_arg(&profile.target, profile.port)
    ));

    if let Some(user) = profile.username.as_deref() {
        if !user.trim().is_empty() {
            args.push(format!("/u:{}", user.trim()));
        }
    }

    // FreeRDP uses `/f` for fullscreen, regardless of multi-monitor spanning.
    if profile.multi_monitor || profile.fullscreen {
        args.push("/f".to_string());
    } else {
        args.push(format!("/size:{}", profile.effective_size()));
    }

    args
}

/// Launch a FreeRDP session for the given profile.
pub fn launch(profile: &Profile) -> Result<(), LaunchError> {
    let freerdp = find_freerdp().ok_or_else(|| {
        LaunchError::NotFound("FreeRDP istemcisi bulunamadi (xfreerdp3 / xfreerdp).".to_string())
    })?;

    let args = args_for(profile);
    match Command::new(&freerdp).args(&args).spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(LaunchError::Spawn(format!("FreeRDP baslatilamadi: {e}"))),
    }
}

/// Run `xfreerdp /version` and return a short summary of the output.
///
/// Best-effort only; failures are swallowed. The executable used is whichever
/// `find_freerdp` locates, falling back to the preferred name if none is found.
pub fn version() -> Option<String> {
    let exe = find_freerdp().unwrap_or_else(|| PathBuf::from(FREERDP_NAMES[0]));
    let output = Command::new(&exe).arg("/version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let first = text.lines().next()?;
    let summary = first
        .split_whitespace()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");
    if summary.trim().is_empty() {
        None
    } else {
        Some(summary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Profile {
        Profile {
            name: "Is Sunucusu".to_string(),
            target: "192.168.1.50".to_string(),
            port: 3389,
            username: Some("admin".to_string()),
            desktop_size: "1280x800".to_string(),
            resolution: crate::profile::Resolution::Windowed,
            multi_monitor: false,
            fullscreen: false,
        }
    }

    #[test]
    fn host_arg_is_forwarded() {
        assert_eq!(
            crate::rdp::host_arg("192.168.1.50", 3389),
            "192.168.1.50:3389"
        );
    }

    #[test]
    fn args_include_host_user_and_size() {
        let args = args_for(&sample());
        assert!(args.contains(&"/v:192.168.1.50:3389".to_string()));
        assert!(args.contains(&"/u:admin".to_string()));
        assert!(args.contains(&"/size:1280x800".to_string()));
    }

    #[test]
    fn args_omit_username_when_absent() {
        let mut p = sample();
        p.username = None;
        let args = args_for(&p);
        assert!(!args.iter().any(|a| a.starts_with("/u:")));
    }

    #[test]
    fn fullscreen_adds_flag_without_size() {
        let mut p = sample();
        p.fullscreen = true;
        let args = args_for(&p);
        assert!(args.contains(&"/f".to_string()));
        assert!(!args.iter().any(|a| a.starts_with("/size:")));
    }

    #[test]
    fn multi_monitor_adds_fullscreen_flag() {
        let mut p = sample();
        p.multi_monitor = true;
        let args = args_for(&p);
        assert!(args.contains(&"/f".to_string()));
        assert!(!args.iter().any(|a| a.starts_with("/size:")));
    }

    #[test]
    fn password_never_in_args() {
        // Passwords are never part of a Profile, but defensively assert that
        // none of the generated arguments look like a credential.
        let args = args_for(&sample());
        for a in &args {
            let lowered = a.to_ascii_lowercase();
            assert!(!lowered.starts_with("/p:"));
            assert!(!lowered.starts_with("/password:"));
            assert!(!lowered.contains("password"));
        }
    }
}
