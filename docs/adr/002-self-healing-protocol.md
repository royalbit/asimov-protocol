# ADR-002: Self-Healing Protocol for Unattended Autonomy

**Status:** Accepted
**Date:** 2025-11-26
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI

## Context

Long autonomous AI sessions (8-10 hours) face a fundamental problem: context compaction.

### The Problem

Claude Code (and similar AI coding assistants) have finite context windows (~200K tokens for Opus 4.5). During long sessions:

1. Context fills with conversation history, tool calls, and file contents
2. Auto-compact triggers at ~95% capacity
3. Conversation is summarized to free space
4. **Rules and guidelines get compressed/lost**
5. AI "forgets" critical constraints

As documented by [DoltHub](https://www.dolthub.com/blog/2025-06-30-claude-code-gotchas/):

> "The summary is cool but Claude Code is **definitely dumber after the compaction**. It doesn't know what files it was looking at and needs to re-read them. It will make mistakes you specifically corrected earlier in the session."

### Failed Approaches

Previous attempts to solve this problem focused on **making rules survive compaction**:

| Approach | Why It Fails |
|----------|--------------|
| Longer CLAUDE.md | More text = more to summarize away |
| Redundancy | Repeated rules still get compressed |
| "CRITICAL" labels | Summarizer doesn't respect importance markers |
| compact_survival sections | No special handling exists in the summarizer |

These approaches are fundamentally fragile because you cannot control what the summarizer preserves.

### The Constraint

For true unattended autonomy (user goes to sleep, wakes up to results), we need:

1. Rules that persist across multiple auto-compact cycles
2. No manual intervention required
3. Recovery from context loss, not prevention of it

### Platform Requirement

**This solution requires Claude Code.** The mechanism depends on:

- **Auto-loaded config** (CLAUDE.md) - Only Claude Code has this
- **File system access** - Read files mid-session
- **Re-read capability** - Reload files after compaction

Other AI tools (ChatGPT, Copilot, Cursor) cannot implement Self-Healing because they lack these capabilities. The **file format** (warmup.yaml) is universal, but the **recovery mechanism** is Claude Code exclusive.

## Decision

**Implement a Self-Healing Protocol that recovers from compaction rather than trying to survive it.**

### The Insight

| Problem | Traditional | Self-Healing |
|---------|-------------|--------------|
| Database crash | Hope transactions survive | Write-ahead log + replay |
| **Context compact** | Hope rules survive | **Checkpoint + re-read from disk** |

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    SELF-HEALING PROTOCOL                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  CLAUDE.md           warmup.yaml           .claude_checkpoint│
│  (auto-loaded)       (full rules)          .yaml (on disk)   │
│  ┌────────────┐      ┌────────────┐        ┌────────────┐   │
│  │ Core rules │      │ Complete   │        │ Session    │   │
│  │ + "re-read │─────▶│ protocol + │───────▶│ state +    │   │
│  │ warmup"    │      │ self_healing│       │ breadcrumbs│   │
│  └────────────┘      └────────────┘        └────────────┘   │
│        │                   │                      │          │
│        └───────────────────┴──────────────────────┘          │
│                    ALL ON DISK                               │
│                 (survives everything)                        │
└─────────────────────────────────────────────────────────────┘
```

### Implementation

1. **CLAUDE.md** (~40 lines) - Auto-loaded by Claude Code
   - Ultra-short core rules
   - Self-healing instruction: "After compaction, re-read warmup.yaml"
   - Key commands

2. **warmup.yaml** (self_healing section)
   - Checkpoint triggers: every 2hr, before commit, when confused
   - Checkpoint file path
   - Core rules summary

3. **.claude_checkpoint.yaml** (auto-generated)
   - Written during session
   - Contains: timestamp, progress, next steps, rules reminder
   - Breadcrumbs for recovery

### Why "Re-read warmup.yaml" Works

The instruction "re-read warmup.yaml" is:
- **Short** (~10 words) - high chance of surviving summarization
- **Actionable** - clear next step for the AI
- **Disk-based** - warmup.yaml is always available

Even if all other rules are lost, this instruction triggers full recovery.

## Rationale

### Why Not Improve Compaction?

1. We don't control Claude Code's summarization algorithm
2. Anthropic's priorities may differ from ours
3. Any fix could change in future versions
4. Recovery is always more reliable than prevention

### Why Checkpoints Every 2 Hours?

1. Matches typical compaction frequency in long sessions
2. Frequent enough to catch context loss early
3. Infrequent enough to not waste tokens
4. Aligns with existing sprint autonomy rules (4hr max)

### Why Three Files?

| File | Role | Why Separate |
|------|------|--------------|
| CLAUDE.md | Bootstrap | Auto-loaded by Claude Code |
| warmup.yaml | Full protocol | Complete rules, manually triggered |
| .claude_checkpoint.yaml | State | Session-specific, not committed |

Separation provides defense in depth. If one mechanism fails, others remain.

## Consequences

### Positive

- **True unattended autonomy**: 8-10hr sessions that follow rules
- **Portable**: Travels with git, works on any machine with Claude Code
- **File format universal**: warmup.yaml can be used with any AI (paste)
- **Battle-tested**: Deployed across 9 production projects
- **No external dependencies**: Pure YAML, no services required

### Negative

- **Claude Code exclusive**: Self-Healing requires Claude Code's specific features
- **Additional files**: CLAUDE.md and checkpoint file
- **Slight overhead**: Checkpoint writes every 2 hours
- **Not guaranteed**: Short instructions may still be lost (defense in depth helps)

### Neutral

- **Learning curve**: Users must understand the self-healing concept
- **Maintenance**: Checkpoint file needs .gitignore entry

## Compliance

This ADR applies to:

1. **All Forge Protocol projects** - Must include self_healing section
2. **forge-protocol init** - Templates include self-healing by default
3. **Documentation** - SKYNET_MODE.md, component docs, README

## Verification

To verify compliance:

```bash
# Check CLAUDE.md exists
test -f CLAUDE.md && echo "✓ CLAUDE.md exists"

# Check self_healing in warmup.yaml
grep -q "self_healing:" warmup.yaml && echo "✓ self_healing section exists"

# Check .gitignore includes checkpoint file
grep -q "claude_checkpoint" .gitignore && echo "✓ checkpoint in .gitignore"
```

## References

### Research

- [DoltHub: Claude Code Gotchas](https://www.dolthub.com/blog/2025-06-30-claude-code-gotchas/) - "definitely dumber after compaction"
- [GitHub: /compact loses CLAUDE.md context](https://github.com/anthropics/claude-code/issues/4517)
- [GitHub: Does Claude include CLAUDE.md after compact?](https://github.com/anthropics/claude-code/issues/2714)
- [Anthropic: Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)
- [ClaudeLog: Auto-Compact FAQ](https://claudelog.com/faqs/what-is-claude-code-auto-compact/)

### Internal Documentation

- [SKYNET MODE Overview](../SKYNET_MODE.md) - Complete autonomous AI development system
- [Self-Healing Component](../components/4-SELF_HEALING.md) - Self-healing specification
- [Setup Guide](../SETUP.md) - Per-project-type setup
