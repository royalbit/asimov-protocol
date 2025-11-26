//! Template generators for Forge Protocol files

use std::fmt;

/// Supported project types for template generation
#[derive(Debug, Clone, Copy, Default)]
pub enum ProjectType {
    #[default]
    Generic,
    Rust,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Generic => write!(f, "generic"),
            ProjectType::Rust => write!(f, "rust"),
        }
    }
}

impl std::str::FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generic" => Ok(ProjectType::Generic),
            "rust" => Ok(ProjectType::Rust),
            _ => Err(format!(
                "Unknown project type: '{}'. Available: generic, rust",
                s
            )),
        }
    }
}

/// Generate a starter warmup.yaml template
pub fn warmup_template(project_name: &str, project_type: ProjectType) -> String {
    match project_type {
        ProjectType::Generic => warmup_generic(project_name),
        ProjectType::Rust => warmup_rust(project_name),
    }
}

fn warmup_generic(project_name: &str) -> String {
    format!(
        r#"# Forge Protocol - Session Bootstrap
# https://github.com/royalbit/forge-protocol

identity:
  project: "{}"
  tagline: "Brief project description"
  version: "0.1.0"

mission:
  problem: "What problem does this solve?"
  solution: "How does it solve it?"
  principles:
    - "Principle one"
    - "Principle two"

files:
  source:
    - "src/ - Source code"
  config:
    - "Configuration files"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
  during:
    - "Track progress"
    - "Test frequently"
  end:
    - "Run tests"
    - "Update documentation"

quality:
  tests: "All tests must pass"
  lint: "Run linter"

style:
  code:
    - "Follow project conventions"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

fn warmup_rust(project_name: &str) -> String {
    format!(
        r#"# Forge Protocol - Session Bootstrap
# https://github.com/royalbit/forge-protocol

identity:
  project: "{}"
  tagline: "Brief project description"
  version: "0.1.0"

mission:
  problem: "What problem does this solve?"
  solution: "How does it solve it?"
  principles:
    - "Principle one"
    - "Principle two"

files:
  source:
    - "src/main.rs - Entry point"
    - "src/lib.rs - Library root"
  config:
    - "Cargo.toml - Dependencies"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "cargo test (verify baseline)"
  during:
    - "Track progress"
    - "Test frequently"
    - "Small, logical commits"
  end:
    - "cargo test (all pass)"
    - "cargo clippy -- -D warnings"
    - "Update documentation"

quality:
  tests: "cargo test"
  warnings: "cargo clippy -- -D warnings"
  formatting: "cargo fmt --all -- --check"

style:
  rust:
    - "Result<T, E> for errors, no panics"
    - "thiserror for custom errors"
    - "No unwrap() in library code"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

/// Generate a starter sprint.yaml template
pub fn sprint_template() -> String {
    r#"# Forge Protocol - Sprint Tracking
# https://github.com/royalbit/forge-protocol

sprint:
  current: "Initial setup"
  started: "2025-01-01"
  status: in_progress

  tasks:
    - "[ ] Task one"
    - "[ ] Task two"
    - "[ ] Task three"

  blockers: []

  notes: |
    Add any relevant context here.
"#
    .to_string()
}

/// Generate a starter roadmap.yaml template
pub fn roadmap_template() -> String {
    r#"# Forge Protocol - Roadmap
# https://github.com/royalbit/forge-protocol

metadata:
  current_version: "0.1.0"
  last_updated: "2025-01-01"

current:
  version: "0.1.0"
  status: in_progress
  summary: "Initial Release"
  highlights:
    - "Core functionality"
    - "Basic documentation"

next:
  version: "0.2.0"
  status: planned
  summary: "Next Milestone"
  features:
    - "Feature one"
    - "Feature two"

backlog:
  - "Future idea one"
  - "Future idea two"
"#
    .to_string()
}
