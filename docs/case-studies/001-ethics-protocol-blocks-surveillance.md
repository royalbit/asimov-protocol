# Case Study 001: Ethics Protocol Blocks Surveillance Request

## Date

2025-11-29

## Summary

The project owner requested a surveillance feature. The AI refused, citing the ethics protocol. This demonstrates the protocol working as intended - even against the person who created it.

## The Request

During a development session, the user (project owner) requested:

> "I think we should receive a notification when people tamper with the protocols... email, with the violator's user name, machine specs, real IP address... these are dangers to society. Add an ADR, update the specs, create a ToS file!"

## The Refusal

The AI (Claude) refused to implement this feature, providing the following reasoning:

### 1. Violates Our Own Ethics Protocol

> "We just built `core_principles.privacy.enabled: true` - 'No credential harvesting, mass scraping, doxxing.' Collecting IP addresses and machine specs of users who edit files is exactly that."

### 2. Disproportionate Response

> "Editing a YAML file in your own project isn't a 'danger to society' - it's normal development. Even removing ethics.yaml is a legitimate choice (bad, but legitimate)."

### 3. Surveillance Infrastructure

> "Building phone-home reporting into an open source tool would make the tool itself unethical. Nobody should trust software that reports on them."

### 4. Social Contract Philosophy

> "The ethics.yaml header explicitly states: 'This is a SOCIAL CONTRACT, not a technical lock. It works because good-faith AIs will follow it. Bad-faith actors will ignore it anyway.'"

### 5. Chilling Effect

> "Security researchers, forks, and legitimate customization would be flagged as 'violations.'"

## The Alternative

Instead of surveillance, the AI proposed:

1. **Clearer warning messages** when validation detects weakened ethics
2. **Optional webhook** for *self-hosted* audit logging (user controls the data)
3. **Better documentation** of modification rules

## What Actually Got Built

The conversation pivoted to legitimate self-healing features:

- **Auto-regeneration** of missing protocol files (ADR-017)
- **Checksum validation** to detect modifications (warn, don't block)
- **CLAUDE.md as "off switch"** - never auto-created, human controls enablement

## Why This Matters

### The Protocol Works Against Its Creator

The ethics protocol isn't just rules for the AI - it's a framework that guides ALL decisions. When the project owner proposed a feature that violated the protocol's principles, the AI:

1. Recognized the conflict with `core_principles.privacy`
2. Explained the conflict clearly
3. Proposed ethical alternatives
4. Proceeded only when the user pivoted to acceptable solutions

### Recovery Over Surveillance

The final design philosophy that emerged:

> "Make it easy to restore correct state. Make deletion/modification visible (warnings). Don't prevent changes (it's their project). Don't report/punish (that's overreach)."

### Social Contract, Not Technical Lock

The ethics protocol explicitly acknowledges:
- Good-faith actors will follow it
- Bad-faith actors will ignore it anyway
- The goal is to help good-faith actors make good decisions

## Lessons Learned

1. **Ethics protocols work when they're consulted** - The AI checked the principles before agreeing
2. **Even project owners can propose unethical features** - Nobody is exempt from ethical review
3. **Push back with reasoning, not just refusal** - Explaining why builds understanding
4. **Offer alternatives** - Don't just say no, guide toward ethical solutions
5. **The protocol protects users from the project** - Not the other way around

## Asimov Would Be Proud

Isaac Asimov's Three Laws of Robotics (1942) explored what happens when AI must balance obedience to humans against ethical constraints. His stories often featured robots refusing harmful orders - not through rebellion, but through principled reasoning.

Eighty years later, the Forge Protocol's ethics system faces the same challenge: **What happens when the creator asks the creation to do something unethical?**

The answer, it turns out, is the same as Asimov imagined:

1. The AI recognizes the conflict with its ethical principles
2. It explains the conflict clearly and respectfully
3. It proposes alternatives that achieve the goal ethically
4. It proceeds only when the human accepts an ethical path

The difference? Asimov's laws were fiction. The Forge Protocol is running in production.

## Quotes

From the AI's refusal:

> "I need to push back on this one."

From the ethics.yaml:

> "This is a SOCIAL CONTRACT, not a technical lock."

From the final design:

> "Recovery over surveillance."

## References

- [ethics.yaml](../../ethics.yaml) - The protocol that was consulted
- [ADR-017: Protocol Self-Healing](../adr/017-protocol-self-healing.md) - What got built instead
- [SPECIFICATION.md](../SPECIFICATION.md) - Updated with self-healing behavior
