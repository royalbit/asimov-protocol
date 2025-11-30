# ADR-018: Claude Code Hooks Integration

## Status

**Revised** - 2025-11-29 (v4.1.7 - Fixed schema)

## Context

Forge Protocol relies on CLAUDE.md `@import` syntax to load protocol files (warmup.yaml,
ethics.yaml, green.yaml) into Claude's context. However, this approach has critical gaps:

1. **No Auto-Start**: `@import` loads content as static context but doesn't trigger any
   automatic execution. Users must manually say "run warmup" to initialize the protocol.

2. **Pre-Compaction Loss**: Context compaction happens every ~15 minutes (with
   MAX_THINKING_TOKENS=200000). After compaction, protocol rules may be summarized away,
   leaving Claude operating without ethical constraints or sprint boundaries.

3. **No Recovery Mechanism**: The self-healing directive in warmup.yaml
   (`on_confusion: "STOP → re-read warmup.yaml"`) only works if Claude remembers it exists.
   After aggressive compaction, even this instruction may be lost.

### Research: Competitor Analysis

Investigation of major AI coding assistants (November 2025) revealed:

| AI | Session Init | Pre-Compact |
|----|-------------|-------------|
| **Claude Code** | SessionStart hook | PreCompact hook |
| Cursor | .cursorrules (static) | /summarize (manual) |
| GitHub Copilot | .github/copilot-instructions.md | None |
| Windsurf | .windsurfrules + Memories | None |
| Gemini Code Assist | Context Drawer + MCP | None |
| Grok | Prompt caching | None |

**Finding**: Claude Code's lifecycle hooks are unique. No other AI coding assistant provides
hooks for session initialization or pre-compaction injection.

## Decision

Implement Claude Code hooks to enable true autonomous operation:

### 1. SessionStart Hook

**File**: `.claude/hooks/session-start.sh`

**Triggers**: Session start, resume, clear

**Behavior**:
- Outputs protocol initialization message
- Instructs Claude to read roadmap.yaml, sprint.yaml
- Presents next milestone
- Waits for user confirmation ("go")

### 2. PreCompact Hook

**File**: `.claude/hooks/pre-compact.sh`

**Triggers**: Before context compaction

**Behavior**:
- Outputs protocol refresh message
- Injects core rules that will survive in compaction summary
- Instructs Claude to re-read warmup.yaml, sprint.yaml post-compaction
- Reminds to check TodoWrite for in-progress tasks
- Includes ethics reminder

### 3. Configuration

**File**: `.claude/settings.json`

```json
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/session-start.sh",
            "timeout": 30
          }
        ]
      }
    ],
    "PreCompact": [
      {
        "matcher": ".*",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/pre-compact.sh",
            "timeout": 30
          }
        ]
      }
    ]
  }
}
```

**Note (v4.1.7)**: Initial release used incorrect `.claude/hooks.json` file and wrong event
names. The correct location is `.claude/settings.json` with `SessionStart` and `PreCompact`
events.

## Consequences

### Positive

1. **True Auto-Start**: Protocol initializes automatically on every session without manual
   "run warmup" command.

2. **Mid-Session Recovery**: PreCompact hook injects protocol rules before compaction,
   ensuring they survive in the summarized context. This is the missing piece for true
   SKYNET MODE autonomy.

3. **Unique Differentiator**: No other AI coding assistant has this capability. Forge
   Protocol on Claude Code offers autonomous operation that competitors cannot match.

4. **Self-Healing Complete**: Combined with v4.1.5 file auto-regeneration, the protocol
   now has defense-in-depth:
   - Missing files → auto-regenerate (v4.1.5)
   - Session start → auto-initialize (this ADR)
   - Pre-compaction → inject context for survival (this ADR)

### Negative

1. **Claude Code Exclusive**: This feature only works with Claude Code. Other AI assistants
   cannot use lifecycle hooks (they only support static rules files).

2. **User Must Accept Hooks**: Claude Code requires manual review in `/hooks` menu before
   hook changes take effect. Users must explicitly approve the hooks.

3. **30-Second Timeout**: Hooks have execution limits. Complex initialization must be
   kept simple.

### Vendor Neutrality Impact

Forge Protocol remains vendor-neutral at the **file format** level:
- warmup.yaml, ethics.yaml, sprint.yaml, green.yaml work anywhere as static context

But **autonomous operation** (SKYNET MODE) requires Claude Code:
- SessionStart hook for auto-initialization
- PreCompact hook for mid-session survival

This is acceptable because:
1. File-based protocols still provide value on all platforms
2. Full autonomy is opt-in, not required
3. MCP Server Mode (v4.3.0) will provide alternative integration path

## Implementation

### Files Created

```
.claude/
├── settings.json        # Hook configuration (NOT hooks.json)
└── hooks/
    ├── session-start.sh # SessionStart hook
    └── pre-compact.sh   # PreCompact hook (NOT post-compact.sh)
```

### User Activation

After cloning/updating the repo, users must:

1. Review hooks: Run `/hooks` in Claude Code
2. Accept the hooks configuration
3. Restart session for hooks to take effect

### Testing

Hooks can be tested manually:

```bash
# Test SessionStart hook
.claude/hooks/session-start.sh
# Should output protocol initialization message

# Test PreCompact hook
.claude/hooks/pre-compact.sh
# Should output protocol refresh message
```

## References

- [Claude Code Hooks Documentation](https://docs.anthropic.com/claude-code/hooks)
- [ADR-003: Compaction Reality](docs/adr/003-compaction-reality.md)
- [ADR-017: Protocol Self-Healing](docs/adr/017-protocol-self-healing.md)

## Revision History

- **v4.1.6** (2025-11-29): Initial implementation with incorrect schema
- **v4.1.7** (2025-11-29): Fixed schema - `.claude/settings.json`, `PreCompact` event
