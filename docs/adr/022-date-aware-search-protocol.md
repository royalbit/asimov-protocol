# ADR-022: Date-Aware Search Protocol (Freshness Protocol)

## Status

**ACCEPTED** - 2025-11-30

## Context

AI models have training data cutoffs. Claude Opus 4.5 and Sonnet 4.5 have a knowledge cutoff of **January 2025**. This creates a specific failure mode that users misinterpret as "hallucination":

1. User asks about something (API, library, law, price, company, etc.)
2. AI gives information that **was correct at training time**
3. Information has changed since cutoff (now 10+ months stale)
4. User receives outdated-but-confident answer
5. User discovers answer is wrong
6. User concludes: "AI hallucinated"

**The AI didn't hallucinate. It gave you January 2025's truth.**

### The Business Problem

Vendors could fix this by enabling web search by default, but:

- Search API calls cost money
- Inference alone is cheaper than search + inference
- Users blame "AI hallucination" not "vendor cost optimization"
- No accountability for stale data

### Real-World Examples

| User Query | AI Response (Jan 2025 data) | Reality (Nov 2025) | User Perception |
|------------|----------------------------|-------------------|-----------------|
| "What's the latest React version?" | "React 18.2" | React 19.x released | "AI hallucinated" |
| "Is Twitter called X?" | Mixed/confused | X fully rebranded | "AI is wrong" |
| "Claude Sonnet 4.5 pricing?" | Gives old pricing | Pricing changed | "AI made it up" |
| "Current OpenAI CEO?" | May give outdated info | Leadership changes | "AI is unreliable" |

### The Core Insight

**Stale data ≠ Hallucination**

- Hallucination: AI generates plausible fiction
- Stale data: AI gives correct-but-outdated facts

Users conflate these. The solution is different:
- Hallucination → Better training, grounding
- Stale data → **Force web search for time-sensitive queries**

## Decision

Implement a **Freshness Protocol** that instructs AI to:

1. **Be aware of its own cutoff date** (explicit in protocol)
2. **Recognize time-sensitive topics** (APIs, versions, prices, laws, current events)
3. **Default to web search** when topic is likely stale
4. **Disclose staleness risk** when search unavailable

### Protocol Definition

```yaml
# freshness.yaml - Date-Aware Search Protocol
freshness:
  status: "REQUIRED"

  model_cutoff:
    claude_opus_4_5: "2025-01"
    claude_sonnet_4_5: "2025-01"
    note: "Update when new models released"

  # Topics that change frequently - always search
  always_search:
    - "current version"
    - "latest release"
    - "pricing"
    - "API endpoint"
    - "library documentation"
    - "current CEO"
    - "stock price"
    - "exchange rate"
    - "recent news"
    - "today"
    - "this week"
    - "this month"
    - "2025"  # Any year after cutoff

  # Time-sensitive domains
  volatile_domains:
    - "cryptocurrency"
    - "AI/ML libraries"
    - "cloud provider APIs"
    - "social media platforms"
    - "startup companies"
    - "regulations"
    - "political"

  behavior:
    when_search_available: "Use WebSearch for time-sensitive queries"
    when_search_unavailable: |
      Disclose: "My knowledge cutoff is [DATE]. This information may be outdated.
      Please verify current status from official sources."

  on_uncertainty:
    - "Check if query contains time-sensitive keywords"
    - "Compare topic to volatile_domains"
    - "If today's date > cutoff + 3 months, prefer search"
    - "When in doubt, search"
```

### Integration with Session Start

Add to `warmup.yaml` session initialization:

```yaml
session_initialization:
  step_0_freshness:
    description: "Load date awareness"
    actions:
      - "Note today's date from <env>"
      - "Calculate months since cutoff"
      - "If > 3 months, enable aggressive search mode"
```

### CLAUDE.md Directive

Add to CLAUDE.md template:

```markdown
FRESHNESS: My cutoff is Jan 2025. Today is in <env>. For time-sensitive topics, SEARCH FIRST.
```

## Consequences

### Positive

1. **Reduces perceived hallucination** - Users get current info
2. **Builds trust** - AI acknowledges its limitations
3. **Explicit protocol** - Not hidden vendor decision
4. **User education** - Explains why search matters

### Negative

1. **Increases token usage** - Search adds context
2. **Slower responses** - Search takes time
3. **Vendor dependency** - Requires search capability

### Neutral

1. **Shifts cost** - From stale answers to search API calls
2. **Changes UX** - Users may need to wait for search

## Implementation

### Phase 1: Documentation (v6.1.0)
- Add freshness.yaml to protocol suite
- Update AI_REALITY.md with stale data section
- Update SPECIFICATION.md

### Phase 2: CLI Validation (v6.2.0)
- Validate freshness.yaml exists
- Warn if model_cutoff is outdated
- Check CLAUDE.md has freshness directive

### Phase 3: Tooling (Future)
- Auto-update model_cutoff from vendor APIs
- Detect stale-data-prone queries in logs

## References

- [LLM Knowledge Cutoff Dates](https://www.ofzenandcomputing.com/knowledge-cutoff-dates-llms/)
- AI_REALITY.md - Part 1.5: Training Data Cutoff
- ADR-016: Green Coding Protocol (related: search vs inference cost)

## Notes

This ADR was created after observing that users consistently misattribute "stale data" errors as "hallucinations." The distinction matters because the solutions are different.

**Hallucination:** AI needs better training/grounding
**Stale data:** AI needs to search

The Freshness Protocol makes AI's temporal limitations explicit and provides a systematic solution.
