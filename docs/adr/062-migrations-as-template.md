# ADR-062: Convert Migrations Protocol to Project Template

**Status:** Accepted
**Date:** 2026-01-03
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Implements:** v10.8.0

---

## Context

The migrations protocol currently exists as a JSON protocol file (`cli/protocols/migrations.json`), loaded for all projects. However, migrations guidance is only relevant for API projects that use databases:

- api-rust (SQLx migrations)
- api-go (GORM migrations)
- api-fastapi (Alembic migrations)
- api-nestjs (TypeORM migrations)
- api-spring (JPA/Flyway migrations)

### Problems

1. **Token waste**: Non-database projects receive migrations protocol content they will never use
2. **Incorrect abstraction**: Migrations is project-specific guidance, not a universal protocol like sycophancy or freshness
3. **Confusing semantics**: The `ProjectType::Migration` enum value was meant for legacy system migrations (COBOL->Java), but the protocol was loaded for all projects

### Current State

- `cli/protocols/migrations.json` - Simple JSON with principle, strategies, red_flags
- `cli/src/templates/migrations.yaml` - Detailed YAML template for legacy system migrations
- Migrations protocol loaded via `PROTOCOL_FILES` constant for all projects
- `compile_protocols_for_type()` only includes migrations for `ProjectType::Migration`

---

## Decision

### 1. Remove Migrations from PROTOCOL_FILES

Stop generating `migrations.json` in `.asimov/protocols/` for all projects.

### 2. Add Migrations Section to API Templates

Add a `migrations` section to the 5 API templates with database-specific guidance:

```yaml
# In api-rust.yaml, api-go.yaml, api-fastapi.yaml, api-nestjs.yaml, api-spring.yaml
migrations:
  principle: "Migration complete = functionally equivalent, not just compiles"
  strategies:
    - test_parity
    - contract_testing
    - behavioral_snapshots
    - shadow_mode
  red_flags:
    - "Skipping tests for speed"
    - "Assuming compilation = correctness"
    - "Silent behavior changes"
```

### 3. Keep ProjectType::Migration

The `ProjectType::Migration` enum value and `migrations.yaml` template remain for explicit legacy system migration projects (COBOL->Java style). These are not API projects.

### 4. Update Protocol Loading

- Remove migrations from `CompiledProtocols` struct
- Remove `compile_protocols_for_type()` migrations logic
- Remove migrations from `PROTOCOL_FILES` array

---

## Implementation

| File | Change |
|------|--------|
| `cli/templates/api-rust.yaml` | Add migrations section |
| `cli/templates/api-go.yaml` | Add migrations section |
| `cli/templates/api-fastapi.yaml` | Add migrations section |
| `cli/templates/api-nestjs.yaml` | Add migrations section |
| `cli/templates/api-spring.yaml` | Add migrations section |
| `cli/src/protocols/mod.rs` | Remove migrations from PROTOCOL_FILES and structs |
| `cli/protocols/migrations.json` | Delete (no longer needed) |
| `.asimov/protocols/migrations.json` | Delete (will not be regenerated) |

---

## Consequences

### Positive
- Reduced token usage for non-database projects
- Cleaner protocol/template separation
- Database-specific guidance in relevant templates only
- Migrations protocol logic removed from protocol compilation

### Negative
- Existing projects with `.asimov/protocols/migrations.json` will have orphaned files
- Minor migration for users who relied on migrations protocol

### Migration

Existing users:
- Orphaned `migrations.json` files are harmless
- Can be manually deleted or left in place
- API projects will get migrations guidance via templates

---

*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
