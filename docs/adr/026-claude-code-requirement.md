# ADR-026: Claude Code Requirement

## Status

**ACCEPTED** - 2025-12-01

## Context

RoyalBit Asimov requires **Claude Code** specifically, not just Claude via MCP-enabled IDEs (Cursor, Windsurf, etc.). This ADR documents why.

### The Thinking Token Advantage

Anthropic's extended thinking recommendations:

| Setting | Tokens | Anthropic Guidance |
|---------|--------|-------------------|
| Minimum | 1,024 | Starting point |
| Complex tasks | 16k-32k | Recommended range |
| Batch threshold | 32k+ | "Use batch processing" |

Asimov's setting:

| Setting | Tokens | Multiplier |
|---------|--------|------------|
| MAX_THINKING_TOKENS | **200,000** | **6x batch threshold** |

### MCP IDE Limitations

| Tool | Standard Limit | Max/Premium | Cost |
|------|----------------|-------------|------|
| **Cursor** | 30k-48k | 200k | PAID premium tier |
| **Windsurf** | Standard | Thinking mode | 1.5x credit multiplier |
| **MCP API direct** | 1,024 min | 128k max | You control |
| **Claude Code** | **200k** | **200k** | **FREE (env var)** |

### Why This Matters

```bash
# Claude Code - FREE
export MAX_THINKING_TOKENS=200000

# Cursor - PAID for Max Mode
# Windsurf - 1.5x credit cost
# MCP direct - Capped at 128k for thinking
```

**Claude Code is the ONLY interface that:**
1. Allows 200k thinking tokens
2. Without paying premium
3. With full configuration control

### The Tradeoff

| 200k Thinking Tokens | Consequence |
|---------------------|-------------|
| Deeper reasoning per turn | ✓ |
| More thorough analysis | ✓ |
| Better autonomous decisions | ✓ |
| More frequent compaction | ✗ (~15 min) |

**Asimov's self-healing compensates for the compaction tradeoff.**

### Additional Claude Code Requirements

Beyond thinking tokens, Asimov requires:

| Feature | Claude Code | MCP IDEs |
|---------|-------------|----------|
| Terminal visibility | ✓ Hooks see output | ? Varies |
| Pre-compact hooks | ✓ Supported | ✗ Not available |
| File re-read mid-session | ✓ Works | ? Varies |
| Auto-loaded CLAUDE.md | ✓ Native | ✗ Different config |

## Decision

### Explicit Requirement

RoyalBit Asimov **requires Claude Code**. This is not a soft preference.

| Layer | Status |
|-------|--------|
| Protocol files (YAML) | Universal - works anywhere |
| CLI tools (validate, init) | Universal - just Rust |
| **Full Asimov Mode** | **Claude Code only** |

### Token Configuration

Recommended `.claude/settings.json`:

```json
{
  "env": {
    "MAX_THINKING_TOKENS": "200000"
  }
}
```

### The Stack

```
┌─────────────────────────────────────────────────────────────┐
│  Claude Opus 4.5 / Sonnet 4.5                               │
│  - The model (velocity source)                              │
├─────────────────────────────────────────────────────────────┤
│  Claude Code                                    ◄── REQUIRED │
│  - MAX_THINKING_TOKENS=200k (6x threshold)                  │
│  - Hooks (pre-compact, session-start)                       │
│  - Terminal visibility                                      │
│  - CLAUDE.md auto-loading                                   │
├─────────────────────────────────────────────────────────────┤
│  Asimov Protocol                                            │
│  - Self-healing (compensates for 200k compaction)           │
│  - Ethics, bounded autonomy, green coding                   │
└─────────────────────────────────────────────────────────────┘
```

### Why Not MCP IDEs?

| Reason | Detail |
|--------|--------|
| Token caps | 30k-48k standard vs 200k |
| Premium pricing | Extra cost for max tokens |
| Missing hooks | No pre-compact visibility |
| Different config | No CLAUDE.md equivalent |

**MCP IDEs get Claude's base capabilities but NOT full potential extraction.**

## Consequences

### Positive

1. **Maximum capability** - 200k thinking tokens
2. **No extra cost** - Free via env var
3. **Self-healing works** - Hooks compensate for compaction
4. **Clear requirement** - No ambiguity about compatibility

### Negative

1. **Vendor lock-in** - Claude Code only
2. **Not for everyone** - Excludes Cursor/Windsurf users
3. **Configuration needed** - Must set env var

### The Trade

| What You Give Up | What You Get |
|------------------|--------------|
| MCP IDE compatibility | 6x thinking depth |
| Alternative interfaces | Full Asimov Mode |
| Flexibility | Maximum autonomous capability |

## Token Efficiency

Asimov also optimizes token usage through YAML:

| Format | Tokens (same content) | vs JSON |
|--------|----------------------|---------|
| JSON | 13,869 | baseline |
| YAML | 12,333 | **-11%** |
| Markdown | 11,612 | -16% |

YAML provides the best balance of:
- Structure (machine-parseable)
- Efficiency (11% better than JSON)
- Readability (human-friendly)

## Implementation

### Documentation Updates

1. README.md - Explicit Claude Code requirement
2. VENDOR_IMPLEMENTATION.md - Why MCP IDEs don't work
3. SETUP.md - MAX_THINKING_TOKENS configuration

### CLI Updates

```bash
asimov init --asimov
# Should warn if not running in Claude Code
```

## References

- [Anthropic Extended Thinking Docs](https://platform.claude.com/docs/en/build-with-claude/extended-thinking)
- [Claude Code MAX_THINKING_TOKENS](https://github.com/anthropics/claude-code/issues/5257)
- [Cursor Token Limits](https://github.com/rinadelph/CursorPlus/blob/main/claude_token_limits.md)
- [ADR-025: Claude Attribution Principle](025-claude-attribution-principle.md)
- [ADR-018: Claude Code Hooks Integration](018-claude-code-hooks-integration.md)
