# Component 3: Quality Gates

> **Tests pass + zero warnings = permission to commit**

## Overview

Quality Gates are the automated checks that must pass before any commit or release:

- **Tests must pass** (100%, no skips)
- **Zero warnings** (linter, compiler)
- **Format check** (consistent style)

No exceptions. No "I'll fix it later."

## The Gates

| Gate | Command (Rust) | Must Be |
|------|----------------|---------|
| Tests | `cargo test` | All pass |
| Lint | `cargo clippy -- -D warnings` | Zero warnings |
| Format | `cargo fmt --check` | No changes needed |
| Docs | `asimov-mode lint-docs` | No errors |
| Protocol | `asimov-mode validate` | Valid YAML |

## Why Zero Warnings?

Warnings are future bugs. Every warning is:
- Technical debt accumulating
- A signal you're ignoring
- A broken window inviting more

**Zero warnings policy:** Fix it now or don't commit.

## Per-Language Gates

### Rust
```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

### Python
```bash
pytest
ruff check .
ruff format --check .
mypy .
```

### Node.js
```bash
npm test
npm run lint
npm run format:check
```

### Go
```bash
go test ./...
golangci-lint run
gofmt -l .
```

### Flutter
```bash
flutter test
dart analyze lib/
dart format --set-exit-if-changed lib/ test/
```

## Enforcement

Quality gates are enforced via:

1. **Pre-commit hooks** - Block bad commits locally
2. **AI discipline** - AI runs gates before committing
3. **CI/CD** - Final verification on push

## Pre-commit Hook Example

```bash
#!/bin/bash
set -e

echo "Running tests..."
cargo test

echo "Running clippy..."
cargo clippy --all-targets -- -D warnings

echo "Checking format..."
cargo fmt --check

echo "Validating protocol..."
asimov-mode validate . || true
asimov-mode lint-docs . || exit 1

echo "All gates passed!"
```

## Relationship to Other Components

| Component | Connection |
|-----------|------------|
| Protocol Files | warmup.yaml defines the gates |
| Sprint Autonomy | Gates must pass before session ends |
| Self-Healing | Gates re-read from warmup.yaml |
| Release Discipline | Gates must pass before release |

---

**Next:** [Component 4: Self-Healing](4-SELF_HEALING.md)
