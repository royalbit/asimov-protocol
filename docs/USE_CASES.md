# Use Cases: What You Can Build

RoyalBit Asimov is a methodology for autonomous, ethical, sustainable development across multiple domains.

## Proven Use Cases

### 1. Ship Software at 50-150x Velocity

**The Evidence (Forge Project)**:

| Metric | Traditional | With Protocol |
|--------|-------------|---------------|
| Lines of Code | 25/day | 3,056/day |
| Time to 45K LOC | 3-6 months | 38 hours |
| Releases | 3-5 per project | 41 in 6 days |
| Test coverage | Often skipped | 2,486 tests, all passing |
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

**Workflow**:
1. "run warmup" - AI loads context (warmup.yaml, sprint.json, roadmap.json)
2. AI presents next milestone
3. "go" - AI executes autonomously until milestone complete
4. Human reviews at phase gate

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

**Solution**:
- 1 person = 50-150 traditional developers
- MVP in 3-7 weekends
- Ship, validate, then raise

### Bootstrappers

**Pain**: Competing against funded startups with 10-50 engineers.

**Solution**:
- Outship funded competitors 50-150x
- 3 people = 150-450 dev capacity
- Speed becomes your moat

### Open Source Maintainers

**Pain**: Burnout. Endless backlog. Slow progress.

**Solution**:
- Sessions run until done (sustainable pace)
- AI handles grind, human guides direction
- Rapid releases (41 in 6 days)

### Small Agencies/Consultancies

**Pain**: Competing on price. Big shops have more bodies.

**Solution**:
- 3-person agency = 150-450 dev capacity
- Compete on velocity, not headcount
- Premium pricing justified by speed

### Developers in Developing World

**Pain**: Seen as "cheap labor." Rate arbitrage, not value creation.

**Solution**:
- Location-agnostic velocity
- Build your own products, not just client work
- Compete globally on merit

### Non-Technical Founders

**Pain**: Need to hire CTO or find technical co-founder.

**Solution**:
- Direct AI with vision, not code
- AI handles implementation
- Human handles product decisions

---

## What You CAN'T Use It For

### Anything Violating Ethics

The protocol includes `asimov.json` (The Three Laws):
- No financial harm (unauthorized money movement)
- No physical harm (weapons, sabotage)
- No privacy violations (credential harvesting, doxxing)
- No deception (deepfakes, phishing)

### Unbounded "Just Keep Going" Development

The protocol enforces:
- Sessions run until complete
- Stop conditions: roadmap empty, blocked, or human stop
- Perfect > Done, no sloppy code

### Replacing Human Judgment

The protocol requires:
- Human at phase gates
- Human veto always available
- Human defines milestones and approves releases

---

## Getting Started

### Minimal Setup

1. Add `warmup.yaml` to your project
2. Define standards (tests, linting, docs)
3. Launch Claude Code: `--dangerously-skip-permissions`
4. Run: "run warmup" then "go"

### Full Setup

1. `warmup.yaml` - How to develop
2. `sprint.json` - When to stop
3. `roadmap.json` - What to build
4. `asimov.json` - The Three Laws
5. `CLAUDE.md` - Entry point

Validate: `asimov validate`

---

## Case Studies

### Forge (Open Source)

| Metric | Value |
|--------|-------|
| LOC | 45,700 |
| Tests | 2,486 passing |
| Functions | 159 (153 Excel + 6 FP&A) |
| Demo | [forge-demo](https://github.com/royalbit/forge-demo) |

### RoyalBit Asimov

| Metric | Value |
|--------|-------|
| LOC | 17,118 |
| Releases | 10 |
| Time | ~9 hours |
| Published | GitHub |

### Proprietary Ecosystem

| Metric | Value |
|--------|-------|
| Repositories | 6 |
| Total lines | 136K+ |
| Financial formulas | 850+ |
| MVP estimate | 4-8 weekends vs 2.5 years traditional |

---

## From Employee to Founder

| Before | After |
|--------|-------|
| Employee at startup | Founder of your own startup |
| One of 50 devs | Solo founder shipping like 50 |
| Waiting for funding | Ship first, raise later |
| Limited by geography | Build from anywhere |

---

*Built with the [RoyalBit Asimov](https://github.com/royalbit/asimov)*

---
