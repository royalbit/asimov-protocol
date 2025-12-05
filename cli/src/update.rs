//! Self-update functionality for RoyalBit Asimov CLI
//!
//! Checks GitHub Releases for new versions and updates the binary in-place.
//! The needs of the many: users stay current without manual intervention.

use std::env;
use std::fs;
use std::path::Path;

/// GitHub API URL for latest release
const GITHUB_RELEASES_URL: &str = "https://api.github.com/repos/royalbit/asimov/releases/latest";

/// Current version from Cargo.toml
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Result of version check
#[derive(Debug, Clone)]
pub struct VersionCheck {
    pub current: String,
    pub latest: String,
    pub update_available: bool,
    pub download_url: Option<String>,
    pub checksums_url: Option<String>,
}

/// Get the appropriate asset name for the current platform
pub fn get_platform_asset() -> Option<&'static str> {
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

/// Parse GitHub API response and extract version check info
/// This is the pure logic, separated from HTTP for testability
pub fn parse_github_response(body: &str, current_version: &str) -> Result<VersionCheck, String> {
    let latest_version = extract_json_string(body, "tag_name")
        .ok_or("Could not parse version from GitHub response")?
        .trim_start_matches('v')
        .to_string();

    let update_available = is_newer_version(&latest_version, current_version);

    // Find download URL for current platform
    let download_url = if update_available {
        get_platform_asset().and_then(|asset_name| find_asset_url(body, asset_name))
    } else {
        None
    };

    // Find checksums.txt URL
    let checksums_url = if update_available {
        find_checksums_url(body)
    } else {
        None
    };

    Ok(VersionCheck {
        current: current_version.to_string(),
        latest: latest_version,
        update_available,
        download_url,
        checksums_url,
    })
}

/// Find the download URL for a specific asset in the GitHub API response
pub fn find_asset_url(body: &str, asset_name: &str) -> Option<String> {
    // Try both with and without space after colon (GitHub uses space)
    let search_with_space = format!("\"name\": \"{}\"", asset_name);
    let search_no_space = format!("\"name\":\"{}\"", asset_name);
    let pos = body
        .find(&search_with_space)
        .or_else(|| body.find(&search_no_space));
    if let Some(pos) = pos {
        // Look for browser_download_url near this position
        let chunk = &body[pos.saturating_sub(500)..body.len().min(pos + 500)];
        extract_json_string(chunk, "browser_download_url").filter(|url| url.contains(asset_name))
    } else {
        None
    }
}

/// Find the checksums.txt URL in the GitHub API response
pub fn find_checksums_url(body: &str) -> Option<String> {
    let pos = body
        .find("\"name\": \"checksums.txt\"")
        .or_else(|| body.find("\"name\":\"checksums.txt\""));
    if let Some(pos) = pos {
        let chunk = &body[pos.saturating_sub(500)..body.len().min(pos + 500)];
        extract_json_string(chunk, "browser_download_url")
            .filter(|url| url.contains("checksums.txt"))
    } else {
        None
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
/// Fetch data from a URL using curl
fn fetch_url(url: &str) -> Result<String, String> {
    let output = std::process::Command::new("curl")
        .args([
            "-s",
            "-H",
            "Accept: application/vnd.github.v3+json",
            "-H",
            "User-Agent: asimov-cli",
            url,
        ])
        .output()
        .map_err(|e| format!("Failed to fetch: {}", e))?;

    if !output.status.success() {
        return Err("Failed to fetch from URL".to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Check for updates by querying GitHub Releases API
pub fn check_for_update() -> Result<VersionCheck, String> {
    check_for_update_from_url(GITHUB_RELEASES_URL)
}

/// Check for updates from a custom URL (for testing)
pub fn check_for_update_from_url(url: &str) -> Result<VersionCheck, String> {
    let body = fetch_url(url)?;
    parse_github_response(&body, CURRENT_VERSION)
}

/// Simple JSON string extraction (avoids adding serde_json dependency)
pub fn extract_json_string(json: &str, key: &str) -> Option<String> {
    // Try with space after colon first (GitHub style), then without
    let search_with_space = format!("\"{}\": \"", key);
    let search_no_space = format!("\"{}\":\"", key);

    let (start, search_len) = json
        .find(&search_with_space)
        .map(|pos| (pos, search_with_space.len()))
        .or_else(|| {
            json.find(&search_no_space)
                .map(|pos| (pos, search_no_space.len()))
        })?;

    let value_start = start + search_len;
    let end = json[value_start..].find('"')?;
    Some(json[value_start..value_start + end].to_string())
}

/// Compare semantic versions (returns true if latest > current)
pub fn is_newer_version(latest: &str, current: &str) -> bool {
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

/// Parse checksums file and find the expected checksum for an asset
pub fn parse_checksums(checksums_content: &str, asset_name: &str) -> Option<String> {
    checksums_content
        .lines()
        .find(|line| line.contains(asset_name))
        .and_then(|line| line.split_whitespace().next())
        .map(|s| s.to_string())
}

/// Calculate SHA256 checksum of a file
pub fn calculate_checksum(file_path: &Path) -> Result<String, String> {
    #[cfg(not(target_os = "windows"))]
    {
        let output = std::process::Command::new("sha256sum")
            .arg(file_path)
            .output()
            .map_err(|e| format!("Failed to calculate checksum: {}", e))?;

        if !output.status.success() {
            return Err("Failed to calculate SHA256 checksum".to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string())
    }

    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("certutil")
            .args(["-hashfile", file_path.to_str().unwrap(), "SHA256"])
            .output()
            .map_err(|e| format!("Failed to calculate checksum: {}", e))?;

        if !output.status.success() {
            return Err("Failed to calculate SHA256 checksum".to_string());
        }

        // certutil output has checksum on second line
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .nth(1)
            .unwrap_or("")
            .trim()
            .replace(" ", "")
            .to_lowercase())
    }
}

/// Verify checksum matches expected
pub fn verify_checksum_match(expected: &str, actual: &str) -> Result<(), String> {
    if actual != expected {
        Err(format!(
            "Checksum mismatch!\n  Expected: {}\n  Actual:   {}",
            expected, actual
        ))
    } else {
        Ok(())
    }
}

/// Download and install the update with optional checksum verification (v8.4.0)
#[cfg_attr(feature = "coverage", coverage(off))]
pub fn perform_update(download_url: &str, checksums_url: Option<&str>) -> Result<(), String> {
    let current_exe = env::current_exe()
        .map_err(|e| format!("Could not determine current executable path: {}", e))?;

    println!("  Downloading update...");

    // Download to temp file
    let temp_dir = env::temp_dir();
    let temp_archive = temp_dir.join("asimov_update.tar.gz");

    download_file(download_url, &temp_archive)?;

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
    extract_archive(&temp_archive, &temp_dir)?;

    // Verify extracted binary exists
    if !temp_binary.exists() {
        return Err(format!(
            "Extracted binary not found at {}",
            temp_binary.display()
        ));
    }

    println!("  Installing...");

    // Replace current executable
    replace_binary(&temp_binary, &current_exe)?;

    // Cleanup
    let _ = fs::remove_file(&temp_archive);
    let _ = fs::remove_file(&temp_binary);

    Ok(())
}

/// Download a file from URL to local path
#[cfg_attr(feature = "coverage", coverage(off))]
pub fn download_file(url: &str, dest: &Path) -> Result<(), String> {
    let download_status = std::process::Command::new("curl")
        .args(["-L", "-o", dest.to_str().unwrap(), url])
        .status()
        .map_err(|e| format!("Failed to download: {}", e))?;

    if !download_status.success() {
        return Err("Download failed".to_string());
    }

    Ok(())
}

/// Extract archive to directory
#[cfg_attr(feature = "coverage", coverage(off))]
pub fn extract_archive(archive: &Path, dest_dir: &Path) -> Result<(), String> {
    #[cfg(not(target_os = "windows"))]
    {
        let extract_status = std::process::Command::new("tar")
            .args([
                "-xzf",
                archive.to_str().unwrap(),
                "-C",
                dest_dir.to_str().unwrap(),
            ])
            .status()
            .map_err(|e| format!("Failed to extract: {}", e))?;

        if !extract_status.success() {
            return Err("Extraction failed".to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        let extract_status = std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!(
                    "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                    archive.display(),
                    dest_dir.display()
                ),
            ])
            .status()
            .map_err(|e| format!("Failed to extract: {}", e))?;

        if !extract_status.success() {
            return Err("Extraction failed".to_string());
        }
    }

    Ok(())
}

/// Replace the current binary with a new one
#[cfg_attr(feature = "coverage", coverage(off))]
pub fn replace_binary(new_binary: &Path, current_exe: &Path) -> Result<(), String> {
    let backup_path = current_exe.with_extension("old");

    // Remove old backup if exists
    let _ = fs::remove_file(&backup_path);

    // Rename current to backup
    fs::rename(current_exe, &backup_path)
        .map_err(|e| format!("Failed to backup current binary: {}", e))?;

    // Copy new binary to current location
    fs::copy(new_binary, current_exe)
        .map_err(|e| format!("Failed to install new binary: {}", e))?;

    // Set executable permissions on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(current_exe)
            .map_err(|e| format!("Failed to get permissions: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(current_exe, perms)
            .map_err(|e| format!("Failed to set permissions: {}", e))?;
    }

    // Remove backup
    let _ = fs::remove_file(&backup_path);

    Ok(())
}

/// Verify SHA256 checksum of downloaded file (v8.4.0)
#[cfg_attr(feature = "coverage", coverage(off))]
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
    let expected_checksum = parse_checksums(&checksums, asset_name)
        .ok_or_else(|| format!("Checksum not found for {}", asset_name))?;

    // Calculate actual checksum
    let actual_checksum = calculate_checksum(file_path)?;

    verify_checksum_match(&expected_checksum, &actual_checksum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

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
        // Without spaces (compact JSON)
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

        // With spaces (GitHub API style)
        let json_spaced = r#"{"tag_name": "v8.16.1", "name": "Release 8.16.1"}"#;
        assert_eq!(
            extract_json_string(json_spaced, "tag_name"),
            Some("v8.16.1".to_string())
        );
        assert_eq!(
            extract_json_string(json_spaced, "name"),
            Some("Release 8.16.1".to_string())
        );
    }

    #[test]
    fn test_current_version_set() {
        assert!(CURRENT_VERSION.contains('.'));
    }

    #[test]
    fn test_get_platform_asset() {
        let asset = get_platform_asset();
        #[cfg(any(
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "windows", target_arch = "x86_64")
        ))]
        {
            assert!(asset.is_some());
            let name = asset.unwrap();
            assert!(name.starts_with("asimov-"));
        }
        #[cfg(not(any(
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "windows", target_arch = "x86_64")
        )))]
        {
            assert!(asset.is_none());
        }
    }

    #[test]
    fn test_version_check_struct() {
        let check = VersionCheck {
            current: "1.0.0".to_string(),
            latest: "1.1.0".to_string(),
            update_available: true,
            download_url: Some("https://example.com/file.tar.gz".to_string()),
            checksums_url: Some("https://example.com/checksums.txt".to_string()),
        };
        assert!(check.update_available);
        assert!(check.download_url.is_some());
        assert!(check.checksums_url.is_some());
    }

    #[test]
    fn test_parse_github_response_update_available() {
        let response = r#"{
            "tag_name": "v9.0.0",
            "name": "Release 9.0.0",
            "assets": [
                {
                    "name": "asimov-x86_64-unknown-linux-gnu.tar.gz",
                    "browser_download_url": "https://github.com/royalbit/asimov/releases/download/v9.0.0/asimov-x86_64-unknown-linux-gnu.tar.gz"
                },
                {
                    "name": "checksums.txt",
                    "browser_download_url": "https://github.com/royalbit/asimov/releases/download/v9.0.0/checksums.txt"
                }
            ]
        }"#;

        let result = parse_github_response(response, "1.0.0").unwrap();
        assert_eq!(result.latest, "9.0.0");
        assert!(result.update_available);
    }

    #[test]
    fn test_parse_github_response_no_update() {
        let response = r#"{"tag_name": "v1.0.0", "name": "Release 1.0.0"}"#;
        let result = parse_github_response(response, "1.0.0").unwrap();
        assert_eq!(result.latest, "1.0.0");
        assert!(!result.update_available);
        assert!(result.download_url.is_none());
        assert!(result.checksums_url.is_none());
    }

    #[test]
    fn test_parse_github_response_invalid() {
        let response = r#"{"error": "not found"}"#;
        let result = parse_github_response(response, "1.0.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_find_asset_url() {
        let response = r#"{
            "assets": [
                {
                    "name": "asimov-x86_64-unknown-linux-gnu.tar.gz",
                    "browser_download_url": "https://github.com/royalbit/asimov/releases/download/v9.0.0/asimov-x86_64-unknown-linux-gnu.tar.gz"
                }
            ]
        }"#;
        let url = find_asset_url(response, "asimov-x86_64-unknown-linux-gnu.tar.gz");
        assert!(url.is_some());
        assert!(url
            .unwrap()
            .contains("asimov-x86_64-unknown-linux-gnu.tar.gz"));
    }

    #[test]
    fn test_find_asset_url_not_found() {
        let response = r#"{"assets": []}"#;
        let url = find_asset_url(response, "nonexistent.tar.gz");
        assert!(url.is_none());
    }

    #[test]
    fn test_find_checksums_url() {
        let response = r#"{
            "assets": [
                {
                    "name": "checksums.txt",
                    "browser_download_url": "https://github.com/royalbit/asimov/releases/download/v9.0.0/checksums.txt"
                }
            ]
        }"#;
        let url = find_checksums_url(response);
        assert!(url.is_some());
        assert!(url.unwrap().contains("checksums.txt"));
    }

    #[test]
    fn test_find_checksums_url_not_found() {
        let response = r#"{"assets": []}"#;
        let url = find_checksums_url(response);
        assert!(url.is_none());
    }

    #[test]
    fn test_parse_checksums() {
        let checksums = r#"abc123def456  asimov-x86_64-unknown-linux-gnu.tar.gz
789xyz000111  asimov-aarch64-apple-darwin.tar.gz"#;

        let checksum = parse_checksums(checksums, "asimov-x86_64-unknown-linux-gnu.tar.gz");
        assert_eq!(checksum, Some("abc123def456".to_string()));

        let checksum2 = parse_checksums(checksums, "asimov-aarch64-apple-darwin.tar.gz");
        assert_eq!(checksum2, Some("789xyz000111".to_string()));

        let missing = parse_checksums(checksums, "nonexistent.tar.gz");
        assert!(missing.is_none());
    }

    #[test]
    fn test_verify_checksum_match_success() {
        let result = verify_checksum_match("abc123", "abc123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_checksum_match_failure() {
        let result = verify_checksum_match("abc123", "xyz789");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Checksum mismatch"));
    }

    #[test]
    fn test_calculate_checksum() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&test_file).unwrap();
        file.write_all(b"test content").unwrap();

        let result = calculate_checksum(&test_file);
        assert!(result.is_ok());
        let checksum = result.unwrap();
        // SHA256 is always 64 hex characters
        assert_eq!(checksum.len(), 64);
    }

    #[test]
    fn test_calculate_checksum_missing_file() {
        let result = calculate_checksum(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_archive_missing_file() {
        let temp_dir = TempDir::new().unwrap();
        let result = extract_archive(Path::new("/nonexistent.tar.gz"), temp_dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_download_file_invalid_url() {
        let temp_dir = TempDir::new().unwrap();
        let dest = temp_dir.path().join("download.tar.gz");
        // This will fail because it's not a valid URL that returns a file
        let result = download_file("http://localhost:99999/nonexistent", &dest);
        // Either fails to connect or gets empty/error response
        assert!(result.is_err() || !dest.exists() || fs::metadata(&dest).unwrap().len() == 0);
    }

    #[test]
    fn test_replace_binary() {
        let temp_dir = TempDir::new().unwrap();

        // Create "new" binary
        let new_binary = temp_dir.path().join("new_binary");
        let mut f = fs::File::create(&new_binary).unwrap();
        f.write_all(b"new binary content").unwrap();

        // Create "current" binary
        let current_exe = temp_dir.path().join("current_binary");
        let mut f = fs::File::create(&current_exe).unwrap();
        f.write_all(b"old binary content").unwrap();

        let result = replace_binary(&new_binary, &current_exe);
        assert!(result.is_ok());

        // Verify the current exe now has new content
        let content = fs::read_to_string(&current_exe).unwrap();
        assert_eq!(content, "new binary content");
    }

    #[test]
    fn test_replace_binary_missing_source() {
        let temp_dir = TempDir::new().unwrap();
        let current_exe = temp_dir.path().join("current");
        fs::write(&current_exe, "content").unwrap();

        let result = replace_binary(Path::new("/nonexistent"), &current_exe);
        assert!(result.is_err());
    }

    #[test]
    fn test_fetch_url_invalid() {
        // Test with an invalid URL that should fail
        let result = fetch_url("http://localhost:99999/invalid");
        // Should fail or return error
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    #[test]
    fn test_check_for_update_integration() {
        // Integration test - actually calls GitHub API
        // May fail if network unavailable, that's expected
        let result = check_for_update();
        // Just verify it doesn't panic, result may be Ok or Err depending on network
        let _ = result;
    }

    #[test]
    fn test_perform_update_invalid_url() {
        // Test with invalid URL
        let result = perform_update("http://localhost:99999/invalid.tar.gz", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_perform_update_with_checksums() {
        // Test with invalid URL but with checksums option
        let result = perform_update(
            "http://localhost:99999/invalid.tar.gz",
            Some("http://localhost:99999/checksums.txt"),
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_archive_valid() {
        let temp_dir = TempDir::new().unwrap();

        // Create a valid tar.gz archive
        let archive_path = temp_dir.path().join("test.tar.gz");
        let test_file = temp_dir.path().join("testfile");
        fs::write(&test_file, "content").unwrap();

        // Create tar.gz using command line
        let status = std::process::Command::new("tar")
            .args([
                "-czf",
                archive_path.to_str().unwrap(),
                "-C",
                temp_dir.path().to_str().unwrap(),
                "testfile",
            ])
            .status();

        if status.is_ok() && status.unwrap().success() {
            let extract_dir = temp_dir.path().join("extract");
            fs::create_dir(&extract_dir).unwrap();

            let result = extract_archive(&archive_path, &extract_dir);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_verify_checksum_match_empty() {
        let result = verify_checksum_match("", "");
        assert!(result.is_ok());
    }

    #[test]
    fn test_download_file_to_temp() {
        // Test downloading to a temp directory (will fail due to invalid URL)
        let temp_dir = TempDir::new().unwrap();
        let dest = temp_dir.path().join("download.tar.gz");
        let result = download_file("http://localhost:99999/file.tar.gz", &dest);
        // Should fail - no server running
        assert!(result.is_err() || !dest.exists());
    }

    #[test]
    fn test_extract_archive_nonexistent_src() {
        let temp_dir = TempDir::new().unwrap();
        let result = extract_archive(Path::new("/nonexistent/archive.tar.gz"), temp_dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_binary_target_dir_missing() {
        let temp_dir = TempDir::new().unwrap();
        let new_binary = temp_dir.path().join("new");
        fs::write(&new_binary, "content").unwrap();

        let result = replace_binary(&new_binary, Path::new("/nonexistent/dir/binary"));
        assert!(result.is_err());
    }

    #[test]
    fn test_version_check_no_update() {
        let check = VersionCheck {
            current: "1.0.0".to_string(),
            latest: "1.0.0".to_string(),
            update_available: false,
            download_url: None,
            checksums_url: None,
        };
        assert!(!check.update_available);
        assert!(check.download_url.is_none());
    }

    #[test]
    fn test_parse_github_response_with_checksums_url() {
        let response = r#"{
            "tag_name": "v99.0.0",
            "assets": [
                {
                    "name": "checksums.txt",
                    "browser_download_url": "https://example.com/checksums.txt"
                }
            ]
        }"#;

        let result = parse_github_response(response, "1.0.0").unwrap();
        assert!(result.update_available);
        assert!(result.checksums_url.is_some());
        assert_eq!(
            result.checksums_url.unwrap(),
            "https://example.com/checksums.txt"
        );
    }

    #[test]
    fn test_fetch_url_connection_error() {
        // Test with invalid URL - exercises error path
        let result = fetch_url("http://localhost:99999/nonexistent");
        // Should fail gracefully
        assert!(result.is_err() || result.unwrap().is_empty());
    }

    #[test]
    fn test_check_for_update_from_invalid_url() {
        let result = check_for_update_from_url("http://localhost:99999/invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_github_response_with_download_url() {
        let response = r#"{
            "tag_name": "v99.0.0",
            "assets": [
                {
                    "name": "asimov-x86_64-unknown-linux-gnu.tar.gz",
                    "browser_download_url": "https://example.com/asimov.tar.gz"
                }
            ]
        }"#;

        let result = parse_github_response(response, "1.0.0").unwrap();
        assert!(result.update_available);
        // download_url depends on platform
    }

    #[test]
    fn test_check_for_update_with_mock_server() {
        use mockito::Server;

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/releases/latest")
            .match_header("Accept", "application/vnd.github.v3+json")
            .with_status(200)
            .with_body(
                r#"{
                "tag_name": "v99.0.0",
                "assets": [
                    {
                        "name": "asimov-x86_64-unknown-linux-gnu.tar.gz",
                        "browser_download_url": "https://example.com/asimov.tar.gz"
                    },
                    {
                        "name": "checksums.txt",
                        "browser_download_url": "https://example.com/checksums.txt"
                    }
                ]
            }"#,
            )
            .create();

        let url = format!("{}/releases/latest", server.url());
        let result = check_for_update_from_url(&url);

        mock.assert();
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(info.update_available);
        assert_eq!(info.latest, "99.0.0");
    }

    #[test]
    fn test_check_for_update_mock_no_update() {
        use mockito::Server;

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/releases/latest")
            .with_status(200)
            .with_body(format!(r#"{{"tag_name": "v{}"}}"#, CURRENT_VERSION))
            .create();

        let url = format!("{}/releases/latest", server.url());
        let result = check_for_update_from_url(&url);

        mock.assert();
        assert!(result.is_ok());
        let info = result.unwrap();
        assert!(!info.update_available);
    }

    #[test]
    fn test_check_for_update_mock_server_error() {
        use mockito::Server;

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/releases/latest")
            .with_status(500)
            .with_body("Internal Server Error")
            .create();

        let url = format!("{}/releases/latest", server.url());
        let result = check_for_update_from_url(&url);

        mock.assert();
        // Should fail due to non-success status
        assert!(result.is_err());
    }
}
