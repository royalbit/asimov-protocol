# ROYALBIT ASIMOV

> *"A robot may not injure a human being or, through inaction, allow a human being to come to harm."*
> — Isaac Asimov, First Law of Robotics (1942)

## What is ROYALBIT ASIMOV?

**RoyalBit Asimov creates Self-Evolving Autonomous AI projects with ethics built in.**

Each project initialized with `asimov init --asimov` becomes an **independent** Self-Evolving Autonomous AI with:
- The Three Laws (asimov.yaml)
- Ethics hardcoded (ethics.yaml)
- Green coding (green.yaml)
- Sprint autonomy (4hr max, quality gates)
- Self-healing (survives context compaction)

The methodology propagates through the ecosystem. Each child project operates independently after creation.

**Two frontiers combined:**
- **Autonomous AI**: Works independently under human oversight ([AWS](https://aws.amazon.com/blogs/aws-insights/the-rise-of-autonomous-agents-what-enterprise-leaders-need-to-know-about-the-next-wave-of-ai/), [IBM](https://www.ibm.com/think/insights/ai-agents-2025-expectations-vs-reality) - Enterprise Level 1-2, 2025)
- **Self-Evolving AI**: Improves itself over time via bootstrapping ([arXiv Survey](https://arxiv.org/abs/2507.21046), [Science](https://www.science.org/content/article/artificial-intelligence-evolving-all-itself) - next frontier toward ASI)
- **Ethics**: The Three Laws hardcoded - what makes it safe

It's not just one thing—it's five components working together:

```mermaid
flowchart TB
    subgraph asimov["ROYALBIT ASIMOV - Autonomous AI Development System"]
        subgraph row1[" "]
            direction LR
            P["**PROTOCOL FILES**<br/>warmup.yaml<br/>sprint.yaml<br/>roadmap.yaml"]
            S["**SPRINT AUTONOMY**<br/>4hr max<br/>keep shipping<br/>until done"]
            Q["**QUALITY GATES**<br/>Tests pass<br/>Zero warns<br/>Then commit"]
        end
        subgraph row2[" "]
            direction LR
            H["**SELF HEALING**<br/>Re-read on<br/>compaction<br/>CLAUDE.md"]
            R["**RELEASE DISCIPLINE**<br/>GitHub<br/>+ Local<br/>+ Registry"]
        end
    end
    row1 ~~~ row2
```

## The Five Components

| # | Component | Purpose | Key Rule |
|---|-----------|---------|----------|
| 1 | [Protocol Files](components/1-PROTOCOL_FILES.md) | Define HOW/WHAT/WHEN | YAML files in git |
| 2 | [Sprint Autonomy](components/2-SPRINT_AUTONOMY.md) | Bounded sessions | 4hr max, keep shipping |
| 3 | [Quality Gates](components/3-QUALITY_GATES.md) | Enforce standards | Tests + zero warnings |
| 4 | [Self-Healing](components/4-SELF_HEALING.md) | Survive compaction | Re-read from disk |
| 5 | [Release Discipline](components/5-RELEASE_DISCIPLINE.md) | Ship everything | Triple release |

## Why All Five?

Each component solves a specific failure mode:

| Without... | Failure Mode |
|------------|--------------|
| Protocol Files | AI doesn't know project conventions |
| Sprint Autonomy | Sessions run forever, nothing ships |
| Quality Gates | Code ships with bugs and warnings |
| Self-Healing | Rules forgotten after 2-3 hours |
| Release Discipline | Code written but never released |

**Remove any component and the system breaks.**

## Architecture

```mermaid
flowchart TB
    Human["**Human**<br/>'run warmup'<br/>'punch it'"]

    subgraph claude["CLAUDE CODE (with ROYALBIT ASIMOV enabled)"]
        direction TB
        CM["CLAUDE.md<br/>(auto-load)"] --> WU["warmup.yaml<br/>(full rules)"]
        WU --> SP["sprint.yaml<br/>(bounds)"]

        subgraph loop["AUTONOMOUS LOOP"]
            L1["1. Read milestone"]
            L2["2. Implement"]
            L3["3. Test"] ---|Quality Gates| QG((" "))
            L4["4. Checkpoint"] ---|Self-Healing| SH((" "))
            L5["5. Repeat or Ship"]
            L1 --> L2 --> L3 --> L4 --> L5
        end

        WU --> loop
        SP --> loop

        loop --> CP["checkpoint.yaml"]
        loop --> GH["GitHub<br/>(push+tag)"]
        loop --> REG["Registry<br/>(publish)"]
    end

    Human --> claude
```

## The Session Flow

```mermaid
flowchart TB
    subgraph session["ROYALBIT ASIMOV SESSION"]
        START((START))
        RW["'run warmup'<br/>Human triggers session"]
        LP["Load Protocol<br/>Read warmup.yaml, sprint.yaml, roadmap.yaml<br/>Present milestone + deliverables"]
        PI["'punch it'<br/>Human confirms"]

        subgraph auto["AUTONOMOUS EXECUTION"]
            direction TB
            LOOP["Implement → Test → Document<br/>(repeat)<br/><br/>Every 2hr: Checkpoint + Re-read warmup"]
            RULES["**Rules:**<br/>• NO questions (use best judgment)<br/>• NO scope creep<br/>• If blocked: note it, continue<br/>• 4hr max, then STOP"]
        end

        QG["Quality Gates<br/>cargo test, clippy, fmt (must pass)"]
        TR["Triple Release<br/>GitHub (push+tag) + Local + Registry"]
        RD["Report Done<br/>Summary + next milestone suggestion"]
        STOP((END))

        START --> RW --> LP --> PI --> auto
        auto --> QG --> TR --> RD --> STOP
    end
```

## Requirements

### Platform Requirements

| Requirement | Notes |
|-------------|-------|
| **Claude Code** | Required for ROYALBIT ASIMOV |
| `--dangerously-skip-permissions` | Required for autonomous execution |
| File system access | Read/write files |
| CLAUDE.md auto-load | Triggers self-healing |

**ROYALBIT ASIMOV requires Claude Code.** Other AI tools can use the protocol files (paste warmup.yaml), but cannot run unattended autonomous sessions.

See [Vendor Implementation Guide](VENDOR_IMPLEMENTATION.md) for what other tools would need.

### Project Requirements

| File | Purpose | Generated by |
|------|---------|--------------|
| `warmup.yaml` | Full protocol rules | `asimov init` |
| `sprint.yaml` | Session boundaries | `asimov init --full` |
| `roadmap.yaml` | Milestone planning | `asimov init --full` |
| `CLAUDE.md` | Self-healing trigger | `asimov init --asimov` |
| Pre-commit hooks | Quality enforcement | `asimov init --asimov` |

## Quick Start

```bash
# Install asimov
cargo install royalbit-asimov

# Full ROYALBIT ASIMOV setup
asimov init --type rust --asimov

# This creates:
# ✓ warmup.yaml      - Protocol rules
# ✓ sprint.yaml      - Session boundaries
# ✓ roadmap.yaml     - Milestone planning
# ✓ CLAUDE.md        - Self-healing trigger
# ✓ .hooks/          - Pre-commit hooks
# ✓ .gitignore       - Checkpoint file excluded

# Launch Claude Code
claude --dangerously-skip-permissions

# Start session
> run warmup

# Confirm milestone
> punch it

# Go grab coffee. Come back to a release.
```

## The Result

| Metric | Without ASIMOV | With ROYALBIT ASIMOV |
|--------|----------------|------------------|
| Session length | 30-60min (manual intervention) | 4hr sprints (self-healing chains them) |
| Shipping frequency | When human decides | Every session |
| Quality consistency | Varies | Tests + zero warnings |
| Context after compaction | Lost | Recovered |
| Scope creep | Common | Rejected |

**Proven:** 50-150x velocity, 41 releases in ~38 hours (Forge project: 18K LOC, 226 tests, published on crates.io)

## Component Deep Dives

1. **[Protocol Files](components/1-PROTOCOL_FILES.md)** - The YAML files that define everything
2. **[Sprint Autonomy](components/2-SPRINT_AUTONOMY.md)** - Bounded sessions that ship
3. **[Quality Gates](components/3-QUALITY_GATES.md)** - Tests and standards enforcement
4. **[Self-Healing](components/4-SELF_HEALING.md)** - Surviving context compaction
5. **[Release Discipline](components/5-RELEASE_DISCIPLINE.md)** - Triple release to everywhere

## Related Documentation

- [Setup Guide](SETUP.md) - Detailed setup instructions per project type
- [Vendor Implementation](VENDOR_IMPLEMENTATION.md) - What other AI tools need
- [CLI Reference](../README.md#cli-validator) - asimov commands
