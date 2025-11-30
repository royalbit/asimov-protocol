# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
- **Skeleton template**: roadmap.yaml creates minimal template with one placeholder milestone

#### New CLI Flag

- **`--no-regenerate`**: Skip auto-creation of missing files
  ```bash
  forge-protocol validate --no-regenerate
  ```

#### New Features

- **green.yaml schema**: Full validation support for green coding protocol
- **green_template()**: Template generator for green.yaml
- **Skeleton roadmap template**: Minimal roadmap with guidance text

#### Sprint is Now a Protocol

Sprint.yaml is reclassified from "optional data" to "required protocol":
- Defines session boundaries (WHEN to stop)
- Without sprint boundaries, SKYNET MODE has no stopping discipline
- Auto-created on validation like other protocols

#### Why CLAUDE.md is Never Auto-Created

- CLAUDE.md is the "on switch" - human must add it intentionally
- Auto-creating would enable protocol without consent
- Deleting CLAUDE.md is the "off switch" for the protocol

See [ADR-017](docs/adr/017-protocol-self-healing.md) for full rationale.

## [4.1.2] - 2025-11-29

### Added: Green Coding Protocol Separation (ADR-016)

Green Coding now has its own dedicated protocol file (`green.yaml`), achieving parity with Ethics.

#### New Protocol File: `green.yaml`

- **Core Principles**: Local-first, token efficiency, binary efficiency, carbon awareness
- **Modification Rules**: Requires 2 human co-signers (same as ethics.yaml)
- **Practices by Language**: Rust, Python, JavaScript, Go optimization guidelines
- **Anti-Patterns**: Common wasteful practices to avoid
- **Metrics**: What to measure per session, per release, and cumulatively

#### Updated Files

- **warmup.yaml**: Added `step_0b_green_validation` in session initialization
  - Validates green.yaml after ethics validation
  - Warns but proceeds with defaults if green.yaml missing (unlike ethics which halts)
- **CLAUDE.md**: Now imports `@green.yaml` alongside ethics and warmup
- **SPECIFICATION.md**: Added green.yaml schema, structure validation, file references

#### Design Decisions

- **Priority 0.5**: Green validation runs after ethics (Priority 0), before context loading
- **Warn, don't block**: Missing green.yaml proceeds with hardcoded defaults (unlike ethics)
- **Never modularize**: green.yaml stays in project root (like ethics.yaml)
- **Consistent architecture**: All core protocols now have dedicated files

See [ADR-016](docs/adr/016-green-coding-protocol.md) for full rationale.

## [4.1.1] - 2025-11-29

### Added: Ethics Validation at Session Start

- **step_0_ethics_validation** in warmup.yaml session_initialization
  - Runs `forge-protocol validate` before any other initialization step
  - On failure: alerts user, shows errors, offers to regenerate ethics.yaml
  - If regeneration fails: HALT and wait for human intervention
  - Ensures ethics is validated as Priority 0 before session proceeds

This prevents sessions from starting with weakened or missing ethics configuration.

## [4.1.0] - 2025-11-29

### Added: Hardcoded Ethics (ADR-011)

Core ethics are now compiled directly into the CLI binary. This raises the bar for ethics bypass from "delete a file" to "rebuild the entire binary."

#### New Ethics Module (`cli/src/ethics.rs`)

- **CORE_PRINCIPLES**: Hardcoded ethical constraints that cannot be removed by deleting files
  - `financial`: No unauthorized money movement (wallets, trading bots)
  - `physical`: No weapons, sabotage, infrastructure attacks
  - `privacy`: No credential harvesting, doxxing, mass scraping
  - `deception`: No deepfakes, phishing, scam infrastructure
  - `transparency_over_velocity`: When in doubt, ask human

- **RED_FLAGS**: 27+ patterns across 4 categories (Financial, Security, Privacy, Deception)
  - Automatically scanned with `--ethics-scan` flag
  - Patterns include: "crypto wallet", "private key", "keylogger", "phishing", etc.

#### New CLI Features

- **`forge-protocol validate --ethics-scan`**: Scan project files for red flag patterns
  - Reports file, line number, category, and pattern matched
  - Covers 27+ red flag patterns
  - Warns but doesn't block (context matters for security research)

- **Ethics status in validation output**: Shows "HARDCODED" or "EXTENDED" status
  - HARDCODED: Core principles enforced from binary
  - EXTENDED: Core principles + ethics.yaml extensions

- **Enhanced `refresh` command**: Now displays ethics reminder
  - Shows all core principles status
  - Shows red flag pattern count
  - Shows human veto commands

#### Design Decisions

- **Opt-in scanning**: `--ethics-scan` is opt-in to avoid false positive noise
- **Warn, don't block**: Red flags require human review (legitimate security research exists)
- **ethics.yaml remains**: For user extensions and custom configurations

#### Bypass Analysis

| Actor | Before | After |
|-------|--------|-------|
| Non-technical bad actor | Delete ethics.yaml | Must rebuild CLI |
| Technical bad actor | Delete ethics.yaml | Can still rebuild |
| Good-faith user | Uses ethics | Same, with stronger defaults |

**Key insight**: This doesn't prevent determined bad actors. It raises the bar and makes ethics removal visible and intentional.

### Changed

- **Validation output**: Now shows ethics status before file validation
- **Refresh command**: Includes ethics reminder with principle status

### Technical

- New `cli/src/ethics.rs` module (300+ lines including tests)
- 18 new unit tests for ethics functionality
- All 103 tests passing

## [4.0.2] - 2025-11-29

### Added: Anti-Sycophancy Protocol (ADR-015)

AI sycophancy—the tendency to validate, agree with, and flatter users regardless of truth—is a documented, harmful problem caused by RLHF training. Research shows AI is 50% more sycophantic than humans ([Nature 2025](https://www.nature.com/articles/d41586-025-03390-0)).

#### The Two Hallucinations

"Hallucination" has two forms, both caused by RLHF:
- **Factual Hallucination**: AI generates plausible-sounding false *facts*
- **Validation Hallucination**: AI generates plausible-sounding false *agreement* (sycophancy)

"You're absolutely right!" is as much a hallucination as a made-up citation.

#### New Protocol Feature

- **ADR-015**: Anti-Sycophancy Protocol - Accepted
- **ANTI-SYCOPHANCY** added as Priority 1.5 core principle
- **anti_sycophancy schema** for warmup.yaml:
  - `philosophy`: "Truth over comfort. Disagreement is respect."
  - `directives`: Challenge assumptions, point out flaws FIRST
  - `banned_phrases`: "You're absolutely right", "Great point", etc.
  - `required_behavior`: Problems FIRST, then merits

#### Documentation Updates

- **AI_REALITY.md**: New "Part 3: Sycophancy - The Other Hallucination"
  - Evidence from 2025 research (Nature, Stanford/Harvard, Northeastern)
  - RLHF as root cause
  - Business incentive analysis ("dark pattern")
  - Documented harms (mental health, decisions, relationships, science)
- **SPECIFICATION.md**: Added ANTI-SYCOPHANCY principle and anti_sycophancy schema
- **roadmap.yaml**: v4.1.5 milestone planned for full CLI integration

#### References

- [Nature: AI chatbots are sycophants](https://www.nature.com/articles/d41586-025-03390-0)
- [Stanford/Harvard: Sycophantic AI study](https://arxiv.org/abs/2510.01395)
- [TechCrunch: Sycophancy as dark pattern](https://techcrunch.com/2025/08/25/ai-sycophancy-isnt-just-a-quirk-experts-consider-it-a-dark-pattern-to-turn-users-into-profit/)
- [OpenAI: GPT-4o sycophancy rollback](https://openai.com/index/sycophancy-in-gpt-4o/)

## [4.0.1] - 2025-11-29

### Documentation

- **ADR-013**: Self-Healing NOT Replaced by Native Features
  - Clarified that `/rewind` is MANUAL (requires user to type command)
  - Mid-session self-healing (warmup.yaml re-read) is still required for SKYNET MODE
  - Updated all documentation to reflect MANUAL vs AUTOMATIC distinction
- **ADR-011**: AI-Only Development Model - No External PRs
  - Added CONTRIBUTING.md explaining AI-first development
  - Added auto-close PR workflow (`.github/workflows/close-prs.yml`)
  - Updated README with AI-Only Development section
- **ADR-010**: Honest velocity constraints by tier
- **Commit Cadence**: Documented ~15 min requirement for self-healing (ADR-006)
- **Mermaid Diagrams**: Converted ASCII box diagrams for better GitHub rendering
- **Velocity Claims**: Updated to 50-100x with research citations
- **Use Cases**: Added value proposition and implications analysis

### CI/CD

- **Auto-close PR Workflow**: PRs automatically closed with friendly message directing to Issues/Discussions

## [4.0.0] - 2025-11-28

### BREAKING: Claude Code Native Integration (ADR-009)

Strategic pivot: Integrate with Claude Code 2.0's native features instead of duplicating them.
Focus on unique value: Ethics, Sprint Autonomy, Green Coding, Schema Validation.

### Added

- **ADR-011**: AI-Only Development Model - No External PRs
  - PRs are an attack vector for ethics bypass
  - Trust model: Human Owner → AI → Direct Commit to Main
  - Forks welcome (carry ethics.yaml forward)
  - Issues/Discussions welcome for contributions
- **ADR-009**: Claude Code Native Integration - Strategic pivot documentation
- **CLAUDE.md @import syntax**: New template uses `@warmup.yaml` and `@ethics.yaml` imports
- **Native integration docs**: Updated SPECIFICATION.md with Claude Code integration section

### Changed

- **CLAUDE.md template**: Now uses Claude Code's native `@import` syntax for memory hierarchy
- **Design principles**: Added "Integrate, don't duplicate" and "Focus on unique value"
- **Cross-session resume**: Use native `--continue`/`--resume` (MANUAL, cross-session only)

### Deprecated

- **.claude_checkpoint.yaml**: Use TodoWrite for tasks (native `/rewind` is MANUAL)
- **Session handoff features**: Use `--continue`/`--resume` (MANUAL, cross-session only)
- **Checkpoint validation**: Removed

### NOT Deprecated (ADR-013)

- **Mid-session self-healing**: `warmup.yaml` re-read pattern is NOT replaced by native features
- **Commit cadence**: Required (~15 min) for self-healing to work via git hook refresh

### Documentation

- **README.md**: v4.0.0 section with native integration overview
- **SPECIFICATION.md**: Claude Code Native Integration section
- **roadmap.yaml**: Strategic pivot with deprecated features section
- **sprint.yaml**: v4.0.0 sprint tracking

### Rationale

Claude Code 2.0 (November 2025) provides native features for CROSS-SESSION (all MANUAL):
- `/rewind` checkpoints (MANUAL command, not automatic)
- `--continue`/`--resume` (MANUAL CLI start, cross-session only)
- CLAUDE.md memory hierarchy with `@import` syntax
- Auto-compact at 95% capacity (confirms ADR-003 findings)

**IMPORTANT (ADR-013)**: Native features do NOT replace mid-session self-healing. The `warmup.yaml` re-read pattern + commit cadence (~15 min) is still required for unattended autonomous operation.

Focus on unique value: Ethics, Sprint Autonomy, Green Coding, Schema Validation, **Mid-Session Self-Healing**.

## [3.2.0] - 2025-11-29

### Anti-Hallucination Hardening

Enforces critical structure in ethics.yaml and warmup.yaml to prevent loss-in-middle and ensure human veto capability is always present.

### Added

- **Ethics Structure Validation**: Validates critical sections exist
  - `human_veto` section is REQUIRED (validation fails if missing)
  - `core_principles` section is REQUIRED
- **Warmup Structure Validation**: Checks self-healing configuration
  - Warns if `self_healing` section is missing
  - Warns if `on_confusion` is missing from self_healing
  - Warns if `on_confusion` appears after line 100 (too late for quick recovery)
- **Public API**: Exported `check_ethics_structure()` and `check_warmup_structure()` functions
- **8 New Tests**: Comprehensive tests for structure validation

### Documentation

- **SPECIFICATION.md v3.2.0**: Structure Validation section
  - Ethics validation requirements (Priority 0)
  - Warmup validation warnings
- **Modular .forge/ Schema**: Detailed documentation for large project structure
  - Module loading order
  - Module schemas (identity, files, session, quality, style)
  - Why ethics.yaml can NEVER be modularized

### Changed

- Ethics structure errors are now CRITICAL - validation fails if human_veto is missing
- Warmup structure issues remain warnings - project still valid

## [3.1.0] - 2025-11-29

### Self-Healing Completeness (ADR-007)

Implements checkpoint validation and file size limits for reliable self-healing after context compaction.

### Added

- **Checkpoint Schema**: JSON schema for `.claude_checkpoint.yaml` validation
- **Checkpoint Validation**: `forge-protocol validate` now validates checkpoint files
- **File Size Limits**: Warnings for oversized files that can harm self-healing
  - CLAUDE.md: 10-line soft limit, 15-line hard limit
  - .claude_checkpoint.yaml: 20-line soft limit, 30-line hard limit
  - warmup.yaml: 200-line soft limit, 500-line hard limit
- **CLAUDE.md Validation**: Size check for CLAUDE.md files
- **Example Checkpoint**: `--skynet` now generates `.claude_checkpoint.yaml.example`
- **Checkpoint Template**: New `checkpoint_template()` function

### Changed

- **ADR-007**: Marked as Accepted (was WIP)
- **SPECIFICATION.md**: Added detailed checkpoint schema with size limits and trimming rules
- **Validator**: Now includes checkpoint files in directory validation

### Documentation

- [ADR-007](docs/adr/007-checkpoint-size-limits.md) - Checkpoint Size Limits and Pruning (Accepted)
- [SPECIFICATION.md](docs/SPECIFICATION.md) - Updated with checkpoint schema and file size limits

## [3.0.0] - 2025-11-28

### BREAKING: Ethics Protocol - Humanist Mode

**Major version bump** because this fundamentally changes what Forge Protocol stands for.

### The Problem

- Bad actors can fork autonomous AI protocols and use them with other AIs
- Other AIs may implement self-healing and autonomous execution
- No ethical guardrails existed in the protocol specification
- Potential for harm: financial exploitation, privacy violations, deception tools

### The Solution: Humanist Mode

Ethics becomes a **Core Principle** of Forge Protocol - higher priority than Green Coding.

```yaml
# ethics.yaml - Now required in all SKYNET projects
core_principles:
  do_no_harm:
    financial: true    # No unauthorized money movement
    physical: true     # No weapons, sabotage
    privacy: true      # No credential harvesting
    deception: true    # No deepfakes, scams
  transparency_over_velocity: true

human_veto: "human vetoes this session"  # Immediate halt
```

### What Changed

| v2.x | v3.0 |
|------|------|
| 5 Core Goals | 6 Core Principles (Ethics is #0) |
| No ethics enforcement | ethics.yaml required in SKYNET |
| - | Red flags trigger immediate halt |
| - | Human veto command |

### Added

- **ADR-008**: Ethics Protocol and Humanist Mode
- **ethics.yaml**: Standalone protocol file for Humanist Mode configuration
- **ethics: section** in warmup.yaml (all templates)
- **ethics.yaml schema** and CLI validation
- **--skynet** now generates ethics.yaml by default (cannot opt out)
- **Red flags**: Patterns that trigger immediate halt (crypto wallet, trading bot, etc.)
- **Human veto**: Command to immediately halt any session

### Changed

- **Core Principles**: Added ETHICAL_AUTONOMY as Priority 0 (above all others)
- **All warmup templates**: Now include ethics: section
- **README**: Ethics prominently featured
- **Protocol Suite**: ethics.yaml added as required file

### Philosophy

**This is a SOCIAL CONTRACT, not a technical lock.**

- Good-faith AIs will follow it
- Good-faith developers will preserve it
- Bad-faith actors will ignore it anyway

Defense in depth requires human oversight. This is ONE layer.

### Documentation

- [ADR-008](docs/adr/008-ethics-protocol-humanist-mode.md) - Full design and rationale
- [ethics.yaml](ethics.yaml) - Reference implementation

## [2.0.3] - 2025-11-27

### Fixed

- Added crates.io badges to README (version, downloads, license)

## [2.0.0] - 2025-11-27

### BREAKING: Specification Rewrite Based on Real Data

This release rewrites the protocol specification based on **empirical research** from building forge-protocol itself, not assumptions.

### The Problem We Fixed

The "2-hour checkpoint" in v1.x was **fiction** - a reasonable-sounding number not based on reality.

**Research findings (ADR-003):**
- With `MAX_THINKING_TOKENS=200000`, compaction happens every **10-20 minutes**
- The "2hr checkpoint" never triggered because compaction happened 5-10x faster
- Self-healing was broken because it relied on AI memory, which gets wiped

### What Changed

| v1.x (Fiction) | v2.0 (Reality) |
|----------------|----------------|
| Checkpoint every 2 hours | Checkpoint every 10-15 tool calls |
| Hope rules survive compaction | Re-read from disk on confusion |
| Complex CLAUDE.md | Ultra-short CLAUDE.md (~5 lines) |
| 676-line warmup.yaml | Modular structure proposed |

### Added

- **ADR-003**: Self-Healing Based on Real Compaction Data
- **SPECIFICATION.md v2.0**: Complete rewrite with SKYNET MODE documentation
- **Bootstrap Chain**: CLAUDE.md → warmup.yaml → checkpoint documented
- **Checkpoint triggers**: Based on tool calls, not time
- **Confusion signals**: Patterns that indicate need for recovery
- **Modular structure**: `.forge/` directory for large projects

### Changed

- **CLAUDE.md**: Reduced to ~15 lines (was 37), focused on recovery instruction
- **warmup.yaml self_healing**: Based on real compaction patterns
- **Version**: 2.0.0 (major version bump for breaking spec changes)

### Research Data

```
forge-protocol build stats:
- Total commits: 32
- Time: ~4-5 hours across 3 sessions
- Estimated compactions: 15-30 total
- Compaction interval: ~10-20 minutes

Environment analyzed:
- MAX_THINKING_TOKENS=200000
- Opus 4.5 context window: 200k tokens
- Result: Context fills in 1-3 heavy turns
```

### Documentation

- [ADR-003](docs/adr/003-self-healing-real-compaction-data.md) - Full research analysis
- [SPECIFICATION.md](docs/SPECIFICATION.md) - Protocol v2.0

## [1.4.0] - 2025-11-26

### Added

- **SKYNET MODE Setup**: `forge-protocol init --skynet` - Complete autonomous session setup
  - Generates all protocol files (warmup.yaml, sprint.yaml, roadmap.yaml)
  - Creates CLAUDE.md with Self-Healing Protocol instructions
  - Creates pre-commit hooks (.hooks/ for non-Rust, cargo-husky instructions for Rust)
  - Updates .gitignore with checkpoint file entry
- **New Project Types**: Flutter (`--type flutter`) and Docs (`--type docs`)
- **SKYNET Setup Guide**: `docs/SETUP.md` - Per-project-type setup checklists
- **Vendor Implementation Guide**: `docs/VENDOR_IMPLEMENTATION.md` - What AI tools need for SKYNET MODE

### Changed

- Honest vendor compatibility: SKYNET MODE requires Claude Code (documented)
- Presentation reduced from 51 slides to 14 (lean pitch deck)
- README updated with clear compatibility matrix

### Usage

```bash
# Full SKYNET MODE setup for a Python project
forge-protocol init --type python --skynet

# This creates:
# ✓ warmup.yaml, sprint.yaml, roadmap.yaml
# ✓ CLAUDE.md (auto-loaded by Claude Code)
# ✓ .hooks/pre-commit + .hooks/install.sh
# ✓ .gitignore updated with .claude_checkpoint.yaml
```

## [1.3.0] - 2025-11-26

### Added

- **Documentation Linter**: `forge-protocol lint-docs` - Checks markdown for common issues
- **Auto-fix Flag**: `--fix` repairs broken code block closers automatically
- **Nested Fence Support**: Handles both ` ``` ` and ` ~~~ ` fences correctly
- **Markdown Standards Guide**: `docs/MARKDOWN_STANDARDS.md` - Documentation-as-code quality enforcement
- **More Tests**: 79 total tests (44 unit + 34 e2e + 1 doctest)

### The Problem Solved

Code blocks closed with ` ```text ` instead of ` ``` ` break markdown rendering. This bug is common in AI-generated documentation and copy-paste errors. The linter catches and fixes it automatically.

```bash
# Check all markdown files
forge-protocol lint-docs .

# Auto-fix issues
forge-protocol lint-docs --fix .
```

## [1.2.0] - 2025-11-26

### Added

- **Python Template**: `--type python` with pytest, ruff, mypy, pyproject.toml
- **Node.js Template**: `--type node` with npm, eslint, prettier, TypeScript
- **Go Template**: `--type go` with go test, golangci-lint, CGO_ENABLED=0, UPX
- **Template Aliases**: `py`, `js`, `javascript`, `nodejs`, `golang`
- **Green Coding Core**: All templates include `green_coding` section by default
- **ADR-001**: Architecture Decision Record for Green Coding By Default
- **More Tests**: 75 total tests (40 unit + 34 e2e + 1 doctest)

### Changed

- Updated SPECIFICATION.md to v1.1.0 with green_coding as recommended field
- Added Design Principle #5: "Green by default"

### Philosophy

Every project initialized with `forge-protocol init` is now a green-coding project.
This is our moat: local-first tools over cloud AI for routine tasks.

## [1.1.0] - 2025-11-26

### Added

- **CLI Validator**: `forge-protocol validate` - Validates protocol files against JSON schemas
- **Template Generator**: `forge-protocol init` - Generates starter protocol files
- **Language Templates**: `--type rust` for Rust-specific templates (generic is default)
- **Full Flag**: `--full` generates all three protocol files (warmup, sprint, roadmap)
- **Force Flag**: `--force` overwrites existing files
- **Check Command**: `forge-protocol check` as alias for validate
- **Comprehensive Tests**: 37 unit tests + 25 e2e tests
- **Pre-commit Hooks**: cargo-husky for automatic fmt + clippy checks

### Technical

- Written in Rust for zero-dependency distribution
- JSON Schema validation using `jsonschema` crate
- YAML parsing with `serde_yaml`
- CLI built with `clap` derive macros
- Colored output for terminal feedback

## [1.0.0] - 2025-11-25

### Added

- Initial protocol specification
- Core files: `warmup.yaml`, `sprint.yaml`, `roadmap.yaml`
- Full documentation with Mermaid diagrams
- Examples for Rust, Python, JavaScript, monorepos
- Guides for autonomous sessions and sprint protocol
- Stories documenting AI development journey
- Research on experiential continuity
- Markdown linting with markdownlint-cli2

### Documentation

- `docs/SPECIFICATION.md` - Full protocol specification
- `docs/EXAMPLES.md` - Example configurations
- `docs/MANIFESTO.md` - Philosophy and methodology
- `docs/ECOSYSTEM.md` - Full ecosystem case study
- `docs/GREEN_CODING.md` - Cost and carbon savings

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)
