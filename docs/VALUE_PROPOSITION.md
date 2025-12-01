# Value Proposition: Why RoyalBit Asimov

## The One-Liner

**"The ethical operating system for Claude's autonomous power."**

Claude Opus 4.5 and Sonnet 4.5 deliver 50-100x velocity. **That's Claude, not Asimov.** ([Anthropic](https://www.anthropic.com/news/claude-opus-4-5))

Asimov ensures you don't destroy yourself in the process:
- **Ethics**: The Three Laws propagate to every child project
- **Bounded Autonomy**: 4hr max sessions, quality gates
- **Sustainability**: Green coding, local validation

---

## The Real Problem

Every company using Claude gets the velocity. The question is:

> **"How do you ensure autonomous AI development doesn't compromise ethics, quality, or sustainability?"**

| Tool | Velocity | Guardrails |
|------|----------|------------|
| Claude alone | 50-100x | None |
| Claude + Asimov | 50-100x | **Ethics, bounded autonomy, sustainability** |

**The velocity is Claude's. The guardrails are Asimov's.**

---

## The Complete Stack

| Layer | Provides | Source |
|-------|----------|--------|
| **Claude Opus 4.5 / Sonnet 4.5** | 50-100x velocity, SWE-bench 80.9% | [Anthropic](https://www.anthropic.com/news/claude-opus-4-5) |
| **Claude Code** | 200k thinking tokens (6x threshold) | [ADR-026](adr/026-claude-code-requirement.md) |
| **Asimov Protocol** | Ethics, bounded autonomy, sustainability | [ADR-025](adr/025-claude-attribution-principle.md) |

**Why Claude Code specifically?** MCP IDEs (Cursor, Windsurf) cap thinking at 30k-48k or charge premium. Claude Code allows 200k FREE.

---

## The Evidence

### Forge Project

| Metric | Industry Standard | With Protocol | Multiplier |
|--------|-------------------|---------------|------------|
| LOC/day | 25 | 3,056 | **122x** |
| Time to 18K LOC | 3-6 months | 38 hours | **100-200x** |
| Releases | 3-5 typical | 41 in 6 days | **10x** |
| Rework | 30-50% | 0% | **Infinite** |

**Verify it**: [github.com/royalbit/forge](https://github.com/royalbit/forge), [crates.io/royalbit-forge](https://crates.io/crates/royalbit-forge)

### The Protocol Built Itself

| Project | LOC | Releases | Hours | Verified |
|---------|-----|----------|-------|----------|
| forge | 18,338 | 41 | ~38 | crates.io |
| asimov | 17,118 | 10 | ~9 | crates.io |
| **Combined** | **35,456** | **51** | **~47** | Both published |

**1 human. 1 AI. 47 hours. 51 releases.**

---

## Who This Is For

### Solo Founders

> "Build your startup in weekends, not years."

| Before | After |
|--------|-------|
| Need $500K+ for team | Ship yourself |
| 3-6 months to MVP | Days to weeks |
| Can't compete with funded startups | Outship them |

### Bootstrappers

> "Compete with funded startups. Actually outship them."

| Before | After |
|--------|-------|
| 10-50 engineers against you | You = 50-150 of them |
| Speed is their advantage | Speed is YOUR advantage |
| Race to fundraise | Race to revenue |

### Open Source Maintainers

> "Maintain without burning out. Ship faster than ever."

| Before | After |
|--------|-------|
| Endless backlog | 41 releases in 6 days |
| Weekend burnout | 4hr bounded sessions |
| Slow progress | Rapid iteration |

### Small Agencies

> "3-person team delivers like 150-450 devs."

| Before | After |
|--------|-------|
| Compete on price | Compete on velocity |
| Need more headcount | Need better methodology |
| Underbid to win | Premium for speed |

### Developers Globally

> "Your location doesn't limit your output."

| Before | After |
|--------|-------|
| "Cheap offshore labor" | Equal velocity anywhere |
| Rate arbitrage | Value creation |
| Build for clients | Build your own |

---

## Differentiators

### vs GitHub Copilot

| Metric | Copilot | RoyalBit Asimov |
|--------|---------|----------------|
| Productivity | 1.1-1.6x | **50-150x** |
| Mode | Autocomplete | **Autonomous sessions** |
| Scope | Line/function | **Full milestones** |
| Self-hosting | No | **Yes** |
| Cost | $19-39/user/mo | **Free (MIT)** |

### vs Devin / SWE-Agent

| Metric | Devin et al. | RoyalBit Asimov |
|--------|--------------|----------------|
| Benchmark success | 13.86% | **51 releases shipped** |
| Maturity | "Still maturing" | **Production proven** |
| Cost | $500/mo+ | **Free (MIT)** |
| Ethics built-in | No | **Yes** |
| Human oversight | Varies | **Structured (gates)** |

### vs Traditional Development

| Metric | Traditional | RoyalBit Asimov |
|--------|-------------|----------------|
| LOC/day | 25 | 3,056 |
| MVP timeline | 3-6 months | Days to weeks |
| Team size | 5-10 | 1 + AI |
| Rework | 30-50% | ~0% |
| Sustainability | Burnout common | 4hr bounded sessions |

---

## The Three Pillars

### 1. Ethics (The Three Laws)

Not bolted on. Baked in. **Claude's power requires responsibility.**

| Principle | What It Means |
|-----------|---------------|
| Do No Harm | No malware, weapons, theft, deception |
| No Harm Through Inaction | Disclose limitations, search when stale |
| Human Veto | "stop" always works |
| 2-Cosigner | Can't quietly remove ethics |

### 2. Bounded Autonomy (Quality Gates)

Claude can work autonomously for hours. **Asimov ensures it ships.**

| Constraint | Why |
|------------|-----|
| 4hr max sessions | Bounded blast radius |
| Quality gates | Tests MUST pass, zero warnings |
| Phase checkpoints | Human review at milestones |
| Self-healing | Recover from compaction |

### 3. Sustainability (Green Coding)

Claude API calls cost money and carbon. **Asimov minimizes both.**

| Metric | Cloud AI | Protocol | Reduction |
|--------|----------|----------|-----------|
| Validation cost | $792/year | $0 | 100% |
| Carbon footprint | ~0.25g/op | ~0.0005g/op | 99.6% |
| Binary size | 150-200 MB | 2.84 MB | 98% |

**Note:** The velocity (50-100x) comes from Claude. Asimov's value is ensuring that velocity is sustainable, ethical, and bounded.

---

## The Sales Pitch

### For the Skeptic

> "Don't believe us. Check the git logs. It's all public."

- [Forge commits](https://github.com/royalbit/forge/commits)
- [Protocol commits](https://github.com/royalbit/asimov/commits)
- [crates.io downloads](https://crates.io/crates/royalbit-forge)

### For the Pragmatist

> "MIT licensed. Free. Try it on your next project."

```bash
cargo install royalbit-asimov
asimov init --type rust
```

### For the Visionary

> "What if every developer could ship like a 50-person team?"

- Democratized software creation
- Global accessibility
- Sustainable by design
- Ethics as foundation

---

## What You Get

### The Protocol (Free)

- `warmup.yaml` - How to develop
- `sprint.yaml` - When to stop
- `roadmap.yaml` - What to build
- `ethics.yaml` - Values and guardrails

### The CLI (Free)

```bash
asimov validate        # Check your files
asimov init            # Generate templates
asimov validate --ethics-check  # Verify ethics
```

### The Methodology (Documented)

- Session workflow
- Milestone sizing
- Release criteria
- Quality gates
- Bounded autonomy

---

## Getting Started

### 5 Minutes

```bash
cargo install royalbit-asimov
cd your-project
asimov init --type rust
```

### First Session

```
Human: "run warmup"
AI: [loads context, presents milestone]
Human: "go"
AI: [executes autonomously]
AI: [reports completion at gate]
```

### First Release

- Tests pass
- Zero warnings
- Docs updated
- Version bumped
- Tagged and pushed

---

## The Bottom Line

**Traditional development**: 25 LOC/day, 3-6 months to MVP, team of 5-10.

**With RoyalBit Asimov**: 3,056 LOC/day, days to weeks to MVP, 1 human + AI.

**The math**: 50-150x velocity, verified, published, free.

**The catch**: None. MIT licensed. Use it. Verify it. Ship with it.

---

*Built with the [RoyalBit Asimov](https://github.com/royalbit/asimov) - 50-150x developer velocity with ethics built in.*
