# ADR-044: Dependency Setup for Coding Standards

**Status:** Accepted
**Date:** 2025-12-06
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

ADR-043 established that pre-commit hooks call quality tools directly (cargo fmt, ruff, eslint, etc.). However, these tools must be installed for the hooks to work.

### The Gap

`asimov init` creates hooks that reference tools, but doesn't help install them:

```bash
# Generated hook references ruff...
if command -v ruff &>/dev/null; then
  ruff format --check .
fi
# ...but ruff may not be installed!
```

### Tool Licenses (All Compatible)

| Tool | License | Safe for Any Project |
|------|---------|---------------------|
| cargo-husky | MIT | Yes |
| ruff | MIT | Yes |
| prettier | MIT | Yes |
| eslint | MIT | Yes |
| golangci-lint | GPL-3.0 | Yes (tool use only) |
| markdownlint-cli2 | MIT | Yes |

**Note:** GPL-3.0 for golangci-lint only applies if you embed/link it. Using it as a dev tool is fine.

## Decision

### `asimov init` Enhancement

When initializing a project, add development dependencies for coding standards tools:

| Project Type | Action |
|--------------|--------|
| **Rust** | Add `cargo-husky` to `Cargo.toml` [dev-dependencies] |
| **Python** | Add `[tool.ruff]` section to `pyproject.toml` |
| **Node** | Add `devDependencies` to `package.json` |
| **Go** | Print install command (can't auto-add due to GPL) |
| **Flutter** | Print install command (pub add) |
| **Docs** | Print install command (npm/brew) |

### `asimov doctor` Enhancement

Check if required tools are installed and report missing ones:

```
$ asimov doctor

CODING STANDARDS TOOLS
  ✓ cargo (rustc 1.75.0)
  ✓ cargo-fmt (installed)
  ✓ cargo-clippy (installed)
  ✗ cargo-husky (not in Cargo.toml dev-dependencies)

  Recommendation: Add to Cargo.toml [dev-dependencies]:
    cargo-husky = { version = "1", default-features = false, features = ["user-hooks"] }
```

### Implementation

#### Rust Projects

```toml
# Added to Cargo.toml [dev-dependencies]
cargo-husky = { version = "1", default-features = false, features = ["user-hooks"] }
```

#### Python Projects

```toml
# Added to pyproject.toml
[tool.ruff]
line-length = 100
target-version = "py311"

[tool.ruff.lint]
select = ["E", "F", "I", "W"]

[project.optional-dependencies]
dev = ["ruff", "pytest"]
```

#### Node Projects

```json
// Added to package.json devDependencies
{
  "devDependencies": {
    "prettier": "^3.0.0",
    "eslint": "^8.0.0"
  }
}
```

#### Go Projects

```
$ asimov init --type go

Installing golangci-lint...
  Note: golangci-lint is GPL-3.0 licensed (tool use is fine)

  Install with:
    go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest

  Or via brew:
    brew install golangci-lint
```

## Consequences

### Positive

1. **Complete setup** - Projects are ready to use after `asimov init`
2. **Discoverable** - `asimov doctor` shows what's missing
3. **License-safe** - Only auto-adds MIT/Apache licensed tools

### Negative

1. **Modifies existing files** - May conflict with existing configs
2. **Go/Flutter manual** - Can't auto-add GPL tool

### Neutral

1. **Optional** - Users can remove added dependencies
2. **Idempotent** - Running twice won't duplicate entries

## Related

- [ADR-043: Direct Coding Standards Enforcement](043-direct-coding-standards.md)
- [ADR-045: Dependency Health Protocol](045-dependency-health.md)

---

**Previous:** [ADR-043](043-direct-coding-standards.md)

---
