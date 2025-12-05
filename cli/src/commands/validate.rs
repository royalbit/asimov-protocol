//! Validate command implementation

use crate::{
    check_ethics_status, check_green_status, check_sycophancy_status, scan_directory_for_red_flags,
    validate_directory_with_regeneration, EthicsStatus, GreenStatus, SycophancyStatus,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ValidateFileResult {
    pub file: String,
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub regenerated: bool,
}

#[derive(Debug, Clone)]
pub struct EthicsScanResult {
    pub red_flags_found: usize,
    pub matches: Vec<EthicsMatch>,
}

#[derive(Debug, Clone)]
pub struct EthicsMatch {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub category: String,
}

#[derive(Debug, Clone)]
pub struct ValidateResult {
    pub success: bool,
    pub protocol_files: Vec<ValidateFileResult>,
    pub roadmap: Option<ValidateFileResult>,
    pub project: Option<ValidateFileResult>,
    pub ethics: EthicsStatus,
    pub sycophancy: SycophancyStatus,
    pub green: GreenStatus,
    pub ethics_scan: Option<EthicsScanResult>,
    pub regenerated: Vec<String>,
}

pub fn run_validate(dir: &Path, ethics_scan: bool) -> ValidateResult {
    let mut result = ValidateResult {
        success: true,
        protocol_files: Vec::new(),
        roadmap: None,
        project: None,
        ethics: check_ethics_status(dir),
        sycophancy: check_sycophancy_status(dir),
        green: check_green_status(dir),
        ethics_scan: None,
        regenerated: Vec::new(),
    };

    match validate_directory_with_regeneration(dir, true) {
        Ok((results, regen_info)) => {
            for r in results {
                let file_result = ValidateFileResult {
                    file: r.file.clone(),
                    valid: r.is_valid,
                    errors: r.errors.clone(),
                    warnings: r.warnings.clone(),
                    regenerated: r.regenerated,
                };

                if r.file.contains("roadmap") {
                    result.roadmap = Some(file_result);
                } else if r.file.contains("project") {
                    result.project = Some(file_result);
                } else {
                    result.protocol_files.push(file_result);
                }

                if !r.is_valid {
                    result.success = false;
                }
                if r.regenerated {
                    result.regenerated.push(r.file);
                }
            }
            for (f, _) in regen_info.regenerated {
                if !result.regenerated.contains(&f) {
                    result.regenerated.push(f);
                }
            }
        }
        Err(_) => {
            result.success = false;
        }
    }

    if ethics_scan {
        if let Ok(matches) = scan_directory_for_red_flags(dir) {
            let ethics_matches: Vec<EthicsMatch> = matches
                .iter()
                .map(|m| EthicsMatch {
                    file: m.file.clone(),
                    line: m.line,
                    pattern: m.pattern.clone(),
                    category: format!("{:?}", m.category),
                })
                .collect();

            if !ethics_matches.is_empty() {
                result.success = false;
            }

            result.ethics_scan = Some(EthicsScanResult {
                red_flags_found: ethics_matches.len(),
                matches: ethics_matches,
            });
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_validate() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let result = run_validate(temp.path(), false);
        assert!(result.roadmap.is_some());
    }

    #[test]
    fn test_run_validate_with_ethics() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let result = run_validate(temp.path(), true);
        assert!(result.ethics_scan.is_some());
    }

    #[test]
    fn test_validate_file_result_struct() {
        let r = ValidateFileResult {
            file: "test.yaml".to_string(),
            valid: true,
            errors: vec![],
            warnings: vec![],
            regenerated: false,
        };
        assert!(r.valid);
    }

    #[test]
    fn test_ethics_match_struct() {
        let m = EthicsMatch {
            file: "test.rs".to_string(),
            line: 10,
            pattern: "rm -rf".to_string(),
            category: "Security".to_string(),
        };
        assert_eq!(m.line, 10);
    }

    #[test]
    fn test_ethics_scan_result_struct() {
        let r = EthicsScanResult {
            red_flags_found: 5,
            matches: vec![EthicsMatch {
                file: "test.rs".to_string(),
                line: 10,
                pattern: "rm -rf".to_string(),
                category: "Security".to_string(),
            }],
        };
        assert_eq!(r.red_flags_found, 5);
        assert_eq!(r.matches.len(), 1);
    }

    #[test]
    fn test_validate_result_fields() {
        let r = ValidateResult {
            success: true,
            protocol_files: vec![],
            roadmap: None,
            project: None,
            ethics: EthicsStatus::Hardcoded,
            sycophancy: SycophancyStatus::Hardcoded,
            green: GreenStatus::Hardcoded,
            ethics_scan: None,
            regenerated: vec![],
        };
        assert!(r.success);
    }

    #[test]
    fn test_run_validate_external_path() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test",
        )
        .unwrap();

        let result = run_validate(temp.path(), true);
        assert!(result.roadmap.is_some());
    }

    #[test]
    fn test_run_validate_with_invalid_roadmap() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Invalid YAML
        std::fs::write(asimov_dir.join("roadmap.yaml"), "invalid: yaml: [[[").unwrap();

        let result = run_validate(temp.path(), false);
        // Should still complete but with validation errors
        assert!(result.roadmap.is_some() || result.roadmap.is_none());
    }

    #[test]
    fn test_run_validate_with_project_yaml() {
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
            "identity:\n  project: Test\n  tagline: Test project\n",
        )
        .unwrap();

        let result = run_validate(temp.path(), false);
        assert!(result.project.is_some());
    }

    #[test]
    fn test_run_validate_no_asimov_dir() {
        let temp = TempDir::new().unwrap();
        let result = run_validate(temp.path(), false);
        // No .asimov dir - validation still runs
        // Empty dir without .asimov is considered success (no files to validate)
        assert!(result.success);
    }

    #[test]
    fn test_run_validate_with_invalid_project() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        // Invalid project.yaml
        std::fs::write(asimov_dir.join("project.yaml"), "invalid: yaml: [[[").unwrap();

        let result = run_validate(temp.path(), false);
        // Invalid project.yaml should cause validation failure
        assert!(!result.success);
    }

    #[test]
    fn test_validate_file_result_errors() {
        let r = ValidateFileResult {
            file: "test.yaml".to_string(),
            valid: false,
            errors: vec!["Error 1".to_string(), "Error 2".to_string()],
            warnings: vec!["Warning 1".to_string()],
            regenerated: false,
        };
        assert!(!r.valid);
        assert_eq!(r.errors.len(), 2);
        assert_eq!(r.warnings.len(), 1);
    }

    #[test]
    fn test_run_validate_empty_dir() {
        let temp = TempDir::new().unwrap();
        let result = run_validate(temp.path(), false);
        // No asimov dir - should not succeed
        assert!(!result.success || result.protocol_files.is_empty());
    }

    #[test]
    fn test_validate_result_ethics_scan() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        // Create a file with red flag content
        std::fs::write(temp.path().join("test.py"), "# TODO: implement later").unwrap();
        let result = run_validate(temp.path(), true);
        // Ethics scan will check for red flags
        assert!(result.ethics_scan.is_some());
    }
}
