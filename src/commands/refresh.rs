//! Refresh command implementation
//! v9.5.0: Enhanced with project migration assistant (ADR-042)
//! v9.6.0: Always regenerate pre-commit hooks (ADR-043)

use crate::templates::{
    detect_project_type, precommit_hook_template, project_template, ProjectType,
};
use crate::{validate_directory_with_regeneration, validator::regenerate_protocol_files};
use std::io::{self, BufRead, Write};
use std::path::Path;

// Unix-specific imports for setting executable permissions
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Options for refresh command
#[derive(Debug, Clone, Default)]
pub struct RefreshOptions {
    /// Auto-accept template defaults without prompting
    pub yes: bool,
    /// Show what would change without writing
    pub dry_run: bool,
}

/// User choice for upgrade prompts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpgradeChoice {
    Accept,
    Keep,
}

#[derive(Debug, Clone)]
pub struct RefreshResult {
    pub success: bool,
    pub is_asimov_project: bool,
    pub files_regenerated: Vec<String>,
    pub files_unchanged: Vec<String>,
    pub protocols_updated: Vec<String>, // v9.0.0: outdated protocols that were updated
    pub protocols_created: Vec<String>, // v9.0.0: missing protocols that were created
    pub protocols_ok: Vec<String>,      // v9.0.0: protocols that matched expected
    // v9.5.0: Migration assistant fields
    pub project_type_detected: Option<ProjectType>,
    pub project_type_was_missing: bool,
    pub coding_standards_upgraded: bool,
    // v9.6.0: Hook regeneration (ADR-043)
    pub hook_regenerated: bool,
    pub dry_run: bool,
    pub error: Option<String>,
}

/// Run refresh with default options (backwards compatible)
pub fn run_refresh(dir: &Path) -> RefreshResult {
    run_refresh_with_options(dir, RefreshOptions::default())
}

/// Run refresh with options (v9.5.0, v9.6.0)
pub fn run_refresh_with_options(dir: &Path, options: RefreshOptions) -> RefreshResult {
    let mut result = RefreshResult {
        success: false,
        is_asimov_project: false,
        files_regenerated: Vec::new(),
        files_unchanged: Vec::new(),
        protocols_updated: Vec::new(),
        protocols_created: Vec::new(),
        protocols_ok: Vec::new(),
        project_type_detected: None,
        project_type_was_missing: false,
        coding_standards_upgraded: false,
        hook_regenerated: false,
        dry_run: options.dry_run,
        error: None,
    };

    if !dir.join(".asimov").is_dir() {
        result.error = Some("Not in an asimov project (.asimov/ not found)".to_string());
        return result;
    }
    result.is_asimov_project = true;

    // v9.0.0: Check and regenerate protocol JSON files
    if !options.dry_run {
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
    }

    // v9.5.0: Check project.yaml for migration opportunities
    let project_yaml_path = dir.join(".asimov").join("project.yaml");
    if project_yaml_path.exists() {
        match check_project_migration(dir, &project_yaml_path, &options) {
            Ok(migration) => {
                result.project_type_detected = Some(migration.project_type);
                result.project_type_was_missing = migration.type_was_missing;
                result.coding_standards_upgraded = migration.coding_standards_upgraded;

                // Apply migrations if not dry run
                if !options.dry_run
                    && (migration.type_was_missing || migration.coding_standards_upgraded)
                {
                    if let Err(e) = apply_project_migration(dir, &project_yaml_path, &migration) {
                        result.error = Some(format!("Migration failed: {}", e));
                        return result;
                    }
                    result.files_regenerated.push("project.yaml".to_string());
                }
            }
            Err(e) => {
                // Non-fatal: project.yaml may be valid but not migrateable
                result.error = Some(format!("Project migration check: {}", e));
            }
        }
    } else {
        // No project.yaml - detect type and offer to create
        let detected_type = detect_project_type(dir);
        result.project_type_detected = Some(detected_type);
        result.project_type_was_missing = true;

        if !options.dry_run {
            let project_type = if options.yes {
                detected_type
            } else {
                prompt_project_type(detected_type)
            };

            // Generate project.yaml
            let template = project_template("my-project", "Project description", project_type);
            if let Err(e) = std::fs::write(&project_yaml_path, template) {
                result.error = Some(format!("Failed to create project.yaml: {}", e));
                return result;
            }
            result.protocols_created.push("project.yaml".to_string());
            result.project_type_detected = Some(project_type);
        }
    }

    // v9.6.0: Always regenerate pre-commit hook (ADR-043 - No SPOF)
    if !options.dry_run {
        if let Some(project_type) = result.project_type_detected {
            if let Err(e) = regenerate_precommit_hook(dir, project_type) {
                // Non-fatal: git might not be initialized
                eprintln!("Note: Could not regenerate pre-commit hook: {}", e);
            } else {
                result.hook_regenerated = true;
            }
        }
    }

    // Also check roadmap.yaml etc.
    if !options.dry_run {
        match validate_directory_with_regeneration(dir, true) {
            Ok((_, regen_info)) => {
                for (file, changed) in regen_info.regenerated {
                    if changed {
                        if !result.files_regenerated.contains(&file) {
                            result.files_regenerated.push(file);
                        }
                    } else if !result.files_unchanged.contains(&file) {
                        result.files_unchanged.push(file);
                    }
                }
                result.success = true;
            }
            Err(e) => {
                result.error = Some(format!("Regeneration failed: {}", e));
            }
        }
    } else {
        result.success = true;
    }

    result
}

/// Migration information for a project
#[derive(Debug, Clone)]
struct ProjectMigration {
    project_type: ProjectType,
    type_was_missing: bool,
    coding_standards_upgraded: bool,
    new_content: Option<String>,
}

/// Check if project.yaml needs migration
fn check_project_migration(
    dir: &Path,
    project_yaml_path: &Path,
    options: &RefreshOptions,
) -> Result<ProjectMigration, String> {
    let content = std::fs::read_to_string(project_yaml_path)
        .map_err(|e| format!("Failed to read project.yaml: {}", e))?;

    let yaml: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content)
        .map_err(|e| format!("Invalid YAML in project.yaml: {}", e))?;

    // Check identity.type
    let type_str = yaml
        .get("identity")
        .and_then(|i| i.get("type"))
        .and_then(|t| t.as_str());

    let type_was_missing = type_str.is_none();
    let project_type = if let Some(t) = type_str {
        t.parse::<ProjectType>()
            .unwrap_or_else(|_| detect_project_type(dir))
    } else {
        let detected = detect_project_type(dir);
        if options.yes {
            detected
        } else {
            prompt_project_type(detected)
        }
    };

    // Check coding_standards
    let has_coding_standards = yaml.get("coding_standards").is_some();
    let has_code_section = yaml
        .get("coding_standards")
        .and_then(|cs| cs.get("code"))
        .is_some();
    let has_documentation_section = yaml
        .get("coding_standards")
        .and_then(|cs| cs.get("documentation"))
        .is_some();
    let has_architecture_section = yaml
        .get("coding_standards")
        .and_then(|cs| cs.get("architecture"))
        .is_some();

    // Need upgrade if: has coding_standards but missing new sections (for programming types)
    let is_programming_type = matches!(
        project_type,
        ProjectType::Rust
            | ProjectType::Python
            | ProjectType::Node
            | ProjectType::Go
            | ProjectType::Flutter
    );

    let needs_coding_upgrade = is_programming_type
        && has_coding_standards
        && (!has_code_section || !has_documentation_section || !has_architecture_section);

    let coding_standards_upgraded = if needs_coding_upgrade {
        if options.yes {
            true
        } else {
            prompt_coding_standards_upgrade() == UpgradeChoice::Accept
        }
    } else {
        false
    };

    // Generate new content if needed
    let new_content = if type_was_missing || coding_standards_upgraded {
        // Preserve user values while upgrading structure
        let name = yaml
            .get("identity")
            .and_then(|i| i.get("name"))
            .and_then(|n| n.as_str())
            .unwrap_or("my-project");
        let tagline = yaml
            .get("identity")
            .and_then(|i| i.get("tagline"))
            .and_then(|t| t.as_str())
            .unwrap_or("Project description");

        Some(project_template(name, tagline, project_type))
    } else {
        None
    };

    Ok(ProjectMigration {
        project_type,
        type_was_missing,
        coding_standards_upgraded,
        new_content,
    })
}

/// Apply migration to project.yaml
fn apply_project_migration(
    _dir: &Path,
    project_yaml_path: &Path,
    migration: &ProjectMigration,
) -> Result<(), String> {
    if let Some(ref new_content) = migration.new_content {
        std::fs::write(project_yaml_path, new_content)
            .map_err(|e| format!("Failed to write project.yaml: {}", e))?;
    }
    Ok(())
}

/// Regenerate pre-commit hook for direct coding standards enforcement (v9.6.0 ADR-043)
fn regenerate_precommit_hook(dir: &Path, project_type: ProjectType) -> Result<(), String> {
    let git_dir = dir.join(".git");
    if !git_dir.is_dir() {
        return Err("Not a git repository".to_string());
    }

    let hooks_dir = git_dir.join("hooks");
    std::fs::create_dir_all(&hooks_dir)
        .map_err(|e| format!("Failed to create hooks directory: {}", e))?;

    let hook_path = hooks_dir.join("pre-commit");
    let hook_content = precommit_hook_template(project_type);

    std::fs::write(&hook_path, hook_content)
        .map_err(|e| format!("Failed to write pre-commit hook: {}", e))?;

    // Make executable (Unix only - Windows doesn't need this)
    #[cfg(unix)]
    {
        let mut perms = std::fs::metadata(&hook_path)
            .map_err(|e| format!("Failed to get hook metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&hook_path, perms)
            .map_err(|e| format!("Failed to set hook permissions: {}", e))?;
    }

    Ok(())
}

/// Prompt user for project type selection
fn prompt_project_type(detected: ProjectType) -> ProjectType {
    let types = [
        ("rust", ProjectType::Rust),
        ("python", ProjectType::Python),
        ("node", ProjectType::Node),
        ("go", ProjectType::Go),
        ("flutter", ProjectType::Flutter),
        ("docs", ProjectType::Docs),
        ("arch", ProjectType::Arch),
        ("generic", ProjectType::Generic),
    ];

    eprintln!();
    eprintln!("Project type not specified. Please select:");
    for (i, (name, _)) in types.iter().enumerate() {
        let marker = if types[i].1 == detected {
            " (detected)"
        } else {
            ""
        };
        eprintln!("  {}. {}{}", i + 1, name, marker);
    }
    eprint!("Enter number [1-8] (default: {}): ", detected);
    io::stderr().flush().ok();

    let stdin = io::stdin();
    let line = stdin.lock().lines().next();

    if let Some(Ok(input)) = line {
        let input = input.trim();
        if input.is_empty() {
            return detected;
        }
        if let Ok(n) = input.parse::<usize>() {
            if n >= 1 && n <= types.len() {
                return types[n - 1].1;
            }
        }
    }

    detected
}

/// Prompt user for coding_standards upgrade
fn prompt_coding_standards_upgrade() -> UpgradeChoice {
    eprintln!();
    eprintln!("coding_standards section can be upgraded to latest template format.");
    eprintln!("This adds documentation and architecture sections.");
    eprint!("[A]ccept template / [K]eep current (default: A): ");
    io::stderr().flush().ok();

    let stdin = io::stdin();
    let line = stdin.lock().lines().next();

    if let Some(Ok(input)) = line {
        let input = input.trim().to_lowercase();
        if input == "k" || input == "keep" {
            return UpgradeChoice::Keep;
        }
    }

    UpgradeChoice::Accept
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
            project_type_detected: Some(ProjectType::Rust),
            project_type_was_missing: false,
            coding_standards_upgraded: false,
            hook_regenerated: true,
            dry_run: false,
            error: None,
        };
        assert!(r.success);
        assert!(!r.files_regenerated.is_empty());
        assert!(!r.protocols_updated.is_empty());
        assert!(r.project_type_detected.is_some());
        assert!(r.hook_regenerated);
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

    #[test]
    fn test_run_refresh_with_options_yes() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.is_asimov_project);
        assert!(result.success);
    }

    #[test]
    fn test_run_refresh_with_options_dry_run() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let options = RefreshOptions {
            yes: false,
            dry_run: true,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.is_asimov_project);
        assert!(result.success);
        assert!(result.dry_run);
    }

    #[test]
    fn test_run_refresh_creates_project_yaml_with_yes() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        // Add Cargo.toml to detect as Rust
        std::fs::write(temp.path().join("Cargo.toml"), "[package]").unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert!(result.project_type_was_missing);
        assert_eq!(result.project_type_detected, Some(ProjectType::Rust));
        assert!(asimov_dir.join("project.yaml").exists());
    }

    #[test]
    fn test_run_refresh_with_existing_project_yaml() {
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
            "identity:\n  name: test\n  type: rust\n  tagline: Test\n",
        )
        .unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert!(!result.project_type_was_missing);
        assert_eq!(result.project_type_detected, Some(ProjectType::Rust));
    }

    #[test]
    fn test_run_refresh_upgrades_coding_standards() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        // Old-style coding_standards (no code/documentation/architecture sections)
        std::fs::write(
            asimov_dir.join("project.yaml"),
            r#"identity:
  name: test
  type: rust
  tagline: Test
coding_standards:
  file_size:
    soft_limit: 1000
  coverage: "100%"
"#,
        )
        .unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert!(result.coding_standards_upgraded);
    }

    #[test]
    fn test_upgrade_choice_values() {
        assert_eq!(UpgradeChoice::Accept, UpgradeChoice::Accept);
        assert_ne!(UpgradeChoice::Accept, UpgradeChoice::Keep);
    }

    #[test]
    fn test_refresh_options_default() {
        let opts = RefreshOptions::default();
        assert!(!opts.yes);
        assert!(!opts.dry_run);
    }

    #[test]
    fn test_refresh_detects_python_type() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        std::fs::write(temp.path().join("pyproject.toml"), "[tool.poetry]").unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert_eq!(result.project_type_detected, Some(ProjectType::Python));
    }

    #[test]
    fn test_refresh_detects_node_type() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        std::fs::write(temp.path().join("package.json"), "{}").unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert_eq!(result.project_type_detected, Some(ProjectType::Node));
    }

    #[test]
    fn test_refresh_regenerates_hook() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        let git_dir = temp.path().join(".git");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::create_dir_all(&git_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        // Add Cargo.toml to detect as Rust
        std::fs::write(temp.path().join("Cargo.toml"), "[package]").unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert!(result.hook_regenerated);

        // Verify hook file exists and has correct content
        let hook_path = git_dir.join("hooks/pre-commit");
        assert!(hook_path.exists());
        let content = std::fs::read_to_string(hook_path).unwrap();
        assert!(content.contains("cargo fmt"));
        assert!(content.contains("asimov refresh || true")); // Soft-fail
    }

    #[test]
    fn test_refresh_hook_not_regenerated_without_git() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        // No .git directory
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();
        std::fs::write(temp.path().join("Cargo.toml"), "[package]").unwrap();

        let options = RefreshOptions {
            yes: true,
            dry_run: false,
        };
        let result = run_refresh_with_options(temp.path(), options);
        assert!(result.success);
        assert!(!result.hook_regenerated); // No git, no hook
    }
}
