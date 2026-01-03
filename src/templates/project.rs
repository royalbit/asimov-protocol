//! Project-related template generators
//! v10.3.1: All templates in flat cli/templates/ with .asimov/templates/ override (ADR-057)

use super::ProjectType;
use std::path::PathBuf;

// ========== Embedded Templates (compile-time from cli/templates/) ==========
// Single source of truth: cli/templates/*.yaml
// Runtime override: .asimov/templates/*.yaml takes priority

// Base project templates
const TEMPLATE_RUST: &str = include_str!("../../templates/rust.yaml");
const TEMPLATE_PYTHON: &str = include_str!("../../templates/python.yaml");
const TEMPLATE_NODE: &str = include_str!("../../templates/node.yaml");
const TEMPLATE_GO: &str = include_str!("../../templates/go.yaml");
const TEMPLATE_FLUTTER: &str = include_str!("../../templates/flutter.yaml");
const TEMPLATE_DOCS: &str = include_str!("../../templates/docs.yaml");
const TEMPLATE_GENERIC: &str = include_str!("../../templates/generic.yaml");
const TEMPLATE_ARCH: &str = include_str!("../../templates/arch.yaml");

// API templates
const TEMPLATE_API_RUST: &str = include_str!("../../templates/api-rust.yaml");
const TEMPLATE_API_GO: &str = include_str!("../../templates/api-go.yaml");
const TEMPLATE_API_FASTAPI: &str = include_str!("../../templates/api-fastapi.yaml");
const TEMPLATE_API_NESTJS: &str = include_str!("../../templates/api-nestjs.yaml");
const TEMPLATE_API_SPRING: &str = include_str!("../../templates/api-spring.yaml");

// Web templates
const TEMPLATE_WEB_NEXTJS: &str = include_str!("../../templates/web-nextjs.yaml");
const TEMPLATE_WEB_REACT: &str = include_str!("../../templates/web-react.yaml");
const TEMPLATE_WEB_VUE: &str = include_str!("../../templates/web-vue.yaml");
const TEMPLATE_WEB_ANGULAR: &str = include_str!("../../templates/web-angular.yaml");

// Monorepo templates
const TEMPLATE_MONO_TURBO: &str = include_str!("../../templates/mono-turbo.yaml");
const TEMPLATE_MONO_NX: &str = include_str!("../../templates/mono-nx.yaml");
const TEMPLATE_MONO_PNPM: &str = include_str!("../../templates/mono-pnpm.yaml");

// Admin template
const TEMPLATE_ADMIN_DASHBOARD: &str = include_str!("../../templates/admin-dashboard.yaml");

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
        ProjectType::Rust => TEMPLATE_RUST.to_string(),
        ProjectType::Python => TEMPLATE_PYTHON.to_string(),
        ProjectType::Node => TEMPLATE_NODE.to_string(),
        ProjectType::Go => TEMPLATE_GO.to_string(),
        ProjectType::Flutter => TEMPLATE_FLUTTER.to_string(),
        ProjectType::Docs => TEMPLATE_DOCS.to_string(),
        ProjectType::Arch => TEMPLATE_ARCH.to_string(),
        ProjectType::Generic | ProjectType::Migration => TEMPLATE_GENERIC.to_string(),
    }
}

/// All embedded template names (beyond base project types)
const EXTENDED_TEMPLATES: &[&str] = &[
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

/// Get any template by name (tries external first, falls back to embedded)
/// v10.3.1: Unified template lookup for all template types
pub fn get_template_by_name(name: &str) -> Option<String> {
    // Try external file first
    if let Some(content) = try_read_template(name) {
        return Some(content);
    }

    // Fall back to embedded template
    match name {
        // Base project templates
        "rust" => Some(TEMPLATE_RUST.to_string()),
        "python" => Some(TEMPLATE_PYTHON.to_string()),
        "node" => Some(TEMPLATE_NODE.to_string()),
        "go" => Some(TEMPLATE_GO.to_string()),
        "flutter" => Some(TEMPLATE_FLUTTER.to_string()),
        "docs" => Some(TEMPLATE_DOCS.to_string()),
        "arch" => Some(TEMPLATE_ARCH.to_string()),
        "generic" => Some(TEMPLATE_GENERIC.to_string()),
        // API templates
        "api-rust" => Some(TEMPLATE_API_RUST.to_string()),
        "api-go" => Some(TEMPLATE_API_GO.to_string()),
        "api-fastapi" => Some(TEMPLATE_API_FASTAPI.to_string()),
        "api-nestjs" => Some(TEMPLATE_API_NESTJS.to_string()),
        "api-spring" => Some(TEMPLATE_API_SPRING.to_string()),
        // Web templates
        "web-nextjs" => Some(TEMPLATE_WEB_NEXTJS.to_string()),
        "web-react" => Some(TEMPLATE_WEB_REACT.to_string()),
        "web-vue" => Some(TEMPLATE_WEB_VUE.to_string()),
        "web-angular" => Some(TEMPLATE_WEB_ANGULAR.to_string()),
        // Monorepo templates
        "mono-turbo" => Some(TEMPLATE_MONO_TURBO.to_string()),
        "mono-nx" => Some(TEMPLATE_MONO_NX.to_string()),
        "mono-pnpm" => Some(TEMPLATE_MONO_PNPM.to_string()),
        // Admin template
        "admin-dashboard" => Some(TEMPLATE_ADMIN_DASHBOARD.to_string()),
        _ => None,
    }
}

/// Backwards compatibility alias
#[deprecated(since = "10.3.1", note = "Use get_template_by_name instead")]
pub fn get_enterprise_template(name: &str) -> Option<String> {
    get_template_by_name(name)
}

/// List all available templates (external + embedded)
pub fn list_templates() -> Vec<String> {
    let mut templates = vec![
        "rust", "python", "node", "go", "flutter", "docs", "arch", "generic",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<Vec<_>>();

    // Add extended templates (api-*, web-*, mono-*, admin-dashboard)
    templates.extend(EXTENDED_TEMPLATES.iter().map(|s| s.to_string()));

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
    fn test_extended_templates_valid_yaml() {
        for name in EXTENDED_TEMPLATES {
            let template = get_template_by_name(name);
            assert!(template.is_some(), "Template {} should exist", name);
            let yaml: Result<serde_yaml_ng::Value, _> = serde_yaml_ng::from_str(&template.unwrap());
            assert!(yaml.is_ok(), "Template {} should be valid YAML", name);
        }
    }

    #[test]
    fn test_list_templates_includes_all() {
        let templates = list_templates();
        // Should include base project templates
        assert!(templates.contains(&"rust".to_string()));
        assert!(templates.contains(&"python".to_string()));
        // Should include extended templates
        assert!(templates.contains(&"api-rust".to_string()));
        assert!(templates.contains(&"web-nextjs".to_string()));
        assert!(templates.contains(&"admin-dashboard".to_string()));
    }

    #[test]
    fn test_get_template_by_name_unknown() {
        assert!(get_template_by_name("nonexistent").is_none());
    }

    #[test]
    fn test_get_template_by_name_base() {
        assert!(get_template_by_name("rust").is_some());
        assert!(get_template_by_name("python").is_some());
        assert!(get_template_by_name("arch").is_some());
    }
}
