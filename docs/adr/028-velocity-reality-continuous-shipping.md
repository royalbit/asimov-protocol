# ADR-028: Velocity Reality - Continuous Shipping

## Status

**ACCEPTED** - 2025-12-01

## Context

The sprint protocol (sprint.yaml) was designed with these constraints:

| Constraint | Original Reasoning |
|------------|-------------------|
| 4h max | Prevent runaway sessions |
| 1 milestone per session | Focus prevents drift |
| 2hr checkpoint | Mid-session sanity check |

### The Reality

With Claude Opus 4.5 / Sonnet 4.5, observed session times:

| Milestone | Actual Time |
|-----------|-------------|
| ADR-027 (BMAD Incompatibility) | ~5 minutes |
| ADR-025/026 (Claude Attribution) | ~30 minutes |
| v7.0.0 (Crate Rename) | ~45 minutes |

**Milestones complete in minutes, not hours.**

### The Problem

The "1 milestone per session" rule creates artificial friction:

```
Claude ships milestone in 5 minutes
→ Protocol says "stop, wait for next session"
→ Human must restart session
→ Friction kills velocity
```

This contradicts ADR-027's core principle: **"BMAD asks. Asimov ships."**

If we stop after every milestone, **we're asking** (for permission to continue).

## Decision

### Remove the 1 Milestone Limit

**OLD:**
```yaml
rules:
  max_hours: 4
  max_milestones: 1  # REMOVED
```

**NEW:**
```yaml
rules:
  max_hours: 4
  max_milestones: unlimited  # Ship until ceiling or user stops
```

### New Sprint Model: Continuous Shipping

```
┌─────────────────────────────────────────────────────────────┐
│  SESSION START                                              │
│  - Read roadmap.yaml                                        │
│  - Validate with `asimov validate`                          │
│  - Present next milestone(s)                                │
│  - User says "go"                                           │
├─────────────────────────────────────────────────────────────┤
│  CONTINUOUS EXECUTION (up to 4h)                            │
│  - Ship milestone 1 → commit, push                          │
│  - Ship milestone 2 → commit, push                          │
│  - Ship milestone N → commit, push                          │
│  - Continue until: 4h ceiling OR roadmap empty OR blocked   │
├─────────────────────────────────────────────────────────────┤
│  SESSION END                                                │
│  - Update roadmap.yaml with completed work                  │
│  - Update CHANGELOG.md                                      │
│  - Final validation                                         │
└─────────────────────────────────────────────────────────────┘
```

### Updated Constraints

| Constraint | Old | New |
|------------|-----|-----|
| Max hours | 4 | 4 (unchanged - safety ceiling) |
| Max milestones | 1 | **Unlimited** |
| Checkpoint | Every 2hr | **Every milestone** (natural breakpoint) |
| Stop condition | After 1 milestone | 4h OR roadmap empty OR blocked |

### The Authority Principle (Updated)

```yaml
authority:
  principle: "Make decisions. Don't ask. Keep shipping."
  stop_when:
    - "4 hour ceiling reached"
    - "Roadmap exhausted"
    - "Blocked by external dependency"
    - "Human says stop"
  never_stop_for:
    - "Completed a milestone"  # NEW: Keep going!
    - "Arbitrary time checkpoints"
```

## Consequences

### Positive

1. **True hands-off** - User says "go", Claude ships until done
2. **Velocity unlocked** - No artificial pauses between milestones
3. **Natural flow** - Commits are checkpoints, not arbitrary time gates
4. **Roadmap velocity** - Multiple versions per session possible

### Negative

1. **Larger sessions** - More changes per session to review
2. **Rollback scope** - If something breaks, more commits to examine

### Mitigation

The negatives are mitigated by:

| Risk | Mitigation |
|------|------------|
| Large sessions | Each milestone is atomic commit |
| Rollback scope | Git history is granular per milestone |
| Quality drift | Tests must pass before each commit |

## Implementation

Update `.asimov/sprint.yaml`:

```yaml
rules:
  max_hours: 4
  max_milestones: unlimited
  must_ship: true
  mantra: "Keep shipping until done or stopped."

authority:
  principle: "Make decisions. Don't ask. Keep shipping."
  stop_when:
    - "4 hour ceiling reached"
    - "Roadmap exhausted"
    - "Blocked by external dependency"
    - "Human says stop"
```

## The New Mantra

> **"Keep shipping until done or stopped."**

Not "ship one thing." Ship everything.

## References

- [ADR-027: BMAD Incompatibility](027-bmad-incompatibility.md)
- [ADR-025: Claude Attribution Principle](025-claude-attribution-principle.md)
- [Sprint Autonomy Component](../components/2-SPRINT_AUTONOMY.md)
