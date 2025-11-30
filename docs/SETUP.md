# ASIMOV MODE Setup Guide

> **One command to full autonomous session capability**

## Quick Start

```bash
# Install asimov-mode
cargo install asimov-mode

# Full ASIMOV MODE setup
asimov-mode init --type rust --asimov

# Launch and go
claude --dangerously-skip-permissions
> run warmup
> punch it
```

## What Gets Created

```
project/
├── .asimov/              # Protocol directory (v6.0.0+)
│   ├── warmup.yaml       # Protocol rules (HOW)
│   ├── sprint.yaml       # Session boundaries (WHEN)
│   ├── roadmap.yaml      # Milestones (WHAT)
│   ├── asimov.yaml       # The Three Laws
│   ├── ethics.yaml       # Ethics configuration
│   ├── green.yaml        # Green coding principles
│   └── sycophancy.yaml   # Anti-sycophancy rules
├── CLAUDE.md             # Self-healing trigger (@.asimov/warmup.yaml)
├── .gitignore            # + .claude_checkpoint.yaml
└── .hooks/               # Pre-commit hooks
    ├── pre-commit
    └── install.sh
```

## Setup by Project Type

### Rust

```bash
asimov-mode init --type rust --asimov
```

**Hooks:** Uses cargo-husky (add to Cargo.toml dev-dependencies)

**Quality gates:**
- `cargo test`
- `cargo clippy -- -D warnings`
- `cargo fmt --check`

### Python

```bash
asimov-mode init --type python --asimov
```

**Hooks:** `.hooks/pre-commit` + `.hooks/install.sh`

**Quality gates:**
- `pytest`
- `ruff check .`
- `ruff format --check .`
- `mypy .`

### Node.js

```bash
asimov-mode init --type node --asimov
```

**Hooks:** `.hooks/pre-commit` (or use husky)

**Quality gates:**
- `npm test`
- `npm run lint`
- `npm run format:check`

### Go

```bash
asimov-mode init --type go --asimov
```

**Hooks:** `.hooks/pre-commit`

**Quality gates:**
- `go test ./...`
- `golangci-lint run`
- `gofmt -l .`

### Flutter

```bash
asimov-mode init --type flutter --asimov
```

**Hooks:** `.hooks/pre-commit`

**Quality gates:**
- `flutter test`
- `dart analyze lib/`
- `dart format --set-exit-if-changed lib/ test/`

### Documentation

```bash
asimov-mode init --type docs --asimov
```

**Hooks:** `.hooks/pre-commit`

**Quality gates:**
- `asimov-mode lint-docs .`
- `markdownlint '**/*.md'`

## Post-Setup Steps

### 1. Install Hooks

**Rust (cargo-husky):**
```bash
# Add to Cargo.toml [dev-dependencies]
cargo-husky = { version = "1", features = ["precommit-hook", "run-cargo-clippy", "run-cargo-fmt"] }

# Install
cargo test
```

**Other languages:**
```bash
./.hooks/install.sh
```

### 2. Edit Protocol Files

```bash
# Edit with your project details
$EDITOR .asimov/warmup.yaml
$EDITOR .asimov/roadmap.yaml
```

### 3. Validate

```bash
asimov-mode validate
```

### 4. Launch ASIMOV MODE

```bash
# Terminal 1: Launch Claude Code
claude --dangerously-skip-permissions

# In Claude Code
> run warmup
> punch it
```

## Verification Checklist

```bash
# Check files exist
ls -la .asimov/ CLAUDE.md

# Validate protocol
asimov-mode validate

# Check hooks installed
ls -la .git/hooks/pre-commit

# Test pre-commit
git commit --allow-empty -m "test" --dry-run
```

## Troubleshooting

### "warmup.yaml not found"
```bash
asimov-mode init --type <your-type> --asimov
```

### "Hooks not running"
```bash
./.hooks/install.sh
# or for Rust
cargo test
```

### "Self-healing not working"
1. Check CLAUDE.md exists
2. Check CLAUDE.md has "@.asimov/warmup.yaml" import
3. Check .asimov/warmup.yaml has self_healing section

### "AI forgets rules"
Say: "Re-read .asimov/warmup.yaml"

## Requirements Summary

| Component | Required |
|-----------|----------|
| Claude Code | Yes |
| `--dangerously-skip-permissions` | Yes (for autonomy) |
| .asimov/ directory | Yes (v6.0.0+) |
| CLAUDE.md | Yes (for self-healing) |
| Pre-commit hooks | Recommended |

## Related Documentation

- [ASIMOV MODE Overview](ASIMOV_MODE.md)
- [Component 1: Protocol Files](components/1-PROTOCOL_FILES.md)
- [Component 4: Self-Healing](components/4-SELF_HEALING.md)
- [Vendor Implementation](VENDOR_IMPLEMENTATION.md)
