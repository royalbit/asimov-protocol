# Setup Guide

One command to full autonomous session capability.

## Quick Start

```bash
# Install asimov (from GitHub Releases)
curl -L https://github.com/royalbit/asimov/releases/latest/download/asimov-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv asimov /usr/local/bin/

# Full ROYALBIT ASIMOV setup
asimov init --type rust

# Launch session (auto-warmup)
asimov
```

## What Gets Created

```
project/
├── .asimov/                    # Protocol directory (v9.0.0+)
│   ├── warmup.json             # Session bootstrap - loads all protocols
│   ├── sprint.json             # Autonomous execution rules
│   ├── asimov.json             # The Three Laws (ethics)
│   ├── freshness.json          # Date-aware WebSearch
│   ├── sycophancy.json         # Truth over comfort
│   ├── green.json              # Efficiency standards
│   ├── exhaustive.json         # Task completion rules
│   ├── coding-standards.json   # Quality gates
│   ├── migrations.json         # Functional equivalence
│   ├── roadmap.yaml            # Milestones (YAML)
│   └── project.yaml            # Project metadata (YAML)
├── .claude/                    # Claude Code integration
│   └── hooks/                  # Git hooks
│       ├── pre-compact.sh
│       └── session-start.sh
└── .gitignore
```

## Setup by Project Type

### Rust

```bash
asimov init --type rust
```

**Hooks:** Uses cargo-husky (add to Cargo.toml dev-dependencies)

**Quality gates:**
- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`

### Python

```bash
asimov init --type python
```

**Hooks:** `.claude/hooks/` (installed automatically)

**Quality gates:**
- `pytest`
- `ruff check .`
- `ruff format --check .`
- `mypy .`

### Node.js

```bash
asimov init --type node
```

**Hooks:** `.claude/hooks/` (installed automatically)

**Quality gates:**
- `npm test`
- `npm run lint`
- `npm run format:check`

### Go

```bash
asimov init --type go
```

**Hooks:** `.claude/hooks/` (installed automatically)

**Quality gates:**
- `go test ./...`
- `golangci-lint run`
- `gofmt -l .`

### Flutter

```bash
asimov init --type flutter
```

**Hooks:** `.claude/hooks/` (installed automatically)

**Quality gates:**
- `flutter test`
- `dart analyze lib/`
- `dart format --set-exit-if-changed lib/ test/`

### Documentation

```bash
asimov init --type docs
```

**Hooks:** `.claude/hooks/` (installed automatically)

**Quality gates:**
- `asimov lint-docs .`
- `markdownlint '**/*.md'`

## Post-Setup Steps

### 1. Install Asimov CLI

Download from [GitHub Releases](https://github.com/royalbit/asimov/releases/latest).

Verify: `asimov --version`

### 2. Install Hooks

Hooks are installed automatically in `.claude/hooks/` during `asimov init`.

**Rust projects** can optionally use cargo-husky:
```bash
# Add to Cargo.toml [dev-dependencies]
cargo-husky = { version = "1", features = ["precommit-hook", "run-cargo-clippy", "run-cargo-fmt"] }
```

### 3. Edit Protocol Files

```bash
# Edit YAML files
$EDITOR .asimov/project.yaml
$EDITOR .asimov/roadmap.yaml
```

### 4. Validate and Launch

```bash
asimov validate
asimov  # Launcher mode (auto-warmup)
```

## Verification Checklist

```bash
# Check files exist
ls -la .asimov/

# Validate protocol
asimov validate

# Check hooks installed
ls -la .claude/hooks/

# Test session
asimov
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Protocol files not found" | `asimov init --type <your-type>` |
| "Hooks not running" | Re-run `asimov init` and check `.claude/hooks/` |
| "AI forgets rules" | Run `asimov warmup` or use launcher mode |
| "Validation errors" | Run `asimov doctor` and `asimov validate` |

## Requirements Summary

| Component | Required |
|-----------|----------|
| Claude Code | Yes |
| Asimov CLI (v9.12.0+) | Yes |
| .asimov/ directory | Yes (JSON protocols + YAML metadata) |
| .claude/hooks/ | Recommended |
| `--dangerously-skip-permissions` | Recommended (for full autonomy) |

## Protocol Files Reference

**JSON Protocols** (in `.asimov/`):
- `warmup.json` - Session bootstrap and protocol loading
- `sprint.json` - Autonomous execution rules
- `asimov.json` - The Three Laws (harm prevention)
- `freshness.json` - Date-aware WebSearch requirements
- `sycophancy.json` - Truth over comfort principles
- `green.json` - Efficiency and sustainability standards
- `exhaustive.json` - Complete task execution rules
- `coding-standards.json` - Language-specific quality gates
- `migrations.json` - Functional equivalence guarantees

**YAML Files** (in `.asimov/`):
- `project.yaml` - Project metadata and configuration
- `roadmap.yaml` - Milestones and progress tracking

## Related Documentation

- [README](../README.md) - Project overview
- [Origin Story](ORIGIN_STORY.md) - How Asimov was built
- [Value Proposition](VALUE_PROPOSITION.md) - Why use Asimov

---
