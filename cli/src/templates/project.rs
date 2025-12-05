//! Project-related template generators

use super::ProjectType;

// Project templates embedded from files (ADR-032)
// Note: paths are relative to this file's location in templates/
const PROJECT_RUST_TEMPLATE: &str = include_str!("project-rust.yaml.tpl");
const PROJECT_PYTHON_TEMPLATE: &str = include_str!("project-python.yaml.tpl");
const PROJECT_NODE_TEMPLATE: &str = include_str!("project-node.yaml.tpl");
const PROJECT_GO_TEMPLATE: &str = include_str!("project-go.yaml.tpl");
const PROJECT_FLUTTER_TEMPLATE: &str = include_str!("project-flutter.yaml.tpl");
const PROJECT_DOCS_TEMPLATE: &str = include_str!("project-docs.yaml.tpl");
const PROJECT_GENERIC_TEMPLATE: &str = include_str!("project-generic.yaml.tpl");

/// Generate a project.yaml template for project-specific configuration (ADR-032)
pub fn project_template(
    project_name: &str,
    project_tagline: &str,
    project_type: ProjectType,
) -> String {
    let template = match project_type {
        ProjectType::Rust => PROJECT_RUST_TEMPLATE,
        ProjectType::Python => PROJECT_PYTHON_TEMPLATE,
        ProjectType::Node => PROJECT_NODE_TEMPLATE,
        ProjectType::Go => PROJECT_GO_TEMPLATE,
        ProjectType::Flutter => PROJECT_FLUTTER_TEMPLATE,
        ProjectType::Docs => PROJECT_DOCS_TEMPLATE,
        ProjectType::Generic => PROJECT_GENERIC_TEMPLATE,
    };

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
        ProjectType::Generic => {
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

- 4hr MAX session, keep shipping, NO scope creep
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
        let yaml: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
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
        ];
        for pt in types {
            let template = project_template("test", "tagline", pt);
            let yaml: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
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
