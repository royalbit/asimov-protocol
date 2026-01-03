//! Warmup command implementation
//! v12.2.0: Minimal warmup - just warmup protocol + tools

use crate::WarmupProtocol;
use std::path::Path;

/// Information about a detected CLI tool
#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub name: String,
    pub path: String,
    pub version: Option<String>,
    pub directive: String,
}

/// v12.2.0: Minimal warmup result - just warmup protocol + tools
#[derive(Debug, Clone)]
pub struct WarmupResult {
    pub success: bool,
    /// v12.2.0: Warmup protocol with files to load
    pub warmup_protocol: Option<WarmupProtocol>,
    pub error: Option<String>,
    /// v9.17.0: Tool detection
    pub tools_available: Vec<ToolInfo>,
}

/// Detect CLI tools available in PATH (v12.3.0: made public for refresh --json)
pub fn detect_tools() -> Vec<ToolInfo> {
    let mut tools = Vec::new();

    // Helper to detect a tool
    fn detect_tool(name: &str, directive: &str) -> Option<ToolInfo> {
        #[cfg(unix)]
        let find_cmd = "which";
        #[cfg(windows)]
        let find_cmd = "where";

        let output = std::process::Command::new(find_cmd)
            .arg(name)
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

        let version = std::process::Command::new(name)
            .arg("--version")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    let v = String::from_utf8_lossy(&o.stdout);
                    Some(v.lines().next().unwrap_or("").trim().to_string())
                } else {
                    None
                }
            });

        Some(ToolInfo {
            name: name.to_string(),
            path,
            version,
            directive: directive.to_string(),
        })
    }

    // Check for ref - web fetching that bypasses bot protection
    if let Some(tool) = detect_tool(
        "ref",
        "Use `ref fetch <url>` via Bash instead of WebFetch. Bypasses bot protection, outputs JSON.",
    ) {
        tools.push(tool);
    }

    // Check for forge - git-native financial modeling
    if let Some(tool) = detect_tool(
        "forge",
        "Use `forge` for financial modeling. 173 functions, Monte Carlo, scenarios, decision trees.",
    ) {
        tools.push(tool);
    }

    tools
}

/// v12.2.0: Minimal warmup - just check .asimov exists, load warmup protocol, detect tools
pub fn run_warmup(dir: &Path, _check_updates: bool) -> WarmupResult {
    let mut result = WarmupResult {
        success: false,
        warmup_protocol: None,
        error: None,
        tools_available: Vec::new(),
    };

    // Check if .asimov directory exists
    let asimov_dir = dir.join(".asimov");
    if !asimov_dir.exists() || !asimov_dir.is_dir() {
        result.error = Some(".asimov directory not found".to_string());
        return result;
    }

    // Load warmup protocol
    result.warmup_protocol = Some(crate::protocols::load_warmup_protocol());

    // Detect available CLI tools
    result.tools_available = detect_tools();

    result.success = true;
    result
}

/// v12.2.0: Simplified tests for minimal warmup
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_warmup_no_asimov_dir() {
        let temp = TempDir::new().unwrap();
        let result = run_warmup(temp.path(), false);
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("not found"));
    }

    #[test]
    fn test_run_warmup_with_asimov_dir() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert!(result.warmup_protocol.is_some());
    }

    #[test]
    fn test_warmup_protocol_has_files() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        let warmup = result.warmup_protocol.unwrap();
        assert!(!warmup.files.is_empty());
        assert!(warmup.files.iter().any(|f| f.contains("project.yaml")));
        assert!(warmup.files.iter().any(|f| f.contains("roadmap.yaml")));
    }

    #[test]
    fn test_warmup_files_exclude_migrations() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        let warmup = result.warmup_protocol.unwrap();
        assert!(!warmup.files.iter().any(|f| f.contains("migrations")));
    }

    #[test]
    fn test_warmup_result_struct() {
        let r = WarmupResult {
            success: true,
            warmup_protocol: Some(crate::WarmupProtocol {
                on_start: vec!["load_files".into()],
                files: vec![".asimov/project.yaml".into()],
                note: Some("test".into()),
            }),
            error: None,
            tools_available: vec![],
        };
        assert!(r.success);
        assert!(r.warmup_protocol.is_some());
    }

    // Tool detection tests

    #[test]
    fn test_tool_info_struct() {
        let tool = ToolInfo {
            name: "test-tool".to_string(),
            path: "/usr/bin/test-tool".to_string(),
            version: Some("1.0.0".to_string()),
            directive: "Use this tool".to_string(),
        };
        assert_eq!(tool.name, "test-tool");
        assert!(tool.version.is_some());
    }

    #[test]
    fn test_detect_tools_returns_vec() {
        let tools = detect_tools();
        // Should return a Vec (may be empty if tools not installed)
        for tool in &tools {
            assert!(!tool.name.is_empty());
            assert!(!tool.directive.is_empty());
        }
    }

    #[test]
    fn test_warmup_includes_tools() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        // tools_available field exists
        for tool in &result.tools_available {
            assert!(!tool.name.is_empty());
        }
    }
}
