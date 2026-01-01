# Formula Derivations

This document explains the mathematical models and formulas used in RoyalBit Asimov claims.

## Monte Carlo Simulation Model

### Source
All Monte Carlo simulations use [Forge](https://github.com/royalbit/forge) with parameters defined in [`models/agent-formulas.yaml`](../models/agent-formulas.yaml).

### Dynamic Swarm Effective Accuracy

```
ds_effective_accuracy = ds_base_accuracy + (1 - ds_base_accuracy) × ds_detection_rate × ds_fix_rate
                      = 0.97 + (1 - 0.97) × 0.75 × 0.9
                      = 0.97 + 0.03 × 0.675
                      = 0.99025
```

**Parameters:**
| Parameter | Value | Source |
|-----------|-------|--------|
| `ds_base_accuracy` | 0.97 | Claude Code SWE-bench performance |
| `ds_detection_rate` | 0.75 | Extended thinking in-context error detection |
| `ds_fix_rate` | 0.90 | Self-correction success rate |

### Fixed Agentic Effective Accuracy

```
fi_channels = fi_agent_count × (fi_agent_count - 1) / 2
            = 4 × 3 / 2 = 6

fi_overhead_factor = 1 - (1 - fi_channel_error_rate)^fi_channels
                   = 1 - (1 - 0.05)^6
                   = 1 - 0.735 = 0.265

fi_effective_pre_correction = fi_base_accuracy × (1 - fi_overhead_factor)
                            = 0.80 × 0.735 = 0.588

fi_effective_accuracy = fi_effective_pre_correction + (1 - fi_effective_pre_correction) × fi_detection_rate × fi_fix_rate
                      = 0.588 + 0.412 × 0.4 × 0.6
                      = 0.588 + 0.099 = 0.687
```

**Parameters:**
| Parameter | Value | Source |
|-----------|-------|--------|
| `fi_base_accuracy` | 0.80 | Typical multi-agent baseline |
| `fi_agent_count` | 4 | Rule of 4 (MIT/Google) |
| `fi_channel_error_rate` | 0.05 | Inter-agent communication error |
| `fi_detection_rate` | 0.40 | Lower due to fragmented context |
| `fi_fix_rate` | 0.60 | Lower due to coordination overhead |

### Success Rate Over N Steps

```
success_rate(n) = effective_accuracy^n
```

| Steps | Dynamic Swarm | Fixed Agentic | Advantage |
|-------|---------------|---------------|-----------|
| 5 | 95.2% | 15.3% | 6.2x |
| 10 | 90.7% | 2.3% | **39x** |
| 20 | 82.2% | 0.05% | **1,502x** |
| 50 | 61.3% | ~0% | ∞ |

---

## O(n^1.724) Communication Overhead

### Source
This figure comes from [VentureBeat's summary](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) of MIT/Google research (December 2024).

**Citation Status:** Secondary source. The original research paper has not been published as of January 2026.

### Interpretation
The exponent 1.724 describes how communication overhead scales with agent count:

```
overhead(n) ∝ n^1.724
```

This is **super-linear** (worse than linear, approaching quadratic):
- 2 agents: 2^1.724 ≈ 3.3x baseline
- 4 agents: 4^1.724 ≈ 10.6x baseline
- 8 agents: 8^1.724 ≈ 35x baseline

### Theoretical Basis
Brooks' Law predicts communication channels scale as:
```
channels = n × (n-1) / 2 = O(n²)
```

The measured 1.724 exponent suggests partial mitigation through:
- Structured communication patterns
- Hub-and-spoke architectures
- Batched message passing

### Our Model's Approach
We don't use 1.724 directly. Instead, we model channel error accumulation:
```
overhead_factor = 1 - (1 - channel_error_rate)^channels
```

With 4 agents and 5% channel error:
```
channels = 4 × 3 / 2 = 6
overhead = 1 - 0.95^6 = 26.5%
```

---

## 17.2x Error Amplification

### Source
Same MIT/Google research via [VentureBeat](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai).

**Citation Status:** Secondary source. Original paper pending.

### Interpretation
When agents operate independently without central coordination:
- Single agent error rate: baseline
- Independent multi-agent error rate: 17.2× baseline

This compounds with the 4.4x error rate for centralized architectures:
```
independent_agents / centralized = 17.2 / 4.4 ≈ 3.9x worse
```

### Our Model
We capture this via reduced detection and fix rates:
- Dynamic Swarm: 75% detection, 90% fix
- Fixed Independent: 40% detection, 60% fix

---

## 74% Error Reduction (HOTL)

### Derivation
This is derived from the 17.2x → 4.4x comparison:

```
reduction = 1 - (4.4 / 17.2) = 1 - 0.256 = 74.4% ≈ 74%
```

**Meaning:** Human-on-the-loop oversight reduces the error amplification from 17.2x to 4.4x, a 74% improvement.

### Source
Derived from MIT/Google figures in VentureBeat article.

---

## Validation

### Reproduce the Calculations

```bash
# Run Python simulation (no dependencies required)
python models/simulate.py

# Validate with Forge (if installed)
forge calculate models/agent-formulas.yaml
```

Sample output from `simulate.py`:
```
 Steps |   Dynamic Swarm + HOTL |   Fixed Agentic (Indep | Advantage
    10 |   90.67% (MC:90.26%) |    2.34% (MC: 2.22%) |       38.8x
    20 |   82.20% (MC:81.99%) |    0.05% (MC: 0.09%) |     1501.7x
```

### Sensitivity Analysis
The model is sensitive to:
1. **Base accuracy** - Higher base → smaller advantage ratio
2. **Detection rate** - Critical for self-correction
3. **Agent count** - Channels grow quadratically

### Limitations
1. Assumes independent step errors (may correlate in practice)
2. Does not model task-specific complexity
3. Fixed agents model is simplified (real frameworks vary)

---

## References

- [agent-formulas.yaml](../models/agent-formulas.yaml) - Primary formula definitions
- [monte-carlo-agents.yaml](../models/monte-carlo-agents.yaml) - Simulation configuration
- [ADR-056](adr/056-extended-thinking-vs-rag-agentic.md) - Extended thinking research
- [VentureBeat - MIT/Google Research](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) - Secondary source for 17.2x and 1.724

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov)*
