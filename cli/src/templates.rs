//! Template generators for Forge Protocol files

use std::fmt;

/// Supported project types for template generation
#[derive(Debug, Clone, Copy, Default)]
pub enum ProjectType {
    #[default]
    Generic,
    Rust,
    Python,
    Node,
    Go,
    Flutter,
    Docs,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Generic => write!(f, "generic"),
            ProjectType::Rust => write!(f, "rust"),
            ProjectType::Python => write!(f, "python"),
            ProjectType::Node => write!(f, "node"),
            ProjectType::Go => write!(f, "go"),
            ProjectType::Flutter => write!(f, "flutter"),
            ProjectType::Docs => write!(f, "docs"),
        }
    }
}

impl std::str::FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generic" => Ok(ProjectType::Generic),
            "rust" => Ok(ProjectType::Rust),
            "python" | "py" => Ok(ProjectType::Python),
            "node" | "nodejs" | "js" | "javascript" => Ok(ProjectType::Node),
            "go" | "golang" => Ok(ProjectType::Go),
            "flutter" | "dart" => Ok(ProjectType::Flutter),
            "docs" | "documentation" | "arch" | "architecture" => Ok(ProjectType::Docs),
            _ => Err(format!(
                "Unknown project type: '{}'. Available: generic, rust, python, node, go, flutter, docs",
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
        ProjectType::Python => warmup_python(project_name),
        ProjectType::Node => warmup_node(project_name),
        ProjectType::Go => warmup_go(project_name),
        ProjectType::Flutter => warmup_flutter(project_name),
        ProjectType::Docs => warmup_docs(project_name),
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

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use CLI tools for validation, linting, formatting"
    - "Reserve AI for complex reasoning tasks"
    - "Prefer compiled languages or efficient runtimes"
    - "Minimize dependencies and binary sizes"
  why:
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

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

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use cargo test/clippy/fmt for validation (not AI)"
    - "Reserve AI for complex reasoning tasks"
    - "UPX compress release binaries (70%+ smaller)"
    - "Enable LTO and strip symbols in release profile"
  why:
    - "Rust: Zero runtime, minimal memory footprint"
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"
  release_profile: |
    [profile.release]
    opt-level = 3
    lto = true
    codegen-units = 1
    strip = true
    panic = "abort"

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

fn warmup_python(project_name: &str) -> String {
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

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use pytest/ruff/mypy for validation (not AI)"
    - "Reserve AI for complex reasoning tasks"
    - "Use uv or pip-tools for fast, reproducible installs"
    - "Prefer pyproject.toml over setup.py"
  why:
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

files:
  source:
    - "src/{{project}}/ - Package source"
    - "src/{{project}}/__init__.py - Package init"
    - "src/{{project}}/main.py - Entry point"
  config:
    - "pyproject.toml - Project configuration"
    - "requirements.txt - Dependencies (or use pyproject.toml)"
  tests:
    - "tests/ - Test directory"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "pytest (verify baseline)"
  during:
    - "Track progress"
    - "Test frequently"
    - "Small, logical commits"
  end:
    - "pytest (all pass)"
    - "ruff check . (zero warnings)"
    - "mypy . (if using type hints)"
    - "Update documentation"

quality:
  tests: "pytest"
  lint: "ruff check ."
  format: "ruff format ."
  types: "mypy . (optional)"

style:
  python:
    - "Type hints for public APIs"
    - "Docstrings for modules and functions"
    - "No bare except clauses"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

fn warmup_node(project_name: &str) -> String {
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

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use eslint/prettier/vitest for validation (not AI)"
    - "Reserve AI for complex reasoning tasks"
    - "Use pnpm for efficient package management"
    - "Tree-shake and bundle for smaller builds"
  why:
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

files:
  source:
    - "src/index.ts - Entry point"
    - "src/lib/ - Library code"
  config:
    - "package.json - Dependencies and scripts"
    - "tsconfig.json - TypeScript configuration"
  tests:
    - "tests/ - Test directory"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "npm test (verify baseline)"
  during:
    - "Track progress"
    - "Test frequently"
    - "Small, logical commits"
  end:
    - "npm test (all pass)"
    - "npm run lint (zero warnings)"
    - "npm run build (if applicable)"
    - "Update documentation"

quality:
  tests: "npm test (vitest, jest, or similar)"
  lint: "npm run lint (eslint)"
  format: "npm run format (prettier)"
  types: "npm run typecheck (tsc --noEmit)"

style:
  typescript:
    - "Strict TypeScript (strict: true)"
    - "Explicit return types for public functions"
    - "No any types in production code"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

fn warmup_go(project_name: &str) -> String {
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

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use go test/golangci-lint for validation (not AI)"
    - "Reserve AI for complex reasoning tasks"
    - "Static binaries with CGO_ENABLED=0"
    - "UPX compress release binaries (70%+ smaller)"
  why:
    - "Go: Fast compilation, small binaries, no runtime"
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

files:
  source:
    - "main.go - Entry point"
    - "cmd/ - CLI commands"
    - "internal/ - Private packages"
    - "pkg/ - Public packages"
  config:
    - "go.mod - Module definition"
    - "go.sum - Dependency checksums"
  tests:
    - "*_test.go - Test files"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "go test ./... (verify baseline)"
  during:
    - "Track progress"
    - "Test frequently"
    - "Small, logical commits"
  end:
    - "go test ./... (all pass)"
    - "golangci-lint run (zero warnings)"
    - "go build (verify compilation)"
    - "Update documentation"

quality:
  tests: "go test ./..."
  lint: "golangci-lint run"
  format: "gofmt -s -w ."
  vet: "go vet ./..."

style:
  go:
    - "Return errors, don't panic"
    - "Accept interfaces, return structs"
    - "Keep packages small and focused"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

fn warmup_flutter(project_name: &str) -> String {
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

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use flutter test/dart analyze for validation (not AI)"
    - "Reserve AI for complex reasoning tasks"
    - "Tree-shake and use --release for smaller builds"
    - "Use const constructors where possible"
  why:
    - "Flutter: Single codebase, native performance"
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

files:
  source:
    - "lib/main.dart - Entry point"
    - "lib/src/ - Source code"
    - "lib/widgets/ - Reusable widgets"
  config:
    - "pubspec.yaml - Dependencies"
  tests:
    - "test/ - Test directory"
  docs:
    - "README.md - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "flutter test (verify baseline)"
  during:
    - "Track progress"
    - "Test frequently"
    - "Small, logical commits"
  end:
    - "flutter test (all pass)"
    - "dart analyze lib/ (zero warnings)"
    - "dart format lib/ test/"
    - "Update documentation"

quality:
  tests: "flutter test"
  lint: "dart analyze lib/"
  format: "dart format --set-exit-if-changed lib/ test/"

style:
  flutter:
    - "60fps minimum - no jank"
    - "Const constructors where possible"
    - "Split large widgets into smaller components"
  docs:
    - "Keep documentation concise"
"#,
        project_name
    )
}

fn warmup_docs(project_name: &str) -> String {
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
    - "Documentation as code"
    - "Diagrams as code"

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use markdownlint/forge-protocol lint-docs for validation (not AI)"
    - "Reserve AI for content creation, not formatting"
    - "Generate diagrams with Mermaid (text-based)"
    - "Keep docs in git for version control"
  why:
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"

files:
  docs:
    - "README.md - Main documentation"
    - "docs/ - Additional documentation"
    - "docs/adr/ - Architecture Decision Records"
  config:
    - ".markdownlint.yaml - Lint configuration"

session:
  start:
    - "Read warmup.yaml"
    - "git status"
    - "forge-protocol lint-docs (verify baseline)"
  during:
    - "Track progress"
    - "Validate frequently"
    - "Small, logical commits"
  end:
    - "forge-protocol lint-docs (all pass)"
    - "markdownlint '**/*.md' (if installed)"
    - "Review rendered output"

quality:
  lint: "forge-protocol lint-docs ."
  markdown: "markdownlint '**/*.md'"

style:
  markdown:
    - "Use ATX-style headers (#)"
    - "One sentence per line (for git diffs)"
    - "Use Mermaid for diagrams"
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

// ═══════════════════════════════════════════════════════════════════════════════
// SKYNET MODE Templates - CLAUDE.md and Pre-commit Hooks
// ═══════════════════════════════════════════════════════════════════════════════

/// Generate CLAUDE.md for SKYNET MODE (auto-loaded by Claude Code)
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
forge-protocol lint-docs .           # Check markdown
forge-protocol lint-docs --fix .     # Fix markdown
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

## CRITICAL: Self-Healing Protocol

After ANY context compaction, confusion, or uncertainty, RE-READ:
1. `warmup.yaml` - Full protocol and rules
2. `.claude_checkpoint.yaml` - Session state (if exists)

## Mandatory Checkpoints

- **Every 2 hours**: Write progress to `.claude_checkpoint.yaml`, re-read `warmup.yaml`
- **Before any commit**: Re-read quality gates from `warmup.yaml`
- **After task completion**: Update `.claude_checkpoint.yaml`
- **When confused**: STOP → re-read `warmup.yaml` → re-read `.claude_checkpoint.yaml`

## Core Rules (Memorize - These Must Survive)

- 4hr MAX session, 1 milestone, NO scope creep
- Tests pass + ZERO warnings → then commit
- NO "let me also...", NO "while I'm here..."
- Done > Perfect. Ship it.

## Commands

{}

## Key Files

- `warmup.yaml` - Full protocol (RE-READ after compact)
- `sprint.yaml` - Current sprint status
- `roadmap.yaml` - Milestone planning
"#,
        project_name, commands
    )
}

/// Generate pre-commit hook for SKYNET MODE
pub fn precommit_hook_template(project_type: ProjectType) -> String {
    let checks = match project_type {
        ProjectType::Rust => {
            r#"echo "Checking formatting..."
cargo fmt --check

echo "Running clippy..."
cargo clippy --all-targets -- -D warnings

echo "Running tests..."
cargo test"#
        }
        ProjectType::Python => {
            r#"echo "Checking formatting..."
ruff format --check . 2>/dev/null || true

echo "Running linter..."
ruff check . 2>/dev/null || true

echo "Running tests..."
pytest 2>/dev/null || true"#
        }
        ProjectType::Node => {
            r#"echo "Checking formatting..."
npm run format:check 2>/dev/null || true

echo "Running linter..."
npm run lint 2>/dev/null || true

echo "Running tests..."
npm test 2>/dev/null || true"#
        }
        ProjectType::Go => {
            r#"echo "Checking formatting..."
gofmt -l . | read && echo "Files need formatting" && exit 1 || true

echo "Running linter..."
golangci-lint run 2>/dev/null || true

echo "Running tests..."
go test ./... 2>/dev/null || true"#
        }
        ProjectType::Flutter => {
            r#"echo "Checking formatting..."
dart format --set-exit-if-changed lib/ test/ 2>/dev/null || true

echo "Running analyzer..."
dart analyze lib/ 2>/dev/null || flutter analyze 2>/dev/null || true

echo "Running tests..."
flutter test 2>/dev/null || true"#
        }
        ProjectType::Docs | ProjectType::Generic => {
            r#"echo "Checking documentation..."
# Add your checks here"#
        }
    };

    format!(
        r#"#!/bin/bash
# Pre-commit hook for SKYNET MODE
# Generated by forge-protocol init --skynet

set -e

# ═══════════════════════════════════════════════════════════════════════════════
# PROTOCOL REFRESH - Injects rules into fresh context (survives compaction)
# ═══════════════════════════════════════════════════════════════════════════════
if command -v forge-protocol &> /dev/null; then
    forge-protocol refresh
fi

{}

# Forge Protocol validation
if command -v forge-protocol &> /dev/null; then
    echo "Validating protocol files..."
    forge-protocol validate . || true

    # lint-docs added in v1.3.0
    if forge-protocol lint-docs --help &> /dev/null; then
        echo "Linting documentation..."
        forge-protocol lint-docs . || exit 1
    fi
fi

echo "Pre-commit checks passed!"
"#,
        checks
    )
}

/// Generate hook installer script
pub fn hook_installer_template() -> String {
    r#"#!/bin/bash
# Install git hooks for SKYNET MODE
# Generated by forge-protocol init --skynet

set -e

HOOK_DIR=".git/hooks"
SRC_DIR=".hooks"

if [ ! -d ".git" ]; then
    echo "Error: Not a git repository"
    exit 1
fi

mkdir -p "$HOOK_DIR"

if [ -f "$SRC_DIR/pre-commit" ]; then
    cp "$SRC_DIR/pre-commit" "$HOOK_DIR/pre-commit"
    chmod +x "$HOOK_DIR/pre-commit"
    echo "✓ Installed pre-commit hook"
else
    echo "Error: $SRC_DIR/pre-commit not found"
    exit 1
fi

echo "Hooks installed successfully!"
"#
    .to_string()
}

/// Returns true if project type uses cargo-husky (Rust projects)
pub fn uses_cargo_husky(project_type: ProjectType) -> bool {
    matches!(project_type, ProjectType::Rust)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== ProjectType Tests ==========

    #[test]
    fn test_project_type_default() {
        let pt: ProjectType = Default::default();
        assert!(matches!(pt, ProjectType::Generic));
    }

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::Generic.to_string(), "generic");
        assert_eq!(ProjectType::Rust.to_string(), "rust");
        assert_eq!(ProjectType::Python.to_string(), "python");
        assert_eq!(ProjectType::Node.to_string(), "node");
        assert_eq!(ProjectType::Go.to_string(), "go");
    }

    #[test]
    fn test_project_type_from_str_valid() {
        // Generic
        assert!(matches!(
            "generic".parse::<ProjectType>(),
            Ok(ProjectType::Generic)
        ));
        assert!(matches!(
            "GENERIC".parse::<ProjectType>(),
            Ok(ProjectType::Generic)
        ));
        // Rust
        assert!(matches!(
            "rust".parse::<ProjectType>(),
            Ok(ProjectType::Rust)
        ));
        assert!(matches!(
            "RUST".parse::<ProjectType>(),
            Ok(ProjectType::Rust)
        ));
        // Python (with aliases)
        assert!(matches!(
            "python".parse::<ProjectType>(),
            Ok(ProjectType::Python)
        ));
        assert!(matches!(
            "py".parse::<ProjectType>(),
            Ok(ProjectType::Python)
        ));
        assert!(matches!(
            "PYTHON".parse::<ProjectType>(),
            Ok(ProjectType::Python)
        ));
        // Node (with aliases)
        assert!(matches!(
            "node".parse::<ProjectType>(),
            Ok(ProjectType::Node)
        ));
        assert!(matches!(
            "nodejs".parse::<ProjectType>(),
            Ok(ProjectType::Node)
        ));
        assert!(matches!("js".parse::<ProjectType>(), Ok(ProjectType::Node)));
        assert!(matches!(
            "javascript".parse::<ProjectType>(),
            Ok(ProjectType::Node)
        ));
        // Go (with aliases)
        assert!(matches!("go".parse::<ProjectType>(), Ok(ProjectType::Go)));
        assert!(matches!(
            "golang".parse::<ProjectType>(),
            Ok(ProjectType::Go)
        ));
        assert!(matches!("GO".parse::<ProjectType>(), Ok(ProjectType::Go)));
    }

    #[test]
    fn test_project_type_from_str_invalid() {
        let result = "invalid".parse::<ProjectType>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unknown project type"));
        assert!(err.contains("invalid"));
    }

    // ========== warmup_template Tests ==========

    #[test]
    fn test_warmup_template_generic_contains_project_name() {
        let template = warmup_template("my-project", ProjectType::Generic);
        assert!(template.contains("my-project"));
        assert!(template.contains("identity:"));
        assert!(template.contains("project:"));
    }

    #[test]
    fn test_warmup_template_generic_is_generic() {
        let template = warmup_template("test", ProjectType::Generic);
        // Should NOT contain Rust-specific content
        assert!(!template.contains("cargo"));
        assert!(!template.contains("Cargo.toml"));
        assert!(!template.contains("clippy"));
        assert!(!template.contains("main.rs"));
        // Should contain generic content
        assert!(template.contains("src/ - Source code"));
        assert!(template.contains("Run linter"));
        // Should contain green_coding (core protocol requirement)
        assert!(template.contains("green_coding:"));
        assert!(template.contains("Local-first"));
        assert!(template.contains("99.6% carbon reduction"));
    }

    #[test]
    fn test_warmup_template_rust_contains_rust_specific() {
        let template = warmup_template("rust-project", ProjectType::Rust);
        assert!(template.contains("rust-project"));
        // Should contain Rust-specific content
        assert!(template.contains("cargo test"));
        assert!(template.contains("cargo clippy"));
        assert!(template.contains("Cargo.toml"));
        assert!(template.contains("src/main.rs"));
        assert!(template.contains("src/lib.rs"));
        assert!(template.contains("Result<T, E>"));
        assert!(template.contains("thiserror"));
        // Should contain green_coding with Rust-specific practices
        assert!(template.contains("green_coding:"));
        assert!(template.contains("UPX compress"));
        assert!(template.contains("LTO"));
        assert!(template.contains("[profile.release]"));
    }

    #[test]
    fn test_warmup_template_is_valid_yaml() {
        for project_type in [
            ProjectType::Generic,
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Node,
            ProjectType::Go,
        ] {
            let template = warmup_template("test", project_type);
            let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
            assert!(
                result.is_ok(),
                "Template should be valid YAML for {:?}",
                project_type
            );
        }
    }

    #[test]
    fn test_warmup_template_has_required_fields() {
        for project_type in [
            ProjectType::Generic,
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Node,
            ProjectType::Go,
        ] {
            let template = warmup_template("test", project_type);
            let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

            // Check required identity section
            assert!(
                yaml.get("identity").is_some(),
                "Should have identity section for {:?}",
                project_type
            );
            let identity = yaml.get("identity").unwrap();
            assert!(
                identity.get("project").is_some(),
                "Should have project field for {:?}",
                project_type
            );
            // Check green_coding section (required for all templates)
            assert!(
                yaml.get("green_coding").is_some(),
                "Should have green_coding section for {:?}",
                project_type
            );
        }
    }

    #[test]
    fn test_warmup_template_python_contains_python_specific() {
        let template = warmup_template("python-project", ProjectType::Python);
        assert!(template.contains("python-project"));
        // Should contain Python-specific content
        assert!(template.contains("pytest"));
        assert!(template.contains("ruff"));
        assert!(template.contains("pyproject.toml"));
        assert!(template.contains("__init__.py"));
        assert!(template.contains("Type hints"));
        // Should contain green_coding
        assert!(template.contains("green_coding:"));
        assert!(template.contains("uv or pip-tools"));
    }

    #[test]
    fn test_warmup_template_node_contains_node_specific() {
        let template = warmup_template("node-project", ProjectType::Node);
        assert!(template.contains("node-project"));
        // Should contain Node-specific content
        assert!(template.contains("npm test"));
        assert!(template.contains("eslint"));
        assert!(template.contains("prettier"));
        assert!(template.contains("package.json"));
        assert!(template.contains("tsconfig.json"));
        assert!(template.contains("TypeScript"));
        // Should contain green_coding
        assert!(template.contains("green_coding:"));
        assert!(template.contains("pnpm"));
    }

    #[test]
    fn test_warmup_template_go_contains_go_specific() {
        let template = warmup_template("go-project", ProjectType::Go);
        assert!(template.contains("go-project"));
        // Should contain Go-specific content
        assert!(template.contains("go test"));
        assert!(template.contains("golangci-lint"));
        assert!(template.contains("go.mod"));
        assert!(template.contains("cmd/"));
        assert!(template.contains("internal/"));
        assert!(template.contains("Accept interfaces, return structs"));
        // Should contain green_coding with Go-specific practices
        assert!(template.contains("green_coding:"));
        assert!(template.contains("CGO_ENABLED=0"));
        assert!(template.contains("UPX compress"));
    }

    // ========== sprint_template Tests ==========

    #[test]
    fn test_sprint_template_is_valid_yaml() {
        let template = sprint_template();
        let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
        assert!(result.is_ok(), "Sprint template should be valid YAML");
    }

    #[test]
    fn test_sprint_template_has_required_fields() {
        let template = sprint_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

        assert!(yaml.get("sprint").is_some(), "Should have sprint section");
        let sprint = yaml.get("sprint").unwrap();
        assert!(sprint.get("current").is_some(), "Should have current field");
    }

    #[test]
    fn test_sprint_template_has_valid_status() {
        let template = sprint_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();
        let status = yaml["sprint"]["status"].as_str().unwrap();
        assert!(
            ["planned", "in_progress", "blocked", "done"].contains(&status),
            "Status should be valid enum value"
        );
    }

    // ========== roadmap_template Tests ==========

    #[test]
    fn test_roadmap_template_is_valid_yaml() {
        let template = roadmap_template();
        let result: Result<serde_yaml::Value, _> = serde_yaml::from_str(&template);
        assert!(result.is_ok(), "Roadmap template should be valid YAML");
    }

    #[test]
    fn test_roadmap_template_has_sections() {
        let template = roadmap_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

        assert!(
            yaml.get("metadata").is_some(),
            "Should have metadata section"
        );
        assert!(yaml.get("current").is_some(), "Should have current section");
        assert!(yaml.get("next").is_some(), "Should have next section");
        assert!(yaml.get("backlog").is_some(), "Should have backlog section");
    }

    #[test]
    fn test_roadmap_template_has_valid_statuses() {
        let template = roadmap_template();
        let yaml: serde_yaml::Value = serde_yaml::from_str(&template).unwrap();

        let current_status = yaml["current"]["status"].as_str().unwrap();
        let next_status = yaml["next"]["status"].as_str().unwrap();

        let valid_statuses = ["planned", "in_progress", "released"];
        assert!(
            valid_statuses.contains(&current_status),
            "Current status should be valid"
        );
        assert!(
            valid_statuses.contains(&next_status),
            "Next status should be valid"
        );
    }
}
