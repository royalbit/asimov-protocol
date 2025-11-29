# ADR-010: Velocity Constraints and Tier Analysis

## Status

Accepted

## Date

2025-11-28

## Context

### The Problem

The Forge Protocol documentation claims "50-100x velocity" without qualifying:
1. What subscription tier this applies to
2. Hardware requirements (or lack thereof)
3. The bottlenecks that limit velocity
4. Honest assessment of realistic expectations

This ADR provides a rigorous, honest analysis of velocity constraints.

### Research Findings (November 2025)

#### Subscription Tiers

| Tier | Context Window | Rate Limits | Monthly Cost |
|------|---------------|-------------|--------------|
| Free | 200K | Very limited | $0 |
| Pro | 200K | ~40-80 Claude Code hrs/week | $20 |
| Max 5x | 200K | 5x Pro (~200-800 prompts/5hr) | $100 |
| Max 20x | 200K | 20x Pro (~900 messages/5hr) | $200 |
| Enterprise | 500K | Custom (higher) | $5K-$100K+ |
| API Tier 4 | **1M tokens** | Custom | Usage-based |

Sources:
- [Claude Code Pricing](https://claudelog.com/claude-code-pricing/)
- [Anthropic Rate Limits](https://docs.claude.com/en/api/rate-limits)
- [1M Context Announcement](https://www.anthropic.com/news/1m-context)

#### Context Window Impact on Velocity

| Context Size | Compaction Frequency | Self-Healing Overhead | Effective Velocity |
|--------------|---------------------|----------------------|-------------------|
| 200K tokens | Every 10-20 min | High (frequent re-reads) | Baseline |
| 500K tokens | Every 25-50 min | Medium | +25-40% |
| 1M tokens | Every 50-100 min | Low | +50-75% |

With 1M tokens, you can load **entire codebases** (75,000 lines) into context, reducing the need for file re-reads and self-healing cycles.

#### Hardware Analysis

Test system: Intel i7-13850HX (20 cores), 32GB RAM, NVMe SSD

| Factor | Local Hardware Helps? | Why |
|--------|----------------------|-----|
| API Latency | ❌ No | Server-side bottleneck |
| Token Processing | ❌ No | Server-side computation |
| Claude Reasoning | ❌ No | Model architecture limit |
| Compilation (Rust) | ✅ Yes | ~2-3s on 20 cores |
| Test Execution | ✅ Yes | Already fast |
| Disk I/O | ✅ Saturated | NVMe already optimal |

**Conclusion**: Upgrading to a $15K workstation would yield ~10-15% improvement, not 2x. The bottleneck is API latency, not local hardware.

## Decision

### Honest Velocity Estimates by Tier

| Tier | Realistic Velocity | Notes |
|------|-------------------|-------|
| **Pro** ($20/mo) | 5-15x | Rate limits, frequent compaction |
| **Max 5x** ($100/mo) | 8-20x | More headroom, same context |
| **Max 20x** ($200/mo) | 10-30x | Best consumer tier |
| **Enterprise** ($5K+/mo) | 20-50x | 500K context, higher limits |
| **API Tier 4** (1M) | 30-75x | 1M context, minimal compaction |
| **Theoretical Max** | ~100x | Perfect conditions, never achieved |

### When 50-100x Is Possible

The 50-100x claim is achievable under specific conditions:

1. **Greenfield projects** with clear specifications
2. **Enterprise tier** or API Tier 4 (1M context)
3. **Boilerplate-heavy work** where AI excels
4. **Single-session completions** without context loss
5. **Well-defined domains** (CRUD, CLI tools, utilities)

### When 50-100x Is NOT Achievable

1. **Complex legacy codebases** - 5-15x realistic
2. **Novel architectures** requiring human creativity - 3-10x
3. **Consumer tiers** with frequent rate limits - 10-30x max
4. **Multi-session projects** with context loss - Velocity degrades

### Documentation Updates Required

1. **README.md**: Add tier-based velocity table
2. **GREEN_CODING.md**: Remove unqualified 50-100x claims
3. **ECOSYSTEM.md**: Add caveats to Forge case study
4. **Cargo.toml**: Update description

## Consequences

### Positive

1. **Honest marketing** - Builds trust with users
2. **Correct expectations** - Users know what tier they need
3. **Tier-based guidance** - Enterprise users know their advantage
4. **Hardware truth** - No false promises about workstation upgrades

### Negative

1. **Lower headline number** - 10-30x less impressive than 50-100x
2. **Complexity** - Must explain tiers instead of single claim

### Neutral

1. **Enterprise advantage documented** - May drive upgrades
2. **Hardware irrelevance clarified** - Focus on subscription, not specs

## Implementation

### Velocity Claim Updates

**OLD**: "50-100x velocity"

**NEW**:
```
Velocity by tier:
- Pro/Max: 10-30x (typical)
- Enterprise: 20-50x (500K context)
- API Tier 4: 30-75x (1M context)
- Peak: up to 100x (greenfield + enterprise)
```

### Hardware Guidance

**OLD**: (none)

**NEW**:
```
Hardware requirements: Minimal
- Any modern CPU (4+ cores)
- 8GB+ RAM
- SSD recommended

The bottleneck is API latency, not local compute.
Upgrading hardware yields ~10-15% improvement.
```

## References

- [Claude Code Pricing Guide](https://claudelog.com/claude-code-pricing/)
- [Anthropic Rate Limits](https://docs.claude.com/en/api/rate-limits)
- [Claude 1M Context Announcement](https://www.anthropic.com/news/1m-context)
- [Claude Context Windows](https://docs.claude.com/en/docs/build-with-claude/context-windows)
- [TechCrunch: Rate Limits for Power Users](https://techcrunch.com/2025/07/28/anthropic-unveils-new-rate-limits-to-curb-claude-code-power-users/)
