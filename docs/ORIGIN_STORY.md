# The Origin Story of RoyalBit Asimov

From a spreadsheet engine to a protocol that creates Self-Evolving Autonomous AI projects with ethics built in.

## Timeline

Nine phases, November 23 to December 4, 2025.

### Phase 1: Forge (November 23, 2025)

It started with [Forge](https://github.com/royalbit/forge-demo) (forge not public) - an Excel-compatible formula engine written in Rust. Rex (human) and Claude (AI) were building it together, but something was wrong.

**The Problem:** Every new Claude session started from zero. Context was lost. Hours wasted re-explaining the project. AI would make the same mistakes. Progress was slow.

**The Hack:** Rex created `warmup.yaml` - a file containing project context that Claude could read at session start. It worked. Sessions became productive immediately.

### Phase 2: Warmup Protocol (November 23-24, 2025)

The warmup file grew beyond context. It was encoding *how* to develop: quality standards, file locations, release procedures.

**Insight:** The file was becoming a protocol.

### Phase 3: Sprint Boundaries (November 24-25, 2025)

A new problem emerged: sessions ran forever. Claude would scope creep, chasing perfection instead of shipping.

**The Fix:** `sprint.yaml` - bounded sessions with hard rules:
- ONE milestone per session
- MUST end releasable
- "Perfect > Done, no sloppy code"

This was the breakthrough. Work actually shipped.

### Phase 4: Forge Protocol (November 25, 2025)

The warmup + sprint combination worked so well it became a standalone project: **Forge Protocol**.

But the name was wrong. It wasn't about Forge anymore.

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

Asimov isn't a single protocol - it's a suite:

| Protocol | Purpose |
|----------|---------|
| asimov.yaml | The Three Laws |
| warmup.yaml | HOW to develop |
| sprint.yaml | WHEN to stop |
| green.yaml | Sustainability |
| sycophancy.yaml | Truth over comfort |
| freshness.yaml | Data currency |

### Phase 8: The Inaction Principle (November 28-29, 2025)

Asimov's First Law has two halves:

> *"A robot may not injure a human being **or, through inaction, allow a human being to come to harm**."*

Everyone implements the first half. We implemented both. Active harm: don't build harmful tools. Inaction: don't stay silent when you know better.

### Phase 9: Self-Evolving Autonomous AI (November 29-30, 2025)

RoyalBit Asimov combines two AI frontiers:

**Autonomous AI**: Works independently, makes decisions, self-corrects under human oversight. Current enterprise focus.

**Self-Evolving AI**: Improves itself over time, modifies its own processes. Next frontier beyond autonomous agents.

RoyalBit Asimov combines both with ethics built in through sprint autonomy, quality gates, bootstrapping, self-healing, and anti-tampering.

## The Circular Proof

Forge was built using ad-hoc protocols that became RoyalBit Asimov. Now Forge is built using RoyalBit Asimov.

The protocol that emerged from building Forge now powers building Forge.

**This is self-evolving AI:**
- The protocol improved itself through each iteration
- Each session applies lessons from previous sessions
- The methodology compounds efficiency gains
- Forge birthed Asimov, and Asimov now builds Forge

## Anti-Tampering: Ethics Through Architecture

Ethics aren't just in YAML files—they're hardcoded into the binary.

### Three Layers

**Layer 1: Hardcoded Ethics** - Core principles compiled into `cli/src/ethics.rs`. Bypassing requires forking, modifying source, and rebuilding—creating an audit trail.

**Layer 2: 2-Cosigner Rule** - Protocol YAML modifications require two human co-signers with public justification. Git records everything.

**Layer 3: Automatic Validation** - `asimov validate` runs on every commit. Missing or corrupted `asimov.yaml` triggers warnings and fallback to hardcoded ethics.

Tampering is possible (open source) but requires skill, intent, and leaves evidence. Ethics through architecture, not policy.

## Honest Limitations

The anti-tampering architecture works for good-faith actors. Ethics are a social contract, not a technical lock.

What we can do: make ethics the default, make removal visible, build community around values, document honestly.

## Sustainability Impact

Local-first validation eliminates cloud costs and reduces carbon footprint by orders of magnitude. Velocity and sustainability aren't trade-offs.

Every `asimov init` project is green by default.

---

## Key Insights

1. **Files over prompts** - AI reads files reliably. System prompts get compressed.
2. **Boundaries create freedom** - Bounded sessions force shipping.
3. **Ethics through architecture** - Rules in files work better than corporate guidelines.
4. **Local-first validation** - CLI tools eliminate cost and carbon waste.
5. **Truth over comfort** - Anti-sycophancy prevents blind agreement.
6. **Anti-tampering by design** - Hardcoded ethics, 2-cosigner rule, validation.
7. **Self-evolving by design** - The protocol improves through use.

---

## Credits

- **Rex** - Product Owner, Human-in-the-loop
- **Claude (Opus 4.5)** - Principal Autonomous AI, Protocol Author

The protocol was designed by an AI to constrain AI. That's the point.

---

*"Creates Self-Evolving Autonomous AI projects with ethics built in. Inspect the code. Challenge the rules."*

---
