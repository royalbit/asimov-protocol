//! Self-update functionality for RoyalBit Asimov CLI
//!
//! Checks GitHub Releases for new versions and updates the binary in-place.
//! The needs of the many: users stay current without manual intervention.

use std::env;
use std::fs;

/// GitHub API URL for latest release
const GITHUB_RELEASES_URL: &str = "https://api.github.com/repos/royalbit/asimov/releases/latest";

/// Current version from Cargo.toml
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result of version check
#[derive(Debug)]
pub struct VersionCheck {
    pub current: String,
    pub latest: String,
    pub update_available: bool,
    pub download_url: Option<String>,
    pub checksums_url: Option<String>,
}

/// Get the appropriate asset name for the current platform
fn get_platform_asset() -> Option<&'static str> {
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    return Some("asimov-x86_64-unknown-linux-gnu.tar.gz");

    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    return Some("asimov-aarch64-unknown-linux-gnu.tar.gz");

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    return Some("asimov-aarch64-apple-darwin.tar.gz");

    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    return Some("asimov-x86_64-apple-darwin.tar.gz");

    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    return Some("asimov-x86_64-pc-windows-msvc.zip");

    #[cfg(not(any(
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "windows", target_arch = "x86_64")
    )))]
    return None;
}

/// Check for updates by querying GitHub Releases API
pub fn check_for_update() -> Result<VersionCheck, String> {
    // Use curl to fetch the release info (available on all platforms)
    let output = std::process::Command::new("curl")
        .args([
            "-s",
            "-H",
            "Accept: application/vnd.github.v3+json",
            "-H",
            "User-Agent: asimov-cli",
            GITHUB_RELEASES_URL,
        ])
        .output()
        .map_err(|e| format!("Failed to fetch release info: {}", e))?;

    if !output.status.success() {
        return Err("Failed to fetch release info from GitHub".to_string());
    }

    let body = String::from_utf8_lossy(&output.stdout);

    // Parse version from JSON (simple extraction without serde_json dependency)
    let latest_version = extract_json_string(&body, "tag_name")
        .ok_or("Could not parse version from GitHub response")?
        .trim_start_matches('v')
        .to_string();

    let update_available = is_newer_version(&latest_version, CURRENT_VERSION);

    // Find download URL for current platform
    let download_url = if update_available {
        get_platform_asset().and_then(|asset_name| {
            // Find the browser_download_url for our asset
            let search = format!("\"name\":\"{}\"", asset_name);
            if let Some(pos) = body.find(&search) {
                // Look for browser_download_url near this position
                let chunk = &body[pos.saturating_sub(500)..body.len().min(pos + 500)];
                extract_json_string(chunk, "browser_download_url")
                    .filter(|url| url.contains(asset_name))
            } else {
                None
            }
        })
    } else {
        None
    };

    // Find checksums.txt URL (v8.4.0)
    let checksums_url = if update_available {
        let search = "\"name\":\"checksums.txt\"";
        if let Some(pos) = body.find(search) {
            let chunk = &body[pos.saturating_sub(500)..body.len().min(pos + 500)];
            extract_json_string(chunk, "browser_download_url")
                .filter(|url| url.contains("checksums.txt"))
        } else {
            None
        }
    } else {
        None
    };

    Ok(VersionCheck {
        current: CURRENT_VERSION.to_string(),
        latest: latest_version,
        update_available,
        download_url,
        checksums_url,
    })
}

/// Simple JSON string extraction (avoids adding serde_json dependency)
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\":\"", key);
    if let Some(start) = json.find(&search) {
        let value_start = start + search.len();
        if let Some(end) = json[value_start..].find('"') {
            return Some(json[value_start..value_start + end].to_string());
        }
    }
    None
}

/// Compare semantic versions (returns true if latest > current)
fn is_newer_version(latest: &str, current: &str) -> bool {
    let parse_version =
        |v: &str| -> Vec<u32> { v.split('.').filter_map(|s| s.parse().ok()).collect() };

    let latest_parts = parse_version(latest);
    let current_parts = parse_version(current);

    for i in 0..3 {
        let l = latest_parts.get(i).copied().unwrap_or(0);
        let c = current_parts.get(i).copied().unwrap_or(0);
        if l > c {
            return true;
        }
        if l < c {
            return false;
        }
    }
    false
}

/// Download and install the update with optional checksum verification (v8.4.0)
pub fn perform_update(download_url: &str, checksums_url: Option<&str>) -> Result<(), String> {
    let current_exe = env::current_exe()
        .map_err(|e| format!("Could not determine current executable path: {}", e))?;

    println!("  Downloading update...");

    // Download to temp file
    let temp_dir = env::temp_dir();
    let temp_archive = temp_dir.join("asimov_update.tar.gz");

    let download_status = std::process::Command::new("curl")
        .args(["-L", "-o", temp_archive.to_str().unwrap(), download_url])
        .status()
        .map_err(|e| format!("Failed to download update: {}", e))?;

    if !download_status.success() {
        return Err("Download failed".to_string());
    }

    // Verify checksum if available (v8.4.0)
    if let Some(checksums_url) = checksums_url {
        println!("  Verifying checksum...");
        if let Some(asset_name) = get_platform_asset() {
            verify_checksum(&temp_archive, checksums_url, asset_name)?;
        }
    }

    println!("  Extracting...");

    // Extract the binary
    let temp_binary = temp_dir.join("asimov");

    #[cfg(not(target_os = "windows"))]
    {
        let extract_status = std::process::Command::new("tar")
            .args([
                "-xzf",
                temp_archive.to_str().unwrap(),
                "-C",
                temp_dir.to_str().unwrap(),
            ])
            .status()
            .map_err(|e| format!("Failed to extract update: {}", e))?;

        if !extract_status.success() {
            return Err("Extraction failed".to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows uses zip files
        let temp_archive = temp_dir.join("asimov_update.zip");
        // Use PowerShell to extract
        let extract_status = std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    temp_archive.display(),
                    temp_dir.display()
                ),
            ])
            .status()
            .map_err(|e| format!("Failed to extract update: {}", e))?;

        if !extract_status.success() {
            return Err("Extraction failed".to_string());
        }
    }

    // Verify extracted binary exists
    if !temp_binary.exists() {
        return Err(format!(
            "Extracted binary not found at {}",
            temp_binary.display()
        ));
    }

    println!("  Installing...");

    // Replace current executable
    // On Unix, we can't replace a running executable directly, so we rename first
    let backup_path = current_exe.with_extension("old");

    // Remove old backup if exists
    let _ = fs::remove_file(&backup_path);

    // Rename current to backup
    fs::rename(&current_exe, &backup_path)
        .map_err(|e| format!("Failed to backup current binary: {}", e))?;

    // Move new binary to current location
    fs::copy(&temp_binary, &current_exe)
        .map_err(|e| format!("Failed to install new binary: {}", e))?;

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&current_exe)
            .map_err(|e| format!("Failed to get permissions: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&current_exe, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    // Cleanup
    let _ = fs::remove_file(&temp_archive);
    let _ = fs::remove_file(&temp_binary);
    let _ = fs::remove_file(&backup_path);

    Ok(())
}

/// Verify SHA256 checksum of downloaded file (v8.4.0)
fn verify_checksum(
    file_path: &std::path::Path,
    checksums_url: &str,
    asset_name: &str,
) -> Result<(), String> {
    // Download checksums.txt
    let output = std::process::Command::new("curl")
        .args(["-sL", checksums_url])
        .output()
        .map_err(|e| format!("Failed to download checksums: {}", e))?;

    if !output.status.success() {
        return Err("Failed to download checksums.txt".to_string());
    }

    let checksums = String::from_utf8_lossy(&output.stdout);

    // Find the expected checksum for our asset
    let expected_checksum = checksums
        .lines()
        .find(|line| line.contains(asset_name))
        .and_then(|line| line.split_whitespace().next())
        .ok_or_else(|| format!("Checksum not found for {}", asset_name))?;

    // Calculate actual checksum using sha256sum (Unix) or certutil (Windows)
    #[cfg(not(target_os = "windows"))]
    let actual_checksum = {
        let output = std::process::Command::new("sha256sum")
            .arg(file_path)
            .output()
            .map_err(|e| format!("Failed to calculate checksum: {}", e))?;

        if !output.status.success() {
            return Err("Failed to calculate SHA256 checksum".to_string());
        }

        String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string()
    };

    #[cfg(target_os = "windows")]
    let actual_checksum = {
        let output = std::process::Command::new("certutil")
            .args(["-hashfile", file_path.to_str().unwrap(), "SHA256"])
            .output()
            .map_err(|e| format!("Failed to calculate checksum: {}", e))?;

        if !output.status.success() {
            return Err("Failed to calculate SHA256 checksum".to_string());
        }

        // certutil output has checksum on second line
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .nth(1)
            .unwrap_or("")
            .trim()
            .replace(" ", "")
            .to_lowercase()
    };

    if actual_checksum != expected_checksum {
        return Err(format!(
            "Checksum mismatch!\n  Expected: {}\n  Actual:   {}",
            expected_checksum, actual_checksum
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_comparison() {
        assert!(is_newer_version("7.8.0", "7.7.0"));
        assert!(is_newer_version("8.0.0", "7.9.9"));
        assert!(is_newer_version("7.7.1", "7.7.0"));
        assert!(!is_newer_version("7.7.0", "7.7.0"));
        assert!(!is_newer_version("7.6.0", "7.7.0"));
        assert!(!is_newer_version("7.7.0", "7.8.0"));
    }

    #[test]
    fn test_extract_json_string() {
        let json = r#"{"tag_name":"v7.8.0","name":"Release 7.8.0"}"#;
        assert_eq!(
            extract_json_string(json, "tag_name"),
            Some("v7.8.0".to_string())
        );
        assert_eq!(
            extract_json_string(json, "name"),
            Some("Release 7.8.0".to_string())
        );
        assert_eq!(extract_json_string(json, "missing"), None);
    }

    #[test]
    fn test_current_version_set() {
        // CURRENT_VERSION comes from CARGO_PKG_VERSION, always valid semver
        assert!(CURRENT_VERSION.contains('.'));
    }
}
