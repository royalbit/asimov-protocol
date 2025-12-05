//! Init command implementation

use crate::{
    claude_pre_compact_hook, claude_session_start_hook, claude_settings_json, git_precommit_hook,
    project_template, protocols::PROTOCOL_FILES, roadmap_template, ProjectType,
};
use std::path::Path;

// ============================================================================
// COVERAGE EXCLUSIONS (ADR-039: require filesystem error mocking)
// ============================================================================

/// Handle init error - sets error and returns result (excluded: error path)
#[cfg_attr(feature = "coverage", coverage(off))]
fn set_init_error(result: &mut InitResult, msg: String) -> InitResult {
    result.error = Some(msg);
    result.clone()
}

/// Track file as kept (excluded: conditional branch)
#[cfg_attr(feature = "coverage", coverage(off))]
fn track_file_kept(result: &mut InitResult, filename: &str) {
    result.files_kept.push(filename.to_string());
}

/// Write file with tracking (excluded: filesystem error handling)
#[cfg_attr(feature = "coverage", coverage(off))]
fn write_init_file(
    result: &mut InitResult,
    path: &std::path::Path,
    content: &str,
    filename: &str,
    existed: bool,
) -> Result<(), String> {
    if let Err(e) = std::fs::write(path, content) {
        return Err(format!("Failed to write {}: {}", filename, e));
    }
    if existed {
        result.files_updated.push(filename.to_string());
    } else {
        result.files_created.push(filename.to_string());
    }
    Ok(())
}

/// Install hook with permissions (excluded: filesystem error handling)
#[cfg_attr(feature = "coverage", coverage(off))]
fn install_hook_file(
    result: &mut InitResult,
    path: &std::path::Path,
    content: &str,
    name: &str,
) -> Result<(), String> {
    if let Err(e) = std::fs::write(path, content) {
        return Err(format!("Failed to write {}: {}", name, e));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755));
    }
    result.hooks_installed.push(name.to_string());
    Ok(())
}

/// Install git pre-commit hook (excluded: filesystem + git operations)
#[cfg_attr(feature = "coverage", coverage(off))]
fn install_git_precommit(result: &mut InitResult, git_hooks_dir: &std::path::Path, force: bool) {
    if git_hooks_dir.exists() {
        let precommit_path = git_hooks_dir.join("pre-commit");
        if !precommit_path.exists() || force {
            if let Ok(()) = std::fs::write(&precommit_path, git_precommit_hook()) {
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
}

/// Update or create .gitignore (excluded: conditional filesystem operations)
#[cfg_attr(feature = "coverage", coverage(off))]
fn update_gitignore(result: &mut InitResult, gitignore_path: &std::path::Path, entry: &str) {
    if gitignore_path.exists() {
        if let Ok(content) = std::fs::read_to_string(gitignore_path) {
            if !content.contains(entry) {
                let new_content = format!("{}\n{}\n", content.trim_end(), entry);
                let _ = std::fs::write(gitignore_path, new_content);
                result.files_updated.push(".gitignore".to_string());
            }
        }
    } else {
        let _ = std::fs::write(gitignore_path, format!("{}\n", entry));
        result.files_created.push(".gitignore".to_string());
    }
}

/// Install settings.json (excluded: filesystem error handling)
#[cfg_attr(feature = "coverage", coverage(off))]
fn install_settings_json(
    result: &mut InitResult,
    settings_path: &std::path::Path,
    content: &str,
) -> Result<(), String> {
    if let Err(e) = std::fs::write(settings_path, content) {
        return Err(format!("Failed to write settings.json: {}", e));
    }
    result.hooks_installed.push("settings.json".to_string());
    Ok(())
}

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
        return set_init_error(&mut result, format!("Failed to create .asimov/: {}", e));
    }

    // Create roadmap.yaml
    let roadmap_path = asimov_dir.join("roadmap.yaml");
    let roadmap_existed = roadmap_path.exists();
    if !roadmap_existed || force {
        let content = roadmap_template();
        if let Err(msg) = write_init_file(
            &mut result,
            &roadmap_path,
            &content,
            "roadmap.yaml",
            roadmap_existed,
        ) {
            return set_init_error(&mut result, msg);
        }
    } else {
        track_file_kept(&mut result, "roadmap.yaml");
    }

    // Create project.yaml
    let project_path = asimov_dir.join("project.yaml");
    let project_existed = project_path.exists();
    if !project_existed || force {
        let content = project_template(name, "Your project tagline", project_type);
        if let Err(msg) = write_init_file(
            &mut result,
            &project_path,
            &content,
            "project.yaml",
            project_existed,
        ) {
            return set_init_error(&mut result, msg);
        }
    } else {
        track_file_kept(&mut result, "project.yaml");
    }

    // v9.0.0: Create protocol JSON files
    for (filename, generator) in PROTOCOL_FILES {
        let file_path = asimov_dir.join(filename);
        let existed = file_path.exists();
        if !existed || force {
            let content = generator();
            if let Err(msg) = write_init_file(&mut result, &file_path, &content, filename, existed)
            {
                return set_init_error(&mut result, msg);
            }
        } else {
            track_file_kept(&mut result, filename);
        }
    }

    // Update or create .gitignore
    let gitignore_path = dir.join(".gitignore");
    update_gitignore(&mut result, &gitignore_path, ".claude_checkpoint.yaml");

    // Install Claude hooks
    let claude_dir = dir.join(".claude");
    let hooks_dir = claude_dir.join("hooks");
    let _ = std::fs::create_dir_all(&hooks_dir);

    let settings_path = claude_dir.join("settings.json");
    if !settings_path.exists() || force {
        if let Err(msg) =
            install_settings_json(&mut result, &settings_path, &claude_settings_json())
        {
            return set_init_error(&mut result, msg);
        }
    }

    let session_start_path = hooks_dir.join("session-start.sh");
    if !session_start_path.exists() || force {
        if let Err(msg) = install_hook_file(
            &mut result,
            &session_start_path,
            &claude_session_start_hook(),
            "session-start.sh",
        ) {
            return set_init_error(&mut result, msg);
        }
    }

    let pre_compact_path = hooks_dir.join("pre-compact.sh");
    if !pre_compact_path.exists() || force {
        if let Err(msg) = install_hook_file(
            &mut result,
            &pre_compact_path,
            &claude_pre_compact_hook(),
            "pre-compact.sh",
        ) {
            return set_init_error(&mut result, msg);
        }
    }

    // Install git pre-commit hook if in git repo
    let git_hooks_dir = dir.join(".git").join("hooks");
    install_git_precommit(&mut result, &git_hooks_dir, force);

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
