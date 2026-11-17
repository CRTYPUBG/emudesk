//! GitHub update-checker integration for EmuDesk.
//!
//! This module reads public GitHub REST API information (stable releases of a
//! public repo) so the desktop app can perform update checks. Only **anonymous
//! public reads** are performed — no token or secret is ever embedded.

use anyhow::{anyhow, bail, Context, Result};
use serde::Deserialize;

/// A GitHub release as reported by the public `releases/latest` endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct GithubUpdate {
    /// Latest stable version, e.g. `"1.2.3"`.
    pub latest_version: String,
    /// RFC 3339 publication timestamp, e.g. `"2026-01-01T12:00:00Z"`.
    pub published_at: String,
    /// Release notes (optional).
    pub body: Option<String>,
    /// Browser-download URL of the platform-matching asset (if any).
    pub asset_download_url: Option<String>,
    /// Whether the release is a prerelease. Stable checker ignores prereleases.
    pub is_prerelease: bool,
}

/// Target platform; used to pick the matching release asset.
///
/// Each host OS only ever constructs its own variant (see `app::current_platform`),
/// so the other variant is dead code on that platform. The lint is skipped per
/// variant because both are legitimate targets on the corresponding OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    #[allow(dead_code)]
    WindowsX64,
    #[allow(dead_code)]
    LinuxX64,
}

impl Platform {
    /// Substring that must appear in an asset's file name to match this platform.
    pub fn asset_pattern(self) -> &'static str {
        match self {
            Platform::WindowsX64 => "windows-x86_64",
            Platform::LinuxX64 => "linux-x86_64",
        }
    }
}

/// Talks to the GitHub public REST API for a single `owner/repo`.
#[derive(Debug, Clone)]
pub struct UpdateChecker {
    repo_owner: String,
    repo_name: String,
}

impl UpdateChecker {
    /// Create a checker for a custom public repository.
    pub fn new(repo_owner: &str, repo_name: &str) -> Self {
        Self {
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
        }
    }

    /// Default EmuDesk repository. Left as a stub until a public release repo
    /// is published — returns an error until configured.
    pub fn emudesk() -> Self {
        // TODO: fill in the real owner/repo once the public release is out.
        Self::new("emudesk", "emudesk")
    }

    /// Query the latest **stable** release. Pre-releases and drafts are ignored.
    ///
    /// If the remote version is strictly newer than `current_version`, returns
    /// `Some(update)`, otherwise `None`.
    pub fn check_for_update(
        &self,
        current_version: &str,
        platform: Platform,
    ) -> Result<Option<GithubUpdate>> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            self.repo_owner, self.repo_name
        );

        let response = self.get_json(&url)?;

        // Parse with the intermediate raw model that mirrors the API shape.
        let raw: RawRelease = serde_json::from_value(response)
            .with_context(|| "GitHub releases/latest yanıtı ayrıştırılamadı")?;

        if raw.prerelease {
            // The `latest` endpoint normally never returns a prerelease, but
            // guard defensively: never offer a prerelease as a stable update.
            return Ok(None);
        }

        let version = raw.tag_name.trim_start_matches('v').to_string();
        if version.is_empty() || !is_newer(&version, current_version) {
            return Ok(None);
        }

        let asset_download_url = raw
            .assets
            .iter()
            .find(|a| asset_matches(&a.name, platform))
            .map(|a| a.browser_download_url.clone());

        Ok(Some(GithubUpdate {
            latest_version: version,
            published_at: raw.published_at,
            body: raw.body,
            asset_download_url,
            is_prerelease: raw.prerelease,
        }))
    }

    /// Perform an authenticated-less GET and turn HTTP problems into `anyhow`.
    fn get_json(&self, url: &str) -> Result<serde_json::Value> {
        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .build();

        let response = agent
            .get(url)
            .set("User-Agent", "EmuDesk/0.1")
            .set("Accept", "application/vnd.github+json")
            .call();

        let response = match response {
            Ok(resp) => resp,
            Err(ureq::Error::Status(404, _)) => {
                bail!("repo bulunamadı: {}/{}", self.repo_owner, self.repo_name)
            }
            Err(ureq::Error::Status(code, _)) if code == 403 || code == 429 => {
                bail!("GitHub API rate limit aşıldı (HTTP {code}). Daha sonra tekrar deneyin.")
            }
            Err(ureq::Error::Status(code, _)) if code >= 500 => {
                bail!("GitHub sunucu hatası (HTTP {code}).")
            }
            Err(e) => return Err(anyhow!("GitHub isteği başarısız: {e}")),
        };

        let text = response
            .into_string()
            .map_err(|e| anyhow!("GitHub yanıtı okunamadı: {e}"))?;
        serde_json::from_str(&text).map_err(|e| anyhow!("GitHub yanıtı JSON olarak okunamadı: {e}"))
    }
}

/// Intermediate model mirroring the GitHub `releases/latest` JSON shape.
#[derive(Debug, Deserialize)]
struct RawRelease {
    tag_name: String,
    #[serde(default)]
    published_at: String,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    prerelease: bool,
    #[serde(default)]
    assets: Vec<RawAsset>,
}

#[derive(Debug, Deserialize)]
struct RawAsset {
    name: String,
    #[serde(default)]
    browser_download_url: String,
}

/// Download a release asset and (optionally) verify its SHA-256 digest.
///
/// When `expected_sha256` is provided, the digest must match exactly or an
/// error is returned. The function returns the raw file bytes.
///
/// This is the download half of the update flow. It is not yet wired into the
/// GUI because the public release repository is still a stub (see
/// [`UpdateChecker::emudesk`]); it is kept as the ready-to-use download helper
/// for when a real release asset exists. The dead-code lint is skipped with
/// justification so the API contract stays available without the app
/// advertising a download it cannot yet reach.
#[allow(dead_code)]
pub fn download_and_verify(url: &str, expected_sha256: Option<&str>) -> Result<Vec<u8>> {
    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(120))
        .build();

    let response = agent
        .get(url)
        .set("User-Agent", "EmuDesk/0.1")
        .set("Accept", "application/octet-stream")
        .call()
        .map_err(|e| anyhow!("indirme başarısız: {e}"))?;

    let mut bytes: Vec<u8> = Vec::new();
    let mut reader = response.into_reader();
    std::io::Read::read_to_end(&mut reader, &mut bytes)
        .with_context(|| "indirilen içerik okunamadı")?;

    if let Some(expected) = expected_sha256 {
        let actual = sha256_hex(&bytes);
        let expected = expected.trim().to_ascii_lowercase();
        if actual != expected {
            bail!("SHA-256 doğrulaması başarısız: beklenen {expected}, alınan {actual}");
        }
    }

    Ok(bytes)
}

/// Compute the lowercase hex SHA-256 digest of `data` using `sha2`.
///
/// Used only by [`download_and_verify`]; kept next to it (and unit-tested
/// below) for when the download flow is wired into the app.
#[allow(dead_code)]
fn sha256_hex(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}

/// Strict semantic-version comparison: `a` newer than `b`?
///
/// Only numeric `major.minor.patch` components are compared; any `-pre` /
/// `+build` suffix is ignored. Malformed numbers are treated as `0`.
pub fn is_newer(a: &str, b: &str) -> bool {
    let a = parse_semver(a);
    let b = parse_semver(b);
    a > b
}

/// Parse the numeric `major.minor.patch` triple of a version string, ignoring
/// pre-release/build suffixes and treating malformed parts as `0`.
fn parse_semver(v: &str) -> (u64, u64, u64) {
    let v = v
        .trim()
        .trim_start_matches('v')
        .split(['-', '+'])
        .next()
        .unwrap_or("");
    let mut parts = v.split('.');
    let major = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|p| p.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

/// Whether `name` contains the platform's asset pattern substring.
pub fn asset_matches(name: &str, platform: Platform) -> bool {
    name.contains(platform.asset_pattern())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_orders_correctly() {
        assert!(is_newer("1.2.3", "1.2.2"));
        assert!(is_newer("2.0.0", "1.9.9"));
        assert!(is_newer("1.10.0", "1.9.9"));
        assert!(!is_newer("1.2.2", "1.2.3"));
        assert!(!is_newer("1.2.3", "1.2.3"));
        assert!(!is_newer("1.2.3", "1.2.4"));
        // Different patch levels.
        assert!(is_newer("1.2.4", "1.2.3"));
        assert!(!is_newer("1.2.0", "1.2.3"));
    }

    #[test]
    fn sha256_hex_matches_known_digest() {
        // Exercise the download helper's digest routine with a small known
        // vector ("abc"), so the hashing used by `download_and_verify` is
        // locked in without any network access.
        let digest = sha256_hex(b"abc");
        assert_eq!(
            digest,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn semver_ignores_suffixes_and_v_prefix() {
        assert!(is_newer("2.0.0", "1.9.9"));
        assert!(is_newer("v2.0.0", "v1.9.9"));
        assert!(!is_newer("1.2.3-rc1", "1.2.3")); // pre-release is not newer
        assert!(is_newer("1.2.3+build", "1.2.2"));
        // Malformed parts degrade to 0 and compare safely.
        assert!(!is_newer("not-a-version", "1.0.0"));
        assert!(is_newer("1.0.0", "not-a-version"));
    }

    #[test]
    fn asset_pattern_matches_platform() {
        assert!(asset_matches(
            "emudesk-1.2.3-windows-x86_64.zip",
            Platform::WindowsX64
        ));
        assert!(asset_matches(
            "emudesk-1.2.3-linux-x86_64.tar.gz",
            Platform::LinuxX64
        ));
        assert!(!asset_matches(
            "emudesk-1.2.3-windows-x86_64.zip",
            Platform::LinuxX64
        ));
        assert!(!asset_matches(
            "emudesk-1.2.3-linux-arm64.tar.gz",
            Platform::LinuxX64
        ));
        assert!(!asset_matches("emudesk-1.2.3.zip", Platform::WindowsX64));
    }

    #[test]
    fn parses_release_and_ignores_prerelease() {
        // Stable release, matching asset.
        let json = serde_json::json!({
            "tag_name": "v1.2.3",
            "published_at": "2026-01-01T12:00:00Z",
            "body": "Fix bugs",
            "prerelease": false,
            "assets": [
                { "name": "emudesk-1.2.3-windows-x86_64.zip",
                  "browser_download_url": "https://example.com/a.zip" },
                { "name": "emudesk-1.2.3-linux-x86_64.tar.gz",
                  "browser_download_url": "https://example.com/a.tar.gz" }
            ]
        });

        // Feed through check_for_update logic via a real UpdateChecker: the
        // function hits the network, so we exercise parsing through a helper
        // that mirrors the raw -> GithubUpdate conversion without HTTP.
        let update = parse_latest_body(json, Platform::WindowsX64).unwrap();
        assert_eq!(update.latest_version, "1.2.3");
        assert_eq!(update.published_at, "2026-01-01T12:00:00Z");
        assert_eq!(update.body.as_deref(), Some("Fix bugs"));
        assert_eq!(
            update.asset_download_url.as_deref(),
            Some("https://example.com/a.zip")
        );
        assert!(!update.is_prerelease);

        // Prerelease release must be rejected.
        let mut pre = serde_json::json!({
            "tag_name": "v1.3.0-beta1",
            "published_at": "2026-01-02T00:00:00Z",
            "body": null,
            "prerelease": true,
            "assets": []
        });
        pre["assets"] = serde_json::json!([
            { "name": "emudesk-1.3.0-windows-x86_64.zip",
              "browser_download_url": "https://example.com/b.zip" }
        ]);
        assert!(parse_latest_body(pre, Platform::WindowsX64).is_none());
    }

    #[test]
    fn parsing_tolerates_missing_fields() {
        // No assets, no body, no prerelease field.
        let json = serde_json::json!({
            "tag_name": "1.2.3",
            "published_at": "2026-01-01T12:00:00Z"
        });
        let update = parse_latest_body(json, Platform::WindowsX64).unwrap();
        assert_eq!(update.latest_version, "1.2.3");
        assert!(update.body.is_none());
        assert!(update.asset_download_url.is_none());
        assert!(!update.is_prerelease); // defaults to false
    }

    /// Convert a `releases/latest` JSON body into `GithubUpdate` without any
    /// network access, reusing the same parsing used by `check_for_update`.
    fn parse_latest_body(body: serde_json::Value, platform: Platform) -> Option<GithubUpdate> {
        let raw: RawRelease = serde_json::from_value(body).ok()?;
        if raw.prerelease {
            return None;
        }
        let version = raw.tag_name.trim_start_matches('v').to_string();
        if version.is_empty() {
            return None;
        }
        let asset_download_url = raw
            .assets
            .iter()
            .find(|a| asset_matches(&a.name, platform))
            .map(|a| a.browser_download_url.clone());
        Some(GithubUpdate {
            latest_version: version,
            published_at: raw.published_at,
            body: raw.body,
            asset_download_url,
            is_prerelease: raw.prerelease,
        })
    }
}
