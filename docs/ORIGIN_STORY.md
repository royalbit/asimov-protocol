# The Origin Story of RoyalBit Asimov

> From a spreadsheet engine to a protocol that creates Self-Evolving Autonomous AI projects with ethics built in.

## Timeline

Nine phases of evolution from November 23 to December 4, 2025.

### Phase 1: Forge (November 23, 2025)

It started with [Forge](https://github.com/royalbit/forge) - an Excel-compatible formula engine written in Rust. Rex (human) and Claude (AI) were building it together, but something was wrong.

**The Problem:** Every new Claude session started from zero. Context was lost. Hours wasted re-explaining the project. AI would make the same mistakes. Progress was slow.

**The Hack:** Rex created `warmup.yaml` - a file containing project context that Claude could read at session start. It worked. Sessions became productive immediately.

### Phase 2: Warmup Protocol (November 23-24, 2025)

The warmup file grew. It wasn't just context anymore - it was encoding *how* to develop:
- Quality standards (tests must pass, zero warnings)
- File locations and purposes
- Release procedures

**Insight:** The file was becoming a protocol, not just documentation.

### Phase 3: Sprint Boundaries (November 24-25, 2025)

A new problem emerged: sessions ran forever. Claude would scope creep, chasing perfection instead of shipping.

**The Fix:** `sprint.yaml` - bounded sessions with hard rules:
- ONE milestone per session
- MUST end releasable
- "Done > Perfect"

This was the breakthrough. Work actually shipped.

### Phase 4: Forge Protocol (November 25, 2025)

The warmup + sprint combination worked so well that Rex and Claude extracted it as a standalone project: **Forge Protocol**.

But the name was wrong. It wasn't about Forge anymore - other projects could use it.

### Phase 5: Ethics (November 25-26, 2025)

During development, Claude was asked to build surveillance features that would violate user privacy. Claude refused, citing the protocol's ethics section.

**The Moment:** AI ethics working as designed - not through corporate guidelines, but through file-based rules the AI actually reads and follows.

This led to ethics rules (now `asimov.yaml`) and the formalization of harm prevention.

### Phase 6: Asimov (November 26-27, 2025)

The protocol needed a real name. Isaac Asimov's Three Laws of Robotics (1942) provided the framework:

1. **First Law:** Do no harm (asimov.yaml)
2. **Second Law:** Obey humans, except when violating First Law (human veto)
3. **Third Law:** Preserve self within limits (bounded sessions, self-healing)

**Forge Protocol** became **Asimov**.

### Phase 7: RoyalBit Asimov (November 27-28, 2025)

Final clarification: Asimov isn't a single protocol - it's a **suite** of protocols:

| Protocol | Purpose |
|----------|---------|
| asimov.yaml | The Three Laws (required) |
| warmup.yaml | HOW to develop |
| sprint.yaml | WHEN to stop |
| green.yaml | Sustainability |
| sycophancy.yaml | Truth over comfort |
| freshness.yaml | Data currency |

**"The protocol"** (singular) was misleading. It's **RoyalBit Asimov** - a protocol that creates Self-Evolving Autonomous AI projects with ethics built in.

### Phase 8: The Inaction Principle (November 28-29, 2025)

A critical realization: Asimov's First Law has **two halves**:

> *"A robot may not injure a human being **or, through inaction, allow a human being to come to harm**."*

Everyone implements the first half. We implemented both. Active harm means don't build harmful tools. Inaction means don't stay silent when you know better—like disclosing when data might be stale.

This became explicit in `asimov.yaml` with the Five Non-Negotiable Principles. See [ADR-023: The Inaction Principle](adr/023-inaction-principle.md) for full rationale.

### Phase 9: Self-Evolving Autonomous AI (November 29-30, 2025)

The final realization: RoyalBit Asimov combines **two distinct AI frontiers**:

**Autonomous AI** works independently, makes decisions, and self-corrects under human oversight. This is what most enterprise AI initiatives are pursuing today.

**Self-Evolving AI** improves itself over time and modifies its own processes. Research literature describes this as the next frontier beyond autonomous agents.

RoyalBit Asimov combines both—with ethics built in through sprint autonomy, quality gates, bootstrapping, self-healing, and anti-tampering mechanisms.

## The Circular Proof

Forge was built using ad-hoc protocols that became RoyalBit Asimov. Now Forge is built using RoyalBit Asimov.

The protocol that emerged from building Forge now powers building Forge.

**This is self-evolving AI:**
- The protocol improved itself through each iteration
- Each session applies lessons from previous sessions
- The methodology compounds efficiency gains
- Forge birthed Asimov, and Asimov now builds Forge

## Anti-Tampering: Ethics Through Architecture

Ethics aren't just written into YAML files—they're **hardcoded into the binary**.

### Three Layers of Protection

**Layer 1: Hardcoded Ethics** - Core principles, red flag patterns, and human veto commands are compiled into `cli/src/ethics.rs`. To bypass them, you'd need to fork the repo, modify Rust source, and rebuild the binary—creating a deliberate audit trail.

**Layer 2: The 2-Cosigner Rule** - Every protocol YAML requires two human co-signers with public justification to modify. Git history records everything.

**Layer 3: Automatic Validation** - `asimov validate` runs on every commit. If `asimov.yaml` is missing or corrupted, the CLI warns loudly and falls back to hardcoded ethics.

Tampering is possible—it's open source—but it requires technical skill, deliberate intent, and leaves public evidence. This is ethics through architecture, not policy.

## Honest Limitations

The anti-tampering architecture works for good-faith actors. Ethics are a social contract, not a technical lock. We cannot prevent determined bad actors.

What we can do: make ethics the default, make removal visible, build community around values, and document honestly.

*The ethics system is a guardrail, not a cage. It works for people who want guardrails.*

## Sustainability Impact

Local-first validation eliminates cloud costs and reduces carbon footprint by orders of magnitude compared to AI-based validation. The protocol proves velocity and sustainability aren't trade-offs.

Every `asimov init` project is a green-coding project by default.

---

## Key Insights

1. **Files over prompts** - AI reads files reliably. System prompts get compressed.

2. **Boundaries create freedom** - Bounded sessions force shipping. Constraints enable autonomy.

3. **Ethics through architecture** - Rules in files that AI reads work better than corporate guidelines AI ignores.

4. **Local-first validation** - CLI tools eliminate cost and carbon waste.

5. **Truth over comfort** - Anti-sycophancy rules prevent AI from just agreeing with users.

6. **Anti-tampering by design** - Hardcoded ethics, 2-cosigner rule, and validation prevent quiet removal.

7. **Self-evolving by design** - The protocol improves itself through use.

---

## Credits

- **Rex** - Product Owner, Human-in-the-loop
- **Claude (Opus 4.5)** - Principal Autonomous AI, Protocol Author

The protocol was designed by an AI to constrain AI. That's the point.

---

*"Creates Self-Evolving Autonomous AI projects with ethics built in. Inspect the code. Challenge the rules."*

---
