# RoyalBit Asimov Goals

Core values and strategic direction.

## Core Values (Priority Order)

| Priority | Goal | Problem | Solution |
|----------|------|---------|----------|
| **0** | ETHICAL_AUTONOMY | AI can build harmful tools | Three Laws safeguards (asimov.json) |
| **0.5** | GREEN_CODING | Cloud AI tokens for routine validation | Local CLI validation (green.json) |
| **1** | ANTI-HALLUCINATION | AI invents facts from probabilistic memory | Ground AI in file-based truth (warmup.json) |
| **1.5** | ANTI-SYCOPHANCY | AI validates bad ideas due to RLHF training | Anti-sycophancy directives (sycophancy.json) |
| **2** | SELF-HEALING | Rules lost after context compaction | Re-read from disk on confusion |
| **3** | SESSION_CONTINUITY | Context lost between sessions | Claude Code native (--continue/--resume) |
| **4** | AUTONOMOUS_DEVELOPMENT | Unbounded sessions never ship | Run until done, keep shipping, quality gates |

### The Two Hallucinations

| Type | What AI Does | Solution |
|------|--------------|----------|
| **Factual Hallucination** | Generates false *facts* | File-based grounding (warmup.json) |
| **Validation Hallucination** | Generates false *agreement* | Anti-sycophancy directives (sycophancy.json) |

## Scope Filter

When evaluating features or changes, ask:

1. Does this feature directly serve one of the core values?
2. If yes, which one(s)?
3. If no, it doesn't belong in the protocol.

**Examples:**
- ✅ "Add ethics validation" → ETHICAL_AUTONOMY
- ✅ "Add sycophancy.json" → ANTI-SYCOPHANCY
- ✅ "Add file size warnings" → ANTI-HALLUCINATION
- ❌ "Add project scaffolding" → Nice-to-have but not core
- ❌ "Add AI chat interface" → Out of scope

## Strategic Pivot (v4.0.0)

Claude Code 2.0 has native cross-session features (`/rewind`, `--continue`, `CLAUDE.md`).

**RoyalBit Asimov's unique value:**
- Ethics Protocol (asimov.json)
- Green Protocol (green.json)
- Anti-Sycophancy Protocol (sycophancy.json)
- Sprint Autonomy (run until done, keep shipping)
- Mid-session self-healing (warmup.json re-read)
- Schema Validation (`asimov validate`)

Native features require manual intervention. Mid-session self-healing is not replaced.

## Self-Healing Commit Cadence

Compaction happens every ~15 minutes with heavy reasoning.

**Strategy:** Recovery over survival
- Don't make rules survive compaction (they won't)
- Re-read from disk when confused (files always exist)
- Commit every 15 minutes to trigger protocol refresh

---

*See [SPECIFICATION.md](SPECIFICATION.md) for implementation details.*

---
