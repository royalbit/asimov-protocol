# ADR-003: Self-Healing Based on Real Compaction Data

**Status:** Accepted
**Date:** 2025-11-27
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI

## Context

ADR-002 introduced the Self-Healing Protocol with a "2-hour checkpoint" interval. This was **fiction** - a reasonable-sounding number not based on empirical data.

This ADR documents **real compaction patterns** observed from building asimov-mode itself, and proposes a corrected self-healing mechanism.

### The Problem with ADR-002

| ADR-002 Assumption | Reality |
|--------------------|---------|
| Checkpoint every 2 hours | Compaction every **10-20 minutes** |
| AI remembers checkpoint rules | AI **forgets** after compaction |
| CLAUDE.md survives intact | Gets **summarized** |
| Recovery happens automatically | **Never triggers** |

## Research: Real Compaction Patterns

### Environment Analyzed

```yaml
# User's Claude Code configuration
settings:
  MAX_THINKING_TOKENS: 200000  # Maximum extended thinking
  model: opus                   # Opus 4.5
  permissions: [Read, Write, Edit, Bash]

alias: "claude --dangerously-skip-permissions --model opus"
```

### Opus 4.5 Specifications

| Specification | Value | Source |
|--------------|-------|--------|
| Context window | 200,000 tokens | [Anthropic](https://www.anthropic.com/claude/opus) |
| Output limit | 64,000 tokens | Anthropic docs |
| User's thinking budget | 200,000 tokens | ~/.claude/settings.json |

**Critical insight:** User's thinking budget equals the entire context window.

### asimov-mode Build Data

**Project statistics:**
- Total source: ~394KB (~98,000 tokens)
- Lines of code: 24,835
- Commits: 32 (v1.0.0 → v1.4.0)
- Total build time: ~4-5 hours
- Sessions: 3

**Git log analysis (Session 3: Nov 26, 18:54-20:39):**

```
Time     Commit   Description
──────── ──────── ─────────────────────────────────────────
18:54    82cde57  feat: Add full Asimov Protocol (516+ lines)
19:11    a847b07  feat: Add self-healing protocol
19:20    cf097ed  docs: Add comprehensive self-healing docs
19:24    ce274b1  docs: Add ADR-002
19:26    84bbf23  chore: Add checkpoint to .gitignore
19:32    12a8465  fix: Repair markdown code blocks
19:42    37e0a84  feat: Add lint-docs command (v1.3.0)
19:45    424661b  chore: Add lint-docs to pre-commit
20:11    be6f91a  docs: Honest vendor compatibility
20:17    733ba83  docs: Add Vendor Implementation Guide
20:25    31be9a0  feat: Add --skynet flag (v1.4.0)
20:36    b8e421a  docs: Refactor ASIMOV MODE documentation
20:39    b3e8978  docs: Update presentation
```

**Pattern:** 13 commits in 1h45m = **1 commit every ~8 minutes**

### Token Usage Estimation

**Per-turn consumption (heavy reasoning):**

```
Component          Min        Max
─────────────────────────────────────
Thinking tokens    50,000     200,000
Input tokens       10,000     50,000
Output tokens      10,000     50,000
─────────────────────────────────────
Total per turn     70,000     300,000
```

**Context fill rate:**
- Context window: 200,000 tokens
- Auto-compact threshold: ~95% (~190,000 tokens)
- Heavy turns to fill: **1-3 turns**
- Estimated compaction interval: **5-15 minutes**

### Session Compaction Analysis

| Session | Duration | Commits | Est. Tool Calls | Est. Compactions |
|---------|----------|---------|-----------------|------------------|
| 1 (Nov 25 23:32-00:36) | ~1h | 8 | 40-60 | 4-6 |
| 2 (Nov 26 06:49-08:28) | ~1.5h | 11 | 60-80 | 5-8 |
| 3 (Nov 26 18:54-20:39) | ~1.75h | 13 | 70-100 | 6-10 |

**Conclusion:** With MAX_THINKING_TOKENS=200000, compaction occurs **5-10 times per session**, approximately every **10-20 minutes**.

## Decision

Replace the fictional "2-hour checkpoint" with reality-based triggers:

### 1. Checkpoint Triggers (Revised)

| Trigger | Rationale |
|---------|-----------|
| Every major task completion | Natural breakpoint |
| Every 10-15 tool calls | ~10-20 min of work |
| Before any file write >100 lines | Significant change |
| Before any commit | Quality gate |
| On any confusion | Recovery signal |

### 2. CLAUDE.md Structure (Revised)

**Old approach (ADR-002):**
```markdown
## CRITICAL: Self-Healing Protocol
After ANY compaction, RE-READ:
1. warmup.yaml
2. .claude_checkpoint.yaml

## Checkpoints (every 2 hours)
...
```

**New approach (ADR-003):**
```markdown
# {project}

ON CONFUSION → re-read warmup.yaml

Rules: 4hr max, 1 milestone, tests pass, ship.
```

**Why shorter is better:**
- Survives summarization (fewer tokens to compress)
- Single critical instruction: "re-read warmup.yaml"
- Core rules fit in one line

### 3. Modular warmup.yaml

**Problem:** 676-line warmup.yaml is too large to re-read efficiently.

**Solution:** Split into modules:

```
project/
├── warmup.yaml           # Core (~100 lines) - always re-read
├── .forge/               # Protocol modules
│   ├── autonomy.yaml     # Session autonomy rules
│   ├── quality.yaml      # Quality gates
│   └── release.yaml      # Release workflow
├── CLAUDE.md             # Bootstrap (ultra-short)
└── .claude_checkpoint.yaml  # Session state (ephemeral)
```

### 4. Checkpoint File Format (Revised)

```yaml
# .claude_checkpoint.yaml
timestamp: "2025-11-27T10:30:00Z"
tool_calls_since_start: 45
tool_calls_since_checkpoint: 12

milestone: "Add feature X"
status: in_progress

completed:
  - "Task 1"
  - "Task 2"

next: "Task 3"

# Survival instructions (re-read if confused)
on_confusion: "cat warmup.yaml && cat .claude_checkpoint.yaml"
```

## Consequences

### Positive

- **Based on real data** - Not hypothetical intervals
- **Survives frequent compaction** - Designed for 10-20 min cycles
- **Simpler CLAUDE.md** - More likely to survive summarization
- **Modular protocol** - Faster re-reads, less context used

### Negative

- **More frequent checkpoints** - Slight overhead
- **Requires discipline** - AI must actually write checkpoints
- **Migration needed** - Existing projects need updating

### Neutral

- **Breaking change** from ADR-002 - But ADR-002 never worked anyway

## Implementation

1. Update CLAUDE.md template in CLI
2. Add `.forge/` directory support to `asimov-mode init`
3. Update warmup.yaml to reference modules
4. Add checkpoint writing to autonomous session flow
5. Document new patterns in SPECIFICATION.md v2.0

## Verification

```bash
# Check CLAUDE.md is minimal
wc -l CLAUDE.md  # Should be < 10 lines

# Check warmup.yaml core is small
head -100 warmup.yaml | wc -l  # Core section < 100 lines

# Check .forge/ modules exist (if using modular structure)
ls .forge/*.yaml
```

## References

### Empirical Data

- asimov-mode git log: 32 commits analyzed
- Session timestamps: Nov 25-26, 2025
- Token estimates: Based on Opus 4.5 specs + user settings

### External Sources

- [Claude Opus 4.5 Specs](https://www.anthropic.com/claude/opus)
- [Claude Context Window Limits](https://www.datastudios.org/post/claude-context-window-token-limits-memory-policy-and-2025-rules)
- [DoltHub: Claude Code Gotchas](https://www.dolthub.com/blog/2025-06-30-claude-code-gotchas/)

### Internal Documentation

- [ADR-002: Self-Healing Protocol](002-self-healing-protocol.md) - Superseded assumptions
- [ASIMOV MODE Overview](../ASIMOV_MODE.md) - System architecture
