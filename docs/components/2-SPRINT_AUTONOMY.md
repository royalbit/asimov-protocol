# Component 2: Sprint Autonomy

> **Bounded sessions that ship - the "Off Switch" in ASIMOV MODE**

## Overview

Sprint Autonomy is the discipline of **bounded, shippable sessions**:

- **One milestone** per session
- **4-hour maximum** (hard stop)
- **Must end shippable** (tests pass, docs updated)

## The Core Rules

| Rule | Why |
|------|-----|
| ONE milestone per session | Focus prevents drift |
| 4-hour maximum | Forces shipping decisions |
| Must end shippable | No "work in progress" commits |
| No scope creep | "Note it for next session" |

## Anti-Patterns (Rejected)

| Anti-Pattern | Response |
|--------------|----------|
| "While I'm here..." | "Noted for next session. Shipping current work." |
| "Let me also..." | "Out of scope. Added to backlog." |
| "This would be better if..." | "Refactoring noted. Shipping as-is." |

## The 2-Hour Checkpoint

Every 2 hours, check:

- Still working on ONE milestone?
- Resisted scope creep?
- Work shippable now?
- Past 4 hours? STOP

## Relationship to Other Components

| Component | Connection |
|-----------|------------|
| Protocol Files | sprint.yaml defines boundaries |
| Quality Gates | Must pass before session ends |
| Self-Healing | Checkpoints align with 2hr rule |
| Release Discipline | Every session ends with release |

---

**Next:** [Component 3: Quality Gates](3-QUALITY_GATES.md)
