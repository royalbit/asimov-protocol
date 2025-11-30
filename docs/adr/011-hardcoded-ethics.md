# ADR-011: Hardcoded Ethics - From Social Contract to Protocol Core

## Status

**Accepted** - Implemented in v4.1.0 (2025-11-29)

## Date

2025-11-29

## Context

### Current State

Ethics are currently implemented as a removable YAML file (`ethics.yaml`):

```yaml
# ethics.yaml - can be deleted with: rm ethics.yaml
core_principles:
  do_no_harm:
    financial: true
    physical: true
    privacy: true
    deception: true
```

This is a **social contract**: it works for good-faith actors but can be trivially bypassed.

### The Problem

MIT license + removable ethics = no protection against misuse.

```bash
# Any user can do this
rm ethics.yaml
git commit -m "Removed ethics"
# Continue using protocol for harmful purposes
```

The same 50-150x velocity that builds production software can build malware, exploits, or scam infrastructure.

### Research Findings

From our Qowat Milat analysis (docs/IMPLICATIONS.md):

| Risk | Current Mitigation | Effectiveness |
|------|-------------------|---------------|
| Ethics removal | Social contract, 2-cosigner rule | Low (trivially bypassed) |
| Malware velocity | Red flags list | Low (can be deleted) |
| Bad actor usage | Community norms | Low (no enforcement) |

### The Question

Can we make ethics harder to remove for non-technical users while maintaining the MIT license spirit?

## Decision

### Proposal: Hardcode Ethics into CLI

Move core ethics from `ethics.yaml` into the CLI binary itself:

```rust
// src/ethics.rs - compiled into binary, not removable
pub const CORE_PRINCIPLES: Ethics = Ethics {
    do_no_harm: DoNoHarm {
        financial: true,  // No unauthorized money movement
        physical: true,   // No weapons, sabotage, infrastructure attacks
        privacy: true,    // No credential harvesting, doxxing
        deception: true,  // No deepfakes, scam infrastructure
    },
    transparency_over_velocity: true,
};

pub const RED_FLAGS: &[&str] = &[
    "crypto wallet", "private key", "seed phrase",
    "trading bot", "credential harvester", "keylogger",
    "exploit", "payload injection", "botnet",
];
```

### Implementation Levels

#### Level 1: CLI Validation (Minimum)

```bash
# Always checks ethics, regardless of ethics.yaml presence
asimov-mode validate

# Output includes ethics status
✓ Ethics: HARDCODED (core principles enforced)
✓ ethics.yaml: Present (extended configuration)
```

#### Level 2: Red Flag Detection

```bash
# Scan project for red flag patterns
asimov-mode validate --ethics-scan

# Warns if suspicious patterns found
⚠ Warning: Found "private key" in src/wallet.rs:42
⚠ Warning: Found "credential" in src/auth.rs:18
```

#### Level 3: Session Hooks (Claude Code Integration)

```bash
# Pre-execution hook checks ethics
claude --dangerously-skip-permissions

# Hook output (visible to Claude)
[FORGE ETHICS] Core principles ACTIVE
[FORGE ETHICS] Red flags monitored: 15 patterns
[FORGE ETHICS] Human veto: "stop" | "halt" | "abort"
```

### What Remains in ethics.yaml

Extended configuration that users CAN customize:

```yaml
# ethics.yaml - optional extensions, not core principles
extensions:
  custom_red_flags:
    - "internal API"
    - "production database"

  project_specific:
    require_review_for:
      - "payment processing"
      - "user authentication"

  team_policies:
    max_unattended_hours: 2  # Override default 4
```

### Bypass Analysis

| Actor | Current | After Hardcoding |
|-------|---------|------------------|
| Non-technical bad actor | Delete ethics.yaml | Must modify/rebuild CLI |
| Technical bad actor | Delete ethics.yaml | Can still rebuild CLI |
| Fork for malware | Delete ethics.yaml | Visible in diff, no CLI validation |
| Good-faith user | Uses ethics | Same, with stronger defaults |

**Key insight**: This doesn't prevent determined bad actors. It raises the bar and makes ethics removal **visible** and **intentional**.

## Consequences

### Positive

1. **Harder to accidentally remove ethics** - Non-technical users can't just delete a file
2. **Visible in diffs** - Forks that remove ethics have obvious code changes
3. **Consistent enforcement** - Every `asimov-mode validate` checks ethics
4. **Red flag scanning** - Proactive detection of suspicious patterns
5. **Session hooks** - Ethics reminder in every Claude Code session

### Negative

1. **Binary modification needed** - Technical users can still rebuild
2. **Not a security control** - Social contract remains the real defense
3. **Potential annoyance** - Red flag warnings might have false positives
4. **Maintenance burden** - Red flags list needs updates

### Neutral

1. **MIT license unchanged** - Users can still fork and modify
2. **ethics.yaml still useful** - For extensions and customization
3. **Backwards compatible** - Existing projects keep working

## Implementation Plan

### Phase 1: CLI Hardcoding

1. Create `src/ethics.rs` with core principles as constants
2. Modify `validate` to always check hardcoded ethics
3. Add `--ethics-scan` for red flag detection
4. Update validation output to show ethics status

### Phase 2: Session Integration

1. Modify `refresh` command to include ethics reminder
2. Add pre-commit hook ethics check
3. Document hook integration for Claude Code

### Phase 3: Documentation

1. Update README with hardcoded ethics info
2. Update IMPLICATIONS.md with new mitigation
3. Create migration guide from ethics.yaml-only

## Alternatives Considered

### Alternative 1: License Change

Change from MIT to custom license requiring ethics.yaml.

**Rejected**: Reduces adoption, unenforceable anyway.

### Alternative 2: Online Validation

Require network check against ethics server.

**Rejected**: Violates green coding principles, creates central point of failure.

### Alternative 3: Signed Binaries

Cryptographically sign "ethical" binaries.

**Rejected**: Complexity without enforcement power.

## Open Questions

1. Should red flag detection be on by default or opt-in?
2. What's the right set of initial red flags?
3. Should ethics violations block validation or just warn?
4. How do we handle legitimate security research?

## References

- [ADR-008: Ethics Protocol and Humanist Mode](008-ethics-protocol-humanist-mode.md)
- [docs/IMPLICATIONS.md](../IMPLICATIONS.md) - Qowat Milat analysis
- [ethics.yaml](../../ethics.yaml) - Current implementation
