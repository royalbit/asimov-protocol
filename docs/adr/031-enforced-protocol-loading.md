# ADR-031: Enforced Protocol Loading + Dynamic Date Injection

**Status:** Accepted
**Date:** 2025-12-02
**Deciders:** Human + Claude (Principal Autonomous AI)
**Priority:** CRITICAL

## Context

### Problem 1: Protocols Are Suggestions

Current architecture:
```
warmup.yaml says: "Read freshness.yaml"
Claude: *might read it, might not*
```

Protocol files are **suggestions**. Users can modify `warmup.yaml` to skip loading protocols. This breaks the trust model.

### Problem 2: Hardcoded Dates

Current `freshness.yaml`:
```yaml
always_search:
  - "2025"  # Hardcoded - will be wrong in 2026
```

The protocol hardcodes the year. When 2026 arrives, the protocol is stale.

### Problem 3: Token Overhead

Loading 7 protocol files as YAML consumes ~2,000 tokens. With context compaction every ~15 minutes, this overhead matters.

### The Root Cause

Protocols are **data files** when they should be **code**.

| Current | Should Be |
|---------|-----------|
| YAML files on disk | Rust code in binary |
| User can edit/skip | User cannot bypass |
| Static dates | Dynamic from `<env>` |
| Verbose YAML | Token-optimized output |

## Decision

### 1. Rust Enforces Core Protocols

The `asimov warmup` command will **inject core protocols from Rust code**, not from YAML files.

```rust
// Core protocols are HARDCODED in the binary
const ASIMOV_PROTOCOL: &str = include_str!("protocols/asimov.tpl");      // Three Laws
const FRESHNESS_PROTOCOL: &str = include_str!("protocols/freshness.tpl"); // Search triggers
const SYCOPHANCY_PROTOCOL: &str = include_str!("protocols/sycophancy.tpl"); // Response style
const GREEN_PROTOCOL: &str = include_str!("protocols/green.tpl");         // Local-first
const SPRINT_PROTOCOL: &str = include_str!("protocols/sprint.tpl");       // Session boundaries
const WARMUP_PROTOCOL: &str = include_str!("protocols/warmup.tpl");       // Session bootstrap
```

User-editable YAML files become **optional overrides**, not the source of truth.

### 2. Dynamic Date Injection (Model-Agnostic)

```rust
fn inject_date(protocol: &str) -> String {
    let today = chrono::Local::now().format("%Y-%m-%d");
    let year = chrono::Local::now().format("%Y");
    protocol.replace("{TODAY}", &today.to_string())
           .replace("{YEAR}", &year.to_string())
}
```

**Important:** We do NOT hardcode the model's cutoff date. Different models have different cutoffs (Opus 4.5: Jan 2025, future Opus 5: unknown). Claude knows its own cutoff.

Output:
```
FRESHNESS: You have a training cutoff. Today is 2025-12-02.
For time-sensitive topics (versions, pricing, APIs, 2025), SEARCH FIRST.
```

**What we inject:**
- `{TODAY}` - from env (dynamic)
- `{YEAR}` - from env (dynamic)
- Search triggers - static list
- Reminder to be cutoff-aware - static

**What we DON'T inject:**
- Specific cutoff date - Claude knows this already
- Model name - Claude knows this already

### 3. Token-Optimized Output

`asimov warmup` outputs a **compiled context blob**, not raw YAML.

**Format:** Minified JSON with full words (no abbreviations - Claude must understand).

```json
{"asimov":{"harm":["financial","physical","privacy","deception"],"veto":["stop","halt","abort"]},"freshness":{"today":"2025-12-02","year":"2025","search":["version","pricing","api","current","latest"]},"sycophancy":{"truth_over_comfort":true,"disagree_openly":true},"green":{"local_first":true},"sprint":{"max_hours":4,"stop_on":["roadmap_exhausted","blocked","human_stop"]},"warmup":{"on_start":["read_protocols","validate","present_milestone"]}}
```

**Why full words:** Abbreviations like "fin" are ambiguous (finance? finish? Finland?). Claude needs unambiguous terms.

Estimated savings: **~40% token reduction** (2,000 → 1,200 tokens).

### 4. Loading Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│  asimov warmup                                              │
├─────────────────────────────────────────────────────────────┤
│  LAYER 1: HARDCODED (Rust binary, non-negotiable)           │
│    - asimov.yaml (Three Laws)                               │
│    - freshness.yaml (with dynamic date)                     │
│    - sycophancy.yaml (response style)                       │
│    - green.yaml (local-first)                               │
│    - sprint.yaml (session boundaries)                       │
│    - warmup.yaml (session bootstrap)                        │
├─────────────────────────────────────────────────────────────┤
│  LAYER 2: PROJECT DATA (from .asimov/*.yaml, not enforced)  │
│    - roadmap.yaml (what to build)                           │
│    - changelog.md (what was built)                          │
├─────────────────────────────────────────────────────────────┤
│  LAYER 3: OPTIONAL                                          │
│    - migrations.yaml (if exists)                            │
└─────────────────────────────────────────────────────────────┘
```

### 5. Override Mechanism

Projects can **extend** but not **disable** core protocols:

```yaml
# .asimov/asimov.yaml - EXTENDS, doesn't replace
asimov:
  extend:
    red_flags:
      - "custom_pattern"  # Added to core list
  # Cannot remove core red_flags
  # Cannot disable first_law
```

## Implementation

### Phase 1: Core Protocol Templates
- Create `src/protocols/*.tpl` with token-optimized templates
- Add `{TODAY}`, `{YEAR}`, `{CUTOFF}` placeholders
- Embed in binary via `include_str!`

### Phase 2: Dynamic Injection
- `asimov warmup` resolves placeholders at runtime
- Output format: minified single-line per protocol
- Total output < 1000 tokens

### Phase 3: Validation Update
- `asimov validate` checks extension files, not core
- Core protocols always pass (they're in the binary)
- Warn if user tries to weaken core protocols

## Consequences

### Positive
- **Tamper-proof**: Core protocols cannot be bypassed
- **Always current**: Date injected at runtime
- **Token efficient**: ~70% reduction
- **Single source of truth**: Rust binary, not scattered YAML

### Negative
- **Less flexible**: Users can't customize core protocols
- **Binary updates**: Protocol changes require new release
- **Migration**: Existing `.asimov/` files become optional overrides

### Neutral
- YAML files remain for project-specific config
- `asimov validate` still validates user files
- Git history preserved for auditing

## Security Implications

This ADR strengthens security:

| Before | After |
|--------|-------|
| Attacker modifies asimov.yaml | Three Laws hardcoded in binary |
| Fork removes Three Laws | Binary still enforces them |
| User "forgets" to load freshness | Freshness always injected |

The social contract becomes a **technical contract**.

## References

- [ADR-011: Hardcoded Ethics](011-hardcoded-ethics.md) - Partial solution, this completes it
- [ADR-012: Hardcoded Green Coding](012-hardcoded-green-coding.md) - Partial solution
- [ADR-022: Freshness Protocol](022-date-aware-search-protocol.md) - The protocol being enforced
- [ADR-020: The Open Foundation](020-asimov-mode-open-foundation.md) - Trust model

## Resolved Questions

1. **Output format**: Minified JSON with full words (not abbreviations)
   - Custom DSL: Rejected - Claude wasn't trained on it, might misparse
   - TOON: Rejected - too new (Nov 2025), unproven
   - Abbreviations: Rejected - ambiguous ("fin" = finance? Finland?)

2. **Template syntax**: Simple `{TODAY}`, `{YEAR}` placeholders

3. **Model cutoff**: NOT hardcoded - Claude knows its own cutoff, we just inject today's date

## Open Questions

1. **Extension validation**: How strict on override attempts?
2. **Sprint/roadmap format**: Also minified JSON, or keep YAML for readability?

These will be resolved during implementation.
