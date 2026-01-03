# ADR-063: Clean Project Templates - Remove Unnecessary Sections

## Status

Accepted

## Context

Project templates currently include two sections that have become confusing and unnecessary:

1. **`deliverables_template`** (introduced in v8.8.0, ADR-034) - A checklist of deliverables for coding projects
2. **`references`** (introduced in v10.3.0) - Reference management configuration for AI research sources

These sections cause confusion because:

- **deliverables_template**: Was intended as a helper checklist but overlaps with the AI's native task tracking capabilities. AI assistants already maintain their own task lists and don't need predefined checklists embedded in project configuration.

- **references**: The `ref` tool integration is handled at the CLI level, not per-project configuration. Projects that need reference management can add it manually, but including it in every template adds noise.

Both sections add visual clutter to project.yaml files without providing meaningful value.

## Decision

Remove `deliverables_template` and `references` sections from all 21 project templates:

**Base templates (8):**
- rust.yaml, python.yaml, node.yaml, go.yaml, flutter.yaml, docs.yaml, generic.yaml, arch.yaml

**API templates (5):**
- api-rust.yaml, api-go.yaml, api-fastapi.yaml, api-nestjs.yaml, api-spring.yaml

**Web templates (4):**
- web-nextjs.yaml, web-react.yaml, web-vue.yaml, web-angular.yaml

**Monorepo templates (3):**
- mono-turbo.yaml, mono-nx.yaml, mono-pnpm.yaml

**Admin template (1):**
- admin-dashboard.yaml

## Consequences

### Positive

- Cleaner, more focused project.yaml files
- Less confusion about what sections are required vs optional
- Reduced template maintenance burden
- Users can still add custom sections if needed

### Negative

- Projects using these sections will need to migrate if they depend on them
- Minor breaking change for existing workflows that read these sections

## Migration

Existing projects with `deliverables_template` or `references` sections in their `.asimov/project.yaml` will continue to work. These sections are simply ignored by the CLI.

To migrate: Remove the sections from your project.yaml or leave them if you have custom automation that reads them.
