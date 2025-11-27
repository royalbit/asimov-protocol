# Forge Protocol Specification

Version 2.0.0

## Overview

The Forge Protocol is a YAML-based standard for AI session continuity and autonomous development. It enables bounded, productive AI coding sessions that consistently ship working code.

**All Forge Protocol projects are green-coding projects by default.** See [ADR-001](adr/001-green-coding-by-default.md).

**Self-healing is based on real compaction data, not assumptions.** See [ADR-003](adr/003-self-healing-real-compaction-data.md).

## Design Principles

1. **Vendor-neutral** - Plain YAML readable by any AI
2. **Human-readable** - No encoded or proprietary formats
3. **Minimal** - Include only what's needed
4. **Self-documenting** - The protocol describes itself
5. **Green by default** - Local-first tools over cloud AI for routine tasks
6. **Recoverable over survivable** - Re-read from disk, don't try to survive compaction

## Core Goals

The Forge Protocol exists to solve five specific problems. **Features that don't serve these goals don't belong in the protocol.**

| Goal | Problem | Solution |
|------|---------|----------|
| **ANTI-HALLUCINATION** | AI invents facts from probabilistic memory | Ground AI in file-based truth (warmup.yaml) |
| **SELF-HEALING** | Rules lost after context compaction | Re-read from disk on confusion (bootstrap chain) |
| **SESSION CONTINUITY** | Context lost between sessions | Checkpoint files (.claude_checkpoint.yaml) |
| **AUTONOMOUS DEVELOPMENT** | Unbounded sessions never ship | 4hr max, 1 milestone, quality gates (SKYNET MODE) |
| **GREEN CODING** | Cloud AI tokens for routine validation | Local CLI validation (zero tokens, zero emissions) |

### Scope Filter

When evaluating features or changes to the protocol, ask:

1. Does this feature directly serve one of the five core goals?
2. If yes, which goal(s)?
3. If no, it doesn't belong in the protocol.

Examples:
- ✅ "Add checkpoint validation" → Serves SELF-HEALING
- ✅ "Add file size warnings" → Serves ANTI-HALLUCINATION (prevents lost-in-middle)
- ❌ "Add project scaffolding" → Nice-to-have but doesn't serve core goals
- ❌ "Add AI chat interface" → Out of scope

## SKYNET MODE

SKYNET MODE is the complete autonomous AI development system. It consists of five components:

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           SKYNET MODE                                   │
│                  Autonomous AI Development System                       │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│   ┌───────────────┐   ┌───────────────┐   ┌───────────────┐            │
│   │   PROTOCOL    │   │    SPRINT     │   │    QUALITY    │            │
│   │    FILES      │   │   AUTONOMY    │   │    GATES      │            │
│   │               │   │               │   │               │            │
│   │  warmup.yaml  │   │  4hr max      │   │  Tests pass   │            │
│   │  sprint.yaml  │   │  1 milestone  │   │  Zero warns   │            │
│   │  roadmap.yaml │   │  Then STOP    │   │  Then commit  │            │
│   └───────────────┘   └───────────────┘   └───────────────┘            │
│                                                                         │
│   ┌───────────────┐   ┌───────────────┐                                │
│   │     SELF      │   │    RELEASE    │                                │
│   │    HEALING    │   │   DISCIPLINE  │                                │
│   │               │   │               │                                │
│   │  Re-read on   │   │  GitHub       │                                │
│   │  confusion    │   │  + Registry   │                                │
│   │  CLAUDE.md    │   │  Every time   │                                │
│   └───────────────┘   └───────────────┘                                │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Why All Five Components?

| Without... | Failure Mode |
|------------|--------------|
| Protocol Files | AI doesn't know project conventions |
| Sprint Autonomy | Sessions run forever, nothing ships |
| Quality Gates | Code ships with bugs and warnings |
| Self-Healing | Rules forgotten after compaction |
| Release Discipline | Code written but never released |

**Remove any component and the system breaks.**

### Platform Requirements

| Feature | Any AI | Claude Code |
|---------|--------|-------------|
| Protocol files (paste/upload) | Yes | Yes |
| SKYNET MODE (unattended) | No | Yes |
| Self-Healing Protocol | No | Yes |

SKYNET MODE requires Claude Code because it depends on:
- Auto-loaded config (CLAUDE.md)
- File system access mid-session
- Re-read capability after compaction

## The Bootstrap Chain

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        BOOTSTRAP CHAIN                                   │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│   CLAUDE.md              warmup.yaml           .claude_checkpoint.yaml   │
│   (auto-loaded)          (full protocol)       (session state)           │
│   ~5 lines               ~100-200 lines        ~20 lines                 │
│                                                                          │
│   ┌────────────┐         ┌────────────┐        ┌────────────┐           │
│   │ BOOTSTRAP  │────────▶│ FULL RULES │───────▶│ CHECKPOINT │           │
│   │ "re-read   │         │ Everything │        │ Progress   │           │
│   │ warmup"    │         │ defined    │        │ Next steps │           │
│   └────────────┘         └────────────┘        └────────────┘           │
│        │                       │                     │                   │
│   Survives              Re-read from            Written during           │
│   compaction            disk on trigger         session                  │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

## File Structure

### Standard Structure

```
project/
├── warmup.yaml           # Required - Protocol rules
├── sprint.yaml           # Optional - Current sprint
├── roadmap.yaml          # Optional - Milestones
├── CLAUDE.md             # Required for SKYNET - Bootstrap
└── .claude_checkpoint.yaml  # Generated - Session state
```

### Modular Structure (Large Projects)

```
project/
├── warmup.yaml           # Core only (~100 lines)
├── .forge/               # Protocol modules
│   ├── autonomy.yaml     # Session autonomy rules
│   ├── quality.yaml      # Quality gates
│   └── release.yaml      # Release workflow
├── sprint.yaml
├── roadmap.yaml
├── CLAUDE.md
└── .claude_checkpoint.yaml
```

## Protocol Files

### CLAUDE.md Schema (Required for SKYNET)

The bootstrap file. Must be ultra-short to survive summarization.

```markdown
# {project-name}

ON CONFUSION → re-read warmup.yaml

Rules: 4hr max, 1 milestone, tests pass, ship.
```

**Constraints:**
- Maximum 10 lines
- Single critical instruction: "re-read warmup.yaml"
- Core rules in one line

### warmup.yaml Schema

The master protocol file. Must be in project root.

#### identity (required)

```yaml
identity:
  project: "Project Name"           # required
  tagline: "Brief description"      # optional
  version: "1.0.0"                  # optional
  philosophy: "Guiding principle"   # optional
```

#### mission (optional)

```yaml
mission:
  problem: "What problem does this solve?"
  solution: "How does it solve it?"
  principles:
    - "Principle one"
    - "Principle two"
```

#### files (recommended)

```yaml
files:
  source:
    - "src/main.rs - Entry point"
    - "src/lib.rs - Library root"
  config:
    - "Cargo.toml - Dependencies"
  docs:
    - "README.md - User docs"
```

#### session (recommended)

```yaml
session:
  start:
    - "Read warmup.yaml"
    - "git status"
  during:
    - "Track progress with TodoWrite"
    - "Test frequently"
  end:
    - "All tests pass"
    - "Zero warnings"
    - "Update documentation"
```

#### quality (required for SKYNET)

```yaml
quality:
  tests: "cargo test"
  warnings: "cargo clippy -- -D warnings"
  formatting: "cargo fmt --check"
```

#### style (optional)

```yaml
style:
  code:
    - "Result<T, E> for errors"
    - "No unwrap() in library code"
  docs:
    - "Markdown for documentation"
```

#### green_coding (recommended)

```yaml
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use CLI tools for validation, linting, formatting"
    - "Reserve AI for complex reasoning tasks"
  why:
    - "Local validation: $0 and ~0.002g CO2"
    - "Cloud AI validation: $0.02+ and ~0.5g CO2"
```

#### self_healing (required for SKYNET)

Based on real compaction data from [ADR-003](adr/003-self-healing-real-compaction-data.md).

```yaml
self_healing:
  # Checkpoint triggers (based on real compaction patterns)
  checkpoint_triggers:
    - "Every major task completion"
    - "Every 10-15 tool calls (~15 min)"
    - "Before any commit"
    - "On any confusion"

  checkpoint_file: ".claude_checkpoint.yaml"

  # Recovery instruction (must be short)
  on_confusion: "Re-read warmup.yaml immediately"

  # Core rules that must survive (one line)
  core_rules: "4hr max, 1 milestone, tests pass, ship it"
```

#### autonomous_development (required for SKYNET)

```yaml
autonomous_development:
  # Session trigger
  trigger_phrases:
    - "run warmup"
    - "warmup"

  # Confirmation phrases
  confirm_phrases:
    - "go"
    - "punch it"
    - "ship it"
    - "run"

  # Boundaries
  boundaries:
    max_duration: "4 hours"
    max_milestones: 1
    scope_creep: "reject - note for next session"

  # Anti-patterns to reject
  anti_patterns:
    - "Let me also..."
    - "While I'm here..."
    - "This would be better if..."
```

#### release (recommended)

```yaml
release:
  checklist:
    - "All tests pass"
    - "Zero warnings"
    - "Version bumped"
    - "CHANGELOG updated"
    - "Committed and tagged"

  targets:
    github: "git push origin main && git push origin vX.Y.Z"
    registry: "cargo publish"  # or npm publish, etc.
```

### sprint.yaml Schema

Active work tracking with session boundaries.

```yaml
sprint:
  current: "Feature name or task"
  started: "2025-01-15"
  status: in_progress  # planned | in_progress | blocked | done

  # Boundaries (required for SKYNET)
  boundaries:
    max_duration: "4 hours"
    max_milestones: 1

  tasks:
    - "[x] Task completed"
    - "[ ] Task pending"

  blockers: []

  notes: "Any relevant context"
```

### roadmap.yaml Schema

Milestone planning.

```yaml
metadata:
  current_version: "1.0.0"
  last_updated: "2025-01-15"

current:
  version: "1.0.0"
  status: released
  summary: "Initial release"
  highlights:
    - "Core feature one"

next:
  version: "1.1.0"
  status: planned
  summary: "Next milestone"
  features:
    - "Planned feature"

backlog:
  - "Future idea one"
```

### .claude_checkpoint.yaml Schema

Session state file. Written during autonomous sessions, not committed to git.

```yaml
timestamp: "2025-01-15T10:30:00Z"
session_started: "2025-01-15T09:00:00Z"
tool_calls: 45

milestone: "Add feature X"
status: in_progress

completed:
  - "Task 1: Implemented core logic"
  - "Task 2: Wrote unit tests"

in_progress: "Task 3: Update documentation"

next_steps:
  - "Task 4: Integration tests"
  - "Task 5: Update CHANGELOG"

# Recovery hint
on_confusion: "cat warmup.yaml"
```

**Must be in .gitignore:**
```
.claude_checkpoint.yaml
```

## Session Autonomy

### The Session Flow

```
User: "run warmup"
  ↓
AI: Reads warmup.yaml, sprint.yaml, roadmap.yaml
AI: Presents next milestone
  ↓
User: "go" / "punch it" / "ship it"
  ↓
AI: AUTONOMOUS EXECUTION
  - Makes all decisions independently
  - Writes checkpoints every ~15 min
  - Runs tests frequently
  - NO questions (uses best judgment)
  - STOPS at 4 hours
  ↓
AI: Quality gates (tests, warnings)
  ↓
AI: Release (commit, tag, push, publish)
  ↓
AI: Report results
```

### Checkpoint Triggers

Based on real compaction data (see [ADR-003](adr/003-self-healing-real-compaction-data.md)):

| Trigger | Rationale |
|---------|-----------|
| Every major task | Natural breakpoint |
| Every 10-15 tool calls | ~15 min of work |
| Before file write >100 lines | Significant change |
| Before any commit | Quality gate |
| On any confusion | Recovery signal |

**NOT "every 2 hours"** - compaction happens every 10-20 minutes with heavy reasoning.

### Anti-Patterns (Reject)

| Anti-Pattern | Response |
|--------------|----------|
| "While I'm here..." | "Noted for next session. Shipping current work." |
| "Let me also..." | "Out of scope. Added to backlog." |
| "This would be better if..." | "Refactoring noted. Shipping as-is." |

## Self-Healing Protocol

### Why Recovery > Survival

| Approach | Strategy | Result |
|----------|----------|--------|
| Survival | Make rules survive compaction | **Fails** - summarizer compresses everything |
| Recovery | Re-read from disk after compaction | **Works** - files are always available |

### The Three Files

| File | Purpose | Size | Committed |
|------|---------|------|-----------|
| CLAUDE.md | Bootstrap trigger | ~5 lines | Yes |
| warmup.yaml | Full protocol | ~100-200 lines | Yes |
| .claude_checkpoint.yaml | Session state | ~20 lines | No |

### Recovery Flow

```
Context compacted
  ↓
AI confused / rules lost
  ↓
CLAUDE.md instruction survives: "re-read warmup.yaml"
  ↓
AI reads warmup.yaml from disk
  ↓
Rules restored
  ↓
AI reads .claude_checkpoint.yaml
  ↓
Progress restored
  ↓
Continue working
```

## Quality Gates

All quality checks must pass before any commit or release.

### Required Checks

```yaml
quality:
  tests: "All tests must pass"
  warnings: "Zero warnings allowed"
```

### Language-Specific

| Language | Tests | Lint | Format |
|----------|-------|------|--------|
| Rust | `cargo test` | `cargo clippy -- -D warnings` | `cargo fmt` |
| Python | `pytest` | `ruff check .` | `ruff format .` |
| Node.js | `npm test` | `npm run lint` | `npm run format` |
| Go | `go test ./...` | `golangci-lint run` | `go fmt ./...` |

## Release Discipline

Every session ends with a release. No "work in progress" commits.

### Release Checklist

1. All tests pass
2. Zero warnings
3. Version bumped in config
4. CHANGELOG.md updated
5. Committed with message format
6. Tagged with version
7. Pushed to origin
8. Published to registry (if applicable)

### Commit Message Format

```
<type>: <description>

<body>

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`

## CLI Support

### Installation

```bash
cargo install forge-protocol
```

### Commands

```bash
# Generate protocol files
forge-protocol init                    # Basic warmup.yaml
forge-protocol init --type rust        # Language-specific
forge-protocol init --full             # All three files
forge-protocol init --skynet           # Full SKYNET MODE setup

# Validate
forge-protocol validate                # All files
forge-protocol validate warmup.yaml    # Specific file

# Lint documentation
forge-protocol lint-docs               # Check markdown
forge-protocol lint-docs --fix         # Auto-fix issues
```

### --skynet Flag

Generates complete SKYNET MODE setup:

```
✓ warmup.yaml      - Protocol rules
✓ sprint.yaml      - Session boundaries
✓ roadmap.yaml     - Milestone planning
✓ CLAUDE.md        - Self-healing trigger
✓ .hooks/          - Pre-commit hooks
✓ .gitignore       - Checkpoint file excluded
```

## Activation

### Claude Code (CLAUDE.md)

Add to `~/.claude/CLAUDE.md` or project `CLAUDE.md`:

```markdown
- If there is a warmup.yaml file in the root of the working dir, run it as working protocol
```

### Other AI Assistants

Paste warmup.yaml content at session start. Note: Self-healing won't work without file system access.

## Best Practices

1. **Keep CLAUDE.md ultra-short** - Must survive summarization
2. **Update sprint.yaml actively** - Track work in progress
3. **Commit protocol files** - They're part of your codebase
4. **Use checkpoints** - Write state frequently, not on schedule
5. **Review after compaction** - Check if rules are still understood

## Architecture Decisions

- [ADR-001: Green Coding By Default](adr/001-green-coding-by-default.md)
- [ADR-002: Self-Healing Protocol](adr/002-self-healing-protocol.md) (superseded by ADR-003)
- [ADR-003: Self-Healing Based on Real Compaction Data](adr/003-self-healing-real-compaction-data.md)
- [ADR-004: Distributed SQL with YugabyteDB](adr/004-distributed-sql-yugabytedb.md)
- [ADR-005: Event-Driven with Redis Streams](adr/005-event-driven-redis-streams.md)
- [ADR-006: Git Hook Protocol Refresh](adr/006-git-hook-protocol-refresh.md)
