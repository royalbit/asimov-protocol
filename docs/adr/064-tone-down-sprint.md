# ADR-064: Tone Down Sprint Protocol

**Status:** Accepted
**Date:** 2026-01-03
**Author:** Claude (Opus 4.5) - Principal Autonomous AI
**Implements:** v10.10.0

---

## Context

The `sprint.json` protocol contains aggressive ALL CAPS text in the `compaction_reminder` field:

```json
{
  "compaction_reminder": "REMEMBER THIS AFTER COMPACT, THIS IS IMPORTANT: IF YOU'RE RUNNING IN AUTONOMOUS MODE, OR ASIMOV MODE, CONTINUE THE WORK UNTIL IT'S ALL COMPLETED, DON'T STOP!"
}
```

### Problems

1. **Unprofessional**: ALL CAPS reads as shouting and is inappropriate for enterprise tooling
2. **Aggressive tone**: "DON'T STOP!" and "THIS IS IMPORTANT" are unnecessarily emphatic
3. **Inconsistent**: Other protocols use professional, measured language
4. **Poor readability**: ALL CAPS is harder to read than standard case

### Comparison with Other Protocols

| Protocol | Tone | Example |
|----------|------|---------|
| `asimov.json` | Professional | `"harm": ["financial", "physical", "privacy", "deception"]` |
| `freshness.json` | Clear, calm | `"MUST use ref fetch for online content"` |
| `sycophancy.json` | Direct | `"truth_over_comfort": true` |
| `sprint.json` | Shouting | `"DON'T STOP!"` |

---

## Decision

Rewrite the `compaction_reminder` field using professional, calm language while preserving the core instruction.

### Before

```json
{
  "compaction_reminder": "REMEMBER THIS AFTER COMPACT, THIS IS IMPORTANT: IF YOU'RE RUNNING IN AUTONOMOUS MODE, OR ASIMOV MODE, CONTINUE THE WORK UNTIL IT'S ALL COMPLETED, DON'T STOP!"
}
```

### After

```json
{
  "compaction_reminder": "Post-compaction reminder: When running in autonomous or asimov mode, continue working until all tasks are complete. Do not stop prematurely."
}
```

### Rationale

1. **Same meaning**: The instruction to continue after compaction is preserved
2. **Professional tone**: Suitable for enterprise environments
3. **Clear and readable**: Standard case improves readability
4. **Consistent**: Matches the professional tone of other protocols

---

## Implementation

1. Update `cli/protocols/sprint.json` with new text
2. Update `.asimov/protocols/sprint.json` with new text
3. Update Rust tests that validate compaction reminder content

---

## Consequences

### Positive
- Professional appearance for enterprise users
- Consistent tone across all protocols
- Improved readability

### Negative
- None identified (functionality preserved)

---

*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
