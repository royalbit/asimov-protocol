# ADR-037: Remove check and schema Commands

**Status:** Accepted
**Date:** 2025-12-03
**Deciders:** Human + Claude (Principal Autonomous AI)

## Context

During comprehensive validation of all Asimov command behaviors (v8.15.1), two commands were identified as unnecessary:

1. **`asimov check <FILE>`** - An alias for `asimov validate <FILE>`
2. **`asimov schema`** - Export JSON schemas for IDE integration (VS Code YAML validation)

## The Problem

### Check Command

The `check` command was a redundant alias for `validate` with no additional functionality. During validation review:

- User: "let's remove it -- validate should be without arguments, validate ALL"
- The simplified `validate` command (no args, validates everything) eliminates the need for a single-file alias

**Redundancy provides no value.**

### Schema Command

The `schema` command exported JSON schemas for editor integration (VS Code YAML autocomplete, error highlighting).

But Asimov is a **CLI for autonomous AI development**:

- Protocols are **hardcoded in the binary** (not user-edited YAML)
- Only `roadmap.yaml` and `project.yaml` are user files
- The workflow is **Claude Code** → not IDE editing

User feedback: "FUCK vscode... this is CLI, autonomous mode -- NO IDE integration"

**IDE integration solves a problem that doesn't exist in the actual workflow.**

## Decision

**Remove both commands.**

### Removed Code

1. `Commands::Check` enum variant
2. `Commands::Schema` enum variant
3. `cmd_schema()` function (~110 lines)
4. All E2E tests for check and schema (8 tests)
5. Unused schema imports (`ASIMOV_SCHEMA`, `FRESHNESS_SCHEMA`, etc.)

### Rationale

1. **Green Coding (ADR-012):** Don't maintain what provides no value
2. **CLI Focus:** Asimov is a terminal tool, not an IDE plugin
3. **Reduced Surface Area:** Fewer commands = simpler CLI = easier documentation
4. **Honest Architecture:** The integration layer is Claude Code hooks, not editor extensions

## Consequences

### Positive

- Simpler CLI (10 commands instead of 12)
- ~120 lines of code removed
- 8 fewer tests to maintain
- Clearer product focus

### Negative

- Users who manually edit protocol files lose schema validation in editors
- Minor impact - they can use generic YAML validation

### Neutral

- `validate` command will be redesigned to validate everything (protocols, roadmap, project.yaml)

## Implementation

1. Remove `Check` and `Schema` from `Commands` enum
2. Remove match arms for both commands
3. Remove `cmd_schema()` function
4. Remove E2E tests (`e2e_check_*`, `e2e_schema_*`)
5. Remove unused schema constant imports
6. Update `--help` output (automatic via clap)

## References

- [ADR-012: Hardcoded Green Coding](012-hardcoded-green-coding.md) - Don't build what provides no value
- [ADR-030: asimov-zed Extension Killed](030-asimov-zed-extension-killed.md) - Similar "no IDE integration" decision
- [ADR-018: Claude Code Hooks Integration](018-claude-code-hooks-integration.md) - The actual integration layer
