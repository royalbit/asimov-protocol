# Vendor Implementation Guide

> **For AI coding tool vendors who want to support SKYNET MODE**

## Executive Summary

The Forge Protocol enables **8-10 hour unattended autonomous AI sessions** with self-healing capabilities. Currently, only **Claude Code** fully supports this because it has the required features.

This document specifies what other AI coding tools (ChatGPT, GitHub Copilot, Cursor, Cody, etc.) need to implement for full Forge Protocol compatibility.

## Why This Matters

| With SKYNET MODE | Without |
|------------------|---------|
| 8-10hr unattended sessions | Manual intervention every 30-60min |
| Self-healing after context loss | Rules forgotten, quality degrades |
| 50-100x developer velocity | Standard AI-assisted velocity |
| "Sleep and wake up to results" | Constant babysitting required |

**User demand:** As the Forge Protocol gains adoption, users will request these features from their preferred AI tools.

## Required Capabilities

### 1. Auto-Loaded Configuration File

**What:** A file that is automatically read at session start, before any user input.

**Why:** Bootstraps the AI with project context, rules, and self-healing instructions.

**Implementation:**

```
Priority order (check each, use first found):
1. .ai/config.yaml      (vendor-neutral, recommended)
2. .forge/warmup.yaml   (Forge Protocol native)
3. CLAUDE.md            (Claude Code compatible)
4. .cursorrules         (Cursor compatible)
5. .github/copilot.md   (Copilot compatible)
```

**Requirements:**
- [ ] File is read automatically at session/conversation start
- [ ] Contents are injected into system context
- [ ] File is re-read after context compaction (critical!)
- [ ] Supports YAML and Markdown formats

**Example auto-load behavior:**
```
Session Start
     │
     ▼
┌─────────────────────┐
│ Check for config    │
│ files in priority   │
│ order               │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Read and inject     │
│ into system context │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Ready for user      │
│ input               │
└─────────────────────┘
```

### 2. File System Access

**What:** Ability to read and write files on the local filesystem during a session.

**Why:**
- Read `warmup.yaml` for full protocol rules
- Write `.claude_checkpoint.yaml` for session state
- Re-read files after context compaction

**Requirements:**
- [ ] Read arbitrary files by path
- [ ] Write files to disk
- [ ] Access persists throughout session
- [ ] Works in both interactive and autonomous modes

**Security model suggestion:**
```yaml
# .ai/permissions.yaml
allow:
  read:
    - "**/*.yaml"
    - "**/*.md"
    - "**/*.json"
    - "src/**/*"
  write:
    - ".claude_checkpoint.yaml"
    - ".ai/session_state.yaml"
deny:
  - "**/.env*"
  - "**/secrets/**"
```

### 3. Context Compaction Recovery

**What:** After context is compressed/summarized, the AI must re-read configuration files.

**Why:** Rules defined in warmup.yaml get lost during compaction. The AI needs to reload them from disk.

**This is the critical feature most tools lack.**

**Requirements:**
- [ ] Detect when context compaction occurs
- [ ] Automatically re-read auto-load config files
- [ ] Preserve "re-read warmup.yaml" instruction through compaction
- [ ] Notify AI that compaction occurred (so it can checkpoint)

**Implementation approaches:**

**Option A: Automatic re-injection**
```
Context Compaction Triggered
           │
           ▼
┌─────────────────────┐
│ Compress/summarize  │
│ conversation        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Re-read config      │◄── CRITICAL STEP
│ files from disk     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Inject fresh config │
│ into new context    │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Add system note:    │
│ "Context compacted, │
│  config reloaded"   │
└─────────────────────┘
```

**Option B: Protected instructions**
- Mark certain instructions as "survive compaction"
- At minimum, preserve: "After compaction, re-read .ai/config.yaml"

### 4. Checkpoint File Support

**What:** AI can write session state to a designated checkpoint file.

**Why:** Enables recovery after compaction, crashes, or session resume.

**Requirements:**
- [ ] Write to `.claude_checkpoint.yaml` or `.ai/checkpoint.yaml`
- [ ] Standard checkpoint format (see below)
- [ ] Periodic writes (every 2 hours recommended)
- [ ] Write before risky operations

**Standard checkpoint format:**
```yaml
# .ai/checkpoint.yaml
timestamp: "2025-11-26T15:30:00Z"
session_started: "2025-11-26T12:00:00Z"
tool: "cursor"  # or "copilot", "chatgpt", etc.
tool_version: "1.2.3"

milestone: "Add user authentication"
completed:
  - "Created auth middleware"
  - "Added JWT token generation"
in_progress: "Writing login endpoint tests"
next_steps:
  - "Implement logout"
  - "Add refresh token support"

rules_reminder:
  - "4hr max session"
  - "ONE milestone only"
  - "Tests must pass before commit"

# Recovery instruction
recovery: "Re-read warmup.yaml, then continue from in_progress"
```

### 5. Autonomous Execution Mode

**What:** AI can work without requiring approval for each action.

**Why:** Unattended sessions require the AI to make decisions independently.

**Requirements:**
- [ ] Execute file operations without per-action approval
- [ ] Run shell commands without per-command approval
- [ ] User opt-in required (security)
- [ ] Configurable scope/permissions

**Example opt-in:**
```bash
# Claude Code
claude --dangerously-skip-permissions

# Proposed standard
ai-tool --autonomous
ai-tool --trust-protocol  # Trust Forge Protocol guardrails
```

## Compatibility Levels

### Level 0: No Support
- Can paste warmup.yaml content manually
- No file access, no auto-load, no recovery
- **Tools:** ChatGPT (web), most chat interfaces

### Level 1: Basic Support
- Auto-loads config file at session start
- Has file system access
- No compaction recovery
- **Tools:** Cursor (partial)

### Level 2: Full Support (SKYNET MODE)
- Auto-loads config
- File system access
- **Compaction recovery** (re-reads config)
- Checkpoint support
- Autonomous mode
- **Tools:** Claude Code

### Level 3: Enhanced Support
- All Level 2 features
- Native Forge Protocol integration
- Protocol validation built-in
- Checkpoint UI/visualization
- **Tools:** (none yet)

## Testing Compliance

Vendors can test their implementation:

```bash
# Install test suite
cargo install forge-protocol

# Run compatibility check
forge-protocol vendor-check

# Expected output for Level 2:
# ✓ Auto-load config: .ai/config.yaml detected and loaded
# ✓ File system access: read/write working
# ✓ Compaction recovery: config re-loaded after simulated compaction
# ✓ Checkpoint support: .ai/checkpoint.yaml written successfully
# ✓ Autonomous mode: available with --autonomous flag
#
# RESULT: Level 2 (Full SKYNET MODE support)
```

## Implementation Priority

For vendors looking to add support, prioritize in this order:

| Priority | Feature | Impact |
|----------|---------|--------|
| **P0** | Auto-load config | Enables basic protocol support |
| **P0** | File system access | Required for everything else |
| **P1** | Compaction recovery | **Unlocks SKYNET MODE** |
| **P2** | Checkpoint support | Enables session resume |
| **P2** | Autonomous mode | Enables unattended operation |

**P1 (Compaction Recovery) is the key differentiator.** This is what makes 8-10 hour unattended sessions possible.

## Reference Implementation

Claude Code's implementation can serve as reference:

| Feature | Claude Code Implementation |
|---------|---------------------------|
| Auto-load | Reads `CLAUDE.md` at session start |
| File access | Full read/write via tools |
| Compaction recovery | CLAUDE.md re-injected after /compact |
| Checkpoint | AI writes to any file path |
| Autonomous | `--dangerously-skip-permissions` flag |

## Proposing to Vendors

When requesting these features from vendors, reference this document:

**GitHub Issues:**
- Title: "Support Forge Protocol for AI autonomous sessions"
- Link: https://github.com/royalbit/forge-protocol/blob/main/docs/VENDOR_IMPLEMENTATION.md
- Key request: "Implement compaction recovery (re-read config after context compression)"

**Template:**
```markdown
## Feature Request: Forge Protocol Support

The Forge Protocol enables 8-10 hour unattended AI coding sessions with
self-healing capabilities. Currently only Claude Code supports this.

**What's needed:**
1. Auto-load config file at session start
2. Re-read config after context compaction (CRITICAL)
3. File system access for checkpoints

**Spec:** https://github.com/royalbit/forge-protocol/blob/main/docs/VENDOR_IMPLEMENTATION.md

**User impact:** 50-100x developer velocity, unattended autonomous sessions

This is becoming a standard for AI-assisted development. Please consider
implementing compatibility.
```

## Vendor Tracker

| Vendor | Level | Auto-load | File Access | Compaction Recovery | Notes |
|--------|-------|-----------|-------------|---------------------|-------|
| **Claude Code** | 2 | ✓ | ✓ | ✓ | Full support |
| Cursor | 1 | ✓ | Partial | ✗ | Has .cursorrules |
| GitHub Copilot | 0 | ✗ | ✗ | ✗ | Chat only |
| ChatGPT | 0 | ✗ | ✗ | ✗ | No file access |
| Cody | 0 | ✗ | ✗ | ✗ | Limited |
| Amazon Q | ? | ? | ? | ? | Needs testing |
| Windsurf | ? | ? | ? | ? | Needs testing |

*Last updated: 2025-11-26*

## FAQ

### Q: Why not just use longer context windows?

Longer context delays but doesn't prevent compaction. At some point, every tool must compress. The question is: what happens after?

### Q: Can't the AI just "remember" the rules?

No. Summarization doesn't preserve importance. Rules get compressed into vague summaries. The only reliable solution is re-reading from disk.

### Q: Why YAML for config?

- Every AI can parse it (if they have file access)
- Human readable/editable
- Git-friendly (diffable, mergeable)
- Schema-validatable

### Q: Is this vendor lock-in to Claude Code?

Currently, yes—because other vendors haven't implemented the required features. This document exists to change that. The protocol itself is vendor-neutral; the implementation requirements are universal.

## Contact

- **Protocol repo:** https://github.com/royalbit/forge-protocol
- **Issues:** https://github.com/royalbit/forge-protocol/issues
- **Discussions:** https://github.com/royalbit/forge-protocol/discussions

---

*The Forge Protocol is open source (MIT). Vendors are encouraged to implement full support and join the ecosystem.*
