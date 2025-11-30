# ADR-013: Self-Healing During Session NOT Replaced by Claude Code Native

## Status

Accepted

## Date

2025-11-29

## Context

### The Mistake

In ADR-009 (Claude Code Native Integration), we incorrectly stated that Claude Code's native features replace our self-healing protocol:

```yaml
# WRONG - What we said:
deprecated:
  checkpoint_validation:
    replaced_by: "Claude Code /rewind checkpoints"
  session_handoff:
    replaced_by: "Claude Code session persistence"
```

### The Reality

Claude Code's native features serve **different purposes**:

| Feature | What It Actually Does | When It Works |
|---------|----------------------|---------------|
| `--continue` | Resume most recent session | **Manual CLI start** |
| `--resume <id>` | Resume specific session | **Manual CLI start** |
| `/rewind` | Restore checkpoint | **Manual command** |
| `Esc+Esc` | Open checkpoint picker | **Manual interaction** |

**None of these are automatic. None work unattended during a live session.**

### The Problem We Were Solving

Self-healing addresses **context compaction during a running session**:

```
Session starts
    ↓
AI works autonomously (ASIMOV MODE)
    ↓
~15 minutes pass (heavy reasoning)
    ↓
Context compaction happens (automatic, not user-triggered)
    ↓
AI loses rules, forgets milestone, gets confused
    ↓
??? WHAT NOW ???
```

**Claude Code native features don't help here.** There's no automatic recovery.

### What Actually Works

The original self-healing protocol in `warmup.yaml`:

```yaml
self_healing:
  on_confusion: "STOP → re-read warmup.yaml → re-read sprint.yaml"

  confusion_signals:
    - "Unsure about project rules"
    - "Forgot what milestone we're working on"
    - "Making decisions that contradict protocol"
    - "About to scope creep ('let me also...', 'while I'm here...')"
```

**This is the only mechanism for mid-session automatic recovery.**

The AI must:
1. Recognize confusion signals
2. STOP what it's doing
3. Re-read warmup.yaml from disk
4. Re-read sprint.yaml from disk
5. Resume with restored context

## Decision

### Reinstate Self-Healing as NOT Deprecated

The self-healing protocol (re-read from disk on confusion) is **NOT replaced** by Claude Code native features.

### Clarify What IS Replaced

| Feature | Old (Asimov Protocol) | New (Claude Code Native) | Scope |
|---------|---------------------|-------------------------|-------|
| Cross-session resume | Custom handoff files | `--continue`/`--resume` | **Between sessions** |
| Manual checkpoint restore | `.claude_checkpoint.yaml` | `/rewind` | **Manual command** |
| Mid-session self-healing | `warmup.yaml` re-read | **NOTHING** | **Still ours** |

### The Correct Mental Model

```
┌─────────────────────────────────────────────────────────────────┐
│                    CLAUDE CODE SESSION                          │
│                                                                 │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │  MID-SESSION (Compaction happens here)                  │   │
│  │                                                         │   │
│  │  Self-healing: warmup.yaml re-read ← ASIMOV PROTOCOL    │   │
│  │  Task tracking: TodoWrite ← Claude Code native          │   │
│  │  Manual restore: /rewind ← Claude Code native           │   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
         ↑                                           ↓
    claude --continue                         Session ends
    claude --resume <id>                      (state persisted)
         ↑                                           ↓
┌─────────────────────────────────────────────────────────────────┐
│                    BETWEEN SESSIONS                             │
│                                                                 │
│  Resume: --continue, --resume ← Claude Code native              │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Consequences

### Positive

1. **Correct understanding** - Self-healing is still our unique value
2. **No false deprecation** - Protocol files remain essential
3. **Clear separation** - Mid-session vs cross-session features

### Negative

1. **Previous documentation was wrong** - Requires correction
2. **Roadmap needs update** - Deprecated section was misleading

### Action Required

1. Update deprecated section in roadmap.yaml
2. Reinstate self-healing as active protocol feature
3. Clarify that `.claude_checkpoint.yaml` is replaced by TodoWrite for task tracking only
4. Keep warmup.yaml re-read pattern as primary self-healing mechanism

## What We Actually Deprecated

| Feature | Status | Replacement |
|---------|--------|-------------|
| `.claude_checkpoint.yaml` file | Deprecated | TodoWrite for task tracking |
| Custom session handoff | Deprecated | `--continue`/`--resume` |
| Checkpoint schema validation | Removed | Native checkpoints |

| Feature | Status | Notes |
|---------|--------|-------|
| Self-healing (re-read warmup.yaml) | **ACTIVE** | No replacement exists |
| warmup.yaml on_confusion pattern | **ACTIVE** | Core protocol feature |
| Sprint autonomy rules | **ACTIVE** | No replacement exists |

## References

- [ADR-009: Claude Code Native Integration](009-claude-code-native-integration.md) - Contains the error
- [ADR-003: Self-Healing Based on Real Compaction Data](003-self-healing-real-compaction-data.md) - Original research
- [warmup.yaml self_healing section](../../warmup.yaml) - The actual implementation
