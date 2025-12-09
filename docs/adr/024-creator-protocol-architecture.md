# ADR-024: Creator Protocol Architecture

## Status

**Accepted** - November 2025

## Context

RoyalBit Asimov was initially described as "a Self-Evolving Autonomous AI protocol" or "Self-Evolving Autonomous AI with ethics built in." This framing, while accurate, misses the bigger architectural insight:

**RoyalBit Asimov is not just a Self-Evolving Autonomous AI - it CREATES Self-Evolving Autonomous AI projects.**

When you run `asimov init --asimov`, the new project becomes an **independent** Self-Evolving Autonomous AI with:
- The Three Laws (asimov.yaml)
- Ethics built in (ethics.yaml)
- Green coding (green.yaml)
- Anti-sycophancy (sycophancy.yaml)
- Sprint autonomy (sprint.yaml)
- Self-healing capabilities

Each child project operates independently of Asimov after initialization.

## Decision

Update all documentation, code, and marketing materials to reflect the **creator** narrative:

### Primary Tagline

> **RoyalBit Asimov creates Self-Evolving Autonomous AI projects with ethics built in.**

### Supporting Narrative

1. **Creator, not just Protocol**: Asimov is a factory for Self-Evolving Autonomous AI projects
2. **Independence After Creation**: Each initialized project becomes autonomous
3. **Propagation**: The Three Laws and ethics propagate through the ecosystem
4. **Bootstrapping Proof**: Asimov was built using Asimov (circular proof)

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                     RoyalBit Asimov                              │
│                                                                  │
│  "Creates Self-Evolving Autonomous AI projects with ethics"     │
│                                                                  │
│                    asimov init --asimov                          │
│                           │                                      │
│           ┌───────────────┼───────────────┐                      │
│           ▼               ▼               ▼                      │
│    ┌──────────┐    ┌──────────┐    ┌──────────┐                 │
│    │  FORGE   │    │  COBOL   │    │   ANY    │                 │
│    │          │    │ BANKING  │    │ PROJECT  │                 │
│    │ Self-    │    │          │    │          │                 │
│    │ Evolving │    │ Self-    │    │ Self-    │                 │
│    │ Auto-    │    │ Evolving │    │ Evolving │                 │
│    │ nomous   │    │ Auto-    │    │ Auto-    │                 │
│    │ AI       │    │ nomous   │    │ nomous   │                 │
│    │          │    │ AI       │    │ AI       │                 │
│    │ ✓ Ethics │    │          │    │          │                 │
│    │ ✓ Green  │    │ ✓ Ethics │    │ ✓ Ethics │                 │
│    │ ✓ Sprint │    │ ✓ Green  │    │ ✓ Green  │                 │
│    └──────────┘    │ ✓ Sprint │    │ ✓ Sprint │                 │
│         │          └──────────┘    └──────────┘                 │
│         │               │               │                        │
│         └───────────────┴───────────────┘                        │
│                         │                                        │
│              INDEPENDENT AFTER CREATION                          │
└─────────────────────────────────────────────────────────────────┘
```

## Consequences

### Positive

1. **Clearer Value Proposition**: "Creates" is more powerful than "is"
2. **Ecosystem Thinking**: Emphasizes the propagation of ethics
3. **Scalability Story**: One protocol creates infinite ethical projects
4. **Differentiation**: No competitor "creates" Self-Evolving AI

### Negative

1. **Terminology Update**: Requires updating all documentation
2. **Consistency Effort**: All child projects need aligned messaging

## Files to Update

### Core

- `cli/Cargo.toml` - Crate description
- `cli/src/main.rs` - CLI help text
- `cli/src/lib.rs` - Crate doc header
- `cli/src/templates.rs` - Generated file headers

### Documentation

- `README.md` - Main tagline
- `docs/SPECIFICATION.md` - Overview
- `docs/ROYALBIT_ASIMOV.md` - Full description
- `docs/VALUE_PROPOSITION.md` - Value prop
- `docs/USE_CASES.md` - Use cases
- `docs/EXECUTIVE_DECK.md` - Presentation
- `docs/TECHNICAL_DECK.md` - Technical presentation

### Child Projects

Each project created with `asimov init --asimov` inherits the terminology through:
- `.asimov/asimov.yaml` header
- `.asimov/ethics.yaml` header
- Generated CLAUDE.md

## Implementation

See commit implementing this ADR for the complete list of changes.

## References

- [ADR-020: Asimov Open Foundation](020-asimov-mode-open-foundation.md)
- [ORIGIN_STORY.md](../ORIGIN_STORY.md) - The bootstrapping narrative
- [arXiv: Survey of Self-Evolving Agents](https://arxiv.org/abs/2507.21046)

---
