# ADR-033: Launcher Mode - `asimov` as Session Entry Point

## Status

Proposed

## Context

Currently, starting an Asimov session requires multiple steps:

```bash
# Current flow (manual)
cd myproject
MAX_THINKING_TOKENS=200000 claude --dangerously-skip-permissions --model opus
# Then inside Claude: "run warmup"
```

Users have created aliases (e.g., `opus`) to simplify Claude Code invocation, but the warmup step still requires manual intervention inside the session.

The `asimov` CLI currently shows help when run without arguments - standard CLI behavior but not optimal for our use case.

## Decision

Change the default behavior of `asimov` (no arguments) to:

1. **Detect environment**: Check if already inside Claude Code
2. **If outside Claude Code**: Launch Claude Code with optimal settings and warmup prompt
3. **If inside Claude Code**: Run warmup (current behavior)

### Launch Configuration

Replicate the proven `opus` alias settings:

```bash
MAX_THINKING_TOKENS=200000 claude --dangerously-skip-permissions --model opus "run asimov warmup"
```

| Setting | Value | Rationale |
|---------|-------|-----------|
| `MAX_THINKING_TOKENS` | 200000 | Extended thinking for complex tasks |
| `--dangerously-skip-permissions` | enabled | Autonomous mode (user accepts risk) |
| `--model` | opus | Best model for autonomous work |
| Initial prompt | "run asimov warmup" | Auto-initialize session |

### Detection Strategy

Check if running inside Claude Code:
1. Check for `CLAUDE_CODE` environment variable (if set by Claude Code)
2. Check parent process name
3. Fallback: assume outside if uncertain, launch Claude Code

### Subcommand Behavior

Existing subcommands unchanged:

```bash
asimov              # NEW: Launch Claude Code + warmup
asimov warmup       # Run warmup (inside Claude Code)
asimov stats        # Show session stats
asimov doctor       # Diagnose issues
asimov replay       # Show history
asimov init         # Initialize project
asimov validate     # Validate files
asimov update       # Self-update
```

## Consequences

### Positive

- **One command to start**: `cd project && asimov`
- **Consistent configuration**: No need for user aliases
- **Lower barrier**: New users don't need to know Claude Code flags
- **Autonomous by default**: Matches Asimov's purpose

### Negative

- **Breaking change**: Users expecting help output get Claude Code launch
- **Claude Code dependency**: Requires Claude Code installed
- **Model lock-in**: Hardcodes Opus (could add `--model` flag later)

### Mitigations

- `asimov help` still shows help
- `asimov --help` still works
- Error message if Claude Code not installed
- Future: `asimov --model sonnet` override

## Implementation

1. Add default command handler in clap
2. Check for Claude Code in PATH
3. Check if inside Claude Code (env/process detection)
4. If outside: exec into Claude Code with settings
5. If inside: run warmup

## References

- User's opus alias: `MAX_THINKING_TOKENS=200000 claude --dangerously-skip-permissions --model opus`
- Claude Code CLI: https://docs.anthropic.com/claude-code
