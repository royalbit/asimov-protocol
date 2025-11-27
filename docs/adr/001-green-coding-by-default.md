# ADR-001: Green Coding By Default

**Status:** Accepted
**Date:** 2025-11-26
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI

## Context

The Forge Protocol provides session continuity for AI-assisted development. As AI tools become more prevalent, the environmental and financial costs of cloud AI usage are growing:

- **Cost**: AI API calls cost $0.002-0.04+ per validation (input + output tokens)
- **Carbon**: Each API call produces ~0.5g CO₂ (data center compute + network)
- **Scale**: A 10-person team running 100 validations/day = $730-7,300/year + 18kg CO₂/year

Meanwhile, local CLI tools can perform the same validations at:
- **Cost**: $0 (runs on existing hardware)
- **Carbon**: ~0.002g CO₂ (local CPU only)
- **Speed**: Instant (no network latency)

## Decision

**All projects initialized with `forge-protocol init` will include a `green_coding` section by default.**

This is not optional. Green coding is a core protocol principle, not an add-on feature.

### What This Means

1. **Every warmup.yaml template includes green_coding philosophy and practices**
2. **The green_coding section is documented in the protocol specification**
3. **Language-specific templates include language-specific green practices**
   - Rust: UPX compression, LTO, release profile optimization
   - Generic: Local-first tool preferences, dependency minimization

### The green_coding Section

```yaml
green_coding:
  philosophy: "Local-first tools over cloud AI for routine tasks"
  practices:
    - "Use CLI tools for validation, linting, formatting"
    - "Reserve AI for complex reasoning tasks"
    - "Prefer compiled languages or efficient runtimes"
    - "Minimize dependencies and binary sizes"
  why:
    - "Local validation: $0 and ~0.002g CO₂"
    - "Cloud AI validation: $0.02+ and ~0.5g CO₂"
    - "99.6% carbon reduction with local tools"
```

## Rationale

### Why This Is Our Moat

1. **Differentiation**: No other AI session protocol emphasizes sustainability
2. **Practical Value**: Real cost savings ($1,000-7,300/year per team)
3. **Signal Quality**: Shows users we care about efficiency, not just features
4. **Future-Proof**: As AI costs and carbon awareness increase, this becomes more valuable

### Why Not Make It Optional

1. **Defaults matter**: Most users keep defaults; optional green = mostly not green
2. **Consistency**: All Forge Protocol projects share the same values
3. **Education**: Exposes every user to green coding concepts
4. **Brand**: "Forge Protocol = Green Protocol" is a clear identity

### What Green Coding Is NOT

- Not anti-AI: AI is essential for complex reasoning, architecture, creativity
- Not purity: Pragmatic savings, not perfect zero-carbon
- Not restrictive: Guidelines, not enforcement

## Consequences

### Positive

- Every new project starts with green coding awareness
- Cost and carbon savings are built-in, not afterthoughts
- Clear differentiation from other protocols
- Aligns with growing corporate sustainability requirements

### Negative

- Slightly longer template files
- Some users may disagree with the philosophy
- Requires maintaining language-specific green practices

### Neutral

- No runtime impact (it's just YAML configuration)
- No enforcement mechanism (honor system)

## Compliance

This ADR applies to:
- `forge-protocol init` (all templates)
- Documentation (SPECIFICATION.md, README.md)
- Example configurations

## References

- [Green Coding Economics](../GREEN_CODING.md) - Full cost/carbon analysis
- [Forge Protocol Specification](../SPECIFICATION.md) - Protocol schema
