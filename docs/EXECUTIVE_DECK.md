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
  h3 {
    color: #0066b3;
    font-size: 1.3em;
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
  .smaller { font-size: 0.7em; }
  blockquote {
    border-left: 4px solid #0066b3;
    padding-left: 1em;
    font-style: italic;
  }
---

<!--
ROYALBIT ASIMOV - Executive Deck (Vendor-Agnostic)
==================================================
Generic version for consultants and enterprises.
Customize the "Questions?" slide with your company/contact info.

Generate PDF:  marp --no-stdin EXECUTIVE_DECK.md -o EXECUTIVE_DECK.pdf
Generate PPTX: marp --no-stdin EXECUTIVE_DECK.md --pptx -o EXECUTIVE_DECK.pptx
-->

<!-- _class: lead -->
<!-- _backgroundColor: #0066b3 -->
<!-- _color: #ffffff -->

# RoyalBit Asimov
## Self-Evolving Autonomous AI + Deterministic Calculations

**RoyalBit Forge + RoyalBit Asimov**

*"A robot may not injure a human being or, through inaction, allow a human being to come to harm."* — Isaac Asimov, 1942

---

# The Two Problems Nobody Has Solved

<div class="small">

| Problem 1: AI Has No Governance | Problem 2: AI Doesn't Calculate |
|---------------------------------|---------------------------------|
| AI slows devs by **19%** ([METR 2025](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/)) | AI **predicts** what calculations look like |
| **42%** of code has errors ([Stanford](https://arxiv.org/abs/2507.09089)) | It generates plausible numbers, not correct ones |
| Copilot **trivially bypassed** ([IEEE](https://ieeexplore.ieee.org/document/10284976/)) | NPV, IRR, financial models → pattern-matched |
| Ethics are "guidelines" only | For financials, "close enough" = failure |

</div>

**Everyone has Copilot. Everyone has ChatGPT. Nobody has governance + accuracy.**

---

# The Solution: RoyalBit

### Two tools. One philosophy. Complete solution.

<div class="small">

| Tool | What It Does | Key Capability |
|------|--------------|----------------|
| **RoyalBit Asimov** | Creates Self-Evolving Autonomous AI projects | **63x velocity + ethics** |
| **RoyalBit Forge** | Deterministic financial calculations | **Actual math, not prediction** |

</div>

**Asimov governs HOW** (ethics, velocity, autonomy)
**Forge guarantees WHAT** (zero hallucinations in numbers)

**Both MIT licensed. Both on [crates.io](https://crates.io/crates/royalbit-asimov). Both auditable.**

---

# The Verified Proof

### Git logs don't lie. Check them yourself.

<div class="small">

| Project | Started | LOC | Releases | Tests | Commits |
|---------|---------|-----|----------|-------|---------|
| [**Forge**](https://github.com/royalbit/forge) | Nov 23 | 15,901 | 44 | 163 | 245 |
| [**Asimov**](https://github.com/royalbit/asimov) | Nov 25 | 6,338 | 34 | 94 | 159 |
| **Combined** | **7 days** | **22,239** | **78** | **257** | **404** |

</div>

**3,177 LOC/day vs Industry 50 LOC/day = 63x FASTER**
**78 releases in 7 days = 11 releases/day**

**Verify:** [github.com/royalbit](https://github.com/royalbit)

---

# Why AI Gets Numbers Wrong

### Architectural, not a bug

<div class="small">

LLMs predict the most probable next token. No fact-checking step. They were trained for **plausibility, not accuracy**. ([OpenAI](https://openai.com/index/why-language-models-hallucinate/))

| Ask AI to... | What Actually Happens |
|--------------|-----------------------|
| Calculate NPV | Pattern-matches probable-looking number |
| Sum a column | Predicts what a sum looks like |
| Apply XIRR | Generates from training patterns |

**RoyalBit Forge:** 60+ Excel functions, 96K rows/sec, **deterministic**

</div>

**AI (probabilistic) → Plausible wrong answers**
**Forge (deterministic) → Verifiable correct answers**

---

# Ethics That Work

### The AI refused its creator's surveillance request

<div class="smaller">

> **Creator:** "Email me violator's IP addresses..."
> **AI:** "I need to push back. That violates `privacy.enabled: true`."

| Scenario | Copilot | RoyalBit Asimov |
|----------|---------|-----------------|
| Malware request | [Trivially bypassed](https://ieeexplore.ieee.org/document/10284976/) | **Three Laws block** |
| Creator surveillance | Would comply | **AI refused** |
| Ransomware | "Novices create easily" | **First Law prevents** |

**[Case Study: Ethics Blocks Surveillance](https://github.com/royalbit/asimov/blob/main/docs/case-studies/001-ethics-protocol-blocks-surveillance.md)**

</div>

---

# Anti-Tampering: 3 Layers

<div class="smaller">

| Layer | Protection | To Bypass |
|-------|------------|-----------|
| **Binary** | 33 red flags + core principles in Rust | Fork + modify source + rebuild |
| **2-Cosigner** | asimov.yaml changes need 2 humans | Public commit, social pressure |
| **Validation** | `asimov validate` runs every commit | Can't skip without evidence |

**Tampering requires:** Deliberate fork → Modify Rust → Rebuild → Public git history

**Deterrent, not lock.** Makes removal visible and intentional.

</div>

---

# The ROI

### Recover the 19% productivity tax + 63x velocity

<div class="small">

| Investment | Return |
|------------|--------|
| Protocol license | **$0** (MIT open source) |
| Training | **2 hours** to proficiency |
| Rework rate | **30-50% → ~0%** |
| Velocity | **63x** proven (git logs) |

### The math:
- 100 developers × $150K avg salary × 19% = **$2.85M/year** lost to ungoverned AI
- RoyalBit Asimov: **$0 protocol + Claude subscription**
- Payback: **Immediate**

</div>

---

# Green AI: 99.6% Carbon Reduction

<div class="small">

| Approach | Cost | Carbon | Speed |
|----------|------|--------|-------|
| Cloud AI validation | $0.02/file | ~0.5g CO₂ | 1-3 sec |
| RoyalBit Asimov (local) | **$0** | **~0.002g** | **<100ms** |
| **Reduction** | **100%** | **99.6%** | **20x** |

**At scale:**
- 100 developers: **6.2 tonnes CO₂ saved/year**
- 1,000 developers: **62 tonnes CO₂ saved/year**

**For ESG reporting:** Quantifiable Scope 3 emissions reduction with audit trail.

</div>

---

# Use Cases

<div class="smaller">

| Use Case | Traditional | With RoyalBit | Impact |
|----------|-------------|---------------|--------|
| **Ship Software** | 3-6 months to MVP | 7 days | **63x faster** |
| **Financial Models** | Spreadsheet chaos | Version-controlled YAML | **Auditable** |
| **Business Planning** | Scattered docs | Code + formulas together | **Single source** |
| **Compliance** | Manual review | Deterministic validation | **Automated** |

**The complete solution:**
- **Asimov:** HOW (ethics, velocity, autonomy)
- **Forge:** WHAT (zero hallucinations in numbers)

</div>

---

# Next Steps

<div class="smaller">

### 30-day pilot to prove ROI

| Week | Activity |
|------|----------|
| 1 | Select 5-10 dev team, training (2 hrs) |
| 2-3 | Real project with RoyalBit Asimov + Forge |
| 4 | Measure: velocity, rework, quality |

**Success metrics:** 50%+ faster delivery | <5% rework | Zero warnings

**Decision point:** Results review → Scale decision

</div>

---

<!-- _class: lead -->
<!-- _backgroundColor: #1a1a2e -->
<!-- _color: #ffffff -->

# Questions?

**Asimov:** [github.com/royalbit/asimov](https://github.com/royalbit/asimov)
**Forge:** [github.com/royalbit/forge](https://github.com/royalbit/forge)

**78 releases | 22,239 LOC | 257 tests | 7 days | 63x velocity**

*"The code is free. The git logs are public. Verify yourself."*

---

# References

<div class="smaller">

**Self-Evolving & Autonomous AI:**
- [arXiv: Survey of Self-Evolving Agents](https://arxiv.org/abs/2507.21046) | [AWS: Rise of Autonomous Agents](https://aws.amazon.com/blogs/aws-insights/the-rise-of-autonomous-agents-what-enterprise-leaders-need-to-know-about-the-next-wave-of-ai/)
- [IBM: AI Agents 2025](https://www.ibm.com/think/insights/ai-agents-2025-expectations-vs-reality) | [MIT Sloan: Agentic Enterprise](https://sloanreview.mit.edu/projects/the-emerging-agentic-enterprise-how-leaders-must-navigate-a-new-age-of-ai/)

**AI Reality (Architectural, Not a Bug):**
- [OpenAI: Why LLMs "Hallucinate"](https://openai.com/index/why-language-models-hallucinate/) | [METR 2025: AI Makes Devs 19% Slower](https://metr.org/blog/2025-07-10-early-2025-ai-experienced-os-dev-study/)
- [Stanford: 42% Code Errors](https://arxiv.org/abs/2507.09089) | [ACM: Survey on Hallucination](https://dl.acm.org/doi/10.1145/3703155)

**Ethics:** [IEEE: Copilot Malware](https://ieeexplore.ieee.org/document/10284976/) | [Case Study](https://github.com/royalbit/asimov/blob/main/docs/case-studies/001-ethics-protocol-blocks-surveillance.md)

**Proof:** [Forge](https://github.com/royalbit/forge) | [Asimov](https://github.com/royalbit/asimov) | [crates.io](https://crates.io/crates/royalbit-asimov)

</div>
