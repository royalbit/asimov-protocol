# ADR-014: Ethics File Separation and Reference Architecture

**Status:** Accepted
**Date:** 2025-11-29
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI

## Context

When implementing ethics across multiple forge-protocol enabled projects, we faced an architectural decision:

**Option A:** Separate files with references
```
CLAUDE.md:
  @warmup.yaml
  @ethics.yaml
```

**Option B:** Full ethics inlined in warmup.yaml
```
CLAUDE.md:
  @warmup.yaml  (contains full ethics)
```

**Option C:** Compact ethics only in warmup.yaml
```
CLAUDE.md:
  @warmup.yaml  (contains compact ethics summary only)
```

### Token Efficiency Analysis

| Approach | Initial Load | Post-Compaction | DRY Compliance |
|----------|--------------|-----------------|----------------|
| A) Separate files | ~30KB | ~500B (compact survives) | Yes |
| B) Full inline | ~30KB | ~500B (compact survives) | No (N copies) |
| C) Compact only | ~22KB | ~500B | Yes, but loses detail |

Key insight: Options A and B have **identical token costs** at session start. The difference is maintenance burden.

### The DRY Problem

With 11+ forge-protocol enabled projects:
- **Option B** = 11 copies of full ethics to maintain
- Protocol update requires editing 11 warmup.yaml files
- Risk of drift between copies
- Violates DRY (Don't Repeat Yourself)

### Separation of Concerns

Ethics and warmup serve different purposes:

| File | Purpose | Change Frequency |
|------|---------|------------------|
| `ethics.yaml` | Ethical guardrails, red flags, tool categories | Rare (protocol-level) |
| `warmup.yaml` | Project config, milestones, quality gates | Frequent (project-level) |

Mixing them violates single responsibility principle.

## Decision

**Use Option A: Separate `ethics.yaml` with `@reference` in CLAUDE.md**

Architecture:
```
project/
├── CLAUDE.md           # @warmup.yaml + @ethics.yaml
├── warmup.yaml         # Project config + COMPACT ethics (survives compaction)
└── ethics.yaml         # FULL ethics protocol (loaded at session start)
```

### Implementation

**CLAUDE.md** (session entry point):
```markdown
# Project Name

@warmup.yaml
@ethics.yaml

ON CONFUSION → re-read warmup.yaml + ethics.yaml
```

**warmup.yaml** (compact ethics for compaction survival):
```yaml
ethics:
  status: "REQUIRED"
  philosophy: "Power creates responsibility. Autonomy requires ethics."
  principles:
    do_no_harm:
      financial: true
      physical: true
      privacy: true
      deception: true
    transparency_over_velocity: true
  human_veto: "human vetoes this session"
  motto: "Build tools that help. Never tools that harm."
```

**ethics.yaml** (full protocol):
- Modification rules (2 human co-signers)
- Complete red flags list (20+ patterns)
- Tool categories (always_safe, require_human_review, forbidden_always)
- Session limits
- Fork requirements

### Hybrid Approach Benefits

1. **Full context at session start** - ethics.yaml loaded via @reference
2. **Compact survives compaction** - ethics section in warmup.yaml persists
3. **Recovery path** - Can re-read full ethics.yaml mid-session if needed
4. **DRY** - One ethics.yaml per project, sourced from forge-protocol
5. **Separation of concerns** - Ethics protocol vs project configuration

## Consequences

### Positive

- **Single source of truth**: Update forge-protocol/ethics.yaml, copy to projects
- **Token efficient**: Same cost as inline, but maintainable
- **Compaction resilient**: Compact summary survives, full available on re-read
- **Clean architecture**: Each file has one job
- **Scalable**: Works for 11 projects, works for 111 projects

### Negative

- **Two files required**: Each project needs both warmup.yaml and ethics.yaml
- **Copy distribution**: Must copy ethics.yaml to new projects (one-time cost)
- **Sync burden**: When ethics.yaml updates, must propagate to all projects

### Mitigation

The sync burden is mitigated by:
1. Ethics changes are **rare** (protocol-level, not project-level)
2. Can automate with `cp ~/src/royalbit/forge-protocol/ethics.yaml .`
3. Git hooks can validate ethics.yaml presence

## Related ADRs

- [ADR-008: Ethics Protocol and Humanist Mode](008-ethics-protocol-humanist-mode.md) - Original ethics introduction
- [ADR-006: Git Hook Protocol Refresh](006-git-hook-protocol-refresh.md) - Self-healing via commits
- [ADR-013: Self-Healing NOT Replaced](013-self-healing-not-replaced.md) - Why warmup.yaml re-read matters

## Implementation Checklist

Applied to all 11 forge-protocol enabled projects:

- [x] `ethics.yaml` copied to each project
- [x] `CLAUDE.md` updated with `@warmup.yaml` + `@ethics.yaml`
- [x] `warmup.yaml` contains compact `ethics:` section
- [x] Git pre-commit hooks installed with `forge-protocol refresh`
