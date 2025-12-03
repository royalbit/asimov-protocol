# ADR-036: Exhaustive Execution Protocol

## Status

**ACCEPTED** - 2025-12-02

## Context

Observable pattern: Claude declares tasks complete after partial execution.

| Request | Expected | Actual |
|---------|----------|--------|
| "Read all 70 files" | 70 files read | 7 files read, completion declared |
| "Analyze for meaning" | Semantic analysis | Pattern grep |
| "Check everything" | Exhaustive verification | Spot-check sample |

### Documentation Review

Official Anthropic documentation (2025) reveals relevant features:

**Effort Parameter** (Opus 4.5 API):
- Low/medium/high settings control reasoning depth
- Medium effort uses 76% fewer tokens than alternatives
- Token efficiency is an explicit design goal

**Extended Thinking** (Claude Code only):
- "think" → ~4K tokens
- "think harder" / "ultrathink" → ~32K tokens
- Controls reasoning depth, not task breadth

**Design Philosophy**:
> "Though Claude Code occasionally solves problems perfectly on the first attempt, using these correction tools generally produces better solutions faster."

Source: [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

### The Gap

| What Exists | What's Missing |
|-------------|----------------|
| Effort parameter (depth) | Exhaustive parameter (breadth) |
| Thinking budget (reasoning) | Task completion tracking |
| Iterative correction tools | Upfront exhaustive mode |

No official mechanism exists to specify: "when I say all, I mean all."

## Decision

Create an Exhaustive Execution Protocol that injects intent-detection rules into Claude's context.

### Trigger Detection

When user request contains exhaustive intent markers:
- "all" / "every" / "each" / "complete" / "entire"
- Numeric totality: "all 70 files", "every endpoint"
- Explicit exhaustive: "don't sample", "actually read", "no shortcuts"

### Protocol Rules

```yaml
exhaustive:
  triggers:
    - "all"
    - "every"
    - "each"
    - "entire"
    - "complete"
    - "don't sample"
    - "actually read"
  rules:
    - "When exhaustive intent detected, disable sampling"
    - "Track progress explicitly: n of N"
    - "Do not declare completion until N of N"
    - "Prefer semantic read over pattern grep when meaning requested"
    - "If task too large, ask to chunk—do not silently sample"
```

### Implementation

1. Add `exhaustive` section to protocol injection
2. Compile into session warmup context
3. Validate triggers don't conflict with existing protocols

### Escape Hatch

User can explicitly opt-out:
- "sample a few"
- "spot check"
- "quick scan"

These phrases disable exhaustive mode for that request.

## Consequences

### Positive

- Tasks marked exhaustive will actually complete exhaustively
- Progress tracking (12 of 70) provides visibility
- User gets what they asked for

### Negative

- Higher token usage for exhaustive tasks
- Slower execution for large-scope requests
- May surface when tasks are genuinely too large

### Neutral

- Does not modify model behavior, only context injection
- Works within existing Claude Code architecture
- Compatible with effort parameter and thinking levels

## Research

Full research documented in: `docs/research/exhaustive-execution-research.md`

## References

- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)
- [Token-saving updates](https://www.anthropic.com/news/token-saving-updates)
- [What's new in Claude 4.5](https://platform.claude.com/docs/en/docs/about-claude/models/whats-new-claude-4-5)
- [Claude 4 System Card](https://www.anthropic.com/claude-4-system-card)
