# ADR-029: MCP Server Mode Killed

## Status

**ACCEPTED** - 2025-12-01

## Context

v7.4.0 was planned as "MCP Server Mode - Expose validation as MCP tools". This ADR documents why we killed it before implementation.

### What Was Planned

```
asimov mcp-server
# Expose asimov validate, asimov schema as MCP tools
# Allow Cursor, Windsurf, etc. to use Asimov validation
```

### Why We Killed It

ADR-026 already established that **Claude Code is required** for full Asimov Mode. MCP would create a degraded experience:

| Capability | Claude Code | MCP IDEs |
|------------|-------------|----------|
| Thinking tokens | **200k** | 30k-48k |
| Pre-compact hooks | ✓ | ✗ |
| Self-healing | ✓ Full | ✗ Broken |
| CLAUDE.md | ✓ Native | ✗ Different |

### The Green Coding Principle

From `green.yaml`:

> *"Ship fast. Ship small. Ship green."*

Building MCP Server Mode would:
1. **Waste development time** on a feature that can't deliver full value
2. **Mislead users** into thinking MCP IDEs work with Asimov
3. **Create support burden** for a degraded experience
4. **Violate green coding** - building code we know is inadequate

### What Already Works

The CLI already works everywhere:

```bash
# These work in ANY environment
asimov validate          # Validate protocol files
asimov init              # Initialize new project
asimov schema            # Export JSON schemas
asimov lint-docs         # Lint documentation
```

**MCP Server Mode would just wrap these commands** - no new capability, just a different interface to the same functions.

### The Token Wall

This is the fundamental blocker:

| Interface | Max Thinking | Asimov Requirement |
|-----------|--------------|-------------------|
| Claude Code | 200k | ✓ Met |
| Cursor standard | 30k-48k | ✗ 4-6x short |
| Cursor Max Mode | 200k | ✗ Premium only |
| Windsurf | Variable | ✗ 1.5x cost multiplier |
| MCP direct | 128k max | ✗ Still 36% short |

**No MCP interface provides 200k thinking tokens at no extra cost.**

## Decision

### Kill MCP Server Mode

Remove from roadmap. Do not implement.

### Document Why

This ADR serves as the permanent record of why MCP was rejected.

### Keep CLI Universal

The CLI tools remain universal. Anyone can run `asimov validate` regardless of their AI interface.

### Full Asimov Mode = Claude Code Only

This is explicit, documented, and non-negotiable until MCP interfaces support:
- 200k thinking tokens
- Pre-compact hooks
- Equivalent configuration to CLAUDE.md

## Consequences

### Positive

1. **No wasted effort** - Don't build what won't work
2. **Clear messaging** - Claude Code required, no ambiguity
3. **Green coding** - Less code, less maintenance, less carbon
4. **Honest product** - Don't mislead users about capabilities

### Negative

1. **Excludes MCP users** - Can't use full Asimov Mode
2. **Perceived limitation** - "Why doesn't it work in Cursor?"

### The Response

When asked "Why no MCP support?":

> *"Asimov requires 200k thinking tokens for full autonomous operation. MCP interfaces cap at 30k-128k. When they support 200k, we'll reconsider. Until then, the CLI tools work everywhere - full Asimov Mode requires Claude Code."*

## Future

If MCP interfaces ever support:
- 200k+ thinking tokens (free)
- Hook equivalents
- CLAUDE.md-style configuration

Then revisit this decision. Until then: **killed**.

## References

- [ADR-026: Claude Code Requirement](026-claude-code-requirement.md)
- [Green Coding Protocol](../../.asimov/green.yaml)
- [Anthropic Extended Thinking](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)
