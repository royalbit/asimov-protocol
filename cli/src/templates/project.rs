//! Project-related template generators
//! v10.0.0: Templates loaded from .asimov/templates/ with embedded fallback (ADR-053)

use super::ProjectType;
use std::path::PathBuf;

// ========== Embedded Fallback Templates (compile-time) ==========
// These are only used if external files are missing

const PROJECT_RUST_TEMPLATE: &str = include_str!("project-rust.yaml.tpl");
const PROJECT_PYTHON_TEMPLATE: &str = include_str!("project-python.yaml.tpl");
const PROJECT_NODE_TEMPLATE: &str = include_str!("project-node.yaml.tpl");
const PROJECT_GO_TEMPLATE: &str = include_str!("project-go.yaml.tpl");
const PROJECT_FLUTTER_TEMPLATE: &str = include_str!("project-flutter.yaml.tpl");
const PROJECT_DOCS_TEMPLATE: &str = include_str!("project-docs.yaml.tpl");
const PROJECT_GENERIC_TEMPLATE: &str = include_str!("project-generic.yaml.tpl");
const PROJECT_ARCH_TEMPLATE: &str = include_str!("project-arch.yaml.tpl");

/// Get the templates directory path
pub fn templates_dir() -> PathBuf {
    PathBuf::from(".asimov/templates")
}

/// Try to read a template from external file, return None if not found
fn try_read_template(name: &str) -> Option<String> {
    let path = templates_dir().join(format!("{}.yaml", name));
    std::fs::read_to_string(&path).ok()
}

/// Get template content, trying external file first, then embedded fallback
fn get_template(project_type: ProjectType) -> String {
    let name = match project_type {
        ProjectType::Rust => "rust",
        ProjectType::Python => "python",
        ProjectType::Node => "node",
        ProjectType::Go => "go",
        ProjectType::Flutter => "flutter",
        ProjectType::Docs => "docs",
        ProjectType::Arch => "arch",
        ProjectType::Generic | ProjectType::Migration => "generic",
    };

    // Try external file first
    if let Some(content) = try_read_template(name) {
        return content;
    }

    // Fall back to embedded template
    match project_type {
        ProjectType::Rust => PROJECT_RUST_TEMPLATE.to_string(),
        ProjectType::Python => PROJECT_PYTHON_TEMPLATE.to_string(),
        ProjectType::Node => PROJECT_NODE_TEMPLATE.to_string(),
        ProjectType::Go => PROJECT_GO_TEMPLATE.to_string(),
        ProjectType::Flutter => PROJECT_FLUTTER_TEMPLATE.to_string(),
        ProjectType::Docs => PROJECT_DOCS_TEMPLATE.to_string(),
        ProjectType::Arch => PROJECT_ARCH_TEMPLATE.to_string(),
        ProjectType::Generic | ProjectType::Migration => PROJECT_GENERIC_TEMPLATE.to_string(),
    }
}

/// List all available templates (external + embedded)
pub fn list_templates() -> Vec<String> {
    let mut templates = vec![
        "rust", "python", "node", "go", "flutter", "docs", "arch", "generic",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    // Add enterprise templates from .asimov/templates/ if they exist
    if let Ok(entries) = std::fs::read_dir(templates_dir()) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".yaml") {
                let template_name = name_str.trim_end_matches(".yaml").to_string();
                if !templates.contains(&template_name) {
                    templates.push(template_name);
                }
            }
        }
    }

    templates.sort();
    templates
}

/// Generate a project.yaml template for project-specific configuration (ADR-032)
/// v10.0.0: Tries external template files first, falls back to embedded
pub fn project_template(
    project_name: &str,
    project_tagline: &str,
    project_type: ProjectType,
) -> String {
    let template = get_template(project_type);

    template
        .replace("{PROJECT_NAME}", project_name)
        .replace("{PROJECT_TAGLINE}", project_tagline)
}

/// Generate CLAUDE.md for RoyalBit Asimov (auto-loaded by Claude Code)
/// v4.0.0: Uses Claude Code native @import syntax for memory hierarchy
pub fn claude_md_template(project_name: &str, project_type: ProjectType) -> String {
    let commands = match project_type {
        ProjectType::Rust => {
            r#"```bash
cargo test                    # Run tests
cargo clippy -- -D warnings   # Lint (must pass)
cargo fmt                     # Format code
cargo build --release         # Release build
```"#
        }
        ProjectType::Python => {
            r#"```bash
pytest                        # Run tests
ruff check . --fix            # Lint and fix
ruff format .                 # Format code
mypy .                        # Type check
```"#
        }
        ProjectType::Node => {
            r#"```bash
npm test                      # Run tests
npm run lint                  # ESLint
npm run format                # Prettier
npm run build                 # Build
```"#
        }
        ProjectType::Go => {
            r#"```bash
go test ./...                 # Run tests
golangci-lint run             # Lint (must pass)
go fmt ./...                  # Format code
go build -o bin/app           # Build
```"#
        }
        ProjectType::Flutter => {
            r#"```bash
flutter test                  # Run tests
dart analyze lib/             # Analyze (must pass)
dart format lib/ test/        # Format code
flutter build apk             # Build Android
```"#
        }
        ProjectType::Docs => {
            r#"```bash
asimov lint-docs .           # Check markdown
asimov lint-docs --fix .     # Fix markdown
markdownlint '**/*.md'               # Standard lint
```"#
        }
        ProjectType::Arch => {
            r#"```bash
asimov lint-docs .           # Check markdown
asimov lint-docs --fix .     # Fix markdown
# Verify diagrams render correctly
```"#
        }
        ProjectType::Generic | ProjectType::Migration => {
            r#"```bash
# Add your project-specific commands here
```"#
        }
    };

    format!(
        r#"# {}

@.asimov/warmup.yaml
@.asimov/asimov.yaml
@.asimov/green.yaml
@.asimov/sycophancy.yaml

## Core Rules

- Keep shipping until done, NO scope creep
- Tests pass + ZERO warnings → then commit
- Done > Perfect. Ship it.

## Recovery

Use native Claude Code features:
- `/rewind` - Restore previous checkpoint
- `--continue` - Resume last session
- `--resume` - Pick specific session

## Commands

{}
"#,
        project_name, commands
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_template_valid_yaml() {
        let template = project_template("test-project", "A test project", ProjectType::Rust);
        let yaml: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&template);
        assert!(yaml.is_ok(), "Project template should be valid YAML");
    }

    #[test]
    fn test_project_template_all_types() {
        let types = [
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Node,
            ProjectType::Go,
            ProjectType::Flutter,
            ProjectType::Docs,
            ProjectType::Generic,
            ProjectType::Migration,
            ProjectType::Arch,
        ];
        for pt in types {
            let template = project_template("test", "tagline", pt);
            let yaml: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&template);
            assert!(yaml.is_ok(), "Template for {:?} should be valid YAML", pt);
        }
    }

    #[test]
    fn test_claude_md_template() {
        let template = claude_md_template("test-project", ProjectType::Rust);
        assert!(template.contains("cargo") || template.contains("test-project"));
    }

    #[test]
    fn test_claude_md_template_all_types() {
        let types = [
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Node,
            ProjectType::Go,
            ProjectType::Flutter,
            ProjectType::Docs,
            ProjectType::Generic,
            ProjectType::Migration,
            ProjectType::Arch,
        ];
        for pt in types {
            let template = claude_md_template("test", pt);
            assert!(
                !template.is_empty(),
                "CLAUDE.md for {:?} should not be empty",
                pt
            );
        }
    }
}
