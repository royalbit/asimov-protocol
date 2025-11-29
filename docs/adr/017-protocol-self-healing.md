# ADR-017: Protocol Self-Healing (Auto-Regeneration)

## Status

Proposed

## Date

2025-11-29

## Context

Protocol files can be deleted accidentally or intentionally. Current behavior:

| File Deleted | Current Behavior |
|--------------|------------------|
| ethics.yaml | Session HALTS, offers manual regeneration |
| green.yaml | Session WARNS, uses hardcoded defaults |
| warmup.yaml | Protocol non-functional |
| CLAUDE.md | Protocol non-functional (by design) |

Problems with current approach:
1. Manual regeneration requires user intervention
2. Accidental deletion breaks workflow
3. No distinction between "deleted" and "never existed"

## Decision

Implement **auto-regeneration** of missing protocol files during validation:

### 1. Auto-Regeneration Behavior

```
forge-protocol validate
```

When validation runs and detects missing files:

| File | Action | Rationale |
|------|--------|-----------|
| ethics.yaml | AUTO-CREATE + WARN | Ethics must exist, user should know |
| green.yaml | AUTO-CREATE + INFO | Green is required but less critical |
| warmup.yaml | AUTO-CREATE + WARN | Core protocol, user should know |
| sprint.yaml | SKIP | Optional file |
| roadmap.yaml | SKIP | Optional file |
| CLAUDE.md | NEVER AUTO-CREATE | Bootstrap must be intentional |

### 2. Regeneration Output

```
forge-protocol validate

⚠️  REGENERATED: ethics.yaml (was missing)
ℹ️  REGENERATED: green.yaml (was missing)
⚠️  REGENERATED: warmup.yaml (was missing)

  ✓ Ethics: HARDCODED (regenerated from defaults)

  OK ./warmup.yaml (warmup) [REGENERATED]
  OK ./ethics.yaml (ethics) [REGENERATED]
  OK ./green.yaml (green) [REGENERATED]

Success: 3 file(s) valid (3 regenerated)
```

### 3. CLAUDE.md Exception

CLAUDE.md is **never** auto-regenerated because:
- It's the bootstrap trigger - human must intentionally add it
- Auto-creating it would enable protocol without consent
- Deleting CLAUDE.md is the "off switch" for the protocol

### 4. Checksum Validation (Future)

For v4.1.5+, add optional checksum validation:

```yaml
# .forge/checksums.yaml
files:
  ethics.yaml:
    sha256: "abc123..."
    last_verified: "2025-11-29T10:00:00Z"
  green.yaml:
    sha256: "def456..."
    last_verified: "2025-11-29T10:00:00Z"
```

On validation:
- Compare current file hash to stored hash
- WARN if modified (not ERROR - modifications may be intentional)
- Update hash after successful validation with `--update-checksums`

### 5. CLI Commands

```bash
# Normal validation (auto-regenerates missing files)
forge-protocol validate

# Skip auto-regeneration
forge-protocol validate --no-regenerate

# Force regeneration even if files exist
forge-protocol init --skynet --force

# Update checksums after intentional modifications
forge-protocol validate --update-checksums
```

## Consequences

### Positive

1. **Resilience**: Accidental deletion doesn't break workflow
2. **Self-healing**: Protocol recovers automatically
3. **Visibility**: Regeneration is logged, not silent
4. **No surveillance**: Recovery without reporting

### Negative

1. **Magic behavior**: Files appear "from nowhere" (mitigated by warnings)
2. **Template drift**: Regenerated files use current templates, not original

### Neutral

1. **CLAUDE.md unchanged**: Still requires manual creation
2. **Git visibility**: Regenerated files appear in `git status`

## Implementation

### Phase 1: Auto-Regeneration (v4.1.5)
- [ ] Detect missing required files during validation
- [ ] Auto-create from templates
- [ ] Display regeneration warnings
- [ ] Add `--no-regenerate` flag

### Phase 2: Checksum Validation (v4.2.0)
- [ ] Generate checksums on `init`
- [ ] Validate checksums on `validate`
- [ ] Add `--update-checksums` flag
- [ ] Store in `.forge/checksums.yaml`

## Alternatives Considered

### 1. Surveillance/Reporting
**Rejected**: Violates privacy principles. Collecting user data for file modifications is overreach.

### 2. Make Files Undeletable
**Rejected**: Impossible on most filesystems, and users have legitimate reasons to remove files.

### 3. Require Manual Regeneration
**Current behavior**: Creates friction for accidental deletion. Auto-regeneration is better UX.

### 4. Silent Regeneration
**Rejected**: Users should know when files are created. Transparency over magic.

## References

- [ADR-011: Hardcoded Ethics](011-hardcoded-ethics.md)
- [ADR-016: Green Coding Protocol](016-green-coding-protocol.md)
- [Forge Protocol Specification](../SPECIFICATION.md)
