# Component 2: Sprint Autonomy

> **Continuous shipping until done or stopped - the "Off Switch" in RoyalBit Asimov**

## Overview

Sprint Autonomy is the discipline of **continuous, hands-off shipping**:

- **Unlimited milestones** per session (ADR-028)
- **Run until done** (no time limit)
- **Keep shipping** until roadmap empty or blocked

## The Core Rules

| Rule | Why |
|------|-----|
| Keep shipping | Milestones complete in minutes, not hours |
| Run until done | Sessions continue until complete or blocked |
| Must end shippable | Every commit is atomic and complete |
| No scope creep | Stay on roadmap, no tangents |

## ADR-028: Velocity Reality

The original "1 milestone per session" rule was artificial friction:

```
OLD: Ship 1 → Stop → Wait for next session → Friction
NEW: Ship 1 → Ship 2 → Ship N → Until done
```

**"Keep shipping until done or stopped."**

## Stop Conditions

| Condition | Action |
|-----------|--------|
| Roadmap exhausted | STOP |
| Blocked by dependency | STOP |
| Human says stop | STOP |
| Completed a milestone | **KEEP GOING** |

## Anti-Patterns (Rejected)

| Anti-Pattern | Response |
|--------------|----------|
| "While I'm here..." | "Is it on the roadmap? No? Skip it." |
| "Let me also..." | "Roadmap or backlog. Pick one." |
| "This would be better if..." | "Ship as-is. Improve in next milestone." |

## Relationship to Other Components

| Component | Connection |
|-----------|------------|
| Protocol Files | sprint.json defines boundaries |
| Quality Gates | Must pass before each commit |
| Self-Healing | Commits are natural checkpoints |
| Release Discipline | Every milestone ends with release |

---

**Next:** [Component 3: Quality Gates](3-QUALITY_GATES.md)

---
