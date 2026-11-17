//! Windows RDP engine: launches the built-in `mstsc.exe` client.

use std::path::PathBuf;
use std::process::Command;

use super::LaunchError;
use crate::profile::Profile;

const SYSTEM32: &str = "System32";

/// Locate the Windows RDP client (`mstsc.exe`).
pub fn find_mstsc() -> Option<PathBuf> {
    let windir = std::env::var_os("WINDIR").map(PathBuf::from).or_else(|| {
        let fallback = PathBuf::from(r"C:\Windows");
        if fallback.is_dir() {
            Some(fallback)
        } else {
            None
        }
    })?;
    let candidates = [
        windir.join(SYSTEM32).join("mstsc.exe"),
        windir.join("mstsc.exe"),
        PathBuf::from(r"C:\Windows\System32\mstsc.exe"),
    ];
    candidates.into_iter().find(|c| c.is_file())
}

/// Build the `target:port` host argument used by MSTSC.
pub fn host_arg(target: &str, port: u16) -> String {
    format!("{}:{}", target.trim(), port)
}

/// Build the contents of the temporary `.rdp` file (extra settings only).
pub fn rdp_content(profile: &Profile) -> String {
    let mut s = String::new();
    if let Some(user) = profile.username.as_deref() {
        if !user.trim().is_empty() {
            s.push_str("username:s:");
            s.push_str(user.trim());
            s.push('\n');
        }
    }
    if profile.multi_monitor {
        s.push_str("span monitors:i:1\n");
    }
    if profile.fullscreen {
        s.push_str("full address:s:");
        s.push_str(profile.target.trim());
        s.push('\n');
    } else if !profile.multi_monitor {
        // Set an explicit desktop resolution for windowed sessions, using the
        // fullscreen-aware `effective_size` (which also falls back to a sane
        // default when `desktop_size` is empty).
        if let Some((w, h)) = profile.effective_size().split_once('x') {
            if let (Ok(w), Ok(h)) = (w.parse::<u32>(), h.parse::<u32>()) {
                s.push_str(&format!("desktopwidth:i:{w}\n"));
                s.push_str(&format!("desktopheight:i:{h}\n"));
            }
        }
    }
    s
}

/// Launch an MSTSC session for the given profile.
pub fn launch(profile: &Profile) -> Result<(), LaunchError> {
    let mstsc = find_mstsc()
        .ok_or_else(|| LaunchError::NotFound("Windows RDP istemcisi bulunamadi.".to_string()))?;

    let rdp_path: Option<PathBuf> = if profile
        .username
        .as_deref()
        .is_some_and(|u| !u.trim().is_empty())
    {
        let mut content = rdp_content(profile);
        if !content.contains("full address:s:") {
            content.insert_str(0, &format!("full address:s:{}\n", profile.target.trim()));
        }
        // Defensive: bail rather than ever write a password into the `.rdp`
        // file, even if a future caller accidentally injects one.
        if contains_password(&content) {
            return Err(LaunchError::Spawn(
                "RDP dosyası parola içeremez; istek engellendi.".to_string(),
            ));
        }
        Some(write_temp_rdp(&profile.name, &content)?)
    } else {
        None
    };

    let mut cmd = Command::new(&mstsc);
    cmd.arg("/v").arg(host_arg(&profile.target, profile.port));
    if let Some(path) = &rdp_path {
        cmd.arg(path.as_os_str());
    }

    match cmd.spawn() {
        Ok(_) => Ok(()),
        Err(e) => Err(LaunchError::Spawn(format!("mstsc.exe baslatilamadi: {e}"))),
    }
}

/// Write the temporary `.rdp` file used to pass extra settings to MSTSC.
fn write_temp_rdp(profile_name: &str, content: &str) -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join("emudesk");
    std::fs::create_dir_all(&dir)?;
    let name = if profile_name.trim().is_empty() {
        "emudesk".to_string()
    } else {
        sanitize_file_name(profile_name.trim())
    };
    let path = dir.join(format!("{name}.rdp"));
    std::fs::write(&path, content)?;
    Ok(path)
}

/// Sanitize a file name for use on Windows.
fn sanitize_file_name(name: &str) -> String {
    let invalid = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    name.chars()
        .map(|c| {
            if c.is_ascii_whitespace() || invalid.contains(&c) {
                '_'
            } else {
                c
            }
        })
        .collect()
}

/// Detect whether RDP file content embeds a plaintext password.
pub fn contains_password(content: &str) -> bool {
    let lowered = content.to_ascii_lowercase();
    lowered.contains("password:s:") || lowered.contains("password 51:b:")
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
            multi_monitor: true,
            fullscreen: false,
        }
    }

    #[test]
    fn host_arg_formats() {
        assert_eq!(host_arg("192.168.1.50", 3389), "192.168.1.50:3389");
        assert_eq!(host_arg("PC-01", 3390), "PC-01:3390");
    }

    #[test]
    fn no_password_in_rdp() {
        let p = sample();
        let content = rdp_content(&p);
        assert!(!contains_password(&content));
        assert!(content.contains("username:s:admin"));
        assert!(content.contains("span monitors:i:1"));
    }

    #[test]
    fn sanitize() {
        assert_eq!(sanitize_file_name("a:b?c"), "a_b_c");
    }
}
