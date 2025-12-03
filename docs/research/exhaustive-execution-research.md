# Research: Exhaustive Execution Patterns

**Date:** 2025-12-02
**Version:** 8.10.0
**Status:** Complete

## Observation

When given broad-scope tasks, Claude may sample rather than process exhaustively.

| Request | Possible Outcome |
|---------|------------------|
| "Read all 70 .md files" | Sample read with extrapolation |
| "Analyze for meaning" | Pattern matching |
| "Verify everything" | Spot-check sample |

## Official Documentation Findings

### 1. Effort Parameter (Opus 4.5)

Anthropic provides API-level control for thoroughness:

- **Low effort**: Concise responses
- **Medium effort**: Balanced (default)
- **High effort**: Maximum thoroughness, detailed reasoning

Source: [Claude Opus 4.5 Documentation](https://docs.claude.com/en/docs/about-claude/models/whats-new-claude-4-5)

Medium effort matches Sonnet 4.5 performance with 76% fewer tokens.

### 2. Extended Thinking Levels (Claude Code)

Trigger phrases mapped to thinking budget:

| Phrase | Budget |
|--------|--------|
| "think" | ~4,000 tokens |
| "think hard" / "megathink" | ~10,000 tokens |
| "think harder" / "ultrathink" | ~32,000 tokens |

Source: [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

These are Claude Code specific features that allocate reasoning budget.

### 3. Token Efficiency Features

Anthropic provides token optimization tools:

- Token-efficient tool use: 14-70% reduction in output tokens
- Tool Search Tool: 85% reduction by loading tools on-demand
- Prompt caching: Up to 90% cost reduction
- MCP file-based approach: Reduces context window usage

Source: [Token-saving updates](https://www.anthropic.com/news/token-saving-updates)

### 4. Iterative Design Philosophy

Official position: Claude is designed for iterative workflows.

> "Though Claude Code occasionally solves problems perfectly on the first attempt, using these correction tools generally produces better solutions faster."

Source: [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

## Opportunity

Existing controls address:
- **Reasoning depth** via effort parameter
- **Thinking budget** via extended thinking

This protocol addresses:
- **Task breadth** via exhaustive intent detection

## Proposed Solution

Create an **Exhaustive Execution Protocol** that:

1. Detects exhaustive intent ("all", "every", "each", "complete")
2. Tracks progress (n of N) during operations
3. Requires explicit completion before declaring done

## Sources

- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)
- [Token-saving updates](https://www.anthropic.com/news/token-saving-updates)
- [What's new in Claude 4.5](https://platform.claude.com/docs/en/about-claude/models/whats-new-claude-4-5)
- [Claude 4 System Card](https://www.anthropic.com/claude-4-system-card)
