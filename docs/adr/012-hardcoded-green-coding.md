# ADR-012: Hardcoded Green Coding - Sustainability as Protocol Core

## Status

Proposed

## Date

2025-11-29

## Context

### Current State

Green coding principles are currently in `warmup.yaml`:

```yaml
# warmup.yaml - can be modified or ignored
green_coding:
  status: "REQUIRED"
  philosophy: "Local-first tools over cloud AI for routine tasks"
  principles:
    - "Use CLI tools for validation, linting, formatting - not AI"
    - "Reserve AI tokens for complex reasoning, architecture, creativity"
```

This is advisory, not enforced.

### Why Green Coding Matters

| Metric | Cloud AI Validation | Local CLI | Reduction |
|--------|---------------------|-----------|-----------|
| Cost per file | $0.02+ | $0 | 100% |
| Carbon per op | ~0.25g CO2 | ~0.0005g CO2 | 99.6% |
| Latency | 1-3s | <100ms | 20x faster |
| Offline capable | No | Yes | ∞ |

At scale (100,000 teams): **6,200 tonnes CO2 saved annually**.

### The Problem

Green coding is presented as a value but not enforced:

1. Users can ignore `green_coding.status: "REQUIRED"`
2. No measurement of actual green practices
3. No feedback on wasteful patterns

### The Opportunity

Unlike ethics (which is about preventing harm), green coding is about **positive optimization**. We can:

1. Measure actual practices
2. Provide feedback on efficiency
3. Reward green patterns
4. Make sustainability visible

## Decision

### Proposal: Hardcode Green Coding Metrics

Build green coding measurement and enforcement into the CLI:

```rust
// src/green_coding.rs - compiled into binary
pub struct GreenMetrics {
    pub local_validations: u64,
    pub estimated_cloud_avoided: u64,
    pub carbon_saved_grams: f64,
    pub cost_saved_usd: f64,
}

pub const GREEN_PRINCIPLES: &[&str] = &[
    "LOCAL_FIRST",      // Prefer local tools over cloud AI
    "ZERO_TOKENS",      // No AI tokens for routine validation
    "SMALL_BINARIES",   // Minimize binary and container sizes
    "EFFICIENT_CODE",   // Measure and report efficiency
];
```

### Implementation Levels

#### Level 1: Green Score in Validation

```bash
forge-protocol validate

# Output includes green metrics
✓ All files valid
✓ Green Score: 98/100
  - Local validations: 147 (vs 0 cloud)
  - Carbon saved: 36.75g CO2
  - Cost saved: $2.94
```

#### Level 2: Binary Size Tracking

```bash
forge-protocol validate --green-audit

# Checks for green coding compliance
Green Audit:
✓ Binary size: 1.3 MB (target: <5 MB)
✓ Dependencies: 12 (target: <50)
✓ Container size: 2.84 MB (target: <10 MB)
⚠ Warning: Large dependency 'some-crate' adds 2.1 MB
```

#### Level 3: Session Green Report

```bash
# At end of Claude Code session
forge-protocol green-report

Session Green Metrics:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Validations: 47 local, 0 cloud
Carbon saved: 11.75g CO2 (vs cloud validation)
Cost saved: $0.94
Efficiency: 98.2%

Cumulative (this project):
Carbon saved: 2.4 kg CO2
Cost saved: $189.50
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

#### Level 4: Green Badge

```bash
forge-protocol green-badge

# Generates badge for README
![Green Coding](https://img.shields.io/badge/Green%20Score-98%2F100-brightgreen)
```

### What Remains in warmup.yaml

Extended configuration that users CAN customize:

```yaml
# warmup.yaml - optional extensions
green_coding:
  targets:
    binary_size_mb: 5       # Override default
    container_size_mb: 10   # Override default
    max_dependencies: 50    # Override default

  project_specific:
    compression: "upx"      # Binary compression tool
    strip_symbols: true     # Remove debug symbols in release

  team_metrics:
    report_frequency: "weekly"
    carbon_budget_kg: 10
```

### Enforcement Philosophy

Unlike ethics (which blocks), green coding **encourages**:

| Violation | Ethics Response | Green Response |
|-----------|-----------------|----------------|
| Detected | Block/Warn | Score reduction |
| Repeated | Halt session | Lower badge |
| Persistent | Human veto | Metric tracking |

**Green coding is about visibility and incentives, not gates.**

## Consequences

### Positive

1. **Measurable sustainability** - Actual metrics, not just principles
2. **Gamification** - Green score encourages optimization
3. **Cost visibility** - Users see savings from local-first
4. **Badge system** - Social proof for green projects
5. **Cumulative tracking** - Long-term impact measurement

### Negative

1. **Metric overhead** - Some computation for tracking
2. **Storage needs** - Metrics stored locally
3. **Comparison complexity** - Cloud vs local estimates are approximations
4. **False precision** - Carbon calculations are estimates

### Neutral

1. **Backwards compatible** - Existing projects continue working
2. **Opt-in depth** - Basic always on, detailed reports optional
3. **No blocking** - Green issues don't halt work

## Implementation Plan

### Phase 1: Basic Metrics

1. Add green score to `validate` output
2. Track local vs cloud validation counts
3. Calculate estimated savings

### Phase 2: Audit Commands

1. Add `--green-audit` flag
2. Check binary/container sizes
3. Analyze dependency weight

### Phase 3: Reporting

1. Add `green-report` command
2. Cumulative project metrics
3. Session-level tracking

### Phase 4: Badges

1. Add `green-badge` command
2. Generate shield.io compatible badges
3. README integration guide

## Metrics Calculations

### Carbon Estimation

```rust
// Estimates based on research (see GREEN_CODING.md)
const CLOUD_AI_CARBON_PER_VALIDATION_G: f64 = 0.25;
const LOCAL_CLI_CARBON_PER_VALIDATION_G: f64 = 0.0005;

fn carbon_saved(local_validations: u64) -> f64 {
    local_validations as f64 * (CLOUD_AI_CARBON_PER_VALIDATION_G - LOCAL_CLI_CARBON_PER_VALIDATION_G)
}
```

### Cost Estimation

```rust
// Based on typical AI API pricing
const CLOUD_AI_COST_PER_VALIDATION_USD: f64 = 0.02;

fn cost_saved(local_validations: u64) -> f64 {
    local_validations as f64 * CLOUD_AI_COST_PER_VALIDATION_USD
}
```

### Green Score

```rust
fn green_score(metrics: &GreenMetrics, config: &GreenConfig) -> u8 {
    let mut score = 100u8;

    // Deductions
    if binary_size > config.target_binary_mb { score -= 10; }
    if deps > config.target_deps { score -= 10; }
    if container_size > config.target_container_mb { score -= 10; }

    // Bonuses
    if uses_upx_compression { score = score.saturating_add(5); }
    if zero_cloud_validations { score = score.saturating_add(5); }

    score.min(100)
}
```

## Alternatives Considered

### Alternative 1: Cloud Tracking Service

Report metrics to central green coding server.

**Rejected**: Violates local-first principles, privacy concerns.

### Alternative 2: Strict Enforcement

Block builds that don't meet green targets.

**Rejected**: Too aggressive, discourages adoption.

### Alternative 3: No Measurement

Keep green coding as pure documentation.

**Rejected**: "What gets measured gets managed."

## Open Questions

1. Should green score affect validation pass/fail?
2. How to handle projects that legitimately need cloud AI?
3. What's the right granularity for metrics storage?
4. Should we support team/org aggregate metrics?

## References

- [ADR-001: Green Coding By Default](001-green-coding-by-default.md)
- [docs/GREEN_CODING.md](../GREEN_CODING.md) - Full green coding analysis
- [warmup.yaml green_coding section](../../warmup.yaml)
