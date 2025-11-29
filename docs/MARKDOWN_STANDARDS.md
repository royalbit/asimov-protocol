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

## Mermaid Diagrams

**Use Mermaid, not ASCII art.** GitHub renders Mermaid natively.

### Why Mermaid Over ASCII

| ASCII Art | Mermaid |
|-----------|---------|
| Breaks on different fonts | Renders consistently |
| Doesn't scale or reflow | Responsive |
| No light/dark theme support | Auto-themes |
| Hard to maintain | Easy to edit |

### Critical Rule: Use Vanilla Mermaid

**DO NOT customize themes or colors** - it breaks GitHub's auto-theming.

```yaml
# WRONG - breaks on light/dark themes
%%{init: {'theme': 'dark'}}%%

# WRONG - custom colors
classDef myStyle fill:#90EE90

# CORRECT - vanilla Mermaid
graph LR
  A[Box A] --> B[Box B]
```

GitHub auto-detects user's theme preference. Any customization overrides this.

### Diagram Types

| Type | Use For | Example |
|------|---------|---------|
| `graph LR/TB` | Architecture, data flow | System diagrams |
| `sequenceDiagram` | Workflows, interactions | API flows |
| `classDiagram` | Data models, types | Type hierarchies |
| `erDiagram` | Data relationships | Database schemas |

### Best Practices

- Max 15-20 elements per diagram
- Clear, descriptive labels
- Use subgraphs for grouping
- Comments with `%%`

### Resources

- [Mermaid Docs](https://mermaid.js.org/)
- [Live Editor](https://mermaid.live/)
- [GitHub Blog](https://github.blog/2022-02-14-include-diagrams-markdown-files-mermaid/)

## Summary

```bash
# One command to enforce documentation standards
forge-protocol lint-docs --fix .
```

**Result:** Documentation that renders correctly everywhere.
