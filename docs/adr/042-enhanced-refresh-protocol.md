# ADR-042: Enhanced Refresh Protocol

**Status:** Accepted
**Date:** 2025-12-06
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

The `asimov refresh` command currently:
1. Regenerates protocol JSON files (asimov.json, green.json, etc.)
2. Validates roadmap.yaml (auto-creates skeleton if missing)

It does NOT:
1. Detect missing `identity.type` in project.yaml
2. Upgrade `coding_standards` section when templates evolve
3. Help migrate existing projects to new template formats

### The Problem

When we update templates (like adding documentation/architecture sections to coding_standards in v9.4.0), existing projects don't benefit from the improvements. Users must manually update their project.yaml files.

### Real Scenario

A project initialized with v9.2.0 has:
```yaml
coding_standards:
  file_size: { soft_limit: 1000 }
  coverage: "100%"
```

After upgrading to v9.4.0, the template now has:
```yaml
coding_standards:
  code:
    file_size: { soft_limit: 1000 }
    coverage: "100%"
  documentation:
    linting: "markdownlint-cli2"
  architecture:
    decisions: "ADR format in docs/adr/"
```

The project doesn't automatically get these improvements.

## Decision

### Enhanced Refresh Behavior

```
asimov refresh
    │
    ├─► Check protocol JSON files (existing behavior)
    │
    ├─► Check project.yaml
    │   │
    │   ├─► identity.type missing?
    │   │       │
    │   │       ├─► Interactive: prompt user with numbered list
    │   │       │   "Project type not detected. Please select:
    │   │       │    1. rust
    │   │       │    2. python
    │   │       │    ..."
    │   │       │
    │   │       └─► --yes flag: auto-detect from markers
    │   │
    │   └─► coding_standards outdated?
    │           │
    │           ├─► Compare with template for detected type
    │           │
    │           ├─► Interactive: show diff and prompt
    │           │   "[A]ccept template / [K]eep current / [M]erge"
    │           │
    │           └─► --yes flag: accept template defaults
    │
    └─► Validate roadmap.yaml (existing behavior)
```

### CLI Changes

```bash
asimov refresh           # Interactive mode (prompts when needed)
asimov refresh --yes     # Non-interactive (auto-accept template defaults)
asimov refresh --dry-run # Show what would change without writing
```

### Project Type Selection

When `identity.type` is missing or invalid:

```
Project type not detected. Please select:
  1. rust
  2. python
  3. node
  4. go
  5. flutter
  6. docs
  7. arch
  8. generic
Enter number [1-8]:
```

### Coding Standards Upgrade

When coding_standards differs from current template:

```
coding_standards has been updated in template v9.4.0:

Current:
  file_size: { soft_limit: 1000 }
  coverage: "100%"

Template:
  code:
    file_size: { soft_limit: 1000 }
    coverage: "100%"
  documentation:
    linting: "markdownlint-cli2"
  architecture:
    decisions: "ADR format in docs/adr/"

[A]ccept template / [K]eep current?
```

### Preservation Rules

1. **Always preserve:**
   - `identity.name` (user-defined)
   - `identity.tagline` (user-defined)
   - `identity.version` (user-defined)
   - Any custom fields not in template

2. **Upgrade candidates:**
   - `coding_standards` (compare with template)
   - `quality` (if template has improvements)
   - `deliverables_template` (if template updated)

3. **Never touch:**
   - `patterns` (project-specific)
   - `files` (project-specific paths)

## Implementation

### Files Modified

- `cli/src/commands/refresh.rs` - Add migration logic
- `cli/src/main.rs` - Add `--yes` and `--dry-run` flags
- `cli/src/templates/mod.rs` - Add `get_template_coding_standards()`

### New Functions

```rust
/// Get coding_standards section for a project type
pub fn get_template_coding_standards(project_type: ProjectType) -> serde_yaml::Value

/// Compare two coding_standards and detect if upgrade needed
pub fn needs_coding_standards_upgrade(
    current: &serde_yaml::Value,
    template: &serde_yaml::Value
) -> bool

/// Prompt user for project type selection
pub fn prompt_project_type() -> ProjectType

/// Prompt user for coding_standards upgrade
pub fn prompt_coding_standards_upgrade(diff: &str) -> UpgradeChoice
```

## Consequences

### Positive

1. **Seamless upgrades** - Projects benefit from template improvements
2. **User control** - Interactive prompts let users decide
3. **Non-breaking** - `--yes` for CI/automation
4. **Visibility** - Shows exactly what would change

### Negative

1. **Complexity** - More logic in refresh command
2. **Prompts** - Interactive mode requires stdin

### Neutral

1. **Backwards compatible** - Old projects continue to work
2. **Opt-in upgrades** - User must explicitly accept changes

## Related

- [ADR-032: Project Context File](032-project-context-file.md)
- [ADR-041: Coding Standards Protocol](041-coding-standards-protocol.md)

---

**Previous:** [ADR-041](041-coding-standards-protocol.md)

---
