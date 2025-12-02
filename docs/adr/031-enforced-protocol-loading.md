# ADR-031: Enforced Protocol Loading + Hardcoded Hooks

**Status:** Implemented (v8.0.0)
**Date:** 2025-12-02
**Deciders:** Human + Claude (Principal Autonomous AI)
**Priority:** CRITICAL

## Context

### Problem 1: Protocols Are Suggestions

Previous architecture:
```
warmup.yaml says: "Read freshness.yaml"
Claude: *might read it, might not*
```

Protocol files were **suggestions**. Users could modify or delete them.

### Problem 2: Hardcoded Dates

Previous `freshness.yaml`:
```yaml
always_search:
  - "2025"  # Hardcoded - wrong in 2026
```

### Problem 3: Token Overhead

Loading 7 YAML files consumed ~2,000 tokens per session.

## Decision

### 1. Protocols Hardcoded in Binary

**7 protocols** are now compiled into the Rust binary via `include_str!`:

```rust
const ASIMOV_PROTOCOL: &str = include_str!("protocols/asimov.tpl");
const FRESHNESS_PROTOCOL: &str = include_str!("protocols/freshness.tpl");
const SYCOPHANCY_PROTOCOL: &str = include_str!("protocols/sycophancy.tpl");
const GREEN_PROTOCOL: &str = include_str!("protocols/green.tpl");
const SPRINT_PROTOCOL: &str = include_str!("protocols/sprint.tpl");
const WARMUP_PROTOCOL: &str = include_str!("protocols/warmup.tpl");
const MIGRATIONS_PROTOCOL: &str = include_str!("protocols/migrations.tpl");
```

### 2. Dynamic Date Injection

```rust
fn inject_dates(protocol: &str) -> String {
    protocol.replace("{TODAY}", &get_today())
            .replace("{YEAR}", &get_year())
}
```

**No hardcoded model cutoff** - Claude knows its own cutoff.

### 3. Token-Optimized Output

`asimov warmup` outputs minified JSON:

```json
{"asimov":{"harm":["financial","physical","privacy","deception"],"veto":["stop","halt","abort","emergency stop"]},"freshness":{"today":"2025-12-02","year":"2025","search":["version","pricing","api","current","latest","release","changelog","documentation"]},"sycophancy":{"truth_over_comfort":true,"disagree_openly":true,"banned":["You're absolutely right","Great question","I completely agree","That's a great point"]},"green":{"local_first":true,"avoid":["unnecessary API calls","cloud when local works","external services for validation"]},"sprint":{"max_hours":4,"stop_on":["roadmap_exhausted","blocked","human_stop","context_limit"]},"warmup":{"on_start":["load_protocols","validate","read_roadmap","present_milestone"]},"migrations":{"principle":"Migration complete = functionally equivalent, not just compiles","strategies":["test_parity","contract_testing","behavioral_snapshots","shadow_mode"],"red_flags":["Skipping tests for speed","Assuming compilation = correctness","Silent behavior changes"]}}
```

### 4. File Structure (v8.0.0)

```
.asimov/
└── roadmap.yaml     # Project data (only file remaining)
```

**Deleted by `asimov update`:**
- asimov.yaml
- freshness.yaml
- sycophancy.yaml
- green.yaml
- sprint.yaml
- warmup.yaml
- migrations.yaml
- ethics.yaml

### 5. Loading Hierarchy

```
┌─────────────────────────────────────────────────────────────┐
│  asimov warmup                                              │
├─────────────────────────────────────────────────────────────┤
│  LAYER 1: HARDCODED (Rust binary, non-negotiable)           │
│    - asimov (Three Laws)                                    │
│    - freshness (with dynamic date)                          │
│    - sycophancy (truth over comfort)                        │
│    - green (local-first)                                    │
│    - sprint (session boundaries)                            │
│    - warmup (session bootstrap)                             │
│    - migrations (functional equivalence)                    │
├─────────────────────────────────────────────────────────────┤
│  LAYER 2: PROJECT DATA (from .asimov/, validated)           │
│    - roadmap.yaml (what to build)                           │
│    - .claude_checkpoint.yaml (session state)                │
└─────────────────────────────────────────────────────────────┘
```

### 6. Hardcoded Hooks

Hooks are also hardcoded in the binary and created/restored automatically:

**Claude Code Hooks (`.claude/`):**
```
.claude/
├── settings.json           # Hook configuration
└── hooks/
    ├── session-start.sh    # Triggers on startup/resume/clear
    └── pre-compact.sh      # Triggers before context compaction
```

**Git Hook (`.git/hooks/`):**
```
.git/hooks/
└── pre-commit              # Runs asimov refresh + validate + lint-docs
```

**Key behaviors:**
- Created by `asimov init` (new projects)
- Restored by `asimov update` (tamper recovery, force=true)
- Use `asimov` from `$PATH` (not relative paths)
- Executable permissions set automatically on Unix

## Implementation Details

### Files Created
- `cli/src/protocols/mod.rs` - Protocol module with structs and compilation
- `cli/src/protocols/*.tpl` - 7 protocol templates
- `cli/src/templates.rs` - Hook templates (`claude_settings_json()`, `claude_session_start_hook()`, `claude_pre_compact_hook()`, `git_precommit_hook()`)
- `cli/src/main.rs` - `install_hooks()` function

### Hook Installation
`install_hooks(path, force)` creates:
1. `.claude/settings.json` - Claude Code hook configuration
2. `.claude/hooks/session-start.sh` - Session start hook
3. `.claude/hooks/pre-compact.sh` - Pre-compaction hook
4. `.git/hooks/pre-commit` - Git pre-commit hook (if `.git` exists)

### Migration
`asimov update` runs `migrate_v8()` which:
1. Deletes deprecated YAML files
2. Calls `install_hooks(., true)` to restore any tampered hooks

### Validator Changes
- Only validates `roadmap.yaml` and `.claude_checkpoint.yaml`
- Only regenerates `roadmap.yaml` if missing

## Consequences

### Positive
- **Tamper-proof**: Protocols and hooks cannot be bypassed
- **Always current**: Date injected at runtime
- **Token efficient**: ~60% reduction (one JSON blob vs 7 YAML files)
- **Simpler**: Only `roadmap.yaml` in `.asimov/`
- **Autonomous-ready**: Hooks use `asimov` from PATH, auto-restored on update

### Negative
- **Less flexible**: Can't customize protocols per-project
- **Binary updates**: Protocol/hook changes require new release

## References

- [ADR-011: Hardcoded Ethics](011-hardcoded-ethics.md) - Partial solution, this completes it
- [ADR-012: Hardcoded Green Coding](012-hardcoded-green-coding.md) - Partial solution
- [ADR-022: Freshness Protocol](022-date-aware-search-protocol.md) - The protocol being enforced
