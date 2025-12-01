# ADR-025: Claude Attribution Principle

## Status

**ACCEPTED** - 2025-12-01

## Context

RoyalBit Asimov documentation previously claimed "50-100x velocity" as a benefit of the protocol. This is **factually incorrect**.

### The Research (December 2025)

Claude Opus 4.5 and Claude Sonnet 4.5 are purpose-built for autonomous development:

| Capability | Evidence | Source |
|------------|----------|--------|
| SWE-bench | 80.9% (state of the art) | [Anthropic](https://www.anthropic.com/news/claude-opus-4-5) |
| Autonomous sessions | 30 minutes without degradation | Anthropic benchmarks |
| Long-horizon tasks | 29% more than competitors | Vending-Bench |
| Token efficiency | 76% fewer tokens at same quality | Anthropic |
| Multi-agent coordination | Built-in capability | Claude 4 architecture |

**The velocity comes from Claude, not from Asimov.**

### The Misattribution

| Document | Incorrect Claim |
|----------|-----------------|
| VALUE_PROPOSITION.md | "Asimov = 50-150x velocity" |
| IMPLICATIONS.md | "Protocol accelerates what's already happening" |
| README.md | "50-100x velocity" as Asimov benefit |
| PRESS_KIT.md | Velocity attributed to protocol |

### The Truth

Every company using Claude Opus 4.5 or Sonnet 4.5 gets the velocity. **With or without Asimov.**

The question is not "how do we go faster?" but:

> **"How do we ensure autonomous AI development doesn't compromise ethics, quality, or sustainability?"**

## Decision

### Reframe Asimov's Role

| OLD Narrative | NEW Narrative |
|---------------|---------------|
| "Asimov enables 10x velocity" | "Claude enables velocity" |
| Asimov = **Enabler** | Asimov = **Guardian** |
| Velocity is the value | **Ethics + Sustainability** is the value |

### The Complete Stack

```
┌─────────────────────────────────────────────────────────────┐
│  VELOCITY SOURCE: Claude Opus 4.5 / Sonnet 4.5              │
│  - 50-100x native capability                                │
│  - SWE-bench 80.9%                                          │
│  - 30-min autonomous sessions                               │
│  - Multi-agent coordination                                 │
├─────────────────────────────────────────────────────────────┤
│  INTERFACE: Claude Code                                     │
│  - MAX_THINKING_TOKENS=200k                                 │
│  - Hooks for self-healing                                   │
│  - Terminal visibility                                      │
├─────────────────────────────────────────────────────────────┤
│  GUARDRAILS: Asimov Protocol                                │
│  - Ethics (Three Laws)                                      │
│  - Bounded autonomy (4hr max, quality gates)                │
│  - Self-healing (compensates for compaction)                │
│  - Anti-hallucination (file-based truth)                    │
│  - Green coding (local validation)                          │
└─────────────────────────────────────────────────────────────┘
```

### Core Principles Are Claude-Centric

| Principle | Relationship to Claude |
|-----------|------------------------|
| ETHICAL AUTONOMY | Guardrail ON Claude's power |
| ANTI-HALLUCINATION | Compensation FOR Claude's architecture |
| FRESHNESS | Compensation FOR Claude's training cutoff |
| SELF-HEALING | Compensation FOR Claude's context compaction |
| SESSION CONTINUITY | Claude Code native feature |
| AUTONOMOUS DEVELOPMENT | Guardrail ON Claude's autonomy |
| GREEN CODING | Reducing Claude API calls |

**Every principle is either guarding against or compensating for Claude.**

### New Value Proposition

**OLD Three Pillars:**
1. Velocity (50-150x)
2. Ethics (Built-In)
3. Green Coding

**NEW Three Pillars:**
1. **Ethics** (The Three Laws)
2. **Bounded Autonomy** (4hr max, quality gates)
3. **Sustainability** (Green coding, local-first)

## Consequences

### Positive

1. **Honest attribution** - Qowat Milat demands truth
2. **Defensible claims** - Can't be challenged on velocity source
3. **Clearer value prop** - Asimov solves the guardrails problem
4. **Strategic positioning** - "Claude + Asimov" not "Asimov alone"

### Negative

1. **Documentation rewrite** - All docs need updating
2. **Marketing shift** - Different pitch
3. **Perceived reduction** - May seem like "less value"

### The Reframe

The perceived reduction is actually an **increase** in defensibility:

| Claim | Defensibility |
|-------|---------------|
| "Asimov = 50x velocity" | Easily challenged |
| "Claude = velocity, Asimov = ethics" | Verified, defensible |

## Implementation

Update all documentation to reflect:

1. **Claude Attribution**: Velocity comes from Claude
2. **Asimov Role**: Guardian, not enabler
3. **Stack Clarity**: Claude → Claude Code → Asimov
4. **Pillar Reorder**: Ethics > Bounded Autonomy > Sustainability

## References

- [Claude Opus 4.5 Announcement](https://www.anthropic.com/news/claude-opus-4-5)
- [Claude Sonnet 4.5 Announcement](https://www.anthropic.com/news/claude-sonnet-4-5)
- [ADR-020: Asimov Open Foundation](020-asimov-mode-open-foundation.md)
- [ADR-023: The Inaction Principle](023-inaction-principle.md)
