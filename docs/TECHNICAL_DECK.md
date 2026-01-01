---
marp: true
theme: uncover
class: invert
paginate: true
backgroundColor: #1a1a2e
color: #ffffff
style: |
  section {
    font-family: 'Segoe UI', Arial, sans-serif;
  }
  h1, h2 {
    color: #ff6b35;
  }
  strong {
    color: #00d4ff;
  }
  code {
    background: #0d0d0d;
    color: #00ff88;
  }
  blockquote {
    border-left: 4px solid #ff6b35;
    padding-left: 1em;
    font-style: italic;
    color: #cccccc;
  }
  table {
    font-size: 0.85em;
  }
  th {
    background: #ff6b35;
    color: white;
  }
  .small { font-size: 0.75em; }
  pre { font-size: 0.8em; }
---

<!--
ROYALBIT ASIMOV - Technical Deck (Simplified)
=============================================
8 slides. Moat first. For engineers.

Generate PDF:  marp --no-stdin TECHNICAL_DECK.md -o TECHNICAL_DECK.pdf
-->

# One Context Beats Many Agents

## The Research Proves It

**RoyalBit Asimov**

*For Engineers*

---

# Brooks' Law for AI Agents

> "Adding manpower to a late software project makes it later." — Fred Brooks, 1975

```
Communication channels = N × (N-1) / 2

4 agents  →  6 channels   (manageable)
10 agents → 45 channels   (chaos)
20 agents → 190 channels  (impossible)
```

**Google/MIT measured it:** Overhead scales with **exponent 1.724**

That's worse than quadratic.

---

# The Research

<div class="small">

| Finding | Value | Source |
|---------|-------|--------|
| Full context vs fragmented accuracy | **95% vs 80%** | [SWE-bench](https://inkeep.com/blog/context-engineering-why-agents-fail) |
| Error amplification (independent agents) | **17.2x** | [Google/MIT](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) |
| Max effective agents | **3-4** | Google/MIT |
| Multi-agent token overhead | **15x** vs single | [Anthropic](https://www.anthropic.com/engineering/multi-agent-research-system) |
| Long context vs RAG | **+7-13%** | [Google DeepMind](https://arxiv.org/abs/2407.16833) |

</div>

> "In 2025, running multiple agents in collaboration only results in fragile systems."
> — [Cognition (Devin)](https://cognition.ai/blog/dont-build-multi-agents)

---

# Asimov vs Fixed Frameworks

| Dimension | LangChain / CrewAI | Asimov |
|-----------|-------------------|--------|
| Context per agent | 8-32k tokens | **200k+** |
| Agent topology | Fixed at design time | **AI-decided at runtime** |
| Coordination overhead | O(n^1.724) | **O(1)** |
| Code understanding | 80% (fragmented) | **95%** (full context) |
| MCP token overhead | 15,000+ tokens/session | **0** (CLI preference) |

**The insight:** Context IS the coordination layer.

---

# How It Works

```bash
# Initialize project with Three Laws
asimov init --type rust

# AI loads context, presents milestone
> run warmup

# You confirm, AI executes autonomously
> go

# Quality gates enforce shipping
# Tests pass → Zero warnings → Tagged → Pushed
```

**One context. AI spawns agents only when benefit > cost.**

---

# The Three Laws

```yaml
first_law:   # Do no harm
  do_no_harm: { financial: true, privacy: true, deception: true }

second_law:  # Obey humans (except when violating First Law)
  human_veto: ["stop", "halt", "abort"]

third_law:   # Self-preserve (bounded sessions)
  bounded_sessions: "Run until done, then stop"
```

**Hardcoded in binary. Can't be quietly disabled.**

---

# The Proof

| Metric | Industry | With Asimov | Multiplier |
|--------|----------|-------------|------------|
| LOC/day | 25-50 | 2,780 | **56x** |
| Time to 22K LOC | 3-6 months | 8 days | **15x** |
| Releases | 3-5/project | 78 in 8 days | **10x/day** |
| Carbon (validation) | ~0.5g CO₂ | ~0.002g | **99.6% less** |

**Verify:** [github.com/royalbit/asimov](https://github.com/royalbit/asimov)

*Git logs don't lie.*

---

# Get Started

```bash
# Install
curl -L https://github.com/royalbit/asimov/releases/latest/download/asimov-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv asimov /usr/local/bin/

# Initialize
asimov init --type rust
```

**Research:** [ADR-054](https://github.com/royalbit/asimov/blob/main/docs/adr/054-dynamic-swarm-vs-fixed-agentic-frameworks.md) — 50+ verified references

**Code:** [github.com/royalbit/asimov](https://github.com/royalbit/asimov)

---
