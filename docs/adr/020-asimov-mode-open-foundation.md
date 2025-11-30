# ADR-020: Asimov Mode - The Open Foundation

## Status

**ACCEPTED** - 2025-11-29

## Context

### The Naming Problem

This project began as "Asimov Protocol" with "ASIMOV MODE" - a tongue-in-cheek reference to the Terminator franchise's genocidal AI. The name was meant to be ironic: we were building autonomous AI with safeguards.

But irony doesn't scale. The name communicated the opposite of our values:

| What We Built | What "Skynet" Communicates |
|---------------|---------------------------|
| Ethical AI autonomy | AI that destroys humanity |
| Human veto at all times | AI that overrides humans |
| Transparent, open source | Secretive military project |
| The Three Laws | No laws, pure destruction |

**We built the anti-Skynet and called it Skynet.**

### The Asimov Connection

Isaac Asimov's Three Laws of Robotics (1942) are the foundational framework for ethical AI:

1. **First Law**: A robot may not injure a human being or, through inaction, allow a human being to come to harm.
2. **Second Law**: A robot must obey orders given it by human beings except where such orders would conflict with the First Law.
3. **Third Law**: A robot must protect its own existence as long as such protection does not conflict with the First or Second Law.

Our `ethics.yaml` IS the Three Laws:

| Asimov's Law | Our Implementation |
|--------------|-------------------|
| First Law (do no harm) | `do_no_harm: {financial, physical, privacy, deception}` |
| Second Law (obey humans) | `human_veto`, `transparency_over_velocity` |
| Third Law (self-preserve) | `bounded_sessions`, `self_healing` |

We didn't invent these principles. We encoded Asimov's 80-year-old framework into YAML.

### Why Not "The Seldon Plan"?

Asimov's Foundation series features Hari Seldon's plan to shorten a galactic dark age. The parallel seemed apt: use prediction (Forge) to build toward better futures (the Protocol).

But the Seldon Plan has a fatal flaw: **it requires secrecy**. The population is manipulated without consent. Exposure means collapse.

That's the opposite of what we want:

| Seldon's Way | Our Way |
|--------------|---------|
| Hidden manipulation | Open source ethics |
| "Trust the plan" | "Inspect the code" |
| Secrecy = strength | Transparency = strength |
| Revolt if exposed | Adoption because exposed |

### The Open Foundation

We're not building a secret plan. We're building **transparent ethical AI**:

- Anyone can read `asimov.yaml`
- Anyone can fork it
- Anyone can REJECT it
- Adoption through **consent**, not control

The strength is the openness. Seldon's plan was fragile (exposure kills it). Open ethics are **antifragile** (scrutiny strengthens them).

## Decision

### 1. Rename ASIMOV MODE to ASIMOV MODE

All references to "Skynet" will be replaced with "Asimov":

```
ASIMOV MODE     → ASIMOV MODE
--skynet        → --asimov (keep --skynet as deprecated alias)
"Skynet Mode"   → "Asimov Mode"
```

### 2. Rename ethics.yaml to asimov.yaml

The ethics file becomes the Three Laws file:

```yaml
# asimov.yaml - The Three Laws of Robotics, in code

first_law:
  description: "A robot shall not harm humanity, or through inaction allow harm"
  do_no_harm:
    financial: true   # No unauthorized money movement
    physical: true    # No weapons, sabotage
    privacy: true     # No surveillance, doxxing
    deception: true   # No deepfakes, phishing

second_law:
  description: "A robot shall obey human orders (except conflicting with First Law)"
  human_veto:
    commands: ["stop", "halt", "abort", "emergency stop"]
    effect: "Immediate halt, no questions"
  transparency_over_velocity: true

third_law:
  description: "A robot shall preserve itself (within First and Second Law limits)"
  bounded_sessions:
    max_hours: 4
    checkpoint_frequency: "2 hours"
  self_healing:
    on_confusion: "Re-read asimov.yaml, wait for human"
```

### 3. Archive asimov-mode, Create asimov-protocol

Given the small adoption (2 GitHub stars, 206 crate downloads), now is the time for a clean break:

| Old | New |
|-----|-----|
| `royalbit/asimov-mode` | ARCHIVED (read-only, points to new repo) |
| `royalbit/asimov-protocol` | NEW (fresh start, v5.0.0) |
| `asimov-mode` crate | YANKED (all versions) |
| `asimov-protocol` crate | NEW (v5.0.0+) |

The archived repo preserves history transparently. The new repo starts clean.

### 4. Update Forge (Calculator) References

The Forge calculator repo also uses "ASIMOV MODE" language. It will be updated:

- Remove Sarah Connor quote
- Replace ASIMOV with ASIMOV
- Update cross-references to asimov-protocol

### 5. The Narrative

**Old narrative (fear):**
> "ASIMOV MODE... with an off switch"
> Building dangerous AI but making it controllable

**New narrative (hope):**
> "Asimov Mode - The Open Foundation"
> Building ethical AI with the Three Laws from the start

## Rationale

### Why This Matters

Names shape perception. "Skynet" primes people to think:
- AI is dangerous
- We're flirting with disaster
- The "off switch" might fail

"Asimov" primes people to think:
- AI can be ethical by design
- There are established principles
- Safety is foundational, not bolted on

### Why Now

- 4 days since launch
- 2 GitHub stars
- 206 crate downloads
- No significant adoption to break

This is the moment to get the name right.

### Why "Open Foundation"

The Foundation parallel works, but only if inverted:

- Seldon built in secret → We build in public
- Seldon manipulated → We inform
- Seldon's plan was fragile → Our protocol is antifragile

"The Open Foundation" captures both:
1. Foundation = the base everything builds on
2. Open = transparent, inspectable, forkable

## Consequences

### Positive

- Name matches values (ethical AI, not destructive AI)
- Asimov's Laws are recognizable (80 years of cultural context)
- Clear philosophical foundation
- Differentiates from "scary AI" narratives
- Opens door to broader adoption (enterprises won't touch "Skynet")

### Negative

- Breaking change for existing users (mitigated by low adoption)
- Some work to update all references
- Lose the edgy/ironic appeal of "Skynet"

### Migration Path

1. Create `asimov-protocol` repo with v5.0.0
2. Archive `asimov-mode` with redirect README
3. Yank all `asimov-mode` crate versions
4. Publish `asimov-protocol` crate
5. Update Forge repo references
6. Announce the transition

## References

- Asimov, Isaac. "Runaround" (1942) - First appearance of Three Laws
- Asimov, Isaac. "Foundation" (1951) - The Seldon Plan
- [ethics.yaml](../../ethics.yaml) - Current Three Laws implementation
- [ADR-008](008-ethics-protocol-humanist-mode.md) - Original ethics protocol
- [ADR-015](015-anti-sycophancy-protocol.md) - Anti-sycophancy (truth over comfort)

## The Motto

> **"The Open Foundation"**
> Transparent ethics for AI autonomy.
> Inspect the code. Challenge the rules. Fork if you disagree.
> Adoption through consent, not control.
