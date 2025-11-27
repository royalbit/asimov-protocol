# Markdown Standards

> **Documentation as Code. Diagrams as Code. Quality Enforced.**

## The Problem

Markdown rendering breaks silently. Common issues:

| Issue | Example | Result |
|-------|---------|--------|
| Code block closer bug | ` ```text ` as closer | Renders as text, not code |
| Unclosed blocks | Missing ` ``` ` | Rest of doc is code |
| Invalid mermaid | Syntax errors | Blank diagram |

## The Solution: forge-protocol lint-docs

The `forge-protocol` CLI includes a documentation linter that catches common issues.

```bash
# Install
cargo install forge-protocol

# Check documentation
forge-protocol lint-docs                 # Check current directory
forge-protocol lint-docs docs/           # Check specific directory
forge-protocol lint-docs README.md       # Check specific file

# Auto-fix issues
forge-protocol lint-docs --fix           # Fix all files
forge-protocol lint-docs docs/ --fix     # Fix specific directory
```

## What It Checks

### Code Block Closer Bug

The linter catches the most common markdown bug: code blocks closed with ` ```lang ` instead of just ` ``` `.

**Wrong:**
~~~
```text
some content here
```text     ← BUG: has language identifier
~~~

**Correct:**
~~~
```text
some content here
```         ← Just backticks
~~~

### Why This Bug Happens

AI assistants sometimes mirror the opening fence when closing. Humans copy-paste incorrectly. The result: broken rendering that's hard to spot.

### How It Works

The linter tracks open/close state for both backtick (` ``` `) and tilde (` ~~~ `) fences:

```rust
if !in_block {
    // Opening a block
    in_block = true;
    block_fence = Some("```");
} else {
    // Closing a block - should be just ```
    if stripped != "```" {
        // ERROR: Closer has language identifier
    }
    in_block = false;
}
```

Nested fences are handled correctly - ` ``` ` inside ` ~~~ ` blocks is treated as content.

## Integration

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash

# Lint documentation
if command -v forge-protocol &> /dev/null; then
    forge-protocol lint-docs . || exit 1
fi
```

### GitHub Actions

```yaml
# .github/workflows/docs.yml
name: Documentation

on: [push, pull_request]

jobs:
  lint-docs:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install forge-protocol
        run: cargo install forge-protocol

      - name: Lint documentation
        run: forge-protocol lint-docs .
```

### Makefile

```makefile
lint-docs:
	forge-protocol lint-docs .

fix-docs:
	forge-protocol lint-docs --fix .
```

## Combined Workflow

For comprehensive documentation quality:

```bash
# 1. Check code blocks (forge-protocol)
forge-protocol lint-docs .

# 2. Check standard markdown rules (markdownlint)
markdownlint-cli2 "**/*.md"

# 3. Validate protocol files
forge-protocol validate
```

## Related Tools

| Tool | Purpose | Install |
|------|---------|---------|
| `forge-protocol lint-docs` | Code block closers | `cargo install forge-protocol` |
| `markdownlint-cli2` | Standard markdown rules | `npm i -g markdownlint-cli2` |
| `mermaid-cli` | Mermaid diagram validation | `npm i -g @mermaid-js/mermaid-cli` |

## Configuration

### .markdownlint.yaml

For markdownlint, use this sensible configuration:

```yaml
# Sensible defaults for documentation
default: true

# Disabled rules
MD013: false  # Line length (prose needs flexibility)
MD033: false  # Inline HTML (badges, etc.)
MD041: false  # First line heading

# Customized rules
MD024:
  siblings_only: true  # Allow duplicate headings in sections
```

## Summary

```bash
# One command to enforce documentation standards
forge-protocol lint-docs --fix .
```

**Result:** Documentation that renders correctly everywhere.
