# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
