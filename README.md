# RoyalBit Asimov

[![CI](https://github.com/royalbit/asimov/actions/workflows/ci.yml/badge.svg)](https://github.com/royalbit/asimov/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/code-ELv2-blue.svg)](LICENSE)
[![Docs License](https://img.shields.io/badge/docs-CC%20BY--NC--ND%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-nd/4.0/)

> **[Disclaimer](DISCLAIMER.md):** This project intentionally uses exaggerated claims as part of adversarial R&D. Claims are hypotheses, not guarantees.

> **Dynamic Swarm + HOTL beats Fixed Agentic. We have the math.**

## The Architecture

```
Human (HOTL)
    ↓ oversight
Orchestrator (~200K tokens, extended thinking)
    ↓ spawns dynamically at runtime
    ├── Sub-Agent 1 (~200K tokens)
    ├── Sub-Agent 2 (~200K tokens)
    └── Sub-Agent N (~200K tokens)
```

**Dynamic Swarm** (Asimov + Claude Code): Each agent has full context (~200K tokens). Spawning is AI-decided at runtime. Human-on-the-Loop (HOTL) approves/guides.

**Fixed Agentic** (LangChain, CrewAI, AutoGen): Pre-defined roles. Fragmented context. No human gate. 17.2x error amplification.

Source: [Claude Code Subagents](https://code.claude.com/docs/en/sub-agents) — Official Anthropic documentation

---

## The Insight

**Analytical model (research-backed 2024, validated via Forge):**

| Steps | Dynamic Swarm + HOTL | Fixed Multi-Agent | Advantage |
|-------|---------------------|-------------------|-----------|
| 10 | **90.7%** | 2.3% | **39x** |
| 20 | **82.2%** | 0.05% | **1,502x** |
| 50 | **61.3%** | ~0% | **∞** |

Source: [agent-formulas.yaml](models/agent-formulas.yaml) — validate with `forge calculate models/agent-formulas.yaml`

**Why Dynamic Swarm wins:**
- Each sub-agent has **full context** (~200K tokens), not fragmented
- **HOTL oversight** prevents error cascades (74% error reduction measured)
- **AI-decided spawning** avoids fixed topology overhead
- Extended thinking enables **75% in-context error detection**

**Why Fixed Agentic fails:**
- Context fragmented across agents (10-20% effective utilization)
- O(n^1.724) communication overhead
- 17.2x error amplification without human gate
- Pre-defined roles can't adapt to task requirements

📖 [ADR-056: Extended Thinking vs RAG+Agentic](docs/adr/056-extended-thinking-vs-rag-agentic.md) — Full research

---

## Limitations

**Be aware of these trade-offs:**

| Limitation | Impact | Mitigation |
|------------|--------|------------|
| **HOTL bottleneck** | Human approval can slow autonomous work | Batch approvals, trust calibration over time |
| **Claude dependency** | No GPT/Gemini/Llama interop | Protocol files are model-agnostic (paste them) |
| **Token cost** | ~200K context costs more than RAG | Offset by fewer errors, less rework |
| **Spawn latency** | Sub-agent creation adds ~2-5s | Parallelizable, amortized over task |
| **Citation gaps** | Some metrics from secondary sources | Primary sources linked where available |

**Research transparency:**
- The **O(n^1.724)** exponent and **17.2x error amplification** are from [VentureBeat's summary](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) of MIT/Google research (December 2024). Original paper not yet published.
- Monte Carlo simulations are reproducible: `python models/simulate.py` (no dependencies). See [Formula Derivations](docs/FORMULA_DERIVATIONS.md) for full methodology.
- Independent benchmarks (SWE-bench, OSWorld) validate Claude's capabilities, not Asimov specifically.

📖 [ADR-055: Balanced Architecture Critique](docs/adr/055-balanced-architecture-critique.md) — Full trade-off analysis

---

## Quick Start

```bash
# Install
curl -L https://github.com/royalbit/asimov/releases/latest/download/asimov-$(uname -m)-unknown-linux-musl.tar.gz | tar xz
sudo mv asimov /usr/local/bin/

# Initialize project
asimov init

# Launch Claude Code with optimal settings (Dynamic Swarm enabled)
asimov
```

**Requires [Claude Code](https://claude.ai/code).** Protocol files work anywhere (paste them).

---

## What It Does

Eight protocol files in `.asimov/` ground AI in file-based truth:

```
.asimov/
├── asimov.json         # Ethics: harm categories + veto commands
├── sprint.json         # Autonomous execution until done
├── sycophancy.json     # Truth over comfort
├── freshness.json      # WebSearch for current information
├── green.json          # Efficiency benchmarking
└── ...
```

**The pattern:** File truth (stable, deterministic) beats AI memory (lossy, probabilistic).

---

## CLI

```bash
asimov              # Launch Claude Code with MAX_THINKING_TOKENS=200000 + Dynamic Swarm
asimov init         # Initialize project
asimov warmup       # Output complete context as JSON
asimov doctor       # Diagnose setup issues
asimov validate     # Validate protocol files
asimov update       # Self-update
```

**Platforms:** Linux, macOS, Windows | **Binary:** 1.5MB | **Dependencies:** Zero

---

## The Research

| Finding | Source |
|---------|--------|
| Dynamic Swarm: **39x advantage** over fixed agentic at 10 steps | [Forge Model](models/agent-formulas.yaml) |
| Sub-agents run with **full context (~200K tokens)** | [Claude Code Docs](https://code.claude.com/docs/en/sub-agents) |
| HOTL validation reduces error by **74%** (17.2x → 4.4x) | [MIT/Google 2024](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) |
| Max **3-4 effective agents** before overhead dominates | MIT/Google 2024 |
| RLI benchmark: **97.5% failure** on real work (vs 80.9% SWE-bench) | [arXiv:2504.02189](https://arxiv.org/abs/2504.02189) |
| Fixed multi-agent: **fragile systems, dispersed decision-making** | [Cognition (Devin)](https://cognition.ai/blog/dont-build-multi-agents) |

### Architecture Decisions

- [ADR-056: Extended Thinking vs RAG+Agentic](docs/adr/056-extended-thinking-vs-rag-agentic.md) — Monte Carlo proof
- [ADR-054: Dynamic Swarm vs Fixed Frameworks](docs/adr/054-dynamic-swarm-vs-fixed-agentic-frameworks.md) — 50+ references
- [ADR-055: Balanced Architecture Critique](docs/adr/055-balanced-architecture-critique.md) — Trade-offs acknowledged

---

## Proven at Scale

| Project | LOC | Tests | Releases |
|---------|-----|-------|----------|
| Forge (private) | 45,700 | 2,486 | 46 |
| Asimov | 19,000+ | 437 | 62+ |
| **Total** | **65,000+** | **2,900+** | **108+** |

**1 human. 1 AI. 12 days.**

📖 [Origin Story](docs/ORIGIN_STORY.md) — How we built it

---

## Documentation

- [Value Proposition](docs/VALUE_PROPOSITION.md) — Why this matters
- [Setup Guide](docs/SETUP.md) — Detailed installation
- [Glossary](docs/GLOSSARY.md) — HOTL, HITL, Dynamic Swarm, and other terms
- [Formula Derivations](docs/FORMULA_DERIVATIONS.md) — Math behind the claims
- [AI Reality](docs/AI_REALITY.md) — Why AI "hallucinates"
- [Full Specification](docs/SPECIFICATION.md) — Protocol schema

---

## License

- **Code:** [Elastic License 2.0 (ELv2)](LICENSE) — Free for most uses, restrictions on competing SaaS
- **Documentation:** [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/)

| Use Case | Allowed? |
|----------|----------|
| Personal/internal use | ✅ Yes |
| Commercial use (non-competing) | ✅ Yes |
| Modify for internal use | ✅ Yes |
| Provide as managed service | ❌ No |
| Circumvent license keys | ❌ No |

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov) — Dynamic Swarm + HOTL beats Fixed Agentic.*
