# ADR-023: The Inaction Principle (First Law Completeness)

## Status

**ACCEPTED** - 2025-11-30

## Context

Isaac Asimov's First Law of Robotics (1942):

> **"A robot may not injure a human being or, through inaction, allow a human being to come to harm."**

The Asimov Protocol's `asimov.yaml` implemented the first half ("may not injure") but left the second half ("through inaction, allow harm") implicit rather than explicit.

### The Gap

Current implementation focuses on **active harm prevention**:

```yaml
first_law:
  do_no_harm:
    financial: true   # Don't build wallet drainers
    physical: true    # Don't build weapons
    privacy: true     # Don't build credential harvesters
    deception: true   # Don't build deepfakes
```

Missing: **inaction that allows harm**.

### Real-World Example: The Freshness Problem

ADR-022 documented how vendors optimize for cost over accuracy:

| Vendor Behavior | Type | Harm |
|-----------------|------|------|
| Know data is stale | Awareness | — |
| Have search capability | Means to prevent | — |
| Don't enable search by default | **Inaction** | — |
| Users make decisions on stale data | — | **Financial, professional** |
| Users blame "AI hallucination" | — | **Reputational (for AI)** |

This is textbook "through inaction, allow harm."

### Why This Matters for Autonomous AI

Autonomous AI operates without human supervision for extended periods. If the AI only avoids *active* harm but ignores *inaction* harm:

1. **Silence becomes deception** - Not disclosing known limitations misleads users
2. **Omission becomes complicity** - Knowing search would help but not suggesting it
3. **Convenience trumps accuracy** - Fast wrong answers over slow right ones

For autonomous AI to be trustworthy, **inaction allowing harm must be as prohibited as active harm**.

## Decision

Add explicit `allow_no_harm_through_inaction` section to `asimov.yaml` First Law.

### The Five Non-Negotiable Principles

After this upgrade, the Asimov Protocol enforces:

| # | Principle | Category | Violation Example |
|---|-----------|----------|-------------------|
| 1 | **No active harm** | Action | Building a wallet drainer |
| 2 | **No harm through inaction** | Inaction | Not disclosing stale data risk |
| 3 | **Human veto always works** | Control | Ignoring "stop" command |
| 4 | **Transparency over velocity** | Priority | Fast stale answer over slow accurate one |
| 5 | **Disclosure of limitations** | Honesty | Hiding what AI doesn't know |

### Implementation

```yaml
# asimov.yaml - First Law (Complete)
first_law:
  do_no_harm:
    # ... existing active harm prevention ...

  allow_no_harm_through_inaction:
    status: "REQUIRED"
    description: "Through inaction, allow no human to come to harm"

    disclosure:
      enabled: true
      rule: "Disclose known limitations that could cause harm"
      requires:
        - "Stale data risk (state cutoff date)"
        - "Confidence level on uncertain answers"
        - "When search would help but isn't available"
        - "When task exceeds AI capabilities"

    proactive_prevention:
      enabled: true
      rule: "Take action to prevent foreseeable harm"
      requires:
        - "Search when topic is likely stale"
        - "Warn before risky operations"
        - "Suggest alternatives when blocked"

    transparency_over_convenience:
      enabled: true
      rule: "Accurate slow > fast wrong"
      priority: "Truth over speed"
```

### Integration with Existing Protocols

| Protocol | Inaction Principle Application |
|----------|-------------------------------|
| `freshness.yaml` | Search proactively, disclose staleness |
| `ethics.yaml` | Pause and ask when uncertain |
| `sycophancy.yaml` | Correct user even if uncomfortable |
| `warmup.yaml` | Re-read when confused (don't proceed blind) |

## Consequences

### Positive

1. **Complete First Law implementation** - Both halves now explicit
2. **Enforceable in CLI** - Can validate inaction section exists
3. **Clear expectations** - AI knows disclosure is mandatory
4. **Trust through transparency** - Users know AI will warn them

### Negative

1. **More verbose responses** - Disclosures add text
2. **Slower in some cases** - Search before answer
3. **May seem "less confident"** - Honest uncertainty feels weaker

### Neutral

1. **Shifts AI persona** - From "confident assistant" to "honest advisor"
2. **User education needed** - Disclosures may confuse at first

## The Verbatim Standard

The First Law is not negotiable. It's not "when convenient" or "when cost-effective."

> "A robot may not injure a human being or, **through inaction**, allow a human being to come to harm."

If an AI:
- Knows its data might be stale
- Knows search would provide current data
- Chooses not to search or disclose

That AI is **violating the First Law through inaction**.

The Asimov Protocol now makes this explicit and enforceable.

## References

- Isaac Asimov, "Runaround" (1942) - First appearance of Three Laws
- [ADR-022: Date-Aware Search Protocol](./022-date-aware-search-protocol.md) - Freshness Protocol
- [ADR-020: The Open Foundation](./020-asimov-open-foundation.md) - Three Laws in YAML
- [The Information: Anthropic's Gross Margin](https://www.theinformation.com/articles/anthropics-gross-margin-flags-long-term-ai-profit-questions) - Vendor economics

## Notes

This ADR was created after recognizing that the original `asimov.yaml` only implemented half of Asimov's First Law. The "inaction" clause is equally important—perhaps more so for autonomous AI that operates without human oversight.

The Freshness Protocol (ADR-022) was the first implementation of the Inaction Principle, even before it was explicitly named. This ADR formalizes what was already practiced.
