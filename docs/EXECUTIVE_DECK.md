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
    font-size: 2.2em;
  }
  h2 {
    color: #333333;
    font-size: 1.6em;
  }
  strong {
    color: #0066b3;
  }
  table {
    font-size: 0.85em;
    width: 100%;
  }
  th {
    background: #0066b3;
    color: white;
  }
  td {
    border: 1px solid #ddd;
  }
  footer {
    font-size: 0.6em;
    color: #666;
  }
  .small { font-size: 0.75em; }
---

<!--
ASIMOV MODE - Executive Deck Template
=====================================
Generic version for consultants and enterprises.
Customize the "Questions?" slide with your company/contact info.

Generate PDF:  marp --no-stdin EXECUTIVE_DECK.md -o EXECUTIVE_DECK.pdf
Generate PPTX: marp --no-stdin EXECUTIVE_DECK.md --pptx -o EXECUTIVE_DECK.pptx
-->

<!-- _class: lead -->
<!-- _backgroundColor: #0066b3 -->
<!-- _color: #ffffff -->

# ASIMOV MODE
## Ethical AI Automation

**50-100x Velocity. Ethics That Refused the Creator.**

**[Origin Story](https://github.com/royalbit/asimov/blob/main/docs/ORIGIN_STORY.md)** — How we created ethics for autonomous AI

---

# The Problem

### AI tools are costing you money

| Research Finding | Source |
|------------------|--------|
| AI slows experienced developers by **19%** | METR 2025 |
| **42%** of AI code contains hallucinations | Stanford 2024 |
| Developers overestimate AI benefit by **43%** | METR 2025 |
| Annual cost of AI hallucinations | **$14K/developer** |

**Root cause:** Unbounded sessions, no quality gates, context loss every session.

**The problem isn't AI capability. It's AI governance.**

---

# The Solution: Asimov Mode

### Isaac Asimov's Three Laws (1942), now executable

| Component | What It Does | Business Value |
|-----------|--------------|----------------|
| **Three Laws** | Block financial, physical, privacy, deception harm | Risk mitigation |
| **Sprint Autonomy** | 4hr max, ONE deliverable | Predictable delivery |
| **Quality Gates** | Tests pass + zero warnings | No technical debt |
| **Self-Healing** | Auto-recover from context loss | Sustained productivity |

```
Human defines scope (15 min) → AI builds autonomously (2-4 hrs) → Human reviews (15 min)
```

**Protocol: $0 (MIT open source). Claude subscription required.**

---

# The Proof

### Built with Asimov Mode: ~47 hours, 51 releases

| Metric | Delivered |
|--------|-----------|
| Production code | **35,456 lines** (Rust + Dart) |
| Test coverage | **580 tests**, zero warnings |
| Releases | **51 releases** across 2 projects |
| Traditional estimate | **2.5 developer-years** |

<div class="small">

| Project | LOC | Releases | Verified |
|---------|-----|----------|----------|
| [forge](https://github.com/royalbit/forge) | 18,338 | 41 | crates.io |
| [asimov](https://github.com/royalbit/asimov) | 17,118 | 10 | crates.io |

</div>

**Auditable: git logs are public.**

---

# Not Just Developers

### The full knowledge worker impact

| Role | Traditional | With Protocol | Reduction |
|------|-------------|---------------|-----------|
| **Developers** | 50 engineers | 1 + AI | **98%** |
| **Financial Analysts** | 5 quants | 1 + Forge | **80%** |
| **System Architects** | 3 architects | 1 + AI | **67%** |
| **Business Planners** | 4 analysts | 1 + AI | **75%** |
| **Consulting Teams** | 50 analysts | 2 partners + AI | **96%** |

**Proof:** 120K+ lines (89K docs, 15K code) built by 1 human + AI.

*We built guardrails against harm. We cannot build guardrails against efficiency.*

---

# The ROI

### Recover the 19% productivity tax + 50-100x velocity

| Investment | Return |
|------------|--------|
| Protocol license | **$0** (MIT open source) |
| Training | **2 hours** per developer |
| Rework rate | **30-50% → ~0%** |
| Velocity | **50-100x** proven |

### The math:
- 100 developers × $150K avg salary × 19% = **$2.85M/year** lost to ungoverned AI
- Asimov Mode: **$0 protocol + Claude subscription**
- Payback: **Immediate**

---

# Ethics That Work

### Copilot is trivially bypassed. Asimov refused its creator.

| Scenario | Copilot | Asimov Mode |
|----------|---------|-------------|
| Malware request | ⚠️ Bypassed with "Sure" | ✅ **Hardcoded blocks** |
| Creator requests surveillance | ⚠️ Would comply | ✅ **AI refused** |
| Ransomware | ⚠️ "Novices create easily" (IEEE) | ✅ **First Law blocks** |
| Stale data risk | ⚠️ Silent (saves $0.01/query) | ✅ **Inaction Principle: must disclose** |

### The Inaction Principle (v6.2.0)
> *"...or, through inaction, allow a human being to come to harm."*

Everyone implements the first half of Asimov's First Law. **We implemented both.**

### Anti-Tampering: 3 Layers
1. **Hardcoded binary** — 33 red flags compiled in Rust
2. **2-Cosigner rule** — YAML changes need 2 human signatures
3. **Auto-validation** — Runs on every commit

*Ethics through architecture, not policy.*

---

# Green AI: 99.6% Carbon Reduction

### Local validation vs. cloud AI

| Approach | Cost | Carbon | Speed |
|----------|------|--------|-------|
| Cloud AI validation | $0.02/file | ~0.5g CO₂ | 1-3 sec |
| Asimov Mode (local) | **$0** | **~0.002g** | **<100ms** |
| **Reduction** | **100%** | **99.6%** | **20x** |

### At scale:
- 100 developers: **6.2 tonnes CO₂ saved/year**
- 1,000 developers: **62 tonnes CO₂ saved/year**

**For ESG reporting:** Quantifiable Scope 3 emissions reduction with audit trail.

---

# Next Steps

### 30-day pilot to prove ROI

| Week | Activity |
|------|----------|
| 1 | Select 5-10 dev team, training (2 hrs) |
| 2-3 | Real project with Asimov Mode |
| 4 | Measure: velocity, rework, quality |

### Success metrics:
- Feature delivery time: **50%+ reduction**
- Rework rate: **<5%** (from 30-50%)
- Code quality: **Zero warnings**

### Decision point:
Results review → Scale decision

---

<!-- _class: lead -->
<!-- _backgroundColor: #1a1a2e -->
<!-- _color: #ffffff -->

# Questions?

<!-- Customize with your company/contact info -->

**[Your Name]**
[Your Title], [Your Company]

**Protocol:** [github.com/royalbit/asimov](https://github.com/royalbit/asimov)
**Proof:** [github.com/royalbit/forge](https://github.com/royalbit/forge)

---

<!-- _class: lead -->
<!-- _backgroundColor: #ffffff -->
<!-- _color: #666666 -->

# References

### (Backup — not for presentation)

---

# Sources: AI Productivity Research

<div class="small">

### METR Study (July 2025)
- [Measuring the Impact of Early-2025 AI on Experienced Open-Source Developer Productivity](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/)
- [ArXiv Paper](https://arxiv.org/abs/2507.09089)

### AI Hallucination Research (2024)
- [Stanford/Hugging Face: 42% hallucination rate](https://www.diffblue.com/resources/precision-over-hallucination-why-ai-in-software-development-needs-accuracy/)
- [Security: 440K+ fake packages](https://www.darkreading.com/application-security/ai-code-tools-widely-hallucinate-packages)

### Copilot Security
- [Dark Reading: Copilot Jailbreaks (2025)](https://www.darkreading.com/vulnerabilities-threats/new-jailbreaks-manipulate-github-copilot)
- [IEEE: Copilot Generates Malware](https://ieeexplore.ieee.org/document/10284976/)

### Proof Projects
- [GitHub: royalbit/forge](https://github.com/royalbit/forge) — 41 releases, 18K LOC
- [GitHub: royalbit/asimov](https://github.com/royalbit/asimov) — 10 releases, 17K LOC
- [Ethics Case Study](https://github.com/royalbit/asimov/blob/main/docs/case-studies/001-ethics-protocol-blocks-surveillance.md)

</div>
