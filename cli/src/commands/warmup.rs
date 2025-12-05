//! Warmup command implementation

use crate::{check_for_update, compile_protocols, resolve_protocol_dir, to_minified_json};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct WarmupResult {
    pub success: bool,
    pub project_name: Option<String>,
    pub project_tagline: Option<String>,
    pub current_version: Option<String>,
    pub current_status: Option<String>,
    pub current_summary: Option<String>,
    pub protocols_json: Option<String>,
    pub update_available: Option<String>,
    pub error: Option<String>,
}

pub fn run_warmup(dir: &Path, check_updates: bool) -> WarmupResult {
    let mut result = WarmupResult {
        success: false,
        project_name: None,
        project_tagline: None,
        current_version: None,
        current_status: None,
        current_summary: None,
        protocols_json: None,
        update_available: None,
        error: None,
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

    let roadmap: serde_yaml::Value = match serde_yaml::from_str(&roadmap_content) {
        Ok(v) => v,
        Err(e) => {
            result.error = Some(format!("Failed to parse roadmap.yaml: {}", e));
            return result;
        }
    };

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

    // Load project.yaml if exists
    let project_path = resolve_protocol_dir(dir).join("project.yaml");
    if let Ok(content) = std::fs::read_to_string(&project_path) {
        if let Ok(project) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
            if let Some(identity) = project.get("identity") {
                result.project_name = identity
                    .get("project")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                result.project_tagline = identity
                    .get("tagline")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
        }
    }

    let _protocols = compile_protocols();
    result.protocols_json = Some(to_minified_json());
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
            "identity:\n  project: MyProject\n  tagline: My tagline\n",
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
            current_version: Some("1.0.0".to_string()),
            current_status: Some("active".to_string()),
            current_summary: Some("Test milestone".to_string()),
            protocols_json: Some("{}".to_string()),
            update_available: None,
            error: None,
        };
        assert!(r.success);
        assert_eq!(r.project_name.unwrap(), "Test");
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
    fn test_warmup_result_protocols() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        let result = run_warmup(temp.path(), false);
        assert!(result.protocols_json.is_some());
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
}
