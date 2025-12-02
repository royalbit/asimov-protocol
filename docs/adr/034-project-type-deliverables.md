# ADR-034: Project-Type-Aware Deliverables

## Status

Proposed

## Context

Users repeatedly specify the same deliverables checklist for coding projects:
- Unit tests
- E2E tests
- Update README
- Update --help
- Commit, push, release

But not all project types need tests:

| Project Type | Tests Needed | Docs Needed |
|--------------|--------------|-------------|
| rust | Yes | Yes |
| python | Yes | Yes |
| node | Yes | Yes |
| go | Yes | Yes |
| flutter | Yes | Yes |
| docs | No | Yes |
| generic | No | Maybe |

## Decision

### 1. Add `deliverables_template` to Project Config

Store in `.asimov/project.yaml` (new file) or extend `roadmap.yaml`:

```yaml
# .asimov/project.yaml
project:
  name: "My Project"
  type: rust  # Determines template

deliverables_template:
  - "Unit tests pass"
  - "E2E tests pass"
  - "Update README if needed"
  - "Update --help if needed"
  - "Commit and push"
  - "GitHub release"
  - "Install to ~/bin"  # Optional, for CLI projects
```

### 2. Update Coding Templates

All coding templates (`rust`, `python`, `node`, `go`, `flutter`) include:

```yaml
deliverables_template:
  - "[ ] Unit tests pass"
  - "[ ] E2E tests pass"
  - "[ ] Update README if needed"
  - "[ ] Update --help if CLI"
  - "[ ] Commit, push, release"
```

### 3. Non-Coding Templates

`docs` and `generic` templates:

```yaml
deliverables_template:
  - "[ ] Content complete"
  - "[ ] Review/proofread"
  - "[ ] Commit and push"
```

### 4. Roadmap Milestone Inheritance

When creating milestones, inherit from template:

```yaml
# roadmap.yaml
current:
  version: "1.0.0"
  summary: "Feature X"
  deliverables:
    - "[ ] Implement feature"
    - "[ ] ~inherit~"  # Expands to project template
```

Or auto-append if `deliverables_template` exists.

## Consequences

### Positive

- No more repeating standard checklist
- Project-type-appropriate defaults
- Consistency across milestones
- New users get guidance automatically

### Negative

- New file/config to maintain
- Template updates need propagation
- Slight complexity increase

## Implementation

1. Create `.asimov/project.yaml` schema
2. Update `asimov init` templates for each project type
3. Update roadmap validation to support inheritance
4. Update `asimov warmup` to show deliverables from template

## References

- ADR-032: Project Context File
- Current templates in `cli/src/templates/`
