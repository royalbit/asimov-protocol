# ADR-009: Claude Code Native Integration

## Status

Accepted (Partially Corrected by ADR-013)

## Date

2025-11-28

## Correction (2025-11-29)

> **IMPORTANT**: This ADR contains an error regarding self-healing.
>
> The original analysis incorrectly stated that Claude Code native features
> replace mid-session self-healing. They do NOT:
>
> - `--continue`/`--resume`: Require **manual CLI start** (cross-session)
> - `/rewind`: Requires **manual command** (not automatic)
> - None of these work **unattended during a live session before compaction**
>
> The `warmup.yaml` re-read pattern for self-healing is **NOT replaced**.
>
> See [ADR-013](013-self-healing-not-replaced.md) for the correction.

## Context

### The Discovery

Research conducted on 2025-11-28 revealed that Claude Code 2.0 (released November 2025) now includes native features that duplicate **some** portions of the RoyalBit Asimov:

| RoyalBit Asimov Feature | Claude Code Native | Status |
|------------------------|-------------------|--------|
| `.claude_checkpoint.yaml` | `/rewind` + checkpoints | **REDUNDANT** (manual only) |
| Session handoff | `--continue`, `--resume` | **REDUNDANT** (cross-session only) |
| Self-healing (mid-session) | **NOTHING** | **NOT REPLACED** (see ADR-013) |
| Context persistence | Memory tool API | **CAN INTEGRATE** |

### Claude Code 2.0 Native Features

1. **Checkpoints**: Automatic code state + conversation context saving
   - `/rewind` command or Esc+Esc to restore
   - Can restore code only, conversation only, or both
   - Tracks all file edits made through Claude's tools

2. **Session Continuity**: Built-in session management
   - `claude --continue` resumes most recent conversation
   - `claude --resume <session-id>` resumes specific session
   - Session state persists automatically

3. **CLAUDE.md Memory Hierarchy**:
   - Recursive loading from cwd up to root
   - `@path/to/import` syntax for including files
   - `/memory` command to view loaded files
   - `#` prefix to add memories during session

4. **Auto-compact at 95%**: Capacity-based, not time-based
   - Compaction triggers at 95% context capacity (25% remaining)
   - Confirms ADR-003 finding that "2hr checkpoint" was fiction

5. **MCP Industry Standard**: Model Context Protocol adoption
   - OpenAI and Google DeepMind have adopted MCP
   - 98.7% token reduction with code execution + MCP APIs
   - Thousands of MCP servers in ecosystem

### What RoyalBit Asimov Uniquely Provides

Features Claude Code does NOT have natively:

1. **Ethics Protocol (Three Laws)** - `asimov.yaml`
   - `human_veto` command
   - `first_law.do_no_harm`
   - Red flag detection
   - Session limits

2. **Green Coding** - Local-first philosophy
   - Zero-token validation
   - CLI tools over cloud AI
   - ESG/carbon metrics

3. **Sprint Autonomy** - Bounded sessions
   - 4hr maximum session duration
   - Keep shipping (ADR-028)
   - Anti-patterns rejection
   - "Done is better than perfect"

4. **Schema Validation** - Protocol file validation
   - `asimov validate`
   - Structure validation
   - File size limits

### The Problem with Current Roadmap

v3.3.0-v3.5.0 planned features that are now redundant:
- Session handoff workflows → Use `--continue`, `--resume`
- Session-end command → Use native checkpoints
- Staleness detection → CLAUDE.md handles this
- Checkpoint schema → Native checkpoints are better

Building these would waste effort duplicating Claude Code's native features.

## Decision

**Strategic Pivot: Stop building what Claude Code has. Integrate with it. Focus on unique value.**

### v4.0.0 - CLAUDE.md Integration (BREAKING)

1. **Generate CLAUDE.md from protocol files**
   - `asimov init --asimov` generates CLAUDE.md that imports warmup.yaml
   - Use Claude Code's native memory hierarchy
   - Include ethics reference in CLAUDE.md

2. **Deprecate `.claude_checkpoint.yaml`**
   - Native `/rewind` checkpoints are superior
   - Session state managed by Claude Code
   - Remove checkpoint schema and validation

3. **Remove session handoff features from roadmap**
   - `--continue` and `--resume` handle this
   - No need for custom session-end command

4. **Keep and enhance unique value**
   - Ethics validation (Priority 0)
   - Green coding metrics
   - Sprint autonomy rules
   - Schema validation

### ~~Future: MCP Server Mode~~ (KILLED)

> **KILLED in v7.4.0** - See [ADR-029](029-mcp-server-mode-killed.md)
>
> MCP interfaces cap thinking tokens at 30k-128k. Asimov requires 200k.
> Green coding principle: Don't build what won't work.

## Consequences

### Positive

1. **No wasted effort** - Don't duplicate Claude Code features
2. **Better UX** - Native features are more integrated
3. **Smaller scope** - Focus on unique value
4. **True integration** - Work WITH Claude Code, not parallel to it
5. **Future-proof** - MCP path for vendor neutrality

### Negative

1. **Breaking change** - v4.0.0 changes file structure
2. **Migration needed** - Existing users must update
3. **Feature removal** - Checkpoint validation removed
4. **Claude Code coupling** - ROYALBIT ASIMOV more tightly coupled

### Neutral

1. **Version jump** - v3.2.0 → v4.0.0 signals breaking change
2. **Roadmap rewrite** - v3.3.0-v3.5.0 replaced

## Implementation

### Phase 1: Documentation (This Session)

- [x] ADR-009: This document
- [ ] Update roadmap.yaml with v4.0.0 plan
- [ ] Update SPECIFICATION.md
- [ ] Update README.md
- [ ] Update sprint.yaml

### Phase 2: CLI Changes (v4.0.0)

- [ ] `--asimov` generates CLAUDE.md with `@warmup.yaml` import
- [ ] Remove checkpoint schema and validation
- [ ] Update CLAUDE.md template
- [ ] Add `asimov migrate` command for v3→v4

### ~~Phase 3: MCP Integration~~ (KILLED)

> MCP Server Mode was killed in v7.4.0 due to token limitations.
> See [ADR-029](029-mcp-server-mode-killed.md).

## References

- [Claude Code Checkpointing Docs](https://code.claude.com/docs/en/checkpointing)
- [Claude Code Memory Docs](https://docs.claude.com/en/docs/claude-code/memory)
- [Code execution with MCP](https://www.anthropic.com/engineering/code-execution-with-mcp)
- [Enabling Claude Code to work more autonomously](https://www.anthropic.com/news/enabling-claude-code-to-work-more-autonomously)
- [ADR-003: Self-Healing Based on Real Compaction Data](003-self-healing-real-compaction-data.md)
