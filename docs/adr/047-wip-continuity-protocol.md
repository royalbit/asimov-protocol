# ADR-047: WIP Continuity Protocol (v9.11.0)

**Status:** Accepted
**Date:** 2025-12-09
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

### The Problem

During autonomous sessions, Claude Code experiences context compaction (summarization) when the context window fills. After compaction:

1. **WIP state is lost** - Claude forgets which task was in progress
2. **User must re-explain** - "We were working on X, you finished Y, now do Z"
3. **Consent is forgotten** - User gave "go" permission, but Claude doesn't remember
4. **Autonomy breaks** - The 4-hour session promise requires babysitting

### Root Cause Analysis

Investigated Claude Code hooks:
- **SessionStart** - Fires on: new session, resume, `/clear`. NOT after compaction.
- **PreCompact** - Fires BEFORE compaction. Output gets summarized too.
- **PostCompact** - DOES NOT EXIST (Claude Code GitHub Issue #3612)

**Gap:** No hook fires AFTER mid-session context compaction. Warmup only runs at session start.

### Failed Approaches

| Approach | Why It Fails |
|----------|--------------|
| CLAUDE.md | Read at session start only |
| MCP servers | External complexity, maintenance burden |
| Manual `/clear` | Can't do in autonomous mode |
| PreCompact hook | Output gets summarized |
| External database | Over-engineered for simple state |

## Decision

### Use Git Pre-Commit as Forcing Function

**Key insight:** Claude ALWAYS commits during autonomous work.

1. Pre-commit hook fires on every `git commit`
2. Hook reads `roadmap.yaml` for WIP state
3. Output is visible to Claude (Bash tool captures stdout)
4. State read fresh from disk (survives compaction)

### The roadmap.yaml IS the Lock File

No separate WIP file needed. Use existing roadmap structure:

```yaml
next:
  - version: "9.11.0"
    deliverables:
      - id: feature-a
        status: done
      - id: feature-b
        status: wip      # <-- Active task
      - id: feature-c
        status: todo
```

Status values:
- `todo` - Not started
- `wip` - Work in progress (only ONE at a time)
- `done` - Complete

### WIP Workflow

```
1. User says "go" on milestone
   → Claude sets first todo deliverable to status: wip
   → Work begins

2. Work in progress
   → Claude commits code
   → Pre-commit outputs: "CONTINUE WORKING ON: feature-b"
   → Claude sees reminder, continues

3. Context compacts (automatic)
   → Claude's memory summarized
   → Next commit triggers pre-commit
   → Claude sees WIP reminder, resumes automatically
   → NO USER INTERVENTION NEEDED

4. Item complete
   → Claude sets item to status: done
   → Claude sets next todo to status: wip
   → Repeat until milestone complete

5. Milestone complete
   → All items status: done
   → Archive to CHANGELOG.md
```

### Pre-Commit Hook Output

When `status: wip` detected in roadmap.yaml:

```
══════════════════════════════════════════════════════════════════════════════
🔥 ACTIVE WIP - CONTINUE THIS TASK
══════════════════════════════════════════════════════════════════════════════
CURRENT TASK: feature-b
PROGRESS: 2/5 items complete

>>> CONTINUE WORKING ON: feature-b <<<
>>> DO NOT ASK FOR PERMISSION - USER CONSENT ALREADY GIVEN <<<
══════════════════════════════════════════════════════════════════════════════
```

### Warmup Enhancement

Warmup also detects WIP state (belt-and-suspenders):

**No WIP active:**
```
MILESTONE READY: 9.11.0 - WIP Continuity Protocol
Say "go" to start autonomous execution.
```

**WIP active:**
```
ACTIVE WIP: feature-b (2/5 complete)
RESUME IMMEDIATELY. User consent given at milestone start.
```

## Implementation

### Pre-Commit Hook (hooks.rs)

```bash
# === WIP CONTINUITY (survives context compaction) ===
if [ -f ".asimov/roadmap.yaml" ] && grep -q "status: wip" .asimov/roadmap.yaml; then
  current=$(grep -B5 "status: wip" .asimov/roadmap.yaml | grep "id:" | tail -1 | sed 's/.*id:[[:space:]]*//')
  if [ -n "$current" ]; then
    total=$(grep -c "status:" .asimov/roadmap.yaml 2>/dev/null || echo "0")
    done_count=$(grep -c "status: done" .asimov/roadmap.yaml 2>/dev/null || echo "0")
    echo ""
    echo "══════════════════════════════════════════════════════════════════════════════"
    echo "🔥 ACTIVE WIP - CONTINUE THIS TASK"
    echo "══════════════════════════════════════════════════════════════════════════════"
    echo "CURRENT TASK: $current"
    echo "PROGRESS: $done_count/$total items complete"
    echo ""
    echo ">>> CONTINUE WORKING ON: $current <<<"
    echo ">>> DO NOT ASK FOR PERMISSION - USER CONSENT ALREADY GIVEN <<<"
    echo "══════════════════════════════════════════════════════════════════════════════"
    echo ""
  fi
fi
```

### Warmup (warmup.rs)

Add WIP detection to warmup output:
1. Parse roadmap.yaml for `next` milestone
2. Check deliverables for `status: wip`
3. If found: output continuation directive
4. If not: output "ready to start" message

## Consequences

### Positive

1. **4-hour sessions work** - Claude resumes automatically after compaction
2. **Zero user intervention** - Consent given once, honored throughout
3. **Simple state** - roadmap.yaml is the single source of truth
4. **Reliable trigger** - Git commits guaranteed during autonomous work
5. **Human readable** - Anyone can see WIP state in roadmap

### Negative

1. **Requires git** - Won't work in non-git projects (rare for dev work)
2. **Extra output** - Pre-commit shows WIP on every commit (acceptable)
3. **Single WIP** - Only one item can be `status: wip` (by design)

### Migration

Existing projects automatically get WIP support after:
1. `asimov refresh` - Regenerates pre-commit hook with WIP detection
2. Add `status:` field to deliverables in roadmap.yaml

## References

- Claude Code GitHub Issue #3612: PostCompact hook request
- ADR-031: Date injection pattern (similar warmup enhancement)
- Asimov Sprint Protocol: Autonomous execution rules
