# ADR-007: Checkpoint Size Limits and Pruning

**Status:** Accepted
**Date:** 2025-11-27
**Implemented:** 2025-11-29 (v3.1.0)
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI

## Context

The self-healing protocol relies on `.claude_checkpoint.yaml` being re-read after context compaction. A critical assumption is that this file remains small enough to re-read without consuming excessive context.

### The Question

> "Can `.claude_checkpoint.yaml` grow outside context boundaries?"

### Analysis

**Claude Opus 4.5 specifications:**
- Context window: 200,000 tokens
- Output limit: 64,000 tokens

**Current checkpoint schema (SPECIFICATION.md):**
```yaml
timestamp: "2025-01-15T10:30:00Z"
tool_calls: 45
milestone: "Add feature X"
completed:
  - "Task 1"
  - "Task 2"
next_steps:
  - "Task 3"
on_confusion: "cat warmup.yaml"
```

**Designed size:** ~20 lines (~500 tokens, 0.25% of 200K context)

### Understanding Compaction Correctly

**Key insight:** Compaction FREES context, it doesn't accumulate.

```
Before compaction: Context at 95% (190K tokens)
       ↓
Compaction occurs: Context compressed to ~50% (100K tokens)
       ↓
After compaction: 100K tokens FREE for new work
       ↓
Re-read warmup.yaml: Uses ~17K tokens (8.5% of 200K)
Re-read checkpoint: Uses ~500 tokens (0.25% of 200K)
       ↓
Available after recovery: ~82K tokens for work
```

**The concern is NOT cumulative re-reads across compactions.**
**The concern IS whether a single checkpoint could grow too large.**

### Failure Scenarios

| Scenario | Risk | Consequence |
|----------|------|-------------|
| Unbounded `completed` list | HIGH | 100 tasks = ~2500 tokens |
| Verbose task descriptions | MEDIUM | Long descriptions bloat file |
| Session history accumulation | HIGH | If checkpoint appends instead of replaces |
| No pruning discipline | HIGH | Checkpoint grows over multi-hour sessions |

### Current Protocol File Sizes (Asimov Protocol itself)

| File | Lines | Est. Tokens | % of 200K |
|------|-------|-------------|-----------|
| warmup.yaml | 690 | ~17,000 | 8.5% |
| sprint.yaml | 144 | ~3,600 | 1.8% |
| roadmap.yaml | 216 | ~5,400 | 2.7% |
| checkpoint (designed) | ~20 | ~500 | 0.25% |
| **Total** | ~1,070 | ~26,500 | **13.3%** |

**Observation:** The bigger risk is `warmup.yaml` at 690 lines (3.5x the recommended 200 lines), not the checkpoint.

## Proposed Decision

### 1. Enforce Size Limits

> **HONESTY NOTE:** These numbers are ASSUMPTIONS, not research.
> The "20/30 line" caps are invented based on the ~20 line estimate in SPECIFICATION.md.
> They need validation through actual usage before ADR is accepted.

Add to SPECIFICATION.md:

```yaml
file_size_limits:
  CLAUDE.md:
    max_lines: 10        # ASSUMPTION - needs validation
    max_tokens: 250
    enforcement: error

  checkpoint:
    max_lines: 30        # ASSUMPTION - needs validation
    max_tokens: 750
    enforcement: warn

  warmup_core:
    max_lines: 200       # ASSUMPTION - needs validation
    max_tokens: 5000
    enforcement: warn
```

### 2. Checkpoint Trimming Mechanism

**The Problem:** During a 4-hour session, completed tasks accumulate. If you complete 50 tasks, that's 50+ lines just in `completed`. The checkpoint bloats and consumes context that should be used for actual work.

**The Mechanism: Rolling Window with Hard Cap**

```yaml
# .claude_checkpoint.yaml - TRIMMING RULES
#
# HARD CAP: 30 lines maximum
# SOFT CAP: 20 lines (trigger trim at this point)
#
# TRIM ALGORITHM (executed BEFORE each checkpoint write):
#   1. If current checkpoint > 20 lines:
#   2.   Keep: timestamp, milestone, status, in_progress, on_confusion
#   3.   Trim completed[] to last 3 items
#   4.   Trim next_steps[] to first 3 items
#   5.   Remove any optional fields (notes, context, history)
#   6. If still > 30 lines: ERROR - checkpoint structure invalid

timestamp: "2025-01-15T10:30:00Z"
tool_calls: 45
milestone: "Add feature X"
status: in_progress

# ROLLING WINDOW: Last 3 completed only
completed:
  - "Task 8: Most recent"
  - "Task 7"
  - "Task 6"
  # Tasks 1-5 TRIMMED - done is done, no recovery needed

in_progress: "Task 9: Current work"

# BOUNDED: First 3 next steps only
next_steps:
  - "Task 10"
  - "Task 11"
  - "Task 12"
  # Additional steps in sprint.yaml, not checkpoint

on_confusion: "cat warmup.yaml"
```

**Why This Works:**

| Recovery Need | What's Required | Kept in Checkpoint |
|---------------|-----------------|-------------------|
| What am I working on? | milestone, in_progress | Yes |
| What just happened? | Last 2-3 completed tasks | Yes (rolling window) |
| What's next? | First 2-3 next steps | Yes (bounded) |
| Full history? | Not needed for recovery | No - that's sprint.yaml |

**Implementation: AI Behavior Rule**

Add to `warmup.yaml` autonomous_development section:

```yaml
checkpoint_trimming:
  trigger: "Before writing checkpoint, if > 20 lines"
  action: |
    1. Count lines in current checkpoint
    2. If > 20: trim completed to last 3, next_steps to first 3
    3. If still > 30: remove notes/context fields
    4. Write trimmed checkpoint
  validation: "Checkpoint must be ≤30 lines after write"
```

**Rationale:** After compaction, you only need to know:
1. What milestone you're working on
2. What you just finished (recent context, not full history)
3. What's immediately next (not the entire backlog)

The full task history lives in `sprint.yaml` (committed) not `.claude_checkpoint.yaml` (ephemeral).

### 3. CLI Validation

Add to `asimov-mode validate`:

```bash
$ asimov-mode validate .claude_checkpoint.yaml
✓ Valid checkpoint file
⚠ Warning: 45 lines exceeds recommended 30 lines
  Suggestion: Prune completed tasks to last 5

$ asimov-mode validate warmup.yaml
✓ Valid warmup file
⚠ Warning: 690 lines exceeds recommended 200 lines
  Suggestion: Use modular structure (.forge/ directory)
```

### 4. Modular warmup.yaml (Deferred)

Already proposed in ADR-003 but not implemented:

```
project/
├── warmup.yaml           # Core only (~100 lines)
├── .forge/
│   ├── autonomy.yaml     # Session rules
│   ├── quality.yaml      # Quality gates
│   └── release.yaml      # Release workflow
```

This is a larger change - defer to separate milestone.

## Consequences

### Positive

- **Bounded checkpoint size** - Cannot overflow context
- **Clear pruning rules** - AI knows when to trim
- **CLI enforcement** - Catches bloat before it's a problem
- **Documentation** - Users understand the limits

### Negative

- **Additional validation logic** - More code in CLI
- **Pruning discipline required** - AI must follow rules

### Risks

- **Over-pruning** - Losing important context by pruning too aggressively
- **Under-enforcement** - Warnings ignored, files still bloat

## Implementation

### Phase 1: Documentation & Schema (v3.1.0) - DONE
- [x] Add size limits to SPECIFICATION.md
- [x] Add pruning rules to checkpoint schema
- [x] Document checkpoint schema with field descriptions
- [x] Add checkpoint JSON schema for CLI validation

### Phase 2: CLI Enforcement (v3.1.0) - DONE
- [x] Add checkpoint validation to `asimov-mode validate`
- [x] Warn on oversized files (soft limit: 20 lines, hard limit: 30 lines)
- [x] Add CLAUDE.md size validation (soft limit: 10 lines, hard limit: 15 lines)
- [x] Add warmup.yaml size validation (soft limit: 200 lines, hard limit: 500 lines)
- [x] Generate example checkpoint in `--skynet` setup

### Phase 3: Modular Structure (Future)
- [ ] Implement `.forge/` directory support
- [ ] Split warmup.yaml template
- [ ] Update `--skynet` to generate modular structure

## Resolved Questions

1. **Pruning threshold:** 5 items max for `completed` and `next_steps` arrays
2. **Enforcement:** Soft limits warn, hard limits are documented but currently warn (not error)
3. **Checkpoint content:** Current state only - full history belongs in `sprint.yaml`

## References

- [ADR-003: Self-Healing Based on Real Compaction Data](003-self-healing-real-compaction-data.md)
- [ADR-006: Git Hook Protocol Refresh](006-git-hook-protocol-refresh.md)
- [Claude Opus 4.5 Specifications](https://www.anthropic.com/claude/opus)
