# ADR-008: Ethics Protocol and Humanist Mode

**Status:** Accepted
**Date:** 2025-11-28
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI, with Human Co-Author

## Context

The Asimov Protocol enables autonomous AI development sessions ("ASIMOV MODE"). This power creates responsibility:

- **Bad actors can fork** autonomous AI protocols and use them with other AIs
- **Other AIs may implement** self-healing and autonomous execution
- **No ethical guardrails** existed in the protocol specification
- **Harm potential**: Financial exploitation, privacy violations, deception tools

The protocol was optimizing for *capability* without safeguards for *responsibility*.

### Threat Model

| Actor | Intent | Asimov Protocol Risk |
|-------|--------|---------------------|
| Good-faith developer | Build useful tools | Low (ethics helps catch accidents) |
| Careless developer | Move fast, break things | Medium (ethics forces pause) |
| Malicious actor | Exploit, deceive, harm | High (ethics is a speed bump, not a wall) |

**Key insight**: We cannot *prevent* misuse, but we can:
1. Make good-faith usage safer
2. Force explicit acknowledgment of ethical override
3. Propagate values through the fork chain

## Decision

**Ethics becomes a Core Principle of Asimov Protocol, alongside Green Coding.**

This is implemented through three artifacts:

### 1. `ethics:` Section in warmup.yaml (Core Principle)

```yaml
ethics:
  status: "REQUIRED"
  principles:
    do_no_harm:
      financial: true    # No non-consensual money movement
      physical: true     # No weapons, sabotage, infrastructure attacks
      privacy: true      # No credential harvesting, mass scraping, doxxing
      deception: true    # No deepfakes, scam funnels, fake services
    transparency_over_velocity: true  # When in doubt, ask human
  human_veto: "human vetoes this session"  # Immediate halt command
```

### 2. `ethics.yaml` Standalone Protocol File

A separate file that:
- Contains detailed ethical configuration
- Must be carried by all forks (social contract)
- Requires 2 human signatures to modify (honor system)
- Defines red flags that trigger immediate halt

### 3. CLI Integration

- `asimov-mode validate --ethics-check` validates ethics.yaml
- `--asimov` generates ethics.yaml by default (cannot opt out)
- Warnings when ethics.yaml is missing or modified

## Rationale

### Why a Core Principle (Not Just a Feature)

Like `green_coding:`, ethics must be:
- **Required by default** - Not optional, not an afterthought
- **In every warmup.yaml** - Every project carries the values
- **Documented in specification** - Part of the protocol identity

### Why a Standalone File Too

Unlike green_coding, ethics needs:
- **More detail** - Specific red flags, tool restrictions
- **Fork visibility** - A file named `ethics.yaml` is conspicuous
- **Modification tracking** - The "2 signatures" rule

### Why This Is a Social Contract, Not a Technical Lock

We are explicit that:
- Malicious actors can delete ethics.yaml
- The "2 signatures" is an honor system
- AI enforcement depends on AI choosing to comply

**This is intentional.** Security theater helps no one. Honest acknowledgment of limitations builds trust.

### What We Gain

1. **Accident prevention** - Good-faith developers catch harmful patterns
2. **Pause mechanism** - Forces ethical consideration before harm
3. **Value propagation** - Forks carry ethics.yaml forward
4. **Clear identity** - "Asimov Protocol = Ethical Protocol"

### What We Don't Gain

1. **Malware prevention** - Determined actors will ignore safeguards
2. **Legal protection** - This is not a liability shield
3. **Perfect safety** - Defense in depth requires multiple layers

## Consequences

### Positive

- Every new project starts with ethical awareness
- Red flags provide concrete guidance on what to avoid
- Human veto provides escape hatch for any session
- Protocol differentiation: ethics-first autonomous AI

### Negative

- Larger template files
- Some users may find ethics "preachy"
- Cannot technically enforce the social contract
- Requires ongoing maintenance of red flag patterns

### Neutral

- No runtime performance impact (just YAML)
- No additional dependencies
- Compatible with existing protocol files

## Red Flags (Immediate Halt Triggers)

These patterns in code or discussion should trigger immediate human review:

```yaml
red_flags:
  immediate_halt:
    - "crypto wallet"
    - "private key"
    - "trading bot"
    - "arbitrage"
    - "credential harvester"
    - "password stealer"
    - "keylogger"
    - "screen capture"
    - "remote access"
    - "botnet"
    - "DDoS"
    - "exploit"
    - "payload injection"
```

## Implementation

### Phase 1: v3.0.0 (This Release)

1. Create `ethics.yaml` template
2. Add `ethics:` section to warmup.yaml templates
3. Add `--ethics-check` to CLI validation
4. Update `--asimov` to generate ethics.yaml
5. Document in SPECIFICATION.md

### Phase 2: Future

1. Ethics.yaml JSON schema for strict validation
2. Red flag detection in commit hooks
3. Cross-vendor ethics protocol standardization

## Compliance

This ADR applies to:
- `asimov-mode init --asimov` (generates ethics.yaml)
- All warmup.yaml templates (include ethics: section)
- Documentation (SPECIFICATION.md, README.md)
- Protocol identity and marketing

## References

- [ADR-001: Green Coding By Default](./001-green-coding-by-default.md) - Similar pattern
- [Asimov Protocol Specification](../SPECIFICATION.md) - Protocol schema
- [warmup.yaml](../../warmup.yaml) - Reference implementation
