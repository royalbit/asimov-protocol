# ADR-036: Exhaustive Execution Protocol

## Status

**ACCEPTED** - 2025-12-02

## Context

Observable pattern: When given broad-scope tasks, Claude may sample rather than process exhaustively.

| Request | Possible Outcome |
|---------|------------------|
| "Read all 70 files" | Sample read with extrapolation |
| "Analyze for meaning" | Pattern matching |
| "Check everything" | Spot-check sample |

### Documentation Review

Official Anthropic documentation (2025) describes available controls:

**Effort Parameter** (Opus 4.5 API):
- Low/medium/high settings control reasoning depth
- Optimizes token usage at each level

**Extended Thinking** (Claude Code):
- "think" → ~4K tokens
- "think harder" / "ultrathink" → ~32K tokens
- Controls reasoning depth

**Design Philosophy**:
> "Though Claude Code occasionally solves problems perfectly on the first attempt, using these correction tools generally produces better solutions faster."

Source: [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

### Opportunity

Existing controls address reasoning depth. This protocol addresses task breadth—ensuring exhaustive execution when explicitly requested.

## Decision

Create an Exhaustive Execution Protocol that injects intent-detection rules into Claude's context.

### Trigger Detection

When user request contains exhaustive intent markers:
- "all" / "every" / "each" / "complete" / "entire"
- Numeric totality: "all 70 files", "every endpoint"
- Explicit: "don't sample", "actually read"

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

### Escape Hatch

User can explicitly opt-out:
- "sample a few"
- "spot check"
- "quick scan"

## Consequences

### Positive

- Tasks marked exhaustive complete exhaustively
- Progress tracking (12 of 70) provides visibility
- User gets what they asked for

### Negative

- Higher token usage for exhaustive tasks
- Slower execution for large-scope requests

### Neutral

- Context injection only, works within existing architecture
- Compatible with effort parameter and thinking levels

## Research

Full research documented in: `docs/research/exhaustive-execution-research.md`

## References

- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)
- [Token-saving updates](https://www.anthropic.com/news/token-saving-updates)
- [What's new in Claude 4.5](https://platform.claude.com/docs/en/docs/about-claude/models/whats-new-claude-4-5)
