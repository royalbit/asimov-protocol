# Ecosystem Pattern: Cross-repo Orchestration

Use the master-roadmap pattern to coordinate work across multiple repositories while maintaining independent warmup.yaml files in each.

## When to Use This Pattern

- Multiple related repositories (API, mobile app, web frontend, docs)
- Shared dependencies between repos (API contracts, types, conventions)
- Phased development where work in one repo enables work in another
- Team coordination across different codebases

## Architecture

```text
acme-ecosystem/
├── acme-api/
│   └── warmup.yaml          # Backend-specific context
├── acme-mobile/
│   └── warmup.yaml          # Flutter-specific context
├── acme-web/
│   └── warmup.yaml          # React-specific context
└── acme-docs/
    ├── warmup.yaml          # Documentation context
    └── master-roadmap.yaml  # Cross-repo orchestration
```

## Per-repo warmup.yaml

Each repository has its own warmup.yaml with repo-specific context:

```yaml
# acme-api/warmup.yaml
identity:
  project: "acme-api"
  ecosystem: "acme"
  role: "Backend API (Rust)"

files:
  source:
    - "src/ - API implementation"
  api:
    - "api/openapi.yaml - API contract (shared with other repos)"

session:
  start:
    - "Read warmup.yaml"
    - "cargo test"
  end:
    - "cargo test"
    - "cargo clippy -- -D warnings"

# Reference to ecosystem coordination
ecosystem:
  master_roadmap: "../acme-docs/master-roadmap.yaml"
  dependencies:
    - "acme-mobile depends on this API"
    - "acme-web depends on this API"
```

## Master Roadmap Structure

The master-roadmap.yaml lives in one repo (typically docs) and coordinates work across all repos:

```yaml
# acme-docs/master-roadmap.yaml
meta:
  version: "1.0.0"
  goal: "Launch MVP with core features"
  status: "ACTIVE"

  # What's already done (don't rebuild)
  completed:
    - "Core API endpoints"
    - "Database schema"
    - "Authentication flow"

  # What to reuse across repos
  shared_assets:
    - "API contract: acme-api/api/openapi.yaml"
    - "Design system: acme-mobile/lib/theme/"
    - "Type definitions: Generated from OpenAPI"

# =============================================================================
# PHASES - Ordered work across repos
# =============================================================================

phase_1_foundation:
  name: "Foundation"
  description: "Verify existing systems work"

  steps:
    - id: "1.1"
      task: "Run all existing tests"
      repo: "acme-api"
      command: "cargo test"
      acceptance: "All tests pass"

    - id: "1.2"
      task: "Verify API health"
      repo: "acme-api"
      command: "curl http://localhost:8080/health"
      acceptance: "Returns 200 OK"

  gate: "All existing functionality verified"

phase_2_backend_auth:
  name: "Backend Authentication"
  description: "Add OAuth to API"
  repo: "acme-api"
  warmup: "acme-api/warmup.yaml"

  dependencies:
    - phase_1_foundation

  steps:
    - id: "2.1"
      task: "User model with OAuth"
      details:
        - "User: id, email, name, oauth_provider, oauth_id"
        - "No password field - OAuth only"
      tests:
        - "User creation from OAuth data"
        - "Duplicate OAuth ID rejected"

    - id: "2.2"
      task: "OAuth endpoints"
      endpoints:
        - "GET /api/v1/auth/google"
        - "GET /api/v1/auth/google/callback"
        - "POST /api/v1/auth/logout"
      tests:
        - "OAuth flow redirects correctly"
        - "Callback creates new user"
        - "Callback returns JWT for existing user"

  acceptance_criteria:
    - "OAuth login works end-to-end"
    - "JWT issued after OAuth"
    - "Existing tests still pass"

  gate: "Authentication working"

phase_3_shared_core:
  name: "Shared Flutter Core"
  description: "Create shared package for mobile apps"
  repo: "acme-flutter-core"

  dependencies:
    - phase_2_backend_auth

  steps:
    - id: "3.1"
      task: "Create shared Flutter package"
      details:
        - "Theme, widgets, utils shared across apps"
        - "Export as package for apps to import"

    - id: "3.2"
      task: "Auth service (shared)"
      file: "lib/services/auth_service.dart"
      details:
        - "signInWithGoogle() -> JWT"
        - "Store JWT securely"
        - "Auto-refresh before expiry"

  gate: "Shared core package ready"

phase_4_mobile_app:
  name: "Mobile App"
  description: "Build Flutter app using shared core"
  repo: "acme-mobile"

  dependencies:
    - phase_3_shared_core

  steps:
    - id: "4.1"
      task: "Import shared package"
      details:
        - "Add acme_core to pubspec.yaml"
        - "Use shared theme and widgets"

    - id: "4.2"
      task: "Dashboard screen"
      screens:
        - "dashboard.dart - Main view"
      features:
        - "Display user data from API"
        - "Use shared components"

  acceptance_criteria:
    - "App builds and runs"
    - "Login with OAuth works"
    - "Dashboard shows real data"

  gate: "Mobile app functional"

# =============================================================================
# EXECUTION
# =============================================================================

execution:
  method: "Phase by phase, repo by repo"

  per_phase:
    - "Read master-roadmap.yaml for phase context"
    - "Read target repo's warmup.yaml for repo context"
    - "Execute steps in order"
    - "Write tests first"
    - "Commit after each step"
    - "Human review at phase gates only"

  human_involvement:
    - "Review at phase gates"
    - "Provide API keys and credentials"
    - "Production deployment decisions"
```

## Key Concepts

### Phase Dependencies

Phases declare dependencies to ensure correct ordering:

```yaml
phase_4_mobile_app:
  dependencies:
    - phase_2_backend_auth    # API must have auth
    - phase_3_shared_core     # Shared package must exist
```

### Repo Targeting

Each phase specifies which repo it targets:

```yaml
phase_2_backend_auth:
  repo: "acme-api"
  warmup: "acme-api/warmup.yaml"
```

### Phase Gates

Each phase has acceptance criteria and a gate:

```yaml
phase_2_backend_auth:
  acceptance_criteria:
    - "OAuth login works"
    - "JWT issued after OAuth"
    - "Existing tests still pass"

  gate: "Authentication working"
```

### Shared Assets

Track what's reusable across repos:

```yaml
meta:
  shared_assets:
    - "API contract: acme-api/api/openapi.yaml"
    - "Design system: acme-mobile/lib/theme/"
```

## Workflow

1. **Start session**: Read master-roadmap.yaml for current phase
2. **Switch to repo**: Read that repo's warmup.yaml for context
3. **Execute phase**: Work through steps, write tests, commit
4. **Gate review**: Human reviews at phase completion
5. **Next phase**: Move to next phase, possibly different repo

## Benefits

- **Independence**: Each repo maintains own warmup.yaml
- **Coordination**: Master roadmap ensures correct ordering
- **Visibility**: Clear picture of what's done
- **Autonomy**: AI works through phases with minimal interruption
- **Quality**: Phase gates enforce standards

---
