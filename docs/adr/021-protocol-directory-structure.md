# ADR-021: Protocol Directory Structure (.asimov/)

## Status

Accepted

## Date

2025-11-30

## Context

### The Problem

The Asimov Protocol currently stores 7 YAML files in the repository root:

```
/
├── asimov.yaml
├── ethics.yaml
├── green.yaml
├── roadmap.yaml
├── sprint.yaml
├── sycophancy.yaml
├── warmup.yaml
├── README.md
├── CHANGELOG.md
├── LICENSE
└── ...
```

This creates several issues:

1. **Root clutter**: 7 protocol files mixed with README, LICENSE, etc.
2. **Poor discoverability**: Users don't know which files are "the protocol"
3. **No namespace**: Protocol files have no clear grouping
4. **Inconsistent with conventions**: `.github/`, `.claude/` use directory namespacing

### Comparison with Other Tools

| Tool | Config Location | Pattern |
|------|-----------------|---------|
| GitHub Actions | `.github/workflows/` | Hidden directory |
| Claude Code | `.claude/` | Hidden directory |
| VS Code | `.vscode/` | Hidden directory |
| Git | `.git/` | Hidden directory |
| Asimov Protocol | `/` (root) | No namespace |

We're the outlier.

## Decision

**Move all protocol files to `.asimov/` directory.**

### New Structure

```
/
├── .asimov/
│   ├── asimov.yaml      # The Three Laws
│   ├── ethics.yaml      # Ethics configuration
│   ├── green.yaml       # Green coding principles
│   ├── roadmap.yaml     # Milestones
│   ├── sprint.yaml      # Current sprint
│   ├── sycophancy.yaml  # Anti-sycophancy rules
│   └── warmup.yaml      # Session initialization
├── README.md
├── CHANGELOG.md
├── LICENSE
└── ...
```

### CLI Changes

The `asimov-mode` CLI will:

1. **Look in `.asimov/` first** (new default)
2. **Fall back to root** (backwards compatibility during transition)
3. **`asimov-mode init`** creates `.asimov/` directory
4. **`asimov-mode migrate`** moves files from root to `.asimov/`
5. **Validation** checks `.asimov/` directory
6. **Regeneration** creates files in `.asimov/`

### CLAUDE.md Changes

```markdown
# Before
@warmup.yaml
@asimov.yaml
@green.yaml

# After
@.asimov/warmup.yaml
@.asimov/asimov.yaml
@.asimov/green.yaml
```

### Why `.asimov/` (Hidden)?

1. **Convention**: Matches `.github/`, `.claude/`, `.vscode/`
2. **Clean root**: Only user-facing files visible by default
3. **Namespace**: Clear "this is Asimov Protocol" grouping
4. **Discoverability**: Developers know to look for dotfiles

### Why Not `protocol/` (Visible)?

1. **Breaks convention**: No other tool uses visible config directories
2. **Still clutters**: Adds another visible directory to root
3. **Inconsistent**: We already use `.claude/` for hooks

## Consequences

### Positive

1. **Clean root**: Only README, CHANGELOG, LICENSE visible
2. **Clear namespace**: All protocol files in one place
3. **Conventional**: Matches industry patterns
4. **Discoverable**: `.asimov/` is self-documenting name

### Negative

1. **Breaking change**: Existing installations need migration
2. **Path updates**: All documentation references change
3. **CLAUDE.md updates**: Import paths change

### Migration Path

```bash
# Automatic migration
asimov-mode migrate

# Or manual
mkdir -p .asimov
mv *.yaml .asimov/
# Update CLAUDE.md imports
```

## Version

This is a **major breaking change**: v6.0.0

- Protocol file locations change
- CLAUDE.md import syntax changes
- CLI default paths change

## Implementation Checklist

- [ ] Create `.asimov/` directory structure
- [ ] Update CLI to look in `.asimov/` first
- [ ] Add `asimov-mode migrate` command
- [ ] Update all documentation
- [ ] Update CLAUDE.md template
- [ ] Update CI workflow paths
- [ ] Update pre-commit hook paths
- [ ] Bump version to 6.0.0
- [ ] Update CHANGELOG

## References

- [ADR-009: Claude Code Native Integration](009-claude-code-native-integration.md)
- [ADR-014: Ethics File Separation](014-ethics-file-separation.md)
