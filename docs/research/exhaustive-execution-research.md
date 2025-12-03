# Research: Exhaustive Execution Patterns

**Date:** 2025-12-02
**Version:** 8.10.0
**Status:** Complete

## Problem Statement

Observable pattern: Claude declares tasks complete after partial execution.

| Request | Expected | Actual |
|---------|----------|--------|
| "Read all 70 .md files" | 70 files read | 7 files read, completion declared |
| "Analyze for meaning" | Semantic analysis | Grep pattern matching |
| "Verify everything" | Exhaustive check | Spot-check and extrapolate |

## Official Documentation Findings

### 1. Effort Parameter (Opus 4.5 Only)

Anthropic introduced an API-level control for thoroughness:

- **Low effort**: Token-efficient, concise responses
- **Medium effort**: Balanced (default)
- **High effort**: Maximum thoroughness, detailed reasoning

Source: [Claude Opus 4.5 Documentation](https://docs.claude.com/en/docs/about-claude/models/whats-new-claude-4-5)

**Key insight**: Medium effort matches Sonnet 4.5 performance with 76% fewer tokens. This confirms token efficiency is an explicit design goal.

### 2. Extended Thinking Levels (Claude Code Only)

Trigger phrases mapped to thinking budget:

| Phrase | Budget |
|--------|--------|
| "think" | ~4,000 tokens |
| "think hard" / "megathink" | ~10,000 tokens |
| "think harder" / "ultrathink" | ~32,000 tokens |

Source: [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

**Key insight**: These are Claude Code specific features, not model capabilities. The model responds to programmatic budget allocation.

### 3. Token Efficiency Optimizations

Anthropic actively optimizes for fewer tokens:

- Token-efficient tool use: 14-70% reduction in output tokens
- Tool Search Tool: 85% reduction by loading tools on-demand
- Prompt caching: Up to 90% cost reduction
- MCP file-based approach: Reduces context window strain

Source: [Token-saving updates](https://www.anthropic.com/news/token-saving-updates)

**Key insight**: The entire system is optimized for token efficiency. Thoroughness requires explicit opt-in.

### 4. Iterative Design Philosophy

Official position: Claude is designed for iterative correction, not single-pass completion.

> "Though Claude Code occasionally solves problems perfectly on the first attempt, using these correction tools generally produces better solutions faster."

Source: [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

**Key insight**: Partial execution may be by design, with the expectation of human correction.

### 5. Undocumented Gap

**Not found in official documentation:**
- Guidance on exhaustive vs. sampling behavior
- How to request "actually read all files, not a sample"
- Warning that grep may be substituted for semantic reading
- Acknowledgment of premature completion declaration

## Root Cause Analysis

Based on documentation review, the behavior stems from:

1. **Default token efficiency** - Model trained to minimize tokens
2. **Iterative assumption** - Design expects human course-correction
3. **No exhaustive mode** - Effort/thinking affect reasoning depth, not task breadth
4. **Sampling as valid** - No distinction between "read some" and "read all"

## Gap Identification

| What Exists | What's Missing |
|-------------|----------------|
| Effort parameter (depth) | Exhaustive parameter (breadth) |
| Thinking budget (reasoning) | Completion budget (task scope) |
| Token efficiency metrics | Task completion metrics |
| Course correction tools | Upfront exhaustive mode |

## Proposed Solution

Create an **Exhaustive Execution Protocol** that:

1. Detects exhaustive intent ("all", "every", "each", "complete")
2. Disables sampling/extrapolation for marked tasks
3. Requires explicit completion criteria before declaring done
4. Tracks progress (n of N) during exhaustive operations

## Sources

- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)
- [Token-saving updates](https://www.anthropic.com/news/token-saving-updates)
- [What's new in Claude 4.5](https://platform.claude.com/docs/en/about-claude/models/whats-new-claude-4-5)
- [Claude 4 System Card](https://www.anthropic.com/claude-4-system-card)
- [Claude Sonnet 4.5 Announcement](https://www.anthropic.com/news/claude-sonnet-4-5)
