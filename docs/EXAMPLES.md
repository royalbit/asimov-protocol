# Asimov Protocol Examples

Real-world configurations for different project types.

## Minimal (Any Project)

The simplest possible configuration:

```yaml
# warmup.yaml
identity:
  project: "My Project"

files:
  source:
    - "src/ - Source code"

session:
  start:
    - "Read warmup.yaml"
```

## Rust Project

```yaml
# warmup.yaml
identity:
  project: "my-rust-app"
  version: "0.1.0"

files:
  source:
    - "src/main.rs - Entry point"
    - "src/lib.rs - Library root"
  config:
    - "Cargo.toml - Dependencies"

session:
  start:
    - "Read warmup.yaml"
    - "cargo test"
  end:
    - "cargo test"
    - "cargo clippy"

quality:
  tests: "cargo test must pass"
  warnings: "cargo clippy -- -D warnings"

style:
  rust:
    - "Result<T, E> for errors"
    - "thiserror for custom errors"
```

## Python Project

```yaml
# warmup.yaml
identity:
  project: "my-python-app"
  version: "1.0.0"

files:
  source:
    - "src/main.py - Entry point"
    - "src/models/ - Data models"
  config:
    - "pyproject.toml - Project config"
    - "requirements.txt - Dependencies"
  tests:
    - "tests/ - Test suite"

session:
  start:
    - "Read warmup.yaml"
    - "pytest"
  end:
    - "pytest"
    - "ruff check"

quality:
  tests: "pytest must pass"
  linting: "ruff check --fix"
  typing: "mypy src/"
```

## JavaScript/TypeScript Project

```yaml
# warmup.yaml
identity:
  project: "my-web-app"
  version: "2.0.0"

files:
  source:
    - "src/index.ts - Entry point"
    - "src/components/ - React components"
  config:
    - "package.json - Dependencies"
    - "tsconfig.json - TypeScript config"

session:
  start:
    - "Read warmup.yaml"
    - "npm test"
  end:
    - "npm test"
    - "npm run lint"

quality:
  tests: "npm test must pass"
  types: "tsc --noEmit"
```

## Documentation Project

```yaml
# warmup.yaml
identity:
  project: "project-docs"
  tagline: "Documentation as code"

files:
  docs:
    - "docs/ - All documentation"
    - "README.md - Landing page"

session:
  start:
    - "Read warmup.yaml"
  during:
    - "Use Mermaid for diagrams"
    - "Keep pages focused"

style:
  docs:
    - "Markdown only"
    - "Mermaid for diagrams"
    - "No ASCII art"
    - "GitHub-compatible"
```

## With Sprint Tracking

```yaml
# warmup.yaml
identity:
  project: "active-project"

protocol:
  files:
    - "warmup.yaml"
    - "sprint.yaml"
    - "roadmap.yaml"

session:
  start:
    - "Read warmup.yaml"
    - "Read sprint.yaml"
    - "git status"
```

```yaml
# sprint.yaml
sprint:
  current: "Add user authentication"
  started: "2025-01-15"
  status: in_progress
  tasks:
    - "[x] Design auth flow"
    - "[x] Implement JWT tokens"
    - "[ ] Add password reset"
    - "[ ] Write tests"
  blockers: []
```

```yaml
# roadmap.yaml
metadata:
  current_version: "1.2.0"
  last_updated: "2025-01-15"

current:
  version: "1.2.0"
  status: released
  summary: "OAuth integration"

next:
  version: "1.3.0"
  status: in_progress
  summary: "User authentication"
  features:
    - "JWT tokens"
    - "Password reset"
    - "Session management"

backlog:
  - "Two-factor auth"
  - "SSO support"
```

## Multi-Language Monorepo

```yaml
# warmup.yaml
identity:
  project: "platform"
  tagline: "Full-stack application"

files:
  backend:
    - "backend/src/ - Rust API"
    - "backend/Cargo.toml"
  frontend:
    - "frontend/src/ - React app"
    - "frontend/package.json"
  shared:
    - "proto/ - Protocol buffers"
    - "docs/ - Documentation"

session:
  start:
    - "Read warmup.yaml"
    - "Identify which component to work on"
  during:
    - "Stay focused on one component"
    - "Update shared types if needed"

quality:
  backend: "cargo test && cargo clippy"
  frontend: "npm test && npm run lint"
```

## Flutter/Dart Project

Mobile-first projects with iOS/Android considerations:

```yaml
# warmup.yaml
identity:
  project: "my-mobile-app"
  version: "0.1.0"

files:
  source:
    - "lib/main.dart - Entry point"
    - "lib/screens/ - Screen widgets"
    - "lib/widgets/ - Reusable components"
    - "lib/services/ - API and data services"
  config:
    - "pubspec.yaml - Dependencies"
    - "analysis_options.yaml - Lint rules"
  assets:
    - "assets/fixtures/ - Demo data (JSON)"
  tests:
    - "test/widgets/ - Widget tests"
    - "test/services/ - Unit tests"

session:
  start:
    - "Read warmup.yaml"
    - "flutter test"
    - "flutter analyze"
  end:
    - "dart format lib/ test/"
    - "flutter analyze"
    - "flutter test"

quality:
  tests: "flutter test must pass"
  analyzer: "flutter analyze (zero warnings)"
  format: "dart format lib/ test/"

style:
  flutter:
    - "Prefer const constructors"
    - "Use RepaintBoundary for expensive widgets"
    - "StatelessWidget when possible"
```

## Business Documentation Project

Non-code projects with financial models, grant applications, or validated YAML:

```yaml
# warmup.yaml
identity:
  project: "acme-business"
  tagline: "Grant applications and financial planning"

files:
  models:
    - "models/assumptions.yaml - Financial assumptions"
    - "models/projections.yaml - Revenue projections"
  grants:
    - "grants/grant-application.md - Main application"
    - "grants/budget.yaml - Grant budget"
  references:
    - "references/data-sources.yaml - Dynamic data with dates"

session:
  start:
    - "Read warmup.yaml"
    - "Check data-sources.yaml for stale data (>90 days)"
  during:
    - "Cite sources with verification dates"
    - "Use conservative estimates"
  end:
    - "Validate YAML files (yamllint)"
    - "Validate markdown (markdownlint)"

quality:
  yaml: "yamllint models/ grants/"
  markdown: "markdownlint-cli2 '**/*.md'"
  formulas: "All calculations must be verifiable"

dynamic_data:
  description: "Track time-sensitive information"
  file: "references/data-sources.yaml"
  stale_threshold: "90 days"
  categories:
    - "Market data (refresh quarterly)"
    - "Grant programs (refresh monthly)"
    - "Competitor pricing (refresh quarterly)"
```

## Multi-repo Ecosystem

When your project spans multiple repositories that work together:

```yaml
# In each repo: warmup.yaml
# Coordinates with sibling repos via shared conventions

# === repo: acme-api/warmup.yaml ===
identity:
  project: "acme-api"
  ecosystem: "acme"  # Links related repos

files:
  source:
    - "src/ - Rust backend"
  docs:
    - "docs/ - API documentation"

session:
  start:
    - "Read warmup.yaml"
    - "Check ecosystem status if cross-repo work"

ecosystem:
  repos:
    - name: "acme-api"
      role: "Backend API"
      warmup: "acme-api/warmup.yaml"
    - name: "acme-mobile"
      role: "Flutter app"
      warmup: "acme-mobile/warmup.yaml"
    - name: "acme-web"
      role: "React frontend"
      warmup: "acme-web/warmup.yaml"
    - name: "acme-docs"
      role: "Business documentation"
      warmup: "acme-docs/warmup.yaml"

  coordination:
    master_roadmap: "acme-docs/master-roadmap.yaml"
    shared_conventions:
      - "API contracts in acme-api/api/openapi.yaml"
      - "Shared types generated from OpenAPI"
```

See [Ecosystem Pattern](ECOSYSTEM_PATTERN.md) for the master-roadmap orchestration pattern.
