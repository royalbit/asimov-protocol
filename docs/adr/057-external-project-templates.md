# ADR-057: External Project Templates with Embedded Fallback

**Status:** Implemented (v10.3.0, refined v10.3.1)
**Date:** 2026-01-01
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Extends:** ADR-053 (External Protocol and Template Architecture)
**Supersedes:** Hardcoded templates in `cli/src/templates/*.rs`

---

## Context

### Current State

Project templates are currently implemented as **hardcoded Rust code** that generates YAML strings:

```rust
// cli/src/templates/project.rs
pub fn project_template(project_type: ProjectType) -> String {
    match project_type {
        ProjectType::Rust => include_str!("project-rust.yaml.tpl").to_string(),
        ProjectType::Python => include_str!("project-python.yaml.tpl").to_string(),
        // ... mixed approach: some .tpl files, some inline code
    }
}
```

**Problems:**
1. **Inconsistent** - Mix of `.tpl` files and inline Rust code
2. **Hard to edit** - Template changes require understanding Rust
3. **Not packageable** - `.tpl` files use relative paths outside crate (same issue we fixed for protocols in v10.2.4)
4. **Duplication** - Enterprise templates in `.asimov/templates/` duplicate embedded templates
5. **No single source of truth** - Templates scattered across multiple locations

### The Protocol Pattern (v10.2.4)

We successfully moved protocols to:
```
cli/protocols/*.json          ← Canonical source (baked in via include_str!)
.asimov/protocols/*.json      ← Runtime overrides (user customization)
```

This same pattern should apply to project templates.

## Decision

### 1. Template File Structure

**v10.3.1 Update:** Flat directory structure (no subdirectories):

```
cli/
├── protocols/                 # Already done (v10.2.4)
│   ├── asimov.json
│   └── ...
├── templates/                 # All 21 templates in flat directory
│   ├── rust.yaml              # Base templates (8)
│   ├── python.yaml
│   ├── node.yaml
│   ├── go.yaml
│   ├── flutter.yaml
│   ├── docs.yaml
│   ├── arch.yaml
│   ├── generic.yaml
│   ├── api-rust.yaml          # API templates (5)
│   ├── api-go.yaml
│   ├── api-fastapi.yaml
│   ├── api-nestjs.yaml
│   ├── api-spring.yaml
│   ├── web-nextjs.yaml        # Web templates (4)
│   ├── web-react.yaml
│   ├── web-vue.yaml
│   ├── web-angular.yaml
│   ├── mono-turbo.yaml        # Monorepo templates (3)
│   ├── mono-nx.yaml
│   ├── mono-pnpm.yaml
│   ├── admin-dashboard.yaml   # Other (1)
│   ├── warmup/                # Warmup-specific templates
│   │   └── ...
│   └── hooks/                 # Hook templates
│       └── ...
└── src/templates/mod.rs       # include_str! loading
```

**Why flat?** v10.3.1 simplified from subdirectories (project/, enterprise/) to a flat structure. All templates are first-class citizens - no "enterprise" vs "project" distinction.

### 2. Embedding Pattern

```rust
// cli/src/templates/project.rs

// All templates embedded at compile time (flat structure)
const TEMPLATE_RUST: &str = include_str!("../../templates/rust.yaml");
const TEMPLATE_PYTHON: &str = include_str!("../../templates/python.yaml");
const TEMPLATE_API_RUST: &str = include_str!("../../templates/api-rust.yaml");
const TEMPLATE_WEB_NEXTJS: &str = include_str!("../../templates/web-nextjs.yaml");
// ... all 21 templates

/// Unified template lookup (v10.3.1)
pub fn get_template_by_name(name: &str) -> Option<String> {
    // 1. Check .asimov/templates/{name}.yaml (runtime override)
    // 2. Fall back to embedded template
}
```

### 3. Template Lifecycle

```
                    INIT TIME                         RUNTIME
                    ─────────                         ───────
Template (rust.yaml)  ──[asimov init]──>  .asimov/project.yaml  ──[asimov warmup]──>  JSON
     ↑                                           ↑
   embedded                                 user-editable
   (compile-time)                           (source of truth)
```

**Flow:**
1. `asimov init -t rust -n myproject` loads rust.yaml template
2. Replaces `{PROJECT_NAME}` and `{PROJECT_TAGLINE}` placeholders
3. Writes result to `.asimov/project.yaml`
4. `asimov warmup` reads `.asimov/project.yaml` (NOT the template)

Templates are **one-time generators**. After init, project.yaml is the source of truth.

### 4. Runtime Override Hierarchy

```
.asimov/templates/rust.yaml     ← User override (highest priority)
cli/templates/rust.yaml         ← Embedded fallback (compile-time)
```

**Override logic (at init time):**
1. Check `.asimov/templates/{name}.yaml`
2. If not found, use embedded default
3. Generate project.yaml from template

### 5. Template Format

All templates use **YAML** (not `.tpl` pseudo-format):

```yaml
# cli/templates/project/rust.yaml
# Canonical Rust project template - embedded at compile time
# Override: .asimov/templates/project/rust.yaml

name: "{PROJECT_NAME}"
tagline: "Built with Rust"
type: rust

identity:
  mission: "..."

quality:
  lint: "cargo clippy -- -D warnings"
  test: "cargo test"
  build: "cargo build --release"

# ADR-034: Standard deliverables for coding projects
deliverables_template:
  - "[ ] Unit tests pass"
  - "[ ] E2E tests pass (if applicable)"
  - "[ ] cargo clippy -- -D warnings (zero warnings)"
  - "[ ] Update README if needed"
  - "[ ] Update --help if CLI"
  - "[ ] Commit and push"
  - "[ ] GitHub release (if applicable)"
```

### 6. Deprecation of .asimov/templates/

The 21 enterprise templates currently in `.asimov/templates/` will be:
1. Moved to `cli/templates/enterprise/`
2. Baked into the binary
3. `.asimov/templates/` becomes **override-only** (not canonical source)

**Migration:**
- `asimov refresh` will NOT regenerate `.asimov/templates/` by default
- Custom templates in `.asimov/templates/` continue to work (override)
- `asimov init --template api-rust` uses embedded template

## Consequences

### Positive

1. **Single source of truth** - All templates in `cli/templates/`
2. **Packageable** - Works with `cargo publish` (no path issues)
3. **Editable** - YAML files, not Rust code
4. **Consistent** - Same pattern as protocols
5. **User customization** - Override via `.asimov/templates/`
6. **Enterprise templates included** - No separate download/install

### Negative

1. **Binary size increase** - ~50KB more (21 enterprise templates)
   - Acceptable trade-off for consistency
2. **Migration effort** - Move existing `.tpl` files to YAML
   - One-time effort, improves maintainability

### Neutral

1. **Performance** - Same as protocols (negligible file I/O vs embedded)
2. **Backward compatibility** - Existing `.asimov/templates/` overrides work

## Implementation

### v10.3.0: Initial Implementation
- Created `cli/templates/project/` and `cli/templates/enterprise/` subdirectories
- Embedded 8 base + 13 extended templates via `include_str!`
- Added `deliverables_template` to all templates
- Added `templates_available` to warmup output

### v10.3.1: Simplification
- Merged to flat `cli/templates/` structure (no subdirectories)
- Unified `get_template_by_name()` API for all templates
- Removed `templates_available` from warmup (templates only relevant at init time)
- Updated `asimov init --help` to list all 21 templates
- Deprecated `get_enterprise_template()` in favor of `get_template_by_name()`

### Future: ADR-034 Completion
- Implement `~inherit~` marker for auto-inheritance in roadmap parsing

## References

- [ADR-053: External Protocol and Template Architecture](053-external-protocols.md) - Extended by this ADR
- [ADR-034: Project-Type-Aware Deliverables](034-project-type-deliverables.md) - Template format includes deliverables
- v10.2.4 - Protocol packaging fix (same pattern)

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov)*
