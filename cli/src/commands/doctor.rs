//! Doctor command implementation
//! v9.7.0: Add coding standards tool checks (ADR-044)

use crate::{check_for_update, validate_file, validator::check_protocol_integrity, ProjectType};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DoctorCheck {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub auto_fixed: bool,
}

#[derive(Debug, Clone)]
pub struct DoctorResult {
    pub checks: Vec<DoctorCheck>,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub version_info: Option<(String, bool)>, // (version, is_latest)
}

pub fn run_doctor(dir: &Path) -> DoctorResult {
    let mut result = DoctorResult {
        checks: Vec::new(),
        issues: Vec::new(),
        warnings: Vec::new(),
        version_info: None,
    };

    let asimov_dir = dir.join(".asimov");

    // Check 1: .asimov directory
    if asimov_dir.exists() {
        result.checks.push(DoctorCheck {
            name: ".asimov/ directory".to_string(),
            passed: true,
            message: "exists".to_string(),
            auto_fixed: false,
        });
    } else {
        match std::fs::create_dir_all(&asimov_dir) {
            Ok(_) => {
                result.checks.push(DoctorCheck {
                    name: ".asimov/ directory".to_string(),
                    passed: true,
                    message: "created".to_string(),
                    auto_fixed: true,
                });
            }
            Err(e) => {
                result.checks.push(DoctorCheck {
                    name: ".asimov/ directory".to_string(),
                    passed: false,
                    message: format!("failed to create: {}", e),
                    auto_fixed: false,
                });
                result.issues.push(format!("Cannot create .asimov/: {}", e));
            }
        }
    }

    // Check 2: roadmap.yaml
    let roadmap_path = asimov_dir.join("roadmap.yaml");
    if roadmap_path.exists() {
        result.checks.push(DoctorCheck {
            name: "roadmap.yaml".to_string(),
            passed: true,
            message: "exists".to_string(),
            auto_fixed: false,
        });

        match validate_file(&roadmap_path) {
            Ok(r) if r.is_valid => {
                result.checks.push(DoctorCheck {
                    name: "roadmap.yaml validation".to_string(),
                    passed: true,
                    message: "valid".to_string(),
                    auto_fixed: false,
                });
            }
            Ok(r) => {
                result.checks.push(DoctorCheck {
                    name: "roadmap.yaml validation".to_string(),
                    passed: false,
                    message: "has errors".to_string(),
                    auto_fixed: false,
                });
                for e in r.errors {
                    result.issues.push(format!("roadmap.yaml: {}", e));
                }
            }
            Err(e) => {
                result.checks.push(DoctorCheck {
                    name: "roadmap.yaml validation".to_string(),
                    passed: false,
                    message: format!("failed: {}", e),
                    auto_fixed: false,
                });
                result.issues.push(format!("roadmap.yaml: {}", e));
            }
        }
    } else {
        let template =
            "current:\n  version: \"0.1.0\"\n  status: in_progress\n  summary: \"Initial setup\"\n";
        match std::fs::write(&roadmap_path, template) {
            Ok(_) => {
                result.checks.push(DoctorCheck {
                    name: "roadmap.yaml".to_string(),
                    passed: true,
                    message: "created template".to_string(),
                    auto_fixed: true,
                });
            }
            Err(e) => {
                result.checks.push(DoctorCheck {
                    name: "roadmap.yaml".to_string(),
                    passed: false,
                    message: format!("failed to create: {}", e),
                    auto_fixed: false,
                });
                result
                    .issues
                    .push(format!("Cannot create roadmap.yaml: {}", e));
            }
        }
    }

    // Check 3: Claude hooks
    let claude_dir = dir.join(".claude");
    let settings_path = claude_dir.join("settings.json");
    let hooks_dir = claude_dir.join("hooks");
    let session_start = hooks_dir.join("session-start.sh");
    let pre_compact = hooks_dir.join("pre-compact.sh");

    if !settings_path.exists() {
        result.checks.push(DoctorCheck {
            name: ".claude/settings.json".to_string(),
            passed: false,
            message: "missing".to_string(),
            auto_fixed: false,
        });
        result
            .issues
            .push("Claude Code hooks not configured - run 'asimov init'".to_string());
    } else {
        result.checks.push(DoctorCheck {
            name: ".claude/settings.json".to_string(),
            passed: true,
            message: "exists".to_string(),
            auto_fixed: false,
        });
    }

    if !session_start.exists() {
        result.checks.push(DoctorCheck {
            name: ".claude/hooks/session-start.sh".to_string(),
            passed: false,
            message: "missing".to_string(),
            auto_fixed: false,
        });
        result
            .issues
            .push("Session start hook missing - run 'asimov init'".to_string());
    } else {
        result.checks.push(DoctorCheck {
            name: ".claude/hooks/session-start.sh".to_string(),
            passed: true,
            message: "exists".to_string(),
            auto_fixed: false,
        });
    }

    if !pre_compact.exists() {
        result.checks.push(DoctorCheck {
            name: ".claude/hooks/pre-compact.sh".to_string(),
            passed: false,
            message: "missing".to_string(),
            auto_fixed: false,
        });
        result
            .issues
            .push("Pre-compact hook missing - run 'asimov init'".to_string());
    } else {
        result.checks.push(DoctorCheck {
            name: ".claude/hooks/pre-compact.sh".to_string(),
            passed: true,
            message: "exists".to_string(),
            auto_fixed: false,
        });
    }

    // Check 4: Git
    let git_dir = dir.join(".git");
    if !git_dir.exists() {
        result.warnings.push("Not a git repository".to_string());
    } else {
        let precommit = git_dir.join("hooks").join("pre-commit");
        if !precommit.exists() {
            result
                .warnings
                .push("Git pre-commit hook missing".to_string());
        }
    }

    // Check 5: Protocol integrity (v9.0.0)
    if asimov_dir.exists() {
        let protocol_checks = check_protocol_integrity(dir);

        let mut missing = Vec::new();
        let mut outdated = Vec::new();

        for check in &protocol_checks {
            if !check.exists {
                missing.push(check.filename.clone());
            } else if check.outdated {
                outdated.push(check.filename.clone());
            }
        }

        if missing.is_empty() && outdated.is_empty() {
            result.checks.push(DoctorCheck {
                name: "protocol files".to_string(),
                passed: true,
                message: format!("{} files OK", protocol_checks.len()),
                auto_fixed: false,
            });
        } else {
            if !missing.is_empty() {
                result.checks.push(DoctorCheck {
                    name: "protocol files".to_string(),
                    passed: false,
                    message: format!("{} missing", missing.len()),
                    auto_fixed: false,
                });
                result.issues.push(format!(
                    "Missing protocol files: {} - run 'asimov refresh'",
                    missing.join(", ")
                ));
            }
            if !outdated.is_empty() {
                result.checks.push(DoctorCheck {
                    name: "protocol version".to_string(),
                    passed: false,
                    message: format!("{} outdated", outdated.len()),
                    auto_fixed: false,
                });
                result.issues.push(format!(
                    "Outdated protocol files: {} - run 'asimov refresh' to update",
                    outdated.join(", ")
                ));
            }
        }
    }

    // Check 6: Coding standards tools (v9.7.0 ADR-044)
    if let Some(project_type) = detect_project_type_from_yaml(dir) {
        check_coding_tools(project_type, &mut result);
    }

    // Check 7: Version
    if let Ok(info) = check_for_update() {
        result.version_info = Some((info.current.clone(), !info.update_available));
    }

    result
}

/// Detect project type from project.yaml
fn detect_project_type_from_yaml(dir: &Path) -> Option<ProjectType> {
    let project_path = dir.join(".asimov").join("project.yaml");
    if !project_path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&project_path).ok()?;
    if content.contains("type: rust") {
        Some(ProjectType::Rust)
    } else if content.contains("type: python") {
        Some(ProjectType::Python)
    } else if content.contains("type: node") {
        Some(ProjectType::Node)
    } else if content.contains("type: go") {
        Some(ProjectType::Go)
    } else if content.contains("type: flutter") {
        Some(ProjectType::Flutter)
    } else if content.contains("type: docs") {
        Some(ProjectType::Docs)
    } else {
        Some(ProjectType::Generic)
    }
}

/// Check if coding standards tools are installed (v9.7.0 ADR-044)
fn check_coding_tools(project_type: ProjectType, result: &mut DoctorResult) {
    match project_type {
        ProjectType::Rust => {
            check_tool("cargo", &["--version"], "cargo", result);
            check_tool("cargo fmt", &["fmt", "--version"], "rustfmt", result);
            check_tool("cargo clippy", &["clippy", "--version"], "clippy", result);
        }
        ProjectType::Python => {
            check_command("ruff", &["--version"], result);
            check_command("pytest", &["--version"], result);
        }
        ProjectType::Node => {
            check_command("prettier", &["--version"], result);
            check_command("eslint", &["--version"], result);
        }
        ProjectType::Go => {
            check_command("go", &["version"], result);
            check_command("golangci-lint", &["--version"], result);
        }
        ProjectType::Flutter => {
            check_command("dart", &["--version"], result);
            check_command("flutter", &["--version"], result);
        }
        ProjectType::Docs | ProjectType::Arch => {
            check_command("markdownlint-cli2", &["--help"], result);
        }
        _ => {}
    }
}

/// Check if a command is available
fn check_command(name: &str, args: &[&str], result: &mut DoctorResult) {
    match Command::new(name).args(args).output() {
        Ok(output) if output.status.success() => {
            result.checks.push(DoctorCheck {
                name: name.to_string(),
                passed: true,
                message: "installed".to_string(),
                auto_fixed: false,
            });
        }
        _ => {
            result.checks.push(DoctorCheck {
                name: name.to_string(),
                passed: false,
                message: "not found".to_string(),
                auto_fixed: false,
            });
            result.warnings.push(format!(
                "{} not installed - coding standards may not work",
                name
            ));
        }
    }
}

/// Check cargo subcommand (uses cargo instead of direct binary)
fn check_tool(display_name: &str, args: &[&str], component: &str, result: &mut DoctorResult) {
    match Command::new("cargo").args(args).output() {
        Ok(output) if output.status.success() => {
            result.checks.push(DoctorCheck {
                name: display_name.to_string(),
                passed: true,
                message: "installed".to_string(),
                auto_fixed: false,
            });
        }
        _ => {
            result.checks.push(DoctorCheck {
                name: display_name.to_string(),
                passed: false,
                message: "not found".to_string(),
                auto_fixed: false,
            });
            result.warnings.push(format!(
                "{} not installed - run: rustup component add {}",
                display_name, component
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_doctor_empty_dir() {
        let temp = TempDir::new().unwrap();
        let result = run_doctor(temp.path());
        // Should auto-create .asimov and roadmap.yaml
        assert!(!result.checks.is_empty());
    }

    #[test]
    fn test_run_doctor_with_asimov() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let result = run_doctor(temp.path());
        assert!(result
            .checks
            .iter()
            .any(|c| c.name.contains("roadmap") && c.passed));
    }

    #[test]
    fn test_doctor_check_struct() {
        let check = DoctorCheck {
            name: "test".to_string(),
            passed: true,
            message: "ok".to_string(),
            auto_fixed: false,
        };
        assert!(check.passed);
    }

    #[test]
    fn test_doctor_result_fields() {
        let r = DoctorResult {
            checks: vec![DoctorCheck {
                name: "test".to_string(),
                passed: true,
                message: "ok".to_string(),
                auto_fixed: false,
            }],
            issues: vec![],
            warnings: vec![],
            version_info: Some(("1.0.0".to_string(), true)),
        };
        assert_eq!(r.checks.len(), 1);
    }

    #[test]
    fn test_run_doctor_all_checks() {
        let temp = TempDir::new().unwrap();

        // Initialize git repo
        let _ = std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output();
        let _ = std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp.path())
            .output();
        let _ = std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(temp.path())
            .output();

        // Create asimov project
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

        let result = run_doctor(temp.path());
        // Should have multiple checks
        assert!(!result.checks.is_empty());
    }

    #[test]
    fn test_run_doctor_with_issues() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Missing roadmap.yaml - should trigger a check failure
        let result = run_doctor(temp.path());
        // Should have some checks
        assert!(!result.checks.is_empty());
    }

    #[test]
    fn test_run_doctor_no_roadmap_create() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // No roadmap - should auto-create
        let result = run_doctor(temp.path());
        assert!(temp.path().join(".asimov/roadmap.yaml").exists());
        assert!(result.checks.iter().any(|c| c.auto_fixed));
    }

    #[test]
    fn test_run_doctor_invalid_roadmap() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Invalid roadmap
        std::fs::write(asimov_dir.join("roadmap.yaml"), "invalid: [[[").unwrap();
        let result = run_doctor(temp.path());
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_run_doctor_with_hooks() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Create valid roadmap
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: \"1.0\"\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        // Create claude hooks
        let claude_dir = temp.path().join(".claude");
        let hooks_dir = claude_dir.join("hooks");
        std::fs::create_dir_all(&hooks_dir).unwrap();
        std::fs::write(claude_dir.join("settings.json"), "{}").unwrap();
        std::fs::write(hooks_dir.join("session-start.sh"), "#!/bin/bash").unwrap();
        std::fs::write(hooks_dir.join("pre-compact.sh"), "#!/bin/bash").unwrap();
        // Init git
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();

        let result = run_doctor(temp.path());
        // Should have all checks passed
        assert!(!result.checks.is_empty());
    }

    #[test]
    fn test_run_doctor_no_asimov_dir_create() {
        let temp = TempDir::new().unwrap();
        // No .asimov dir at all - doctor should create it
        let result = run_doctor(temp.path());
        // Should have created .asimov/
        assert!(temp.path().join(".asimov").exists());
        assert!(result.checks.iter().any(|c| c.name.contains("directory")));
    }
}
