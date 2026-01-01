---
marp: true
theme: default
paginate: true
backgroundColor: #ffffff
color: #1a1a2e
style: |
  section {
    font-family: 'Segoe UI', 'Helvetica Neue', Arial, sans-serif;
  }
  h1 {
    color: #0066b3;
    font-size: 2.4em;
  }
  h2 {
    color: #333333;
    font-size: 1.8em;
  }
  strong {
    color: #0066b3;
  }
  table {
    font-size: 0.9em;
    width: 100%;
  }
  th {
    background: #0066b3;
    color: white;
  }
  blockquote {
    border-left: 4px solid #0066b3;
    padding-left: 1em;
    font-style: italic;
    font-size: 0.95em;
  }
  .small { font-size: 0.8em; }
---

<!--
ROYALBIT ASIMOV - Executive Deck (Simplified)
=============================================
5 slides. Moat first. Less is more.

Generate PDF:  marp --no-stdin EXECUTIVE_DECK.md -o EXECUTIVE_DECK.pdf
-->

<!-- _class: lead -->
<!-- _backgroundColor: #0066b3 -->
<!-- _color: #ffffff -->

# One Context Beats Many Agents

## The Research Proves It

**RoyalBit Asimov**

---

# The Moat

> "Adding manpower to a late software project makes it later." — Fred Brooks, 1975

**Brooks' Law applies to AI agents.**

| Agents | Communication Channels | Result |
|--------|------------------------|--------|
| 4 | 6 | Manageable |
| 10 | 45 | Chaos |

<div class="small">

| Finding | Source |
|---------|--------|
| **95% vs 80%** code accuracy (full context vs fragmented) | [SWE-bench](https://inkeep.com/blog/context-engineering-why-agents-fail) |
| **17.2x error amplification** with independent agents | [Google/MIT 2024](https://venturebeat.com/orchestration/research-shows-more-agents-isnt-a-reliable-path-to-better-enterprise-ai) |
| Max **3-4 agents** before overhead dominates | Google/MIT |

</div>

---

# The Solution

**One 200k-token context. AI decides when to spawn agents.**

| Dimension | Fixed Frameworks | Asimov |
|-----------|------------------|--------|
| Context per agent | 8-32k tokens | **200k+** |
| Agent topology | Fixed at design | **AI-decided at runtime** |
| Coordination | O(n^1.724) | **O(1)** |
| Code accuracy | 80% | **95%** |

> "In 2025, running multiple agents in collaboration only results in fragile systems."
> — [Cognition (Devin)](https://cognition.ai/blog/dont-build-multi-agents)

---

# The Proof

**56x velocity. Ethics built in. Verified on GitHub.**

| Metric | Traditional | With Asimov |
|--------|-------------|-------------|
| LOC/day | 25-50 | **2,780** |
| MVP timeline | 3-6 months | **Days** |
| Rework | 30-50% | **~0%** |

<div class="small">

- **78 releases** in 8 days
- **22,239 LOC** with 257 tests
- **Three Laws** ethics hardcoded in binary
- **99.6% carbon reduction** vs cloud validation

**Verify:** [github.com/royalbit/asimov](https://github.com/royalbit/asimov)

</div>

---

<!-- _class: lead -->
<!-- _backgroundColor: #1a1a2e -->
<!-- _color: #ffffff -->

# Get Started

```bash
asimov init --type rust
```

**Research:** [ADR-054](https://github.com/royalbit/asimov/blob/main/docs/adr/054-dynamic-swarm-vs-fixed-agentic-frameworks.md) — 50+ verified references

**Code:** [github.com/royalbit/asimov](https://github.com/royalbit/asimov)

*"The git logs are public. Verify yourself."*

---
