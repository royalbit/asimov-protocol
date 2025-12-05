//! Refresh command implementation

use crate::{validate_directory_with_regeneration, validator::regenerate_protocol_files};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct RefreshResult {
    pub success: bool,
    pub is_asimov_project: bool,
    pub files_regenerated: Vec<String>,
    pub files_unchanged: Vec<String>,
    pub protocols_updated: Vec<String>, // v9.0.0: outdated protocols that were updated
    pub protocols_created: Vec<String>, // v9.0.0: missing protocols that were created
    pub protocols_ok: Vec<String>,      // v9.0.0: protocols that matched expected
    pub error: Option<String>,
}

pub fn run_refresh(dir: &Path) -> RefreshResult {
    let mut result = RefreshResult {
        success: false,
        is_asimov_project: false,
        files_regenerated: Vec::new(),
        files_unchanged: Vec::new(),
        protocols_updated: Vec::new(),
        protocols_created: Vec::new(),
        protocols_ok: Vec::new(),
        error: None,
    };

    if !dir.join(".asimov").is_dir() {
        result.error = Some("Not in an asimov project (.asimov/ not found)".to_string());
        return result;
    }
    result.is_asimov_project = true;

    // v9.0.0: Check and regenerate protocol JSON files
    match regenerate_protocol_files(dir) {
        Ok(protocol_results) => {
            for (filename, was_different) in protocol_results {
                let file_path = dir.join(".asimov").join(&filename);
                let existed_before = file_path.exists() || was_different;

                if was_different {
                    if existed_before {
                        result.protocols_updated.push(filename);
                    } else {
                        result.protocols_created.push(filename);
                    }
                } else {
                    result.protocols_ok.push(filename);
                }
            }
        }
        Err(e) => {
            result.error = Some(format!("Protocol regeneration failed: {}", e));
            return result;
        }
    }

    // Also check roadmap.yaml etc.
    match validate_directory_with_regeneration(dir, true) {
        Ok((_, regen_info)) => {
            for (file, changed) in regen_info.regenerated {
                if changed {
                    result.files_regenerated.push(file);
                } else {
                    result.files_unchanged.push(file);
                }
            }
            result.success = true;
        }
        Err(e) => {
            result.error = Some(format!("Regeneration failed: {}", e));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_refresh_not_asimov() {
        let temp = TempDir::new().unwrap();
        let result = run_refresh(temp.path());
        assert!(!result.success);
        assert!(!result.is_asimov_project);
    }

    #[test]
    fn test_run_refresh_with_asimov() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let result = run_refresh(temp.path());
        assert!(result.success);
        assert!(result.is_asimov_project);
    }

    #[test]
    fn test_refresh_result_fields() {
        let r = RefreshResult {
            success: true,
            is_asimov_project: true,
            files_regenerated: vec!["file.json".to_string()],
            files_unchanged: vec![],
            protocols_updated: vec!["outdated.json".to_string()],
            protocols_created: vec![],
            protocols_ok: vec!["ok.json".to_string()],
            error: None,
        };
        assert!(r.success);
        assert!(!r.files_regenerated.is_empty());
        assert!(!r.protocols_updated.is_empty());
    }

    #[test]
    fn test_run_refresh_in_asimov_dir() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let result = run_refresh(temp.path());
        assert!(result.is_asimov_project);
    }

    #[test]
    fn test_run_refresh_with_regeneration() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let result = run_refresh(temp.path());
        assert!(result.is_asimov_project);
        assert!(result.success);
    }
}
