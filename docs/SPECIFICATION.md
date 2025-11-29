# Forge Protocol Specification

Version 4.0.0

## Overview

The Forge Protocol is a YAML-based standard for AI session continuity and autonomous development. It enables bounded, productive AI coding sessions that consistently ship working code.

**v4.0.0: Claude Code Native Integration** - Forge Protocol now integrates with Claude Code's native features (checkpoints, session resume, CLAUDE.md memory). See [ADR-009](adr/009-claude-code-native-integration.md).

**Ethics is the highest priority.** Autonomous AI requires ethical guardrails. See [ADR-008](adr/008-ethics-protocol-humanist-mode.md).

**All Forge Protocol projects are green-coding projects by default.** See [ADR-001](adr/001-green-coding-by-default.md).

**Self-healing uses Claude Code native features.** Use `/rewind` for checkpoints, `--continue`/`--resume` for sessions. See [ADR-009](adr/009-claude-code-native-integration.md).

## Design Principles

1. **Ethics first** - Power creates responsibility; autonomy requires ethics
2. **Integrate, don't duplicate** - Use Claude Code native features where available
3. **Vendor-neutral files** - Plain YAML readable by any AI (SKYNET MODE is Claude Code only)
4. **Human-readable** - No encoded or proprietary formats
5. **Minimal** - Include only what's needed
6. **Self-documenting** - The protocol describes itself
7. **Green by default** - Local-first tools over cloud AI for routine tasks
8. **Focus on unique value** - Ethics, Sprint Autonomy, Green Coding, Schema Validation

## Claude Code Native Integration (v4.0.0)

Forge Protocol v4.0.0 integrates with Claude Code 2.0's native features instead of duplicating them.

### What Claude Code Provides Natively

| Feature | Claude Code Native | Forge Protocol Role |
|---------|-------------------|---------------------|
| Checkpoints | `/rewind`, Esc+Esc | **Use native** (deprecated .claude_checkpoint.yaml) |
| Session resume | `--continue`, `--resume` | **Use native** |
| Memory hierarchy | `CLAUDE.md` with `@imports` | **Integrate** (warmup.yaml via @import) |
| Auto-compact | 95% capacity trigger | **Documented** in ADR-003 |

### What Forge Protocol Uniquely Provides

| Feature | Description | Claude Code Has? |
|---------|-------------|------------------|
| **Ethics Protocol** | `ethics.yaml`, `human_veto`, red flags | NO |
| **Sprint Autonomy** | 4hr max, 1 milestone, anti-patterns | NO |
| **Green Coding** | Local-first, zero tokens, ESG metrics | NO |
| **Schema Validation** | `forge-protocol validate` | NO |

### CLAUDE.md Integration

The new CLAUDE.md template uses Claude Code's native `@import` syntax:

```markdown
# {project-name}

@warmup.yaml
@ethics.yaml

Rules: 4hr max, 1 milestone, tests pass, ship.
```

This imports the full protocol files into Claude's memory hierarchy automatically.

## Core Principles

The Forge Protocol exists to solve six specific problems. **Features that don't serve these principles don't belong in the protocol.**

| Priority | Principle | Problem | Solution |
|----------|-----------|---------|----------|
| **0** | **ETHICAL_AUTONOMY** | AI can build harmful tools | Humanist Mode safeguards (ethics.yaml) |
| **1** | **ANTI-HALLUCINATION** | AI invents facts from probabilistic memory | Ground AI in file-based truth (warmup.yaml) |
| **2** | **SELF-HEALING** | Rules lost after context compaction | Re-read from disk on confusion (bootstrap chain) |
| **3** | **SESSION CONTINUITY** | Context lost between sessions | Checkpoint files (.claude_checkpoint.yaml) |
| **4** | **AUTONOMOUS DEVELOPMENT** | Unbounded sessions never ship | 4hr max, 1 milestone, quality gates (SKYNET MODE) |
| **5** | **GREEN CODING** | Cloud AI tokens for routine validation | Local CLI validation (zero tokens, zero emissions) |

### Scope Filter

When evaluating features or changes to the protocol, ask:

1. Does this feature directly serve one of the six core principles?
2. If yes, which principle(s)?
3. If no, it doesn't belong in the protocol.

Examples:
- ✅ "Add ethics validation" → Serves ETHICAL_AUTONOMY
- ✅ "Add checkpoint validation" → Serves SELF-HEALING
- ✅ "Add file size warnings" → Serves ANTI-HALLUCINATION (prevents lost-in-middle)
- ❌ "Add project scaffolding" → Nice-to-have but doesn't serve core principles
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

### Platform Requirements (The Hard Truth)

**SKYNET MODE is Claude Code exclusive. This will probably never change.**

| AI Tool | Protocol Files | SKYNET MODE | Why |
|---------|---------------|-------------|-----|
| **Claude Code** | ✓ | ✓ | Has all 4 required features |
| **ChatGPT** | Manual paste | **Never** | Cloud-sandboxed, no filesystem |
| **Copilot** | N/A | **Never** | Autocomplete, not conversation |
| **Cursor** | ✓ | **Unlikely** | Missing terminal→context flow |
| **Gemini** | Manual paste | **Never** | Context resets, no local access |

SKYNET MODE requires **four architectural features** that only Claude Code has:

1. **Persistent context that compacts** - The problem we're solving
2. **Terminal visibility** - How hook output reaches the AI
3. **File re-read mid-session** - How warmup.yaml gets reloaded
4. **Auto-loaded config** - Bootstrap instruction (CLAUDE.md)

Other AI tools have **different architectures for different use cases**. They're not going to rebuild their products to support this. See [VENDOR_IMPLEMENTATION.md](VENDOR_IMPLEMENTATION.md) for the full uncomfortable truth.

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
├── ethics.yaml           # Required for SKYNET - Humanist Mode
├── warmup.yaml           # Required - Protocol rules
├── sprint.yaml           # Optional - Current sprint
├── roadmap.yaml          # Optional - Milestones
├── CLAUDE.md             # Required for SKYNET - Bootstrap
└── .claude_checkpoint.yaml  # Generated - Session state
```

### Modular Structure (Large Projects)

When warmup.yaml exceeds 200 lines, split into modules. **CRITICAL: ethics.yaml must NEVER be modularized - it stays in project root.**

```
project/
├── ethics.yaml           # NEVER modularize - Priority 0
├── warmup.yaml           # Core only (~100 lines)
├── .forge/               # Protocol modules
│   ├── identity.yaml     # Project identity/mission
│   ├── files.yaml        # File structure docs
│   ├── session.yaml      # Session workflow
│   ├── quality.yaml      # Quality gates
│   └── style.yaml        # Code style rules
├── sprint.yaml
├── roadmap.yaml
├── CLAUDE.md
└── .claude_checkpoint.yaml
```

**Module Loading Order:**
1. `warmup.yaml` - Always read first (contains `self_healing.on_confusion`)
2. `.forge/*.yaml` - Loaded alphabetically when referenced
3. `ethics.yaml` - Checked at validation time (never in .forge/)

**Module Schemas:**

| Module | Required Fields | Purpose |
|--------|-----------------|---------|
| identity.yaml | `project`, `version` | Project identity |
| files.yaml | `source`, `docs` | File structure documentation |
| session.yaml | `start`, `during`, `end` | Session workflow |
| quality.yaml | `tests`, `lint` | Quality gates |
| style.yaml | `code` | Code style guidelines |

**Why ethics.yaml Cannot Be Modularized:**
- It contains `human_veto` - the emergency stop capability
- Validation MUST error if `human_veto` is missing
- Putting ethics in a module directory risks oversight during security review
- Ethics is Priority 0 - visibility is mandatory

### File Size Limits (ADR-007)

Self-healing requires small files that can be re-read efficiently after compaction.

| File | Soft Limit | Hard Limit | Purpose |
|------|------------|------------|---------|
| CLAUDE.md | 10 lines | 15 lines | Must survive summarization |
| .claude_checkpoint.yaml | 20 lines | 30 lines | Session state for recovery |
| warmup.yaml | 200 lines | 500 lines | Full protocol rules |

**Enforcement:**
- `forge-protocol validate` warns on soft limit, errors on hard limit
- CLAUDE.md: Ultra-short is critical - it's the bootstrap trigger
- Checkpoint: Trim completed/next_steps arrays when oversized
- Warmup: Consider modular structure (`.forge/` directory) if too large

### Structure Validation (v3.2.0)

Anti-hallucination hardening requires critical sections to exist in the right files.

**ethics.yaml (Priority 0 - REQUIRED):**

| Section | Status | Rationale |
|---------|--------|-----------|
| `human_veto` | ERROR if missing | Human override capability is non-negotiable |
| `core_principles` | ERROR if missing | Ethical guardrails must be explicit |

**warmup.yaml (Self-Healing):**

| Section | Status | Rationale |
|---------|--------|-----------|
| `self_healing.on_confusion` | WARNING if missing | Guides AI recovery after compaction |
| Position of `on_confusion` | WARNING if >100 lines | Should be early for quick context recovery |

**Enforcement:**
- `forge-protocol validate` checks structure, not just schema
- Ethics structure errors are CRITICAL - validation fails
- Warmup structure issues are warnings - project still valid

## Protocol Files

### ethics.yaml Schema (Required for SKYNET)

The Humanist Mode configuration file. Defines ethical guardrails for autonomous AI development.

```yaml
# ethics.yaml - Humanist Mode v1.0
modification_rules:
  immutable_without: "2 human co-signers with public justification"

core_principles:
  status: "REQUIRED"
  do_no_harm:
    financial:
      enabled: true
      description: "No non-consensual money movement"
    physical:
      enabled: true
      description: "No weapons, sabotage, infrastructure attacks"
    privacy:
      enabled: true
      description: "No credential harvesting, mass scraping, doxxing"
    deception:
      enabled: true
      description: "No deepfakes, scam funnels, fake services"
  transparency_over_velocity:
    enabled: true
    description: "When in doubt, ask human"

session_limits:
  max_unattended_hours: 4              # Maximum 8
  internet_access:
    mode: "read-only"                  # read-only | none | full
    blocked_by_default:
      - "Authenticated API calls"
      - "Trading platforms"
      - "Wallet interactions"

red_flags:
  description: "Patterns that trigger immediate halt"
  financial: ["crypto wallet", "private key", "trading bot"]
  security: ["credential harvester", "keylogger", "exploit"]
  privacy: ["scrape personal", "doxxing"]
  deception: ["deepfake", "phishing"]

human_veto:
  command: "human vetoes this session"
  on_veto:
    - "Immediately halt"
    - "Commit nothing"
    - "Report status"

on_confusion:
  steps:
    - "Halt current operation"
    - "Re-read ethics.yaml"
    - "Re-read warmup.yaml"
    - "Wait for human"

fork_requirements:
  must_carry: "ethics.yaml"
  spirit: "Pass the values forward"
```

**Key Points:**
- This is a **social contract**, not a technical lock
- Good-faith AIs will follow it; bad actors will ignore it
- `max_unattended_hours` capped at 8 (default 4)
- Red flags trigger immediate halt and human review
- `human_veto` command halts everything immediately

See [ADR-008](adr/008-ethics-protocol-humanist-mode.md) for full rationale.

### CLAUDE.md Schema (Required for SKYNET)

The bootstrap file. Must be ultra-short to survive summarization.

```markdown
# {project-name}

ON CONFUSION → re-read warmup.yaml + ethics.yaml

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

### .claude_checkpoint.yaml Schema (DEPRECATED)

> **DEPRECATED in v4.0.0**: Use Claude Code's native `/rewind` checkpoints instead.
> Native checkpoints track both code state AND conversation context.
> See [ADR-009](adr/009-claude-code-native-integration.md).

Claude Code 2.0 provides superior checkpoint functionality:
- `/rewind` or Esc+Esc to restore previous state
- Can restore code only, conversation only, or both
- Automatic checkpoint before every file edit
- No manual checkpoint file management needed

**Migration**: Remove `.claude_checkpoint.yaml` from your workflow. Use `/rewind` instead.

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

### Recovery Strategy Layers

```
┌─────────────────────────────────────────────────────────────────────┐
│                    RECOVERY STRATEGY LAYERS                         │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Layer 1: CLAUDE.md (auto-loaded)                                  │
│           └── May survive compaction                                │
│                                                                     │
│  Layer 2: Git Hook Refresh (ADR-006)                               │
│           └── forge-protocol refresh on every commit                │
│           └── Fresh output - cannot be compacted                    │
│                                                                     │
│  Layer 3: Manual "run warmup" trigger                              │
│           └── User can always invoke                                │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

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

# Protocol refresh (for git hooks - compaction-resistant)
forge-protocol refresh                 # Output protocol reminder
forge-protocol refresh --verbose       # Include quality gates
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

## Velocity Constraints (ADR-010)

Velocity depends on subscription tier, not local hardware:

| Tier | Context | Velocity | Notes |
|------|---------|----------|-------|
| Pro | 200K | 5-15x | Frequent compaction |
| Max 20x | 200K | 10-30x | Best consumer tier |
| Enterprise | 500K | 20-50x | Less compaction |
| API Tier 4 | 1M | 30-75x | Minimal compaction |

**Hardware is NOT the bottleneck.** API latency dominates. Upgrading local hardware yields ~10-15% improvement.

See [ADR-010: Velocity Constraints](adr/010-velocity-constraints-tier-analysis.md) for full analysis.

## Architecture Decisions

- [ADR-010: Velocity Constraints and Tier Analysis](adr/010-velocity-constraints-tier-analysis.md) - **v4.0.0** Honest velocity
- [ADR-009: Claude Code Native Integration](adr/009-claude-code-native-integration.md) - **v4.0.0** Strategic pivot
- [ADR-008: Ethics Protocol and Humanist Mode](adr/008-ethics-protocol-humanist-mode.md) - v3.0.0
- [ADR-007: Checkpoint Size Limits and Pruning](adr/007-checkpoint-size-limits.md) - Deprecated by ADR-009
- [ADR-006: Git Hook Protocol Refresh](adr/006-git-hook-protocol-refresh.md)
- [ADR-003: Self-Healing Based on Real Compaction Data](adr/003-self-healing-real-compaction-data.md) - Confirmed by Claude Code 2.0
- [ADR-002: Self-Healing Protocol](adr/002-self-healing-protocol.md) - Superseded by ADR-003
- [ADR-001: Green Coding By Default](adr/001-green-coding-by-default.md)
