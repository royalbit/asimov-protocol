//! Project-related template generators
//! v10.3.0: Templates in cli/templates/ with .asimov/templates/ override (ADR-057)

use super::ProjectType;
use std::path::PathBuf;

// ========== Embedded Project Templates (compile-time from cli/templates/) ==========
// Single source of truth: cli/templates/project/*.yaml
// Runtime override: .asimov/templates/*.yaml takes priority

const PROJECT_RUST_TEMPLATE: &str = include_str!("../../templates/project/rust.yaml");
const PROJECT_PYTHON_TEMPLATE: &str = include_str!("../../templates/project/python.yaml");
const PROJECT_NODE_TEMPLATE: &str = include_str!("../../templates/project/node.yaml");
const PROJECT_GO_TEMPLATE: &str = include_str!("../../templates/project/go.yaml");
const PROJECT_FLUTTER_TEMPLATE: &str = include_str!("../../templates/project/flutter.yaml");
const PROJECT_DOCS_TEMPLATE: &str = include_str!("../../templates/project/docs.yaml");
const PROJECT_GENERIC_TEMPLATE: &str = include_str!("../../templates/project/generic.yaml");
const PROJECT_ARCH_TEMPLATE: &str = include_str!("../../templates/project/arch.yaml");

// ========== Embedded Enterprise Templates (compile-time from cli/templates/) ==========
// API templates
const ENTERPRISE_API_RUST: &str = include_str!("../../templates/enterprise/api-rust.yaml");
const ENTERPRISE_API_GO: &str = include_str!("../../templates/enterprise/api-go.yaml");
const ENTERPRISE_API_FASTAPI: &str = include_str!("../../templates/enterprise/api-fastapi.yaml");
const ENTERPRISE_API_NESTJS: &str = include_str!("../../templates/enterprise/api-nestjs.yaml");
const ENTERPRISE_API_SPRING: &str = include_str!("../../templates/enterprise/api-spring.yaml");
// Web templates
const ENTERPRISE_WEB_NEXTJS: &str = include_str!("../../templates/enterprise/web-nextjs.yaml");
const ENTERPRISE_WEB_REACT: &str = include_str!("../../templates/enterprise/web-react.yaml");
const ENTERPRISE_WEB_VUE: &str = include_str!("../../templates/enterprise/web-vue.yaml");
const ENTERPRISE_WEB_ANGULAR: &str = include_str!("../../templates/enterprise/web-angular.yaml");
// Monorepo templates
const ENTERPRISE_MONO_TURBO: &str = include_str!("../../templates/enterprise/mono-turbo.yaml");
const ENTERPRISE_MONO_NX: &str = include_str!("../../templates/enterprise/mono-nx.yaml");
const ENTERPRISE_MONO_PNPM: &str = include_str!("../../templates/enterprise/mono-pnpm.yaml");
// Admin template
const ENTERPRISE_ADMIN_DASHBOARD: &str =
    include_str!("../../templates/enterprise/admin-dashboard.yaml");

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

/// All embedded enterprise template names
const ENTERPRISE_TEMPLATES: &[&str] = &[
    "api-rust",
    "api-go",
    "api-fastapi",
    "api-nestjs",
    "api-spring",
    "web-nextjs",
    "web-react",
    "web-vue",
    "web-angular",
    "mono-turbo",
    "mono-nx",
    "mono-pnpm",
    "admin-dashboard",
];

/// Get an enterprise template by name
/// v10.3.0: Tries external file first, falls back to embedded
pub fn get_enterprise_template(name: &str) -> Option<String> {
    // Try external file first
    if let Some(content) = try_read_template(name) {
        return Some(content);
    }

    // Fall back to embedded template
    match name {
        "api-rust" => Some(ENTERPRISE_API_RUST.to_string()),
        "api-go" => Some(ENTERPRISE_API_GO.to_string()),
        "api-fastapi" => Some(ENTERPRISE_API_FASTAPI.to_string()),
        "api-nestjs" => Some(ENTERPRISE_API_NESTJS.to_string()),
        "api-spring" => Some(ENTERPRISE_API_SPRING.to_string()),
        "web-nextjs" => Some(ENTERPRISE_WEB_NEXTJS.to_string()),
        "web-react" => Some(ENTERPRISE_WEB_REACT.to_string()),
        "web-vue" => Some(ENTERPRISE_WEB_VUE.to_string()),
        "web-angular" => Some(ENTERPRISE_WEB_ANGULAR.to_string()),
        "mono-turbo" => Some(ENTERPRISE_MONO_TURBO.to_string()),
        "mono-nx" => Some(ENTERPRISE_MONO_NX.to_string()),
        "mono-pnpm" => Some(ENTERPRISE_MONO_PNPM.to_string()),
        "admin-dashboard" => Some(ENTERPRISE_ADMIN_DASHBOARD.to_string()),
        _ => None,
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

    // Add embedded enterprise templates
    templates.extend(ENTERPRISE_TEMPLATES.iter().map(|s| s.to_string()));

    // Add custom templates from .asimov/templates/ if they exist
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

    #[test]
    fn test_enterprise_templates_valid_yaml() {
        for name in ENTERPRISE_TEMPLATES {
            let template = get_enterprise_template(name);
            assert!(
                template.is_some(),
                "Enterprise template {} should exist",
                name
            );
            let yaml: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&template.unwrap());
            assert!(
                yaml.is_ok(),
                "Enterprise template {} should be valid YAML",
                name
            );
        }
    }

    #[test]
    fn test_list_templates_includes_enterprise() {
        let templates = list_templates();
        // Should include base project templates
        assert!(templates.contains(&"rust".to_string()));
        assert!(templates.contains(&"python".to_string()));
        // Should include enterprise templates
        assert!(templates.contains(&"api-rust".to_string()));
        assert!(templates.contains(&"web-nextjs".to_string()));
        assert!(templates.contains(&"admin-dashboard".to_string()));
    }

    #[test]
    fn test_get_enterprise_template_unknown() {
        assert!(get_enterprise_template("nonexistent").is_none());
    }
}
