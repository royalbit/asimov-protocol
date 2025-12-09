# Component 4: Self-Healing

> **Recover from context compaction - re-read rules from disk**

## Overview

Self-Healing solves a critical problem: **AI forgets rules after context compaction**.

During autonomous sessions, Claude Code compresses conversation history. Rules get summarized away. The AI "forgets" project conventions.

Self-Healing fixes this by **re-reading rules from disk** instead of trying to preserve them in memory.

## The Reality (ADR-003)

**Early assumption:** Checkpoint based on time intervals
**Research found:** Compaction happens every **10-20 minutes** with heavy reasoning

```
With MAX_THINKING_TOKENS=200000:
- Context window: 200k tokens
- Thinking budget: 200k tokens
- Context fills in: 1-3 heavy turns
- Compaction interval: ~10-20 minutes
```

See [ADR-003](../adr/003-self-healing-real-compaction-data.md) for the full research.

## The Problem

```
Session Start                    After Compaction
─────────────                    ────────────────
Full rules loaded                Rules summarized
Quality gates clear       →      "There were some rules"
Patterns understood              Patterns forgotten
AI follows conventions           AI makes mistakes
```

As documented by DoltHub:
> "Claude Code is definitely dumber after the compaction. It doesn't know what files it was looking at and needs to re-read them."

## The Solution

```mermaid
flowchart LR
    subgraph healing["SELF-HEALING PROTOCOL - ALL ON DISK (survives compaction)"]
        CM["**CLAUDE.md**<br/>(auto-loaded)<br/><br/>Ultra-short +<br/>'re-read'"]
        WU["**warmup.json**<br/>(protocol loader)<br/><br/>Loads all<br/>protocols"]
        PR["**protocols/**<br/>(full rules)<br/><br/>Complete<br/>protocol + gates"]
        CP["**checkpoint**<br/>(session state)<br/><br/>Progress<br/>+ hints"]

        CM --> WU --> PR --> CP
    end
```

## The Three Files

### 1. CLAUDE.md (Auto-loaded, Ultra-Short)

Must survive summarization. Single critical instruction:

```markdown
# Project Name

ON CONFUSION → run warmup (loads all protocols)

Rules: run until done, keep shipping, tests pass.
```

### 2. warmup.json (Protocol Loader)

Entry point that loads all protocol files. Re-read when confused:

```json
{
  "protocol": "warmup",
  "description": "RoyalBit Asimov - Session warmup entry point",
  "on_start": [
    "load_protocols",
    "load_project",
    "validate",
    "read_roadmap",
    "present_milestone"
  ],
  "load": [
    "asimov.json",
    "sprint.json",
    "coding-standards.json",
    "freshness.json",
    "green.json",
    "exhaustive.json",
    "sycophancy.json"
  ]
}
```

### 3. Session State (WIP Continuity)

Written during session. Contains progress and recovery state. Format determined by implementation (may be checkpoint file or other mechanism).

## Recovery Triggers

Based on real compaction data, NOT time-based intervals:

| Trigger | Rationale |
|---------|-----------|
| Every major task | Natural breakpoint |
| Every 10-15 tool calls | ~15 min of work |
| Before file write >100 lines | Significant change |
| Before any commit | Quality gate |
| On ANY confusion | Recovery signal |

Compaction happens every 10-20 minutes with heavy reasoning, so recovery mechanisms are designed around task completion rather than time intervals.

## Platform Requirement

**Self-Healing requires Claude Code.** Other AI tools lack:

| Capability | Required | ChatGPT | Copilot | Claude Code |
|------------|----------|---------|---------|-------------|
| Auto-load config | Yes | No | No | Yes |
| File system access | Yes | No | No | Yes |
| Re-read mid-session | Yes | No | No | Yes |

## Relationship to Other Components

| Component | Connection |
|-----------|------------|
| Protocol Files | warmup.json loads all protocol files |
| Sprint Autonomy | Recovery aligns with task completion |
| Quality Gates | Re-read before running gates |
| Release Discipline | Recovery state before release |

---

**Next:** [Component 5: Release Discipline](5-RELEASE_DISCIPLINE.md)

---
