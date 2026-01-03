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
    pub license: Option<String>,              // v9.8.0: Detected project license
}

pub fn run_doctor(dir: &Path) -> DoctorResult {
    let mut result = DoctorResult {
        checks: Vec::new(),
        issues: Vec::new(),
        warnings: Vec::new(),
        version_info: None,
        license: None,
    };

    // v9.8.0: Detect project license (ADR-045)
    result.license = detect_license(dir);

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

    // v10.6.0: Removed .claude/ checks (ADR-060)
    // asimov warmup outputs all context directly - no Claude-specific hooks needed

    // Check 3: Git
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
        // Check 7: Dependency health audit tools (v9.8.0 ADR-045)
        check_audit_tools(project_type, &mut result);
    }

    // Check 8: Version
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

/// Detect project license from LICENSE file or package manifest (v9.8.0 ADR-045)
pub fn detect_license(dir: &Path) -> Option<String> {
    // Check LICENSE file variants
    for name in [
        "LICENSE",
        "LICENSE.md",
        "LICENSE.txt",
        "LICENCE",
        "LICENCE.md",
    ] {
        let path = dir.join(name);
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                let content_lower = content.to_lowercase();
                if content_lower.contains("mit license")
                    || content_lower.contains("permission is hereby granted, free of charge")
                {
                    return Some("MIT".to_string());
                }
                if content_lower.contains("apache license") && content_lower.contains("version 2.0")
                {
                    return Some("Apache-2.0".to_string());
                }
                if content_lower.contains("gnu general public license") {
                    if content_lower.contains("version 3") {
                        return Some("GPL-3.0".to_string());
                    }
                    return Some("GPL".to_string());
                }
                if content_lower.contains("gnu lesser general public") {
                    return Some("LGPL".to_string());
                }
                if content_lower.contains("bsd") {
                    if content_lower.contains("3-clause") || content_lower.contains("three clause")
                    {
                        return Some("BSD-3-Clause".to_string());
                    }
                    if content_lower.contains("2-clause") || content_lower.contains("two clause") {
                        return Some("BSD-2-Clause".to_string());
                    }
                    return Some("BSD".to_string());
                }
                if content_lower.contains("proprietary")
                    || content_lower.contains("all rights reserved")
                {
                    return Some("Proprietary".to_string());
                }
            }
        }
    }

    // Check Cargo.toml for Rust projects
    let cargo_path = dir.join("Cargo.toml");
    if cargo_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo_path) {
            for line in content.lines() {
                if line.starts_with("license") && line.contains('=') {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() >= 2 {
                        let license = parts[1].trim().trim_matches('"').trim_matches('\'');
                        return Some(license.to_string());
                    }
                }
            }
        }
    }

    // Check package.json for Node projects
    let pkg_path = dir.join("package.json");
    if pkg_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&pkg_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(license) = json.get("license").and_then(|l| l.as_str()) {
                    return Some(license.to_string());
                }
            }
        }
    }

    // Check pyproject.toml for Python projects
    let pyproject_path = dir.join("pyproject.toml");
    if pyproject_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&pyproject_path) {
            for line in content.lines() {
                if line.contains("license") && line.contains('=') {
                    let parts: Vec<&str> = line.split('=').collect();
                    if parts.len() >= 2 {
                        let license = parts[1].trim().trim_matches('"').trim_matches('\'');
                        if !license.is_empty() && !license.starts_with('{') {
                            return Some(license.to_string());
                        }
                    }
                }
            }
        }
    }

    None
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
            // Note: --help exits 2, --version exits 0
            check_command("markdownlint-cli2", &["--version"], result);
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

/// Check if dependency health audit tools are installed (v9.8.0 ADR-045)
fn check_audit_tools(project_type: ProjectType, result: &mut DoctorResult) {
    match project_type {
        ProjectType::Rust => {
            check_optional_command(
                "cargo-audit",
                &["audit", "--version"],
                "cargo install cargo-audit",
                result,
            );
            check_optional_command(
                "cargo-deny",
                &["deny", "--version"],
                "cargo install cargo-deny",
                result,
            );
            check_optional_command(
                "cargo-outdated",
                &["outdated", "--version"],
                "cargo install cargo-outdated",
                result,
            );
        }
        ProjectType::Python => {
            check_optional_command("pip-audit", &["--version"], "pip install pip-audit", result);
            check_optional_command(
                "pip-licenses",
                &["--version"],
                "pip install pip-licenses",
                result,
            );
        }
        ProjectType::Node => {
            check_optional_command(
                "license-checker",
                &["--version"],
                "npm install -g license-checker",
                result,
            );
        }
        ProjectType::Go => {
            check_optional_command(
                "govulncheck",
                &["--help"],
                "go install golang.org/x/vuln/cmd/govulncheck@latest",
                result,
            );
        }
        _ => {}
    }
}

/// Check if an optional audit command is available
fn check_optional_command(name: &str, args: &[&str], install_cmd: &str, result: &mut DoctorResult) {
    // For cargo subcommands, use cargo as the base command
    let (cmd, actual_args): (&str, Vec<&str>) = if name.starts_with("cargo-") {
        ("cargo", args.to_vec())
    } else {
        (name, args.to_vec())
    };

    match Command::new(cmd).args(&actual_args).output() {
        Ok(output) if output.status.success() => {
            result.checks.push(DoctorCheck {
                name: format!("{} (audit)", name),
                passed: true,
                message: "installed".to_string(),
                auto_fixed: false,
            });
        }
        _ => {
            result.checks.push(DoctorCheck {
                name: format!("{} (audit)", name),
                passed: false,
                message: "not found".to_string(),
                auto_fixed: false,
            });
            result.warnings.push(format!(
                "{} not installed - dependency health checks will be skipped. Install: {}",
                name, install_cmd
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
            license: Some("MIT".to_string()), // v9.8.0
        };
        assert_eq!(r.checks.len(), 1);
        assert_eq!(r.license, Some("MIT".to_string()));
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

    // v9.8.0: License detection tests (ADR-045)
    #[test]
    fn test_detect_license_mit() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("LICENSE"),
            "MIT License\n\nCopyright (c) 2025\n\nPermission is hereby granted...",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("MIT".to_string()));
    }

    #[test]
    fn test_detect_license_apache() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("LICENSE"),
            "Apache License\nVersion 2.0, January 2004\n...",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("Apache-2.0".to_string()));
    }

    #[test]
    fn test_detect_license_gpl3() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("LICENSE"),
            "GNU General Public License\nVersion 3, 29 June 2007\n...",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("GPL-3.0".to_string()));
    }

    #[test]
    fn test_detect_license_bsd3() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("LICENSE"),
            "BSD 3-Clause License\n\nCopyright...",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("BSD-3-Clause".to_string()));
    }

    #[test]
    fn test_detect_license_proprietary() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("LICENSE"),
            "Proprietary License\nAll Rights Reserved.\n...",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("Proprietary".to_string()));
    }

    #[test]
    fn test_detect_license_from_cargo_toml() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\nlicense = \"MIT\"\n",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("MIT".to_string()));
    }

    #[test]
    fn test_detect_license_from_package_json() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("package.json"),
            r#"{"name": "test", "version": "1.0.0", "license": "ISC"}"#,
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("ISC".to_string()));
    }

    #[test]
    fn test_detect_license_from_pyproject() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("pyproject.toml"),
            "[project]\nname = \"test\"\nversion = \"0.1.0\"\nlicense = \"Apache-2.0\"\n",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("Apache-2.0".to_string()));
    }

    #[test]
    fn test_detect_license_none() {
        let temp = TempDir::new().unwrap();
        // No license files at all
        let license = detect_license(temp.path());
        assert_eq!(license, None);
    }

    #[test]
    fn test_detect_license_priority() {
        let temp = TempDir::new().unwrap();
        // LICENSE file takes priority over Cargo.toml
        std::fs::write(
            temp.path().join("LICENSE"),
            "MIT License\n\nPermission is hereby granted...",
        )
        .unwrap();
        std::fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nlicense = \"Apache-2.0\"\n",
        )
        .unwrap();
        let license = detect_license(temp.path());
        assert_eq!(license, Some("MIT".to_string()));
    }

    #[test]
    fn test_run_doctor_with_license() {
        let temp = TempDir::new().unwrap();
        std::fs::write(
            temp.path().join("LICENSE"),
            "MIT License\nPermission is hereby granted...",
        )
        .unwrap();
        let result = run_doctor(temp.path());
        assert_eq!(result.license, Some("MIT".to_string()));
    }
}
