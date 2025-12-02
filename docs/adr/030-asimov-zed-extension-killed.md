# ADR-030: asimov-zed Extension Killed

**Status:** Accepted
**Date:** 2025-12-02
**Deciders:** Human + Claude (Principal Autonomous AI)

## Context

In v7.10.0, we shipped [asimov-zed](https://github.com/royalbit/asimov-zed) - a Zed editor extension providing syntax highlighting for Asimov protocol files (`.asimov/*.yaml`).

The extension was modeled after [forge-zed](https://github.com/royalbit/forge-zed), which provides syntax highlighting for Forge formula files that users actively edit by hand.

## The Problem

**We don't edit YAML files manually.**

The Asimov workflow is:
1. Claude Code reads protocol files autonomously
2. Claude Code validates via `asimov validate`
3. Claude Code edits/commits autonomously
4. Human reviews PRs (not raw YAML in an editor)

The asimov-zed extension solves a problem that doesn't exist in the actual workflow.

| Extension | User Edits Files? | Value |
|-----------|-------------------|-------|
| **forge-zed** | Yes - formulas are hand-authored | Real |
| **asimov-zed** | No - AI authors protocol files | None |

## Decision

**Kill asimov-zed.** Delete the repository entirely (local + gitolite origin) and remove all references.

### Rationale

1. **Green Coding (ADR-012):** Don't maintain what provides no value
2. **Scope Discipline:** Extension was scope creep from forge-zed success
3. **Honest Architecture:** The integration layer is Claude Code hooks, not editor extensions

### What asimov-zed Was

- Zed WASM extension
- Three Laws highlighting
- Protocol section detection
- Anti-pattern highlighting

### Why It Seemed Like a Good Idea

forge-zed was successful and useful. The assumption was "if it works for Forge, it works for Asimov." But:

- Forge files are **user-authored** (formulas, scenarios)
- Asimov files are **AI-authored** (protocols, roadmaps)

Different authorship model = different tooling needs.

## Consequences

### Positive

- One less repository to maintain
- Clearer architecture documentation
- Honest about actual workflow

### Negative

- External users who DO manually edit protocol files lose syntax highlighting
- Minor - they can still use generic YAML highlighting

### Neutral

- forge-zed remains valuable and maintained
- Claude Code hooks remain the true integration layer

## Implementation

1. Delete `asimov-zed` repository entirely (local + gitolite origin)
2. Remove "Editor Extensions" section from README.md
3. Update ECOSYSTEM.md to remove asimov-zed references
4. Update any other docs referencing the extension

## References

- [ADR-012: Hardcoded Green Coding](012-hardcoded-green-coding.md) - Don't build what provides no value
- [ADR-029: MCP Server Mode Killed](029-mcp-server-mode-killed.md) - Similar "don't build what won't work" decision
- [ADR-018: Claude Code Hooks Integration](018-claude-code-hooks-integration.md) - The actual integration layer
