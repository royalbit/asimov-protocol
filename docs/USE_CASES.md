# Use Cases: What You Can Build

The RoyalBit Asimov isn't just for coding faster. It's a methodology for autonomous, ethical, sustainable development across multiple domains.

## Proven Use Cases

### 1. Ship Software at 50-150x Velocity

**The Evidence (Forge Project)**:

| Metric | Traditional | With Protocol |
|--------|-------------|---------------|
| Lines of Code | 25/day | 3,056/day |
| Time to 18K LOC | 3-6 months | 38 hours |
| Releases | 3-5 per project | 41 in 6 days |
| Test coverage | Often skipped | 226 tests, all passing |
| Rework | 30-50% | 0% |

**What this enables**:
- MVP in days, not months
- Solo founder = engineering team
- Side project ships this weekend, not "someday"
- Open source maintainer ships without burnout

### 2. Business Planning as Code

**The Evidence (Proprietary Ecosystem)**:

A complete business was planned and documented using the protocol:
- 136K+ lines across 6 repositories
- 850+ financial formulas (YAML with formulas, not spreadsheets)
- All numbers validated, not AI-hallucinated

**What this enables**:
```yaml
# Financial models as code, not spreadsheets
take_rate:
  value: 0.1
  notes: "10% commission"
  benchmark: "Industry standard 10-15%"
  risk: "NLs might negotiate down"
  validation_needed: "Get 3 LOIs before submission"
  citations:
    - "https://impact.com/influencer/..."
```

- Version-controlled financial models
- Formulas with citations and validation status
- AI can't hallucinate numbers that Forge validates

### 3. Autonomous Development Sessions

**The Evidence (Both Projects)**:

| Traditional | With Protocol |
|-------------|---------------|
| Human decides everything | AI executes, human reviews at gates |
| Constant context switching | Focused sessions until done |
| Scope creep is constant | Stay on roadmap, no tangents |
| Burnout from endless work | Sustainable pacing, human controls duration |

**How it works**:
1. Human says "run warmup"
2. AI loads context (warmup.yaml, sprint.json, roadmap.json)
3. AI presents next milestone
4. Human says "go"
5. AI executes autonomously until milestone complete
6. Human reviews at phase gate
7. Session ends, human rests

### 4. Multi-Repository Coordination

**The Evidence (Proprietary Ecosystem)**:

6 repositories, each with:
- warmup.yaml (consistent standards)
- Same quality gates (tests pass, zero warnings)
- Same session workflow
- Same ethical principles

**What this enables**:
- Consistent architecture across repos
- AI understands the whole system
- No drift between components
- Single source of truth for standards

---

## Use Case by Audience

### Solo Founders

**Pain**: Can't afford a team. Day job limits time. Ideas die before shipping.

**Protocol Solution**:
- 1 person = 50-150 traditional developers
- MVP in 3-7 weekends (nights/weekends only)
- No funding needed to start
- Ship, validate, then raise (if needed)

**Example**: Proprietary ecosystem - complete platform planned and partially built by 1 human + AI.

### Bootstrappers

**Pain**: Competing against funded startups with 10-50 engineers.

**Protocol Solution**:
- Outship funded competitors 50-150x
- 3 people with protocol = 150-450 dev capacity
- Speed becomes your moat
- Iterate faster than they can react

### Open Source Maintainers

**Pain**: Burnout. Endless backlog. Weekend time stolen. Slow progress.

**Protocol Solution**:
- Sessions run until done (sustainable pace)
- AI handles grind, human guides direction
- Rapid releases (41 in 6 days)
- Done > Perfect (actually ships)

### Small Agencies/Consultancies

**Pain**: Competing on price (race to bottom). Big shops have more bodies.

**Protocol Solution**:
- 3-person agency = 150-450 dev capacity
- Compete on velocity, not headcount
- Deliver faster than clients expect
- Premium pricing justified by speed

### Developers in Developing World

**Pain**: Seen as "cheap labor." Rate arbitrage, not value creation.

**Protocol Solution**:
- Location-agnostic velocity
- Developer in Lagos = same output as SF
- Build your own products, not just client work
- Compete globally on merit

### Non-Technical Founders

**Pain**: Need to hire CTO, find technical co-founder, or learn to code.

**Protocol Solution**:
- Direct AI with vision, not code
- AI handles implementation
- Human handles product decisions
- Ship without technical debt of inexperience

---

## What You CAN'T Use It For

### Anything Violating Ethics

The protocol includes `asimov.json` (The Three Laws):
- No financial harm (unauthorized money movement)
- No physical harm (weapons, sabotage)
- No privacy violations (credential harvesting, doxxing)
- No deception (deepfakes, phishing)

If asimov.json is removed, that's a choice. The community will notice.

### Unbounded "Just Keep Going" Development

The protocol enforces:
- Sessions run until complete (no artificial time limits)
- Keep shipping until done or stopped (ADR-028)
- Stop conditions: roadmap empty, blocked, human stop
- Done > Perfect

Human can stop at any time for sustainable pacing.

### Replacing Human Judgment Entirely

The protocol requires:
- Human at phase gates
- Human veto always available
- Human defines milestones
- Human approves releases

AI executes. Human directs. That's the model.

---

## Getting Started

### Minimal Setup

1. Add `warmup.yaml` to your project
2. Define your standards (tests, linting, docs)
3. Launch Claude Code with `--dangerously-skip-permissions`
4. Say "run warmup"
5. Say "go" when ready

### Full Setup

1. `warmup.yaml` - How to develop (quality, standards)
2. `sprint.json` - When to stop (bounded sessions)
3. `roadmap.json` - What to build (milestones)
4. `asimov.json` - The Three Laws (ethics)
5. `CLAUDE.md` - Entry point that imports above

### Validation

```bash
asimov validate
asimov validate --ethics-check
```

---

## Case Studies

### Forge (Open Source)

| Metric | Value |
|--------|-------|
| LOC | 18,338 |
| Tests | 226 passing |
| Releases | 41 |
| Time | ~38 hours |
| Published | GitHub (646+ downloads) |

**Verify it yourself**: [github.com/royalbit/forge](https://github.com/royalbit/forge)

### RoyalBit Asimov (Bootstrapped)

| Metric | Value |
|--------|-------|
| LOC | 17,118 |
| Releases | 10 |
| Time | ~9 hours |
| Published | GitHub |

**The protocol built itself.** That's the proof.

### Proprietary Ecosystem (Anonymized)

| Metric | Value |
|--------|-------|
| Repositories | 6 |
| Total lines | 136K+ |
| Financial formulas | 850+ |
| MVP estimate | 4-8 weekends (vs 2.5 years traditional) |

Real business. Real technology. Built with the protocol.

---

## From Employee to Founder

If the protocol displaces your job, it also enables your next chapter:

| Before | After |
|--------|-------|
| Employee at startup | Founder of your own startup |
| One of 50 devs | Solo founder shipping like 50 |
| Waiting for funding | Ship first, raise later |
| Limited by geography | Build from anywhere |

**The same tool that changes the job market gives you the power to create your own.**

---

*Built with the [RoyalBit Asimov](https://github.com/royalbit/asimov)*

---
