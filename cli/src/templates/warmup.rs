//! Warmup template generators for different project types

use super::ProjectType;

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
# See: asimov.yaml for full configuration (ADR-031)
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
# See: asimov.yaml for full configuration (ADR-031)
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

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
        r#"# RoyalBit Asimov - Session Bootstrap
# https://github.com/royalbit/asimov

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
# ETHICS - Humanist Mode (HIGHEST PRIORITY)
# ═══════════════════════════════════════════════════════════════════════════════
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."

# ═══════════════════════════════════════════════════════════════════════════════
# GREEN CODING - Zero tokens. Zero emissions.
# ═══════════════════════════════════════════════════════════════════════════════
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use markdownlint/asimov lint-docs for validation (not AI)"
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
    - "asimov lint-docs (verify baseline)"
  during:
    - "Track progress"
    - "Validate frequently"
    - "Small, logical commits"
  end:
    - "asimov lint-docs (all pass)"
    - "markdownlint '**/*.md' (if installed)"
    - "Review rendered output"

quality:
  lint: "asimov lint-docs ."
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(!template.contains("cargo"));
        assert!(!template.contains("Cargo.toml"));
        assert!(template.contains("src/ - Source code"));
        assert!(template.contains("green_coding:"));
    }

    #[test]
    fn test_warmup_template_rust_contains_rust_specific() {
        let template = warmup_template("rust-project", ProjectType::Rust);
        assert!(template.contains("rust-project"));
        assert!(template.contains("cargo test"));
        assert!(template.contains("cargo clippy"));
        assert!(template.contains("Cargo.toml"));
        assert!(template.contains("green_coding:"));
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
            assert!(
                yaml.get("identity").is_some(),
                "Should have identity section for {:?}",
                project_type
            );
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
        assert!(template.contains("pytest"));
        assert!(template.contains("ruff"));
        assert!(template.contains("pyproject.toml"));
    }

    #[test]
    fn test_warmup_template_node_contains_node_specific() {
        let template = warmup_template("node-project", ProjectType::Node);
        assert!(template.contains("npm test"));
        assert!(template.contains("eslint"));
        assert!(template.contains("package.json"));
    }

    #[test]
    fn test_warmup_template_go_contains_go_specific() {
        let template = warmup_template("go-project", ProjectType::Go);
        assert!(template.contains("go test"));
        assert!(template.contains("golangci-lint"));
        assert!(template.contains("go.mod"));
    }

    #[test]
    fn test_warmup_template_flutter_contains_flutter_specific() {
        let template = warmup_template("test-app", ProjectType::Flutter);
        assert!(template.contains("flutter") || template.contains("dart"));
    }

    #[test]
    fn test_warmup_template_docs_contains_docs_specific() {
        let template = warmup_template("test-docs", ProjectType::Docs);
        assert!(template.contains("documentation") || template.contains("markdown"));
    }
}
