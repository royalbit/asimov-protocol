# Self-Healing Protocol

> **The key enabler for unattended 8-10 hour autonomous AI sessions.**

## Requirements

| Requirement | Why |
|-------------|-----|
| **Claude Code** | Auto-loads CLAUDE.md, has file system access |
| **CLAUDE.md** | Triggers re-read after compaction |
| **warmup.yaml** | Contains full protocol rules on disk |

**This feature is Claude Code exclusive.** Other AI tools (ChatGPT, Copilot, Cursor) lack the required capabilities:
- No auto-loaded config file
- No file system access mid-session
- No ability to re-read files after context compaction

See the main [README Compatibility section](../README.md#compatibility) for details.

## Executive Summary

The Self-Healing Protocol solves a fundamental problem with long AI sessions: context compaction loses rules. Instead of trying to make rules survive compaction (fragile), we make the AI re-read rules from disk after compaction (reliable).

**Result:** True unattended autonomy. Start a session, go to sleep, wake up to completed work that followed all the rules.

## The Problem

### Context Window Limitations

AI assistants like Claude Code have finite context windows (~200K tokens for Opus 4.5). During long sessions:

1. Context fills up with conversation history
2. Auto-compact triggers at ~95% capacity
3. Conversation is summarized to free space
4. Rules and guidelines get compressed/lost
5. AI "forgets" important constraints
6. Quality degrades, scope creeps, rules violated

### Why "Survive Compaction" Fails

Common approaches try to make rules survive summarization:

- **Longer CLAUDE.md** - More text = more to summarize away
- **Redundancy** - Repeated rules still get compressed
- **"CRITICAL" labels** - Summarizer doesn't respect importance markers
- **compact_survival sections** - No special handling exists

These approaches are fundamentally fragile because you can't control what the summarizer keeps.

## The Solution: Recover, Don't Survive

### The Insight

| Approach | Database Equivalent | Reliability |
|----------|---------------------|-------------|
| Survive compaction | Hope transactions survive crash | Fragile |
| **Recover from compaction** | **Write-ahead log + replay** | **Reliable** |

The Self-Healing Protocol treats compaction like a database treats crashes: assume it will happen, and build recovery mechanisms.

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         SELF-HEALING ARCHITECTURE                        │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────────┐                                                   │
│  │    CLAUDE.md     │  ← Auto-loaded by Claude Code at session start    │
│  │  (~40 lines)     │                                                   │
│  │                  │                                                   │
│  │  • Core rules    │                                                   │
│  │  • Self-healing  │──┐                                                │
│  │    instruction   │  │                                                │
│  └──────────────────┘  │                                                │
│                        │  "After compaction, re-read warmup.yaml"       │
│                        │                                                │
│                        ▼                                                │
│  ┌──────────────────┐     ┌──────────────────┐                         │
│  │   warmup.yaml    │     │ .claude_checkpoint│                         │
│  │  (full protocol) │     │      .yaml        │                         │
│  │                  │     │                   │                         │
│  │  • All rules     │     │  • Timestamp      │                         │
│  │  • self_healing  │────▶│  • Progress       │  Written every 2hrs     │
│  │    section       │     │  • Next steps     │  and at milestones      │
│  │  • Checkpoints   │     │  • Rules reminder │                         │
│  └──────────────────┘     └──────────────────┘                         │
│           │                         │                                   │
│           │    ON DISK (survives    │                                   │
│           │    everything)          │                                   │
│           └─────────────────────────┘                                   │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### The Recovery Flow

```
Session Start
     │
     ▼
┌─────────────────┐
│ Load CLAUDE.md  │  ← Happens automatically
│ Run warmup.yaml │  ← User says "run warmup"
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Autonomous Work │◄──────────────────────┐
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ Auto-compact    │                       │
│ triggers        │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ Rules lost in   │                       │
│ summarization   │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ 2hr checkpoint  │                       │
│ fires           │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ "Re-read        │                       │
│ warmup.yaml"    │  ← This instruction   │
│ survives        │    is short enough    │
└────────┬────────┘    to survive         │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ AI reads        │                       │
│ warmup.yaml     │                       │
│ from DISK       │                       │
└────────┬────────┘                       │
         │                                │
         ▼                                │
┌─────────────────┐                       │
│ Rules restored! │───────────────────────┘
└─────────────────┘
```

## Implementation

### 1. CLAUDE.md (Required)

Create in your project root. Claude Code auto-loads this file.

```markdown
# Project Name

## CRITICAL: Self-Healing Protocol (Survives Auto-Compact)

After ANY context compaction, confusion, or uncertainty, RE-READ:
1. `warmup.yaml` - Full protocol and rules
2. `.claude_checkpoint.yaml` - Session state (if exists)

## Mandatory Checkpoints

- **Every 2 hours**: Write progress to `.claude_checkpoint.yaml`, re-read `warmup.yaml`
- **Before any commit**: Re-read quality gates from `warmup.yaml`
- **After task completion**: Update `.claude_checkpoint.yaml`
- **When confused**: STOP → re-read `warmup.yaml` → re-read `.claude_checkpoint.yaml`

## Core Rules (Memorize - These Must Survive)

- 4hr MAX session, 1 milestone, NO scope creep
- Tests pass + ZERO warnings → then commit
- NO "let me also...", NO "while I'm here..."
- Done > Perfect. Ship it.

## Commands

```
# Your language-specific commands
cargo test                    # Rust
flutter test                  # Flutter
npm test                      # JavaScript
```

## Key Files

- `warmup.yaml` - Full protocol (RE-READ after compact)
- Your other important files...
```

**Key principles:**
- Keep it SHORT (~40 lines)
- Focus on self-healing instructions
- Include only the most critical rules
- List the commands (language-specific)

### 2. warmup.yaml (self_healing section)

Add this section to your warmup.yaml:

```yaml
# ═══════════════════════════════════════════════════════════════════════════════
# SELF-HEALING PROTOCOL - Survives Auto-Compact (CRITICAL FOR AUTONOMY)
# ═══════════════════════════════════════════════════════════════════════════════
self_healing:
  description: |
    CRITICAL: After context compaction, rules are summarized and may be lost.
    This protocol ensures recovery through periodic re-reading from disk.

  checkpoint_file: ".claude_checkpoint.yaml"

  mandatory_triggers:
    every_2_hours:
      actions:
        - "Write progress to .claude_checkpoint.yaml"
        - "Re-read this warmup.yaml completely"
        - "Verify: Am I following the rules? (4hr max, 1 milestone, no scope creep)"
      why: "Rules may have been lost during auto-compact"

    before_any_commit:
      actions:
        - "Re-read quality gates section"
        - "Verify: tests pass, zero warnings"
      why: "Ensure quality gates aren't bypassed after context loss"

    after_task_completion:
      actions:
        - "Update .claude_checkpoint.yaml with completed task"
        - "Check remaining time (4hr max from session start)"
      why: "Maintain progress breadcrumbs for recovery"

    when_confused_or_uncertain:
      actions:
        - "STOP current work"
        - "Re-read .claude_checkpoint.yaml (session state)"
        - "Re-read this warmup.yaml (full rules)"
        - "Resume from last checkpoint"
      why: "Confusion often indicates context loss from compaction"

  checkpoint_format:
    example: |
      # .claude_checkpoint.yaml - Auto-generated, READ AFTER COMPACT
      timestamp: "2025-11-26T03:00:00Z"
      session_started: "2025-11-26T01:00:00Z"
      milestone: "Current milestone name"
      completed:
        - "Task 1"
        - "Task 2"
      in_progress: "Task 3"
      next_steps:
        - "Task 4"
      rules_reminder:
        - "4hr max - check session_started above"
        - "1 milestone only - no scope creep"
        - "Tests pass + zero warnings before commit"
      # After reading this, RE-READ warmup.yaml for full protocol

  core_rules_summary:
    description: "Ultra-short rules that MUST survive summarization"
    rules:
      - "4hr MAX session, 1 milestone, NO scope creep"
      - "Tests pass + ZERO warnings → then commit"
      - "NO 'let me also', NO 'while I'm here'"
      - "Done > Perfect. Ship it."
```

### 3. .claude_checkpoint.yaml (Auto-generated)

The AI writes this during the session. Example:

```yaml
# .claude_checkpoint.yaml
# Auto-generated by AI during session
# READ THIS AFTER CONTEXT COMPACTION

timestamp: "2025-11-26T03:00:00Z"
session_started: "2025-11-26T01:00:00Z"

milestone: "Add self-healing protocol to all projects"

completed:
  - "Created CLAUDE.md for forge"
  - "Added self_healing to forge warmup.yaml"
  - "Created CLAUDE.md for forge-protocol"

in_progress: "Documenting self-healing protocol"

next_steps:
  - "Create docs/SELF-HEALING.md"
  - "Update other projects"
  - "Commit and push all changes"

rules_reminder:
  - "4hr max - started at 01:00, stop by 05:00"
  - "1 milestone only - self-healing protocol"
  - "NO scope creep - don't start new features"
  - "Tests pass + zero warnings before commit"

# IMPORTANT: After reading this, RE-READ warmup.yaml for full protocol
```

**Add to .gitignore:**
```
.claude_checkpoint.yaml
```

This file is session-specific and shouldn't be committed.

## Why Each Component Matters

| Component | Purpose | Survives Compact? | Recovery |
|-----------|---------|-------------------|----------|
| **CLAUDE.md** | Bootstrap rules + self-healing instruction | Partially (system prompt) | Auto-loaded |
| **warmup.yaml** | Full protocol with all rules | No | Re-read from disk |
| **self_healing section** | Checkpoint triggers | No | Included in warmup.yaml |
| **.claude_checkpoint.yaml** | Session state breadcrumbs | N/A (on disk) | Always available |

The key insight: **"Re-read warmup.yaml"** is short enough to survive summarization. Even if everything else is lost, the AI knows to reload the full rules.

## Language-Specific Templates

### Rust Projects

```yaml
# In warmup.yaml
self_healing:
  checkpoint_file: ".claude_checkpoint.yaml"
  mandatory_triggers:
    every_2_hours:
      - "Write checkpoint"
      - "Re-read warmup.yaml"
    before_any_commit:
      - "cargo test"
      - "cargo clippy -- -D warnings"
  core_rules_summary:
    - "4hr MAX, 1 milestone, NO scope creep"
    - "Tests pass + ZERO warnings → commit"
```

### Flutter Projects

```yaml
self_healing:
  checkpoint_file: ".claude_checkpoint.yaml"
  mandatory_triggers:
    every_2_hours:
      - "Write checkpoint"
      - "Re-read warmup.yaml"
    before_any_commit:
      - "flutter test"
      - "dart analyze"
  core_rules_summary:
    - "4hr MAX, 1 milestone, NO scope creep"
    - "Tests pass + ZERO analyzer warnings → commit"
    - "60fps performance - no jank"
```

### Documentation Projects

```yaml
self_healing:
  checkpoint_file: ".claude_checkpoint.yaml"
  mandatory_triggers:
    every_2_hours:
      - "Write checkpoint"
      - "Re-read warmup.yaml"
    before_any_commit:
      - "markdownlint **/*.md"
  core_rules_summary:
    - "4hr MAX, 1 milestone, NO scope creep"
    - "ZERO lint errors → commit"
```

## Verification

### Test the Protocol

1. Start a session: `claude --dangerously-skip-permissions`
2. Say: "run warmup"
3. Confirm AI acknowledges self-healing protocol
4. Start a long task
5. Monitor for checkpoint behavior at 2hr marks

### Check Files Exist

```bash
# Should exist and be tracked by git
ls -la CLAUDE.md warmup.yaml

# Should be in .gitignore
grep claude_checkpoint .gitignore
```

## Research Background

This protocol is based on research into Claude Code's auto-compact behavior:

- [DoltHub: Claude Code Gotchas](https://www.dolthub.com/blog/2025-06-30-claude-code-gotchas/) - **"definitely dumber after compaction"**
- [Claude Code Compaction](https://stevekinney.com/courses/ai-development/claude-code-compaction)
- [GitHub: /compact loses CLAUDE.md context](https://github.com/anthropics/claude-code/issues/4517)
- [GitHub: Does Claude include CLAUDE.md after compact?](https://github.com/anthropics/claude-code/issues/2714)
- [Why Claude Forgets](https://www.arsturn.com/blog/why-does-claude-forget-things-understanding-auto-compact-context-windows)
- [Anthropic: Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)

> *"The summary is cool but Claude Code is **definitely dumber after the compaction**. It doesn't know what files it was looking at and needs to re-read them. It will make mistakes you specifically corrected earlier in the session."*
> — DoltHub

Key findings:
- Auto-compact triggers at ~95% context capacity
- Rules in CLAUDE.md may not survive compaction
- Summarization doesn't respect "importance" markers
- The only reliable recovery is re-reading from disk

## Adoption

The Self-Healing Protocol is currently deployed in:

| Project | Language | Status |
|---------|----------|--------|
| Forge | Rust | Production |
| Forge Protocol | Rust | Production |
| Forge-Zed | Rust/WASM | Production |
| Mouvify API | Rust | Production |
| Mouvify Enrich | Rust | Production |
| Mouvify Proto | Flutter | Production |
| Mouvify Arch | Docs | Production |
| Mouvify Business | Forge YAML | Production |
| Mouvify Demo | Shell | Production |

**Total: 9 production projects, all with self-healing enabled.**

## FAQ

### Q: Does CLAUDE.md survive compaction?

Partially. It's loaded as a system prompt, but the GitHub issues suggest it may not be fully preserved. That's why we include explicit "re-read warmup.yaml" instructions.

### Q: Why not just use a longer CLAUDE.md?

Longer = more to compress = more likely to lose important parts. Short, focused CLAUDE.md with self-healing instructions is more reliable.

### Q: What if the "re-read warmup.yaml" instruction gets lost?

It's short enough (~10 words) that it has a high chance of surviving summarization. The checkpoint triggers also help - if the AI writes a checkpoint, it naturally reads the rules.

### Q: Can I skip CLAUDE.md and just use warmup.yaml?

You can, but CLAUDE.md is auto-loaded while warmup.yaml requires explicit reading. Having both provides defense in depth.

### Q: Does this work with other AI assistants?

**No.** Self-Healing requires capabilities that only Claude Code currently provides:

| Capability | Claude Code | ChatGPT | Copilot | Cursor |
|------------|-------------|---------|---------|--------|
| Auto-load config file | ✓ (CLAUDE.md) | ✗ | ✗ | ✓ (.cursorrules) |
| Read files mid-session | ✓ | ✗ | ✗ | Limited |
| Re-read after compaction | ✓ | ✗ | ✗ | ? |

The **file format** (warmup.yaml) is vendor-neutral - any AI can parse it if you paste it. But the **Self-Healing mechanism** requires Claude Code's specific features.

If other vendors add equivalent capabilities in the future, the protocol could work there too.

## Conclusion

The Self-Healing Protocol is the **TCP of AI autonomy** - reliable delivery over an unreliable channel.

Instead of hoping rules survive compaction, we accept that they won't and build robust recovery. The result: true unattended autonomy that actually works.

```
Start session → Go to sleep → Wake up to completed work
```

**Requirements:** Claude Code (for now). If other AI tools add auto-load config + file system access, the protocol will work there too.

**Ship fast. Ship small. Ship green. Ship while you sleep.**
