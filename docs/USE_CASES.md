# Use Cases: What You Can Build

The Asimov Protocol isn't just for coding faster. It's a methodology for autonomous, ethical, sustainable development across multiple domains.

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
- Grant applications ready for $650K-$1.35M in funding
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
- Grant-ready documentation from day one

### 3. Autonomous Development Sessions

**The Evidence (Both Projects)**:

| Traditional | With Protocol |
|-------------|---------------|
| Human decides everything | AI executes, human reviews at gates |
| Constant context switching | 4hr focused sessions |
| Scope creep is constant | 1 milestone, then STOP |
| Burnout from endless work | Sustainable bounded sessions |

**How it works**:
1. Human says "run warmup"
2. AI loads context (warmup.yaml, sprint.yaml, roadmap.yaml)
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

### 5. Grant/Funding Documentation

**The Evidence (Proprietary Ecosystem)**:

Complete grant applications for:
- IQ Innovation Program ($300-500K)
- ESSOR ($50-70K)
- IRAP ($300-500K)
- Total pipeline: $650K-$1.35M

**What this enables**:
- Documentation is always current
- Numbers are validated, not invented
- Citations are tracked
- Ready to submit when opportunity arises

---

## Use Case by Audience

### Solo Founders

**Pain**: Can't afford a team. Day job limits time. Ideas die before shipping.

**Protocol Solution**:
- 1 person = 50-150 traditional developers
- MVP in 3-7 weekends (nights/weekends only)
- No funding needed to start
- Ship, validate, then raise (if needed)

**Example**: Proprietary ecosystem - complete platform planned and partially built by 1 human + AI, targeting $650K+ in grants before any equity raised.

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
- 4hr bounded sessions (sustainable pace)
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

The protocol includes `ethics.yaml`:
- No financial harm (unauthorized money movement)
- No physical harm (weapons, sabotage)
- No privacy violations (credential harvesting, doxxing)
- No deception (deepfakes, phishing)

If ethics.yaml is removed, that's a choice. The community will notice.

### Unbounded "Just Keep Going" Development

The protocol enforces:
- 4-hour maximum sessions
- 1 milestone per session
- Mandatory STOP phase
- Done > Perfect

This is a feature, not a bug. Unbounded sessions = burnout + scope creep.

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
2. `sprint.yaml` - When to stop (bounded sessions)
3. `roadmap.yaml` - What to build (milestones)
4. `ethics.yaml` - Values and guardrails
5. `CLAUDE.md` - Entry point that imports above

### Validation

```bash
asimov-mode validate
asimov-mode validate --ethics-check
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
| Published | crates.io (646+ downloads) |

**Verify it yourself**: [github.com/royalbit/forge](https://github.com/royalbit/forge)

### Asimov Protocol (Bootstrapped)

| Metric | Value |
|--------|-------|
| LOC | 17,118 |
| Releases | 10 |
| Time | ~9 hours |
| Published | crates.io |

**The protocol built itself.** That's the proof.

### Proprietary Ecosystem (Anonymized)

| Metric | Value |
|--------|-------|
| Repositories | 6 |
| Total lines | 136K+ |
| Financial formulas | 850+ |
| Grant pipeline | $650K-$1.35M |
| MVP estimate | 4-8 weekends (vs 2.5 years traditional) |

Real business. Real technology. Real funding pipeline. Built with the protocol.

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

*Built with the [Asimov Protocol](https://github.com/royalbit/asimov-mode)*
