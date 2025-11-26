//! Template generators for Forge Protocol files

/// Generate a starter warmup.yaml template
pub fn warmup_template(project_name: &str) -> String {
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
  config:
    - "Cargo.toml - Dependencies"
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
  lint: "cargo clippy -- -D warnings"

style:
  code:
    - "Follow project conventions"
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
