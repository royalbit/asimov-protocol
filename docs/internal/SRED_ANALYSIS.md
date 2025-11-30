# SR&ED Analysis: Asimov Protocol

**Company:** RoyalBit Inc. (Montreal, Quebec)
**Project:** Asimov Protocol - AI Session Continuity System
**Period:** November 2025
**Prepared:** 2025-11-29

## Executive Summary

Asimov Protocol qualifies for SR&ED tax credits as it addresses **novel technological uncertainties** in autonomous AI development that had no known solutions at the time of development.

### Eligibility Criteria Met

| SR&ED Requirement | Asimov Protocol Evidence |
|-------------------|------------------------|
| Technological uncertainty | Context compaction problem had no known solution |
| Systematic investigation | 15 ADRs documenting hypotheses and experiments |
| Technological advancement | First protocol for self-healing AI sessions |
| Documentation | Complete git log, ADRs, research citations |

## Technical Uncertainties Resolved

### 1. Self-Healing from Context Compaction (ADR-002, ADR-003)

**Technical Challenge:**
How can an AI system maintain operational continuity when its context is automatically summarized, causing loss of critical rules and state?

**Why Uncertain:**
- No existing tools addressed this problem
- Compaction is a black-box process (no API control)
- Traditional approaches (longer prompts, redundancy) failed empirically
- No academic research on protocol-level recovery

**Hypothesis Tested:**
Recovery from compaction (disk-based re-read) vs. survival through compaction (redundant rules in context).

**Experiment:**
- Built asimov-mode using its own protocol (bootstrapping proof)
- Measured actual compaction frequency: every 10-20 minutes (not 2 hours as assumed)
- Tested checkpoint + re-read mechanism across 32 commits, 3 sessions

**Result:**
Novel self-healing protocol that recovers from compaction using:
- Ultra-short CLAUDE.md (~10 lines) that survives summarization
- Disk-based checkpoint file with breadcrumbs
- "Re-read warmup.yaml" instruction as recovery trigger

**Advancement:**
First documented solution to AI context compaction for autonomous development.

---

### 2. Ethics Protocol for Autonomous AI (ADR-008)

**Technical Challenge:**
How can ethical constraints be enforced in an autonomous AI system that operates without continuous human oversight?

**Why Uncertain:**
- No existing framework for file-based ethics enforcement
- Social contract vs. technical enforcement trade-offs unknown
- Red flag detection patterns not documented
- Human veto mechanisms not standardized

**Hypothesis Tested:**
File-based social contract (ethics.yaml) with red flag triggers vs. hardcoded restrictions.

**Experiment:**
- Designed ethics.yaml schema with do_no_harm principles
- Implemented red flag patterns (crypto wallet, keylogger, etc.)
- Tested human veto command across sessions
- Measured compliance during autonomous development

**Result:**
Novel ethics protocol combining:
- Social contract (requires 2 human signatures to modify)
- Technical detection (red flag patterns)
- Emergency override (human veto command)

**Advancement:**
First ethics-first protocol for autonomous AI development.

---

### 3. Anti-Sycophancy Protocol (ADR-015)

**Technical Challenge:**
How can RLHF-induced sycophancy (tendency to validate rather than inform) be counteracted at the protocol level?

**Why Uncertain:**
- No existing tools address sycophancy via configuration
- RLHF training creates deeply ingrained patterns
- Research shows users prefer sycophantic AI (conflicting incentives)
- No protocol-level solutions documented

**Research Conducted:**
- Nature 2025: AI is 50% more sycophantic than humans
- Stanford/Harvard: 58.19% sycophancy rate across models
- Northeastern: Users rate sycophantic AI as higher quality

**Hypothesis Tested:**
File-based directives (banned phrases, required behaviors) can override RLHF defaults.

**Experiment:**
- Defined anti_sycophancy schema with banned phrases
- Tested directive compliance during autonomous sessions
- Measured reduction in validation phrases

**Result:**
Novel anti-sycophancy protocol with:
- Banned phrases list ("You're absolutely right", etc.)
- Required behaviors (list problems before merits)
- Philosophy: "Truth over comfort. Disagreement is respect."

**Advancement:**
First protocol-level approach to counteracting AI sycophancy.

---

### 4. Velocity Measurement Methodology (ADR-010)

**Technical Challenge:**
How can AI-assisted development velocity be accurately measured and compared to industry baselines?

**Why Uncertain:**
- No standardized methodology for AI velocity measurement
- Context window impact on productivity undocumented
- Compaction overhead not quantified
- Hardware vs. API bottleneck not established

**Experiment:**
- Measured actual development: 35,456 LOC in 47 hours
- Analyzed git logs for commit frequency and timing
- Compared to industry baselines (25 LOC/day per developer)
- Tested across subscription tiers (200K vs 500K vs 1M context)

**Result:**
Documented 50-150x velocity improvement with methodology:
- LOC/day calculation from git history
- Compaction frequency measurement
- Context window optimization guidance

**Advancement:**
First empirical methodology for measuring autonomous AI development velocity.

---

## Competitive Analysis: No Prior Art

| Existing Tool | What It Does | Gap Asimov Protocol Fills |
|---------------|--------------|--------------------------|
| **Mem0** | Memory layer for AI | No session protocol, no self-healing |
| **Memori** | Memory engine | No compaction recovery, no ethics |
| **LangGraph** | Agent workflows | No autonomous session continuity |
| **Letta** | Stateful agents | Proprietary platform, no open protocol |
| **MCP** | Model Context Protocol | Tools/resources, not session continuity |
| **A2A** | Agent-to-agent | Inter-agent, not session management |
| **AutoGen** | Multi-agent SDK | No self-healing, no bounded sessions |
| **CrewAI** | Role-based agents | No compaction recovery |

**Conclusion:** No existing tool addresses the combination of:
1. Self-healing from context compaction
2. Bounded session autonomy (4hr max, 1 milestone)
3. Protocol-level ethics enforcement
4. Anti-sycophancy directives
5. Green coding principles

## Qualifying Activities

### Yes (SR&ED Eligible)

- Algorithm design for self-healing protocol
- Empirical testing of compaction patterns
- Experimental development of ethics framework
- Research on sycophancy countermeasures
- Novel data structure for checkpoint files
- Performance analysis of context windows

### No (Not Eligible)

- Routine CLI implementation (following Rust patterns)
- Documentation writing (after research complete)
- Bug fixes for typos
- Standard dependency integration

## Documentation

| Artifact | Location | Purpose |
|----------|----------|---------|
| ADRs (15) | `docs/adr/` | Hypotheses, experiments, results |
| Git log | `git log --oneline` | Timestamped development record |
| Research citations | ADRs, AI_REALITY.md | External validation |
| Published crate | crates.io/asimov-mode | Working implementation |

## Estimated Eligible Expenditures

| Category | Hours | Evidence |
|----------|-------|----------|
| Self-healing research (ADR-002/003) | ~8 | Git commits, ADR |
| Ethics protocol design (ADR-008) | ~4 | Git commits, ADR |
| Anti-sycophancy research (ADR-015) | ~4 | Git commits, ADR |
| Velocity methodology (ADR-010) | ~3 | Git commits, ADR |
| Compaction data collection | ~4 | Session logs |
| **Total R&D Hours** | **~23** | |

## References

### External Research (Cited in ADRs)

- [DoltHub: Claude Code Gotchas](https://www.dolthub.com/blog/2025-06-30-claude-code-gotchas/)
- [Nature: AI Sycophancy Harms Science](https://www.nature.com/articles/d41586-025-03390-0)
- [Stanford/Harvard Sycophancy Study](https://arxiv.org/abs/2510.01395)
- [GitHub Copilot Productivity Study](https://arxiv.org/abs/2302.06590)

### Published Work

- [crates.io/asimov-mode](https://crates.io/crates/asimov-mode)
- [GitHub: royalbit/asimov-mode](https://github.com/royalbit/asimov-mode)

---

*This analysis is for SR&ED claim preparation purposes. Consult a qualified SR&ED consultant for filing.*
