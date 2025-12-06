//! Init command implementation

use crate::{
    claude_pre_compact_hook, claude_session_start_hook, claude_settings_json, git_precommit_hook,
    project_template, protocols::PROTOCOL_FILES, roadmap_template, ProjectType,
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
        error: None,
    };

    let project_type = match type_str.to_lowercase().as_str() {
        "rust" | "rs" => ProjectType::Rust,
        "python" | "py" => ProjectType::Python,
        "node" | "nodejs" | "javascript" | "js" | "typescript" | "ts" => ProjectType::Node,
        "go" | "golang" => ProjectType::Go,
        "flutter" | "dart" => ProjectType::Flutter,
        "docs" | "documentation" => ProjectType::Docs,
        "generic" => ProjectType::Generic,
        other => {
            result.error = Some(format!("Unknown project type: '{}'. Valid types: rust, python, node, go, flutter, docs, generic", other));
            return result;
        }
    };
    result.project_type = Some(project_type);

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
        let content = project_template(name, "Your project tagline", project_type);
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

    // Install Claude hooks
    let claude_dir = dir.join(".claude");
    let hooks_dir = claude_dir.join("hooks");
    let _ = std::fs::create_dir_all(&hooks_dir);

    let settings_path = claude_dir.join("settings.json");
    if !settings_path.exists() || force {
        if let Err(e) = std::fs::write(&settings_path, claude_settings_json()) {
            result.error = Some(format!("Failed to write settings.json: {}", e));
            return result;
        }
        result.hooks_installed.push("settings.json".to_string());
    }

    let session_start_path = hooks_dir.join("session-start.sh");
    if !session_start_path.exists() || force {
        if let Err(e) = std::fs::write(&session_start_path, claude_session_start_hook()) {
            result.error = Some(format!("Failed to write session-start.sh: {}", e));
            return result;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(
                &session_start_path,
                std::fs::Permissions::from_mode(0o755),
            );
        }
        result.hooks_installed.push("session-start.sh".to_string());
    }

    let pre_compact_path = hooks_dir.join("pre-compact.sh");
    if !pre_compact_path.exists() || force {
        if let Err(e) = std::fs::write(&pre_compact_path, claude_pre_compact_hook()) {
            result.error = Some(format!("Failed to write pre-compact.sh: {}", e));
            return result;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ =
                std::fs::set_permissions(&pre_compact_path, std::fs::Permissions::from_mode(0o755));
        }
        result.hooks_installed.push("pre-compact.sh".to_string());
    }

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

    result.success = true;
    result
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
            error: None,
        };
        assert!(r.success);
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
        assert!(result.error.unwrap().contains("Unknown project type"));
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
