# ADR-019: Session Start Auto-Response Directive

## Status

Accepted

## Date

2025-11-29

## Context

v4.1.7 implemented Claude Code hooks (ADR-018) with SessionStart and PreCompact events. However, testing revealed a critical limitation:

**SessionStart hooks inject context but do NOT trigger automatic Claude response.**

When a SessionStart hook fires:
1. The hook script runs
2. stdout is injected into Claude's context as a `<system-reminder>`
3. Claude sees the injected content
4. **Claude waits for user input** - it does NOT automatically respond

This means the user still had to type "run warmup" to trigger the protocol initialization, defeating the purpose of the SessionStart hook.

### Root Cause

Claude Code's hook system is designed for context injection, not response triggering. The `SessionStart` event:
- Supports only `type: "command"` (not `type: "prompt"`)
- Injects stdout into context when exit code is 0
- Does NOT force Claude to generate a response

This is architectural - hooks provide information, not instructions to act.

### Evidence

From the Claude Code hooks documentation:
> "For context injection (like in UserPromptSubmit or SessionStart), any text written to stdout with exit code 0 is automatically added to Claude's context."

The documentation says "added to context" - not "triggers a response."

## Decision

Add an explicit `ON SESSION START` directive to CLAUDE.md that instructs Claude to act immediately when it detects the SessionStart hook output.

### The Directive

```markdown
ON SESSION START: Immediately read roadmap.yaml, run `asimov-mode validate`, present next milestone. Do NOT wait for user prompt.
```

### Why This Works

1. **CLAUDE.md is always loaded** - Claude reads it at session start
2. **The directive is an instruction** - "Do NOT wait for user prompt" is explicit
3. **Combined with hook output** - Claude sees both the directive AND the injected context
4. **Redundancy is good** - If the hook fails, the directive still guides behavior

### The Combined Solution

| Component | Role | Failure Mode |
|-----------|------|--------------|
| SessionStart hook | Inject detailed instructions | Hook may not fire (first run bug) |
| CLAUDE.md directive | Instruct Claude to act immediately | Requires Claude compliance |
| Together | True auto-initialization | Redundant - either can work alone |

## Consequences

### Positive

- True auto-initialization without user prompt
- CLAUDE.md directive survives even if hooks fail
- Documents the limitation for future reference
- Increases CLAUDE.md line limit to 15 (was 10) to accommodate directive

### Negative

- Relies on Claude following instructions (social contract)
- Adds complexity to CLAUDE.md template
- Workaround for platform limitation, not ideal solution

### Neutral

- May be obsoleted if Anthropic adds "auto-response" hook type
- Other AI tools can use the CLAUDE.md directive pattern

## Alternatives Considered

### 1. File Feature Request with Anthropic

Requesting an "auto-response" hook type that forces Claude to respond.

**Rejected because:** Unknown timeline, need solution now.

### 2. UserPromptSubmit Hook

Use `UserPromptSubmit` to inject instructions on first user message.

**Rejected because:** Still requires one user message, not truly automatic.

### 3. Accept Current Behavior

User types "go" or "run warmup" to start.

**Rejected because:** Defeats the purpose of autonomous initialization.

## Implementation

1. Update CLAUDE.md with `ON SESSION START` directive
2. Update SPECIFICATION.md with new CLAUDE.md schema
3. Increase CLAUDE.md soft limit from 10 to 15 lines
4. Document in CHANGELOG.md

## References

- [ADR-018: Claude Code Hooks Integration](018-claude-code-hooks-integration.md)
- [Claude Code Hooks Reference](https://code.claude.com/docs/en/hooks)
- [GitHub Issue #10997: SessionStart hook first-run bug](https://github.com/anthropics/claude-code/issues/10997)
