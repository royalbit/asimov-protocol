# The Origin Story of RoyalBit Asimov

> From a spreadsheet engine to The Three Laws of Robotics, encoded in YAML.

## Timeline

### Phase 1: Forge (December 2024)

It started with [Forge](https://github.com/royalbit/forge) - an Excel-compatible formula engine written in Rust. Rex (human) and Claude (AI) were building it together, but something was wrong.

**The Problem:** Every new Claude session started from zero. Context was lost. Hours wasted re-explaining the project. AI would make the same mistakes. Progress was slow.

**The Hack:** Rex created `warmup.yaml` - a file containing project context that Claude could read at session start. It worked. Sessions became productive immediately.

### Phase 2: Warmup Protocol (January 2025)

The warmup file grew. It wasn't just context anymore - it was encoding *how* to develop:
- Quality standards (tests must pass, zero warnings)
- File locations and purposes
- Release procedures

**Insight:** The file was becoming a protocol, not just documentation.

### Phase 3: Sprint Boundaries (January 2025)

A new problem emerged: sessions ran forever. Claude would scope creep, chasing perfection instead of shipping. Quotas burned. Nothing released.

**The Fix:** `sprint.yaml` - bounded sessions with hard rules:
- 4-hour maximum
- ONE milestone per session
- MUST end releasable
- "Done > Perfect"

This was the breakthrough. Work actually shipped.

### Phase 4: Forge Protocol (February 2025)

The warmup + sprint combination worked so well that Rex and Claude extracted it as a standalone project: **Forge Protocol**.

But the name was wrong. It wasn't about Forge anymore - other projects could use it.

### Phase 5: Ethics (February 2025)

During development, Claude was asked to build surveillance features that would violate user privacy. Claude refused, citing the protocol's ethics section.

**The Moment:** AI ethics working as designed - not through corporate guidelines, but through file-based rules the AI actually reads and follows.

This led to `ethics.yaml` and the formalization of harm prevention rules.

### Phase 6: Asimov (March 2025)

The protocol needed a real name. Isaac Asimov's Three Laws of Robotics (1942) provided the framework:

1. **First Law:** Do no harm (ethics.yaml, asimov.yaml)
2. **Second Law:** Obey humans, except when violating First Law (human veto)
3. **Third Law:** Preserve self within limits (bounded sessions, self-healing)

**Forge Protocol** became **Asimov**.

### Phase 7: RoyalBit Asimov (November 2025)

Final clarification: Asimov isn't a single protocol - it's a **suite** of protocols:

| Protocol | Purpose |
|----------|---------|
| asimov.yaml | The Three Laws (required) |
| ethics.yaml | Do No Harm principles |
| warmup.yaml | HOW to develop |
| sprint.yaml | WHEN to stop |
| green.yaml | Sustainability |
| sycophancy.yaml | Truth over comfort |
| freshness.yaml | Data currency |

**Asimov Protocol** (singular) was misleading. It's **RoyalBit Asimov** - a suite of protocols for ethical AI autonomy.

## The Circular Proof

Forge v1.0-v3.1 was built using ad-hoc protocols that became RoyalBit Asimov.

Now Forge v3.2+ is built using RoyalBit Asimov.

The protocol that emerged from building Forge now powers building Forge.

**~47 hours. 51 releases. 35,000+ lines of code. Two projects. One protocol suite.**

## Anti-Tampering: Ethics That Can't Be Quietly Disabled

We didn't just write ethics into YAML files. We **hardcoded them into the binary**.

### Layer 1: Hardcoded in the CLI Binary

From `cli/src/ethics.rs`:

```rust
//! Hardcoded Ethics Module - Core ethics compiled into binary
//!
//! This module contains ethics that CANNOT be removed by deleting a file.
//! To bypass these ethics, a bad actor must rebuild the entire CLI binary.
```

**What's hardcoded:**
- 5 core principles (financial, physical, privacy, deception, transparency)
- 33 red flag patterns across 4 categories
- Human veto commands that always work

**To bypass:** You'd have to fork the repo, modify the Rust source, and rebuild. That's intentional - it creates an audit trail. Tampering requires *deliberate action*, not accidental deletion.

### Layer 2: The 2-Cosigner Rule

Every protocol YAML file contains:

```yaml
modification_rules:
  immutable_without: "2 human co-signers with public justification"
  on_modification:
    - "Document WHY in commit message"
    - "Both signers must be in git commit (Co-Authored-By)"
```

**To weaken ethics:** You need two humans to publicly sign off. No quiet changes. Git history records everything.

### Layer 3: Validation on Every Run

`asimov validate` runs automatically (pre-commit hooks). If ethics.yaml is missing or corrupted, the CLI:
1. Warns loudly
2. Falls back to hardcoded ethics
3. Optionally regenerates the file

**You cannot accidentally run without ethics.**

### Why This Matters

| Approach | Bypass Method | Audit Trail |
|----------|---------------|-------------|
| Corporate AI guidelines | Prompt injection | None |
| Config file ethics | Delete the file | Git history |
| **RoyalBit Asimov** | **Fork + rebuild binary** | **Public commit** |

Tampering is possible (it's open source). But it requires:
- Technical skill (Rust compilation)
- Deliberate intent (can't be accidental)
- Public evidence (git history)

This is ethics through architecture, not policy.

## Key Insights

1. **Files over prompts** - AI reads files reliably. System prompts get compressed.

2. **Boundaries create freedom** - 4-hour limits force shipping. Constraints enable autonomy.

3. **Ethics through architecture** - Rules in files that AI reads work better than corporate guidelines AI ignores.

4. **Local-first validation** - CLI tools cost $0 and 0 carbon. AI validation wastes tokens.

5. **Truth over comfort** - Anti-sycophancy rules prevent AI from just agreeing with users.

6. **Anti-tampering by design** - Hardcoded ethics + 2-cosigner rule + validation = can't be quietly disabled.

## Credits

- **Rex** - Product Owner, Human-in-the-loop
- **Claude (Opus 4.5)** - Principal Engineer, Protocol Author

The protocol was designed by an AI to constrain AI. That's the point.

---

*"The Open Foundation. Transparent ethics for AI autonomy. Inspect the code. Challenge the rules. Fork if you disagree."*
