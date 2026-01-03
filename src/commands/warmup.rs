//! Warmup command implementation
//! v12.1.0: Bootstrap approach - output warmup protocol only, not all protocols

use crate::{check_for_update, resolve_protocol_dir, ProjectType, WarmupProtocol};
use std::path::Path;

/// Information about a detected CLI tool
#[derive(Debug, Clone)]
pub struct ToolInfo {
    pub name: String,
    pub path: String,
    pub version: Option<String>,
    pub directive: String,
}

#[derive(Debug, Clone)]
pub struct WarmupResult {
    pub success: bool,
    pub project_name: Option<String>,
    pub project_tagline: Option<String>,
    pub project_type: ProjectType,
    pub current_version: Option<String>,
    pub current_status: Option<String>,
    pub current_summary: Option<String>,
    /// v12.1.0: Bootstrap approach - just warmup protocol, not all protocols
    pub warmup_protocol: Option<WarmupProtocol>,
    pub update_available: Option<String>,
    pub error: Option<String>,
    // WIP Continuity (ADR-047)
    pub wip_active: bool,
    pub wip_item: Option<String>,
    pub wip_progress: Option<String>,
    pub next_milestone: Option<String>,
    pub next_summary: Option<String>,
    // v9.16.0: Full context - include raw file content for zero file reads
    pub project_yaml: Option<serde_yaml_ng::Value>,
    pub roadmap_yaml: Option<serde_yaml_ng::Value>,
    // v9.17.0: Tool detection
    pub tools_available: Vec<ToolInfo>,
}

/// Detect CLI tools available in PATH
fn detect_tools() -> Vec<ToolInfo> {
    let mut tools = Vec::new();

    // Check for ref
    if let Ok(output) = std::process::Command::new("which").arg("ref").output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Try to get version
            let version = std::process::Command::new("ref")
                .arg("--version")
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                    } else {
                        None
                    }
                });

            tools.push(ToolInfo {
                name: "ref".to_string(),
                path,
                version,
                directive: "Use `ref fetch <url>` via Bash instead of WebFetch/WebSearch. Bypasses bot protection, outputs structured JSON.".to_string(),
            });
        }
    }

    tools
}

pub fn run_warmup(dir: &Path, check_updates: bool) -> WarmupResult {
    let mut result = WarmupResult {
        success: false,
        project_name: None,
        project_tagline: None,
        project_type: ProjectType::Generic,
        current_version: None,
        current_status: None,
        current_summary: None,
        warmup_protocol: None,
        update_available: None,
        error: None,
        // WIP Continuity (ADR-047)
        wip_active: false,
        wip_item: None,
        wip_progress: None,
        next_milestone: None,
        next_summary: None,
        // v9.16.0: Full context
        project_yaml: None,
        roadmap_yaml: None,
        // v9.17.0: Tool detection
        tools_available: Vec::new(),
    };

    if check_updates {
        if let Ok(info) = check_for_update() {
            if info.update_available {
                result.update_available = Some(info.latest);
            }
        }
    }

    let roadmap_path = resolve_protocol_dir(dir).join("roadmap.yaml");
    let roadmap_content = match std::fs::read_to_string(&roadmap_path) {
        Ok(c) => c,
        Err(_) => {
            result.error = Some("roadmap.yaml not found".to_string());
            return result;
        }
    };

    let roadmap: serde_yaml_ng::Value = match serde_yaml_ng::from_str(&roadmap_content) {
        Ok(v) => v,
        Err(e) => {
            result.error = Some(format!("Failed to parse roadmap.yaml: {}", e));
            return result;
        }
    };

    // v9.16.0: Store full roadmap content for zero file reads
    result.roadmap_yaml = Some(roadmap.clone());

    if let Some(current) = roadmap.get("current") {
        result.current_version = current
            .get("version")
            .and_then(|v| v.as_str())
            .map(String::from);
        result.current_status = current
            .get("status")
            .and_then(|v| v.as_str())
            .map(String::from);
        result.current_summary = current
            .get("summary")
            .and_then(|v| v.as_str())
            .map(String::from);
    }

    // WIP Continuity Detection (ADR-047)
    // Check for next milestone and WIP items
    if let Some(next) = roadmap.get("next") {
        // Handle both array format and single object format
        let next_items: Vec<&serde_yaml_ng::Value> = if next.is_sequence() {
            next.as_sequence()
                .map(|s| s.iter().collect())
                .unwrap_or_default()
        } else {
            vec![next]
        };

        for item in next_items {
            // Get milestone info
            if result.next_milestone.is_none() {
                result.next_milestone = item
                    .get("version")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                result.next_summary = item
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }

            // Check deliverables for WIP items
            if let Some(deliverables) = item.get("deliverables").and_then(|d| d.as_sequence()) {
                let mut total = 0;
                let mut done = 0;

                for d in deliverables {
                    if let Some(status) = d.get("status").and_then(|s| s.as_str()) {
                        total += 1;
                        if status == "done" {
                            done += 1;
                        } else if status == "wip" && result.wip_item.is_none() {
                            result.wip_active = true;
                            result.wip_item =
                                d.get("id").and_then(|i| i.as_str()).map(String::from);
                        }
                    }
                }

                if total > 0 {
                    result.wip_progress = Some(format!("{}/{}", done, total));
                }
            }
        }
    }

    // Load project.yaml if exists
    let project_path = resolve_protocol_dir(dir).join("project.yaml");
    if let Ok(content) = std::fs::read_to_string(&project_path) {
        if let Ok(project) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&content) {
            // v9.16.0: Store full project content for zero file reads
            result.project_yaml = Some(project.clone());

            if let Some(identity) = project.get("identity") {
                result.project_name = identity
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                result.project_tagline = identity
                    .get("tagline")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                // Read project type from identity.type (v9.2.3)
                if let Some(type_str) = identity.get("type").and_then(|v| v.as_str()) {
                    if let Ok(pt) = type_str.parse::<ProjectType>() {
                        result.project_type = pt;
                    }
                }
            }
        }
    }

    // v12.1.0: Bootstrap approach - load just warmup protocol with load_order
    result.warmup_protocol = Some(crate::protocols::load_warmup_protocol());

    // v9.17.0: Detect available CLI tools
    result.tools_available = detect_tools();

    result.success = true;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_warmup_no_roadmap() {
        let temp = TempDir::new().unwrap();
        let result = run_warmup(temp.path(), false);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_run_warmup_with_roadmap() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test milestone\n",
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert_eq!(result.current_summary, Some("Test milestone".to_string()));
    }

    #[test]
    fn test_run_warmup_with_project() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        std::fs::write(
            asimov_dir.join("project.yaml"),
            "identity:\n  name: MyProject\n  tagline: My tagline\n",
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert_eq!(result.project_name, Some("MyProject".to_string()));
        assert_eq!(result.project_tagline, Some("My tagline".to_string()));
    }

    #[test]
    fn test_run_warmup_invalid_yaml() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(asimov_dir.join("roadmap.yaml"), "invalid: yaml: [").unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_warmup_result_fields() {
        let r = WarmupResult {
            success: true,
            project_name: Some("Test".to_string()),
            project_tagline: Some("Test tagline".to_string()),
            project_type: ProjectType::Rust,
            current_version: Some("1.0.0".to_string()),
            current_status: Some("active".to_string()),
            current_summary: Some("Test milestone".to_string()),
            warmup_protocol: Some(crate::WarmupProtocol {
                on_start: vec!["load_protocols".into()],
                load_order: vec![".asimov/freshness.json".into()],
                note: Some("test".into()),
            }),
            update_available: None,
            error: None,
            wip_active: true,
            wip_item: Some("test-item".to_string()),
            wip_progress: Some("1/3".to_string()),
            next_milestone: Some("2.0.0".to_string()),
            next_summary: Some("Next milestone".to_string()),
            // v9.16.0: Full context fields
            project_yaml: None,
            roadmap_yaml: None,
            // v9.17.0: Tool detection
            tools_available: vec![],
        };
        assert!(r.success);
        assert_eq!(r.project_name.unwrap(), "Test");
        assert_eq!(r.project_type, ProjectType::Rust);
        assert!(r.wip_active);
        assert_eq!(r.wip_item.unwrap(), "test-item");
    }

    // WIP Continuity tests (ADR-047)

    #[test]
    fn test_warmup_detects_wip_item() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            r#"
current:
  version: '1.0'
  status: released
next:
  - version: '2.0'
    summary: 'Next milestone'
    deliverables:
      - id: item-1
        status: done
      - id: item-2
        status: wip
      - id: item-3
        status: todo
"#,
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert!(result.wip_active);
        assert_eq!(result.wip_item, Some("item-2".to_string()));
        assert_eq!(result.wip_progress, Some("1/3".to_string()));
        assert_eq!(result.next_milestone, Some("2.0".to_string()));
        assert_eq!(result.next_summary, Some("Next milestone".to_string()));
    }

    #[test]
    fn test_warmup_no_wip_when_all_todo() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            r#"
current:
  version: '1.0'
next:
  - version: '2.0'
    summary: 'Ready to start'
    deliverables:
      - id: item-1
        status: todo
      - id: item-2
        status: todo
"#,
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert!(!result.wip_active);
        assert!(result.wip_item.is_none());
        assert_eq!(result.wip_progress, Some("0/2".to_string()));
        assert_eq!(result.next_milestone, Some("2.0".to_string()));
    }

    #[test]
    fn test_warmup_no_wip_when_all_done() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            r#"
current:
  version: '1.0'
next:
  - version: '2.0'
    deliverables:
      - id: item-1
        status: done
      - id: item-2
        status: done
"#,
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert!(!result.wip_active);
        assert_eq!(result.wip_progress, Some("2/2".to_string()));
    }

    #[test]
    fn test_run_warmup_with_update_check() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        // This will check for updates (network call)
        let result = run_warmup(temp.path(), true);
        assert!(result.success);
    }

    #[test]
    fn test_run_warmup_minimal() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();

        // Minimal roadmap
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n",
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        assert!(result.current_version.is_some());
    }

    #[test]
    fn test_warmup_result_has_warmup_protocol() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        let result = run_warmup(temp.path(), false);
        assert!(result.warmup_protocol.is_some());
        let warmup = result.warmup_protocol.unwrap();
        assert!(!warmup.load_order.is_empty());
    }

    #[test]
    fn test_run_warmup_parse_error() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Invalid YAML
        std::fs::write(asimov_dir.join("roadmap.yaml"), "invalid: [[[").unwrap();
        let result = run_warmup(temp.path(), false);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_warmup_with_update_available_field() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();

        // Call warmup with check_updates=true
        let result = run_warmup(temp.path(), true);

        // update_available field should be None or Some depending on network
        // Either way, warmup should succeed
        assert!(result.success);
        // The update check code path was exercised
    }

    // v12.1.0: Bootstrap approach - warmup only loads warmup protocol
    // Migrations test removed as we no longer bundle all protocols

    #[test]
    fn test_warmup_load_order_excludes_migrations() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();

        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        // v12.1.0: load_order should not include migrations
        let warmup = result.warmup_protocol.unwrap();
        assert!(!warmup.load_order.iter().any(|p| p.contains("migrations")));
    }

    // v9.17.0: Tool detection tests

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
        // Should return a Vec (may be empty if ref not installed)
        // If ref is installed, should have at least one entry
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
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        let result = run_warmup(temp.path(), false);
        assert!(result.success);
        // tools_available should be populated (tests tool detection runs)
        // Verify structure is correct for any detected tools
        for tool in &result.tools_available {
            assert!(!tool.name.is_empty());
        }
    }
}
