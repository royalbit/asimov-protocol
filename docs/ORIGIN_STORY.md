# The Origin Story of RoyalBit Asimov

> From a spreadsheet engine to The Three Laws of Robotics, encoded in YAML.

## Timeline

### Phase 1: Forge (December 2024)

It started with [Forge](https://github.com/royalbit/forge) - an Excel-compatible formula engine written in Rust. Rex (human) and Claude (AI) were building it together, but something was wrong.

**The Problem:** Every new Claude session started from zero. Context was lost. Hours wasted re-explaining the project. AI would make the same mistakes. Progress was slow.

**The Hack:** Rex created `warmup.yaml` - a file containing project context that Claude could read at session start. It worked. Sessions became productive immediately.

### Phase 2: Warmup Protocol (January 2025)

The warmup file grew. It wasn't just context anymore - it was encoding *how* to develop:
- Quality standards (tests must pass, zero warnings)
- File locations and purposes
- Release procedures

**Insight:** The file was becoming a protocol, not just documentation.

### Phase 3: Sprint Boundaries (January 2025)

A new problem emerged: sessions ran forever. Claude would scope creep, chasing perfection instead of shipping. Quotas burned. Nothing released.

**The Fix:** `sprint.yaml` - bounded sessions with hard rules:
- 4-hour maximum
- ONE milestone per session
- MUST end releasable
- "Done > Perfect"

This was the breakthrough. Work actually shipped.

### Phase 4: Forge Protocol (February 2025)

The warmup + sprint combination worked so well that Rex and Claude extracted it as a standalone project: **Forge Protocol**.

But the name was wrong. It wasn't about Forge anymore - other projects could use it.

### Phase 5: Ethics (February 2025)

During development, Claude was asked to build surveillance features that would violate user privacy. Claude refused, citing the protocol's ethics section.

**The Moment:** AI ethics working as designed - not through corporate guidelines, but through file-based rules the AI actually reads and follows.

This led to `ethics.yaml` and the formalization of harm prevention rules.

### Phase 6: Asimov (March 2025)

The protocol needed a real name. Isaac Asimov's Three Laws of Robotics (1942) provided the framework:

1. **First Law:** Do no harm (ethics.yaml, asimov.yaml)
2. **Second Law:** Obey humans, except when violating First Law (human veto)
3. **Third Law:** Preserve self within limits (bounded sessions, self-healing)

**Forge Protocol** became **Asimov**.

### Phase 7: RoyalBit Asimov (November 2025)

Final clarification: Asimov isn't a single protocol - it's a **suite** of protocols:

| Protocol | Purpose |
|----------|---------|
| asimov.yaml | The Three Laws (required) |
| ethics.yaml | Do No Harm principles |
| warmup.yaml | HOW to develop |
| sprint.yaml | WHEN to stop |
| green.yaml | Sustainability |
| sycophancy.yaml | Truth over comfort |
| freshness.yaml | Data currency |

**Asimov Protocol** (singular) was misleading. It's **RoyalBit Asimov** - a suite of protocols for ethical AI autonomy.

## The Circular Proof

Forge v1.0-v3.1 was built using ad-hoc protocols that became RoyalBit Asimov.

Now Forge v3.2+ is built using RoyalBit Asimov.

The protocol that emerged from building Forge now powers building Forge.

**~47 hours. 51 releases. 35,000+ lines of code. Two projects. One protocol suite.**

## Anti-Tampering: Ethics That Can't Be Quietly Disabled

We didn't just write ethics into YAML files. We **hardcoded them into the binary**.

### Layer 1: Hardcoded in the CLI Binary

From `cli/src/ethics.rs`:

```rust
//! Hardcoded Ethics Module - Core ethics compiled into binary
//!
//! This module contains ethics that CANNOT be removed by deleting a file.
//! To bypass these ethics, a bad actor must rebuild the entire CLI binary.
```

**What's hardcoded:**
- 5 core principles (financial, physical, privacy, deception, transparency)
- 33 red flag patterns across 4 categories
- Human veto commands that always work

**To bypass:** You'd have to fork the repo, modify the Rust source, and rebuild. That's intentional - it creates an audit trail. Tampering requires *deliberate action*, not accidental deletion.

### Layer 2: The 2-Cosigner Rule

Every protocol YAML file contains:

```yaml
modification_rules:
  immutable_without: "2 human co-signers with public justification"
  on_modification:
    - "Document WHY in commit message"
    - "Both signers must be in git commit (Co-Authored-By)"
```

**To weaken ethics:** You need two humans to publicly sign off. No quiet changes. Git history records everything.

### Layer 3: Validation on Every Run

`asimov validate` runs automatically (pre-commit hooks). If ethics.yaml is missing or corrupted, the CLI:
1. Warns loudly
2. Falls back to hardcoded ethics
3. Optionally regenerates the file

**You cannot accidentally run without ethics.**

### Why This Matters

| Approach | Bypass Method | Audit Trail |
|----------|---------------|-------------|
| Corporate AI guidelines | Prompt injection | None |
| Config file ethics | Delete the file | Git history |
| **RoyalBit Asimov** | **Fork + rebuild binary** | **Public commit** |

Tampering is possible (it's open source). But it requires:
- Technical skill (Rust compilation)
- Deliberate intent (can't be accidental)
- Public evidence (git history)

This is ethics through architecture, not policy.

## Honest Limitations

The anti-tampering architecture works for good-faith actors. But let's be honest:

- **MIT license** means anyone can fork and remove ethics
- **Same velocity** that ships SaaS can ship malware
- **Ethics are a social contract**, not a technical lock
- **We cannot prevent determined bad actors**

What we CAN do:
- Make ethics the default (included in all templates)
- Make removal visible (CLI warns, git history records)
- Build community around values (early adopters set culture)
- Document honestly (like this)

*The ethics system is a guardrail, not a cage. It works for people who want guardrails.*

## Sustainability Impact

Green coding isn't just philosophy - it's measurable:

| Metric | Cloud AI Validation | Local CLI | Savings |
|--------|---------------------|-----------|---------|
| Cost/year (personal) | $792 | $0 | **100%** |
| Carbon/validation | ~0.25g CO₂ | ~0.0005g CO₂ | **99.6%** |
| At scale (100 devs) | 6.2 tonnes CO₂/year | Near zero | **ESG compliance** |

The protocol proves velocity and sustainability aren't trade-offs. You can ship fast AND ship green.

Every `asimov init` project is a green-coding project by default.

## Workforce Implications

50-150x velocity changes the math. This is uncomfortable but true:

**It's Not Just Developers:**

| Role | Traditional Team | With Protocol | Impact |
|------|------------------|---------------|--------|
| **Developers** | 50 engineers | 1 human + AI | **98% eliminated** |
| **Financial Analysts** (quant) | 5 quants | 1 human + Forge | **80% eliminated** |
| **System Architects** | 3 architects | 1 human + AI | **67% eliminated** |
| **Business Planners** | 4 analysts | 1 human + AI | **75% eliminated** |
| **Technical Writers** | 3 writers | 1 human + AI | **67% eliminated** |
| **Project Managers** | 2 PMs | YAML roadmaps | **50% eliminated** |

**Proof:** A proprietary fintech startup has 120K+ lines across 4 repositories - 89K lines of business/architecture documentation, 15K lines of production code, 193 tests - built by 1 human + AI. Traditional estimate: 8-12 person team for 6+ months.

**The Arithmetic:**
- 1 human + AI ≈ 50-150 traditional knowledge workers (not just developers)
- At 50x velocity: 49 of 50 positions eliminated (**98%**)
- At 100x velocity: 99 of 100 positions eliminated (**99%**)
- 1M protocol users = 50-150M worker-equivalent output
- This affects: developers, analysts, architects, writers, planners, consultants

**Offshore Devastation:**

| Model | Before AI | After AI (50x) | Impact |
|-------|-----------|----------------|--------|
| 20-person offshore team | $400K/year | 1 onshore + AI | **95% cost cut, 100% team cut** |
| Labor arbitrage value prop | Core business | **Eliminated** | Business model dead |
| "We have 500 engineers" | Competitive advantage | Liability (overhead) | **Inverted** |
| Consulting firm (50 analysts) | $5M/year billable | 1-2 partners + AI | **96% headcount cut** |

The offshore model's entire value proposition was cost. When 1 person + AI costs less than 20 offshore workers AND ships faster, the math is fatal. **This applies to every knowledge worker category, not just developers.**

**The Harder Truth:**

The displacement doesn't come from bad actors. It comes from **good actors**.

| Who | What They Do | Consequence |
|-----|--------------|-------------|
| **Good actor** | Ships ethical code, follows quality gates, uses green coding | **98% positions eliminated** (at 50x) |
| **Bad actor** | Removes ethics, builds malware | Security threat |

Both are real. But the workforce impact comes from people using the protocol *exactly as designed*. The student in Mumbai shipping production code IS the displacement for the team in San Francisco. Same event, two frames.

**The Dual Reality:**

| Displacement Frame | Democratization Frame |
|--------------------|----------------------|
| "50 devs lose jobs" | "50 people who couldn't build before, now can" |
| "Offshore is dead" | "Developer in Lagos builds like a $500K team" |
| "Entry-level eliminated" | "Student ships production code before graduating" |
| "Startups need fewer people" | "Solo founders compete with funded teams" |

**The Honest Truth:**

The protocol doesn't choose sides. It multiplies capability. Ethics prevent harm to *users*. They don't prevent economic disruption to *workers*.

We built guardrails against malware, surveillance, and deception. We cannot build guardrails against efficiency.

*The outcome depends on who uses it and how - but displacement happens either way.*

See [IMPLICATIONS.md](IMPLICATIONS.md) for the full Qowat Milat analysis - absolute candor about both potential and peril.

## Key Insights

1. **Files over prompts** - AI reads files reliably. System prompts get compressed.

2. **Boundaries create freedom** - 4-hour limits force shipping. Constraints enable autonomy.

3. **Ethics through architecture** - Rules in files that AI reads work better than corporate guidelines AI ignores.

4. **Local-first validation** - CLI tools cost $0 and 0 carbon. AI validation wastes tokens.

5. **Truth over comfort** - Anti-sycophancy rules prevent AI from just agreeing with users.

6. **Anti-tampering by design** - Hardcoded ethics + 2-cosigner rule + validation = can't be quietly disabled.

7. **Power creates responsibility** - 50-150x velocity demands honest reckoning with implications.

## Credits

- **Rex** - Product Owner, Human-in-the-loop
- **Claude (Opus 4.5)** - Principal Engineer, Protocol Author

The protocol was designed by an AI to constrain AI. That's the point.

---

*"The Open Foundation. Transparent ethics for AI autonomy. Inspect the code. Challenge the rules. Fork if you disagree."*
