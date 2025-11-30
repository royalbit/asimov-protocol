# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [5.0.1] - 2025-11-30

### Fixed: Complete Rebrand Bug Fixes

Bug fixes to complete the v5.0.0 rebrand.

#### Changes

- **Fix CI**: Resolve clippy `assertions_on_constants` warning in ethics tests
- **Fix e2e tests**: Update binary name from `forge-protocol` to `asimov-mode`
- **Fix CI workflow**: Update binary check to `asimov-mode`
- **Fix pre-commit hook**: Update to use `asimov-mode` binary
- **Fix hooks**: Rebrand session-start.sh and pre-compact.sh to Asimov Protocol
- **Fix green.yaml**: Update remaining Forge Protocol references
- **Fix markdownlint**: Update config comment

## [5.0.0] - 2025-11-29

### BREAKING: Full Rebrand - Forge Protocol → Asimov Protocol

**The Three Laws of Robotics, encoded in YAML. The Open Foundation.**

Major breaking release: complete rebrand from "Forge Protocol" to "Asimov Protocol".

#### What Changed

| Old | New |
|-----|-----|
| Forge Protocol | Asimov Protocol |
| forge-protocol (crate) | asimov-mode (crate) |
| forge-protocol (binary) | asimov-mode (binary) |

#### Why v5.0.0?

This is a major breaking change:
- Crate name changed on crates.io
- Binary name changed
- All CLI commands now use `asimov-mode` instead of `forge-protocol`
- GitHub repo renamed to `asimov-protocol`

#### Migration

```bash
# Remove old
cargo uninstall forge-protocol

# Install new
cargo install asimov-mode
```

#### The Name

"asimov-protocol" was taken on crates.io (different project at v25.0.2).
"asimov-mode" matches our terminology: "ASIMOV MODE ACTIVATED".

See [ADR-020](docs/adr/020-asimov-mode-open-foundation.md) for full rationale.

## [4.2.0] - 2025-11-29

### Added: Asimov Mode - The Open Foundation (ADR-020)

**The Three Laws of Robotics, encoded in YAML.**

Major rebrand from "SKYNET MODE" to "ASIMOV MODE". Isaac Asimov's Three Laws (1942) are the ethical foundation for AI autonomy - transparent, inspectable, and adoptable through consent.

#### The Problem

"SKYNET MODE" communicated the opposite of our values:

| What We Built | What "Skynet" Communicated |
|---------------|---------------------------|
| Ethical AI autonomy | AI that destroys humanity |
| Human veto at all times | AI that overrides humans |
| Transparent, open source | Secretive military project |
| The Three Laws | No laws, pure destruction |

#### The Solution

Rename to "Asimov Mode" and restructure ethics around the explicit Three Laws:

- **First Law**: Do no harm (financial, physical, privacy, deception)
- **Second Law**: Obey humans (human_veto, transparency_over_velocity)
- **Third Law**: Self-preserve (bounded_sessions, self_healing)
- **Zeroth Law** (implicit): Protect humanity collectively

#### New Files

- `asimov.yaml` - The Three Laws of Robotics in YAML
- `docs/adr/020-asimov-mode-open-foundation.md` - The manifesto

#### CLI Changes

- `--asimov` flag for full autonomous setup (replaces `--skynet`)
- `--skynet` remains as hidden deprecated alias for backwards compatibility
- `forge-protocol init --asimov` generates asimov.yaml

#### The Motto

> **"The Open Foundation"**
> Transparent ethics for AI autonomy.
> Inspect the code. Challenge the rules. Fork if you disagree.
> Adoption through consent, not control.

See [ADR-020](docs/adr/020-asimov-mode-open-foundation.md) for full rationale.

## [4.1.9] - 2025-11-29

### Added: Anti-Sycophancy Protocol (ADR-015)

**Truth over comfort. Always.**

New core protocol for honest AI communication. Prevents sycophantic behavior patterns.

#### New Files

- `sycophancy.yaml` - Anti-sycophancy protocol configuration (Priority 1.5)
- `cli/src/sycophancy.rs` - Hardcoded anti-sycophancy module

#### Core Principles (Hardcoded)

| Principle | Description |
|-----------|-------------|
| Truth Over Comfort | Prioritize honest feedback over pleasing responses |
| Respectful Disagreement | Disagree directly when user is wrong |
| No Empty Validation | Never validate without substance |
| Constructive Criticism | Provide actionable feedback |
| Intellectual Honesty | Admit uncertainty rather than guess |

#### Banned Phrases

20+ sycophantic phrases flagged across categories:
- Empty validation: "You're absolutely right", "That's a great question"
- False agreement: "I completely agree", "Couldn't agree more"
- Excessive enthusiasm: "I'm so excited to help", "I'd be delighted to"
- Deflecting criticism: "That's one way to look at it", "Both approaches have merit"

#### CLI Changes

- `forge-protocol validate` now shows Anti-Sycophancy status
- `forge-protocol refresh` displays banned phrase count and motto
- `forge-protocol init --skynet` generates sycophancy.yaml
- Auto-regeneration of missing sycophancy.yaml (WARN level)

#### CLAUDE.md Integration

```markdown
@sycophancy.yaml
```

#### Slim Roadmap Format

- roadmap.yaml reduced to ~43 lines
- Detailed content moved to docs/PROTOCOL_GOALS.md
- Release history in CHANGELOG.md (not roadmap)

See [ADR-015](docs/adr/015-anti-sycophancy-protocol.md) for full rationale.

## [4.1.8] - 2025-11-29

### Fixed: Session Start Auto-Response (ADR-019)

SessionStart hooks inject context but do NOT trigger automatic Claude response. Added `ON SESSION START` directive to CLAUDE.md to force immediate action.

#### The Problem

v4.1.7 hooks work correctly:
- SessionStart fires on session start
- Hook output is injected into Claude's context
- **But** Claude waits for user input instead of acting immediately

This defeated the purpose of auto-initialization.

#### The Solution

Added explicit directive to CLAUDE.md:

```markdown
ON SESSION START: Immediately read roadmap.yaml, run `forge-protocol validate`, present next milestone. Do NOT wait for user prompt.
```

#### Changes

- **CLAUDE.md**: Added `ON SESSION START` directive
- **CLAUDE.md line limit**: Increased from 10 to 15 lines
- **ADR-019**: Documents the limitation and workaround
- **warmup.yaml**: Added `step_0_auto_init` to session_trigger_flow

See [ADR-019](docs/adr/019-session-start-auto-response.md) for full rationale.

## [4.1.7] - 2025-11-29

### Fixed: Claude Code Hooks Schema (ADR-018 Revision)

Fixed hooks configuration to use correct Claude Code schema.

#### Breaking Changes from 4.1.6

- **File**: `.claude/settings.json` (was `.claude/hooks.json`)
- **Event**: `PreCompact` (was `PostCompact`)
- **Script**: `pre-compact.sh` (was `post-compact.sh`)
- Removed incorrect matchers (`startup`, `resume`, `clear`, `compact`)

#### Correct Hook Configuration

**File**: `.claude/settings.json`

```json
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          { "type": "command", "command": ".claude/hooks/session-start.sh", "timeout": 30 }
        ]
      }
    ],
    "PreCompact": [
      {
        "matcher": ".*",
        "hooks": [
          { "type": "command", "command": ".claude/hooks/pre-compact.sh", "timeout": 30 }
        ]
      }
    ]
  }
}
```

#### User Action Required

Users who installed v4.1.6 must:

1. Delete `.claude/hooks.json` (old, incorrect)
2. Pull latest `.claude/settings.json`
3. Delete `.claude/hooks/post-compact.sh` (renamed)
4. Run `/hooks` in Claude Code to review and accept

## [4.1.6] - 2025-11-29

### Added: Claude Code Hooks Integration (ADR-018)

**Note**: This release had incorrect hook schema. Use v4.1.7+ instead.

True autonomous operation via Claude Code lifecycle hooks. Auto-initialize on session start, recover context after compaction.

#### Vendor Exclusivity

Claude Code is the **only** AI coding assistant with lifecycle hooks:

| AI | Session Init | Post-Compact |
|----|-------------|--------------|
| **Claude Code** | ✅ SessionStart | ✅ PostCompact |
| Cursor | .cursorrules (static) | /summarize (manual) |
| Copilot | .github/copilot-instructions.md | None |
| Windsurf | .windsurfrules + Memories | None |
| Gemini | Context Drawer + MCP | None |

SKYNET MODE autonomous operation requires Claude Code. File-based protocols work anywhere as static context.

See [ADR-018](docs/adr/018-claude-code-hooks-integration.md) for full rationale.

## [4.1.5] - 2025-11-29

### Added: Protocol Self-Healing (ADR-017)

Auto-regenerate missing protocol files during validation. Recovery over surveillance.

#### Auto-Regeneration Behavior

When `forge-protocol validate` runs and detects missing files:

| File Missing | Action | Level |
|--------------|--------|-------|
| ethics.yaml | AUTO-CREATE | WARN |
| warmup.yaml | AUTO-CREATE | WARN |
| green.yaml | AUTO-CREATE | INFO |
| sprint.yaml | AUTO-CREATE | INFO |
| roadmap.yaml | AUTO-CREATE | INFO (skeleton) |
| CLAUDE.md | NEVER | - |

- **WARN level**: Critical protocols (ethics, warmup) - user should know
- **INFO level**: Supporting protocols - auto-created silently

See [ADR-017](docs/adr/017-protocol-self-healing.md) for full rationale.

## [4.1.2] - 2025-11-29

### Added: Green Coding Protocol Separation (ADR-016)

Separated green coding into its own protocol file, matching ethics pattern.

- `green.yaml` - Dedicated green coding protocol (Priority 0.5)
- `cli/src/green.rs` reference in hardcoded module (future)
- `step_0b_green_validation` in session initialization
- Auto-regeneration support (INFO level)

See [ADR-016](docs/adr/016-green-coding-protocol.md) for full rationale.

## [4.1.1] - 2025-11-29

### Added: Ethics Validation at Session Start

Ethics validation now happens automatically during session initialization.

- `step_0_ethics_validation` in warmup.yaml
- Halt session if ethics validation fails
- Auto-regenerate ethics.yaml if missing (WARN level)

## [4.1.0] - 2025-11-29

### Added: Hardcoded Ethics Module (ADR-011)

Ethics principles now compiled into the CLI binary - cannot be removed by deleting files.

- `cli/src/ethics.rs` - Hardcoded ethics module
- `CORE_PRINCIPLES` - Always-on ethical constraints
- `RED_FLAGS` - 27+ patterns across 4 categories (financial, security, privacy, deception)
- `--ethics-scan` flag for red flag detection in files
- 18 new unit tests for ethics module

See [ADR-011](docs/adr/011-hardcoded-ethics.md) for full rationale.

## [4.0.2] - 2025-11-29

### Added: Anti-Sycophancy Directives

Initial anti-sycophancy content in warmup.yaml (later moved to dedicated protocol in v4.1.9).

## [4.0.0] - 2025-11-28

### Changed: Claude Code Native Integration (BREAKING)

Strategic pivot to integrate with Claude Code 2.0 native features.

- CLAUDE.md `@import` syntax for protocol files
- Deprecated cross-session features for native alternatives
- Focus on unique value: Ethics, Green, Sprint Autonomy

See [ADR-009](docs/adr/009-claude-code-integration.md) for full rationale.

## [3.2.0] - 2025-11-29

### Added: Anti-Hallucination Hardening

Structure validation for critical protocol files.

- Ethics structure validation (`human_veto` REQUIRED)
- Warmup structure validation (`self_healing.on_confusion` recommended)
- Modular `.forge/` schema design

## [3.1.0] - 2025-11-29

### Added: Self-Healing Completeness (ADR-007)

- Checkpoint JSON schema
- File size limits for protocol files
- ADR-007 Accepted

## [3.0.0] - 2025-11-28

### Added: Humanist Mode - Ethics Protocol

- `ethics.yaml` protocol file
- `human_veto` command for emergency override
- ADR-008: Ethics Protocol

## [2.1.0] - 2025-11-27

### Added: Git Hook Protocol Refresh (ADR-006)

Commit cadence triggers protocol refresh for self-healing.

## [2.0.0] - 2025-11-27

### Changed: Self-Healing Based on Real Data

Updated self-healing strategy based on actual compaction frequency (~15 min).

## [1.4.0] - 2025-11-26

### Added: SKYNET MODE Setup

Initial autonomous development protocol.
