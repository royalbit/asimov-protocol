//! Init command implementation
//! v9.7.0: Add dev dependencies for coding standards tools (ADR-044)

use crate::{
    get_template_by_name, git_precommit_hook, protocols::PROTOCOL_FILES, roadmap_template,
    ProjectType,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct InitResult {
    pub success: bool,
    pub project_type: Option<ProjectType>,
    pub files_created: Vec<String>,
    pub files_updated: Vec<String>,
    pub files_kept: Vec<String>,
    pub hooks_installed: Vec<String>,
    pub deps_added: Vec<String>, // v9.7.0: Dev dependencies added
    pub install_instructions: Vec<String>, // v9.7.0: Manual install instructions
    pub error: Option<String>,
}

pub fn run_init(dir: &Path, name: &str, type_str: &str, force: bool) -> InitResult {
    let mut result = InitResult {
        success: false,
        project_type: None,
        files_created: Vec::new(),
        files_updated: Vec::new(),
        files_kept: Vec::new(),
        hooks_installed: Vec::new(),
        deps_added: Vec::new(),
        install_instructions: Vec::new(),
        error: None,
    };

    // v10.3.1: Support all 21 templates, map to base ProjectType for hooks/deps
    let template_name = type_str.to_lowercase();
    let project_type = match template_name.as_str() {
        // Base templates
        "rust" | "rs" => ProjectType::Rust,
        "python" | "py" => ProjectType::Python,
        "node" | "nodejs" | "javascript" | "js" | "typescript" | "ts" => ProjectType::Node,
        "go" | "golang" => ProjectType::Go,
        "flutter" | "dart" => ProjectType::Flutter,
        "docs" | "documentation" => ProjectType::Docs,
        "arch" | "architecture" => ProjectType::Arch,
        "generic" => ProjectType::Generic,
        // Extended templates map to their base types
        "api-rust" => ProjectType::Rust,
        "api-go" => ProjectType::Go,
        "api-fastapi" => ProjectType::Python,
        "api-nestjs" => ProjectType::Node,
        "api-spring" => ProjectType::Generic, // Java not yet a ProjectType
        "web-nextjs" | "web-react" | "web-vue" | "web-angular" => ProjectType::Node,
        "mono-turbo" | "mono-nx" | "mono-pnpm" => ProjectType::Node,
        "admin-dashboard" => ProjectType::Node,
        other => {
            result.error = Some(format!(
                "Unknown template: '{}'. Use 'asimov init --help' to see all templates",
                other
            ));
            return result;
        }
    };
    result.project_type = Some(project_type);

    // Normalize template name for lookup
    let template_key = match template_name.as_str() {
        "rs" => "rust",
        "py" => "python",
        "nodejs" | "javascript" | "js" | "typescript" | "ts" => "node",
        "golang" => "go",
        "dart" => "flutter",
        "documentation" => "docs",
        "architecture" => "arch",
        other => other,
    };

    let asimov_dir = dir.join(".asimov");
    if let Err(e) = std::fs::create_dir_all(&asimov_dir) {
        result.error = Some(format!("Failed to create .asimov/: {}", e));
        return result;
    }

    // Create roadmap.yaml
    let roadmap_path = asimov_dir.join("roadmap.yaml");
    let roadmap_existed = roadmap_path.exists();
    if !roadmap_existed || force {
        let content = roadmap_template();
        if let Err(e) = std::fs::write(&roadmap_path, content) {
            result.error = Some(format!("Failed to write roadmap.yaml: {}", e));
            return result;
        }
        if roadmap_existed {
            result.files_updated.push("roadmap.yaml".to_string());
        } else {
            result.files_created.push("roadmap.yaml".to_string());
        }
    } else {
        result.files_kept.push("roadmap.yaml".to_string());
    }

    // Create project.yaml
    let project_path = asimov_dir.join("project.yaml");
    let project_existed = project_path.exists();
    if !project_existed || force {
        // v10.3.1: Use unified template lookup for all 21 templates
        let template = get_template_by_name(template_key).unwrap_or_else(|| {
            // Fallback to generic if template not found (shouldn't happen)
            get_template_by_name("generic").unwrap()
        });
        let content = template
            .replace("{PROJECT_NAME}", name)
            .replace("{PROJECT_TAGLINE}", "Your project tagline");
        if let Err(e) = std::fs::write(&project_path, content) {
            result.error = Some(format!("Failed to write project.yaml: {}", e));
            return result;
        }
        if project_existed {
            result.files_updated.push("project.yaml".to_string());
        } else {
            result.files_created.push("project.yaml".to_string());
        }
    } else {
        result.files_kept.push("project.yaml".to_string());
    }

    // v9.0.0: Create protocol JSON files
    for (filename, generator) in PROTOCOL_FILES {
        let file_path = asimov_dir.join(filename);
        let existed = file_path.exists();
        if !existed || force {
            let content = generator();
            if let Err(e) = std::fs::write(&file_path, &content) {
                result.error = Some(format!("Failed to write {}: {}", filename, e));
                return result;
            }
            if existed {
                result.files_updated.push(filename.to_string());
            } else {
                result.files_created.push(filename.to_string());
            }
        } else {
            result.files_kept.push(filename.to_string());
        }
    }

    // Update or create .gitignore
    let gitignore_path = dir.join(".gitignore");
    let gitignore_entry = ".claude_checkpoint.yaml";
    if gitignore_path.exists() {
        if let Ok(content) = std::fs::read_to_string(&gitignore_path) {
            if !content.contains(gitignore_entry) {
                let new_content = format!("{}\n{}\n", content.trim_end(), gitignore_entry);
                let _ = std::fs::write(&gitignore_path, new_content);
                result.files_updated.push(".gitignore".to_string());
            }
        }
    } else {
        let _ = std::fs::write(&gitignore_path, format!("{}\n", gitignore_entry));
        result.files_created.push(".gitignore".to_string());
    }

    // v10.6.0: Removed .claude/ directory creation (ADR-060)
    // Users who want Claude-specific hooks can create them manually
    // asimov warmup outputs all context directly - no hooks needed

    // Install git pre-commit hook if in git repo
    let git_hooks_dir = dir.join(".git").join("hooks");
    if git_hooks_dir.exists() {
        let precommit_path = git_hooks_dir.join("pre-commit");
        if !precommit_path.exists() || force {
            if let Err(e) = std::fs::write(&precommit_path, git_precommit_hook()) {
                // Non-fatal
                let _ = e;
            } else {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(
                        &precommit_path,
                        std::fs::Permissions::from_mode(0o755),
                    );
                }
                result.hooks_installed.push("git pre-commit".to_string());
            }
        }
    }

    // v9.7.0: Setup dev dependencies for coding standards (ADR-044)
    setup_dev_dependencies(dir, project_type, &mut result);

    result.success = true;
    result
}

/// Setup dev dependencies for coding standards tools (v9.7.0 ADR-044)
fn setup_dev_dependencies(dir: &Path, project_type: ProjectType, result: &mut InitResult) {
    match project_type {
        ProjectType::Rust => setup_rust_deps(dir, result),
        ProjectType::Python => setup_python_deps(dir, result),
        ProjectType::Node => setup_node_deps(dir, result),
        ProjectType::Go => setup_go_deps(result),
        ProjectType::Flutter => setup_flutter_deps(result),
        ProjectType::Docs | ProjectType::Arch | ProjectType::Generic => setup_docs_deps(result),
        ProjectType::Migration => {} // No deps needed
    }
}

/// Add cargo-husky to Cargo.toml [dev-dependencies]
fn setup_rust_deps(dir: &Path, result: &mut InitResult) {
    let cargo_toml = dir.join("Cargo.toml");
    if !cargo_toml.exists() {
        return;
    }

    if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
        // Check if cargo-husky already present
        if content.contains("cargo-husky") {
            return;
        }

        // Add cargo-husky to [dev-dependencies]
        let husky_dep =
            "cargo-husky = { version = \"1\", default-features = false, features = [\"user-hooks\"] }";

        let new_content = if content.contains("[dev-dependencies]") {
            // Add after existing [dev-dependencies]
            content.replace(
                "[dev-dependencies]",
                &format!("[dev-dependencies]\n{}", husky_dep),
            )
        } else {
            // Add new section at end
            format!(
                "{}\n\n[dev-dependencies]\n{}\n",
                content.trim_end(),
                husky_dep
            )
        };

        if std::fs::write(&cargo_toml, new_content).is_ok() {
            result
                .deps_added
                .push("cargo-husky (Cargo.toml)".to_string());
        }
    }
}

/// Add ruff config to pyproject.toml
fn setup_python_deps(dir: &Path, result: &mut InitResult) {
    let pyproject = dir.join("pyproject.toml");

    let ruff_config = r#"
[tool.ruff]
line-length = 100
target-version = "py311"

[tool.ruff.lint]
select = ["E", "F", "I", "W"]
"#;

    if pyproject.exists() {
        if let Ok(content) = std::fs::read_to_string(&pyproject) {
            if content.contains("[tool.ruff]") {
                return; // Already configured
            }
            let new_content = format!("{}\n{}", content.trim_end(), ruff_config);
            if std::fs::write(&pyproject, new_content).is_ok() {
                result
                    .deps_added
                    .push("[tool.ruff] (pyproject.toml)".to_string());
            }
        }
    } else {
        // Create new pyproject.toml with ruff config
        let content = format!(
            "[project]\nname = \"\"\nversion = \"0.1.0\"\n{}",
            ruff_config
        );
        if std::fs::write(&pyproject, content).is_ok() {
            result
                .deps_added
                .push("pyproject.toml with [tool.ruff]".to_string());
        }
    }

    result
        .install_instructions
        .push("pip install ruff pytest".to_string());
}

/// Add prettier/eslint to package.json devDependencies
fn setup_node_deps(dir: &Path, result: &mut InitResult) {
    let package_json = dir.join("package.json");

    if package_json.exists() {
        if let Ok(content) = std::fs::read_to_string(&package_json) {
            if content.contains("prettier") && content.contains("eslint") {
                return; // Already configured
            }

            // Try to parse and modify JSON
            if let Ok(mut json) = serde_json::from_str::<serde_json::Value>(&content) {
                let dev_deps = json.as_object_mut().and_then(|obj| {
                    obj.entry("devDependencies")
                        .or_insert_with(|| serde_json::json!({}))
                        .as_object_mut()
                });

                if let Some(deps) = dev_deps {
                    let mut added = false;
                    if !deps.contains_key("prettier") {
                        deps.insert("prettier".to_string(), serde_json::json!("^3.0.0"));
                        added = true;
                    }
                    if !deps.contains_key("eslint") {
                        deps.insert("eslint".to_string(), serde_json::json!("^8.0.0"));
                        added = true;
                    }
                    if added {
                        if let Ok(new_content) = serde_json::to_string_pretty(&json) {
                            if std::fs::write(&package_json, new_content).is_ok() {
                                result
                                    .deps_added
                                    .push("prettier, eslint (package.json)".to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    result.install_instructions.push("npm install".to_string());
}

/// Print install instructions for Go (golangci-lint is GPL)
fn setup_go_deps(result: &mut InitResult) {
    result
        .install_instructions
        .push("go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest".to_string());
    result
        .install_instructions
        .push("Note: golangci-lint is GPL-3.0 licensed (tool use is fine)".to_string());
}

/// Print install instructions for Flutter
fn setup_flutter_deps(result: &mut InitResult) {
    result
        .install_instructions
        .push("dart pub add --dev test".to_string());
}

/// Print install instructions for docs projects
fn setup_docs_deps(result: &mut InitResult) {
    result
        .install_instructions
        .push("npm install -g markdownlint-cli2".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_init() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "TestProject", "rust", false);
        assert!(result.success);
        assert!(result.files_created.contains(&"roadmap.yaml".to_string()));
        assert!(result.files_created.contains(&"project.yaml".to_string()));
    }

    #[test]
    fn test_run_init_all_types() {
        for t in &["rust", "python", "node", "go", "flutter", "docs", "generic"] {
            let temp = TempDir::new().unwrap();
            let result = run_init(temp.path(), "Test", t, false);
            assert!(result.success, "Failed for type: {}", t);
        }
    }

    #[test]
    fn test_run_init_force() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(asimov_dir.join("roadmap.yaml"), "old: data").unwrap();

        let result = run_init(temp.path(), "New", "rust", true);
        assert!(result.success);
    }

    #[test]
    fn test_init_result_fields() {
        let r = InitResult {
            success: true,
            project_type: None,
            files_created: vec!["roadmap.yaml".to_string()],
            files_updated: vec![],
            files_kept: vec![],
            hooks_installed: vec!["pre-commit".to_string()],
            deps_added: vec!["cargo-husky".to_string()],
            install_instructions: vec![],
            error: None,
        };
        assert!(r.success);
        assert!(!r.deps_added.is_empty());
    }

    #[test]
    fn test_init_rust_adds_cargo_husky() {
        let temp = TempDir::new().unwrap();
        // Create Cargo.toml
        std::fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname = \"test\"\nversion = \"0.1.0\"\n",
        )
        .unwrap();

        let result = run_init(temp.path(), "Test", "rust", false);
        assert!(result.success);
        assert!(result.deps_added.iter().any(|d| d.contains("cargo-husky")));

        // Verify Cargo.toml was updated
        let content = std::fs::read_to_string(temp.path().join("Cargo.toml")).unwrap();
        assert!(content.contains("cargo-husky"));
    }

    #[test]
    fn test_init_python_creates_pyproject() {
        let temp = TempDir::new().unwrap();

        let result = run_init(temp.path(), "Test", "python", false);
        assert!(result.success);
        assert!(result
            .install_instructions
            .iter()
            .any(|i| i.contains("ruff")));

        // Verify pyproject.toml was created
        assert!(temp.path().join("pyproject.toml").exists());
        let content = std::fs::read_to_string(temp.path().join("pyproject.toml")).unwrap();
        assert!(content.contains("[tool.ruff]"));
    }

    #[test]
    fn test_init_node_adds_devdeps() {
        let temp = TempDir::new().unwrap();
        // Create package.json
        std::fs::write(
            temp.path().join("package.json"),
            r#"{"name": "test", "version": "1.0.0"}"#,
        )
        .unwrap();

        let result = run_init(temp.path(), "Test", "node", false);
        assert!(result.success);

        // Verify package.json was updated
        let content = std::fs::read_to_string(temp.path().join("package.json")).unwrap();
        assert!(content.contains("prettier"));
        assert!(content.contains("eslint"));
    }

    #[test]
    fn test_init_go_has_instructions() {
        let temp = TempDir::new().unwrap();

        let result = run_init(temp.path(), "Test", "go", false);
        assert!(result.success);
        assert!(result
            .install_instructions
            .iter()
            .any(|i| i.contains("golangci-lint")));
    }

    #[test]
    fn test_run_init_with_existing_gitignore() {
        let temp = TempDir::new().unwrap();
        // Create existing .gitignore
        std::fs::write(temp.path().join(".gitignore"), "*.log\n").unwrap();

        let result = run_init(temp.path(), "Test", "rust", false);
        assert!(result.success);

        // Verify .gitignore was updated
        let gitignore = std::fs::read_to_string(temp.path().join(".gitignore")).unwrap();
        assert!(gitignore.contains(".claude_checkpoint.yaml"));
    }

    #[test]
    fn test_run_init_docs_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "DocsProject", "docs", false);
        assert!(result.success);
        assert!(matches!(result.project_type, Some(ProjectType::Docs)));
    }

    #[test]
    fn test_run_init_flutter_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "FlutterApp", "flutter", false);
        assert!(result.success);
        assert!(matches!(result.project_type, Some(ProjectType::Flutter)));
    }

    #[test]
    fn test_run_init_invalid_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "Test", "invalid_type", false);
        // Should fail with unknown type
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("Unknown template"));
    }

    #[test]
    fn test_run_init_python_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "PyProject", "python", false);
        assert!(result.success);
        assert!(matches!(result.project_type, Some(ProjectType::Python)));
    }

    #[test]
    fn test_run_init_node_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "NodeProject", "node", false);
        assert!(result.success);
        assert!(matches!(result.project_type, Some(ProjectType::Node)));
    }

    #[test]
    fn test_run_init_go_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "GoProject", "go", false);
        assert!(result.success);
        assert!(matches!(result.project_type, Some(ProjectType::Go)));
    }

    #[test]
    fn test_run_init_generic_type() {
        let temp = TempDir::new().unwrap();
        let result = run_init(temp.path(), "GenericProject", "generic", false);
        assert!(result.success);
        assert!(matches!(result.project_type, Some(ProjectType::Generic)));
    }
}
