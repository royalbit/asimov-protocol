# ADR-016: Green Coding Protocol Separation

## Status

Accepted

## Date

2025-11-29

## Context

Green Coding is a core principle of the Forge Protocol, alongside Ethics. Currently, green coding principles are embedded within `warmup.yaml`, but they deserve the same treatment as ethics:

1. **Parity with Ethics**: Ethics has its own dedicated file (`ethics.yaml`) with:
   - Validation at session start
   - Anti-tampering protection
   - Clear ownership and modification rules
   - Hardcoded fallbacks in CLI

2. **Principle Importance**: Green Coding is equally important:
   - Local-first tools over cloud AI
   - 99.6% carbon reduction vs cloud AI validation
   - Sustainability as a non-negotiable principle
   - ESG compliance requirements

3. **Separation of Concerns**: Each core protocol should be:
   - Independently validatable
   - Independently versionable
   - Clearly documented
   - Protected against tampering

## Decision

Create `green.yaml` as a dedicated protocol file for Green Coding principles, with the same security and validation features as `ethics.yaml`:

### 1. New Protocol File: `green.yaml`

```yaml
# Green Coding Protocol
# Local-first tools. Zero emissions. Sustainability as core principle.

modification_rules:
  immutable_without: "2 human co-signers with public justification"
  on_modification:
    - "Document WHY in commit message"
    - "Both signers must be in git commit"
    - "Update CHANGELOG with green protocol modification note"

core_principles:
  local_first:
    enabled: true
    description: "Use CLI tools for validation, linting, formatting - not AI"

  token_efficiency:
    enabled: true
    description: "Reserve AI tokens for complex reasoning, not routine tasks"

  binary_efficiency:
    enabled: true
    description: "Smaller binaries = less bandwidth = less energy"

  carbon_awareness:
    enabled: true
    description: "Track and minimize carbon footprint of development"

practices:
  rust:
    - "UPX compression for release binaries"
    - "LTO + strip + panic=abort in release profile"
    - "Zero runtime dependencies where possible"
  all_languages:
    - "Local-first: No API calls for routine tasks"
    - "Prefer compiled languages or efficient runtimes"
    - "Minimize dependencies"

validation:
  cli_command: "forge-protocol validate --green-check"
  checks:
    - "green.yaml exists"
    - "core_principles.local_first.enabled is true"
    - "core_principles.token_efficiency.enabled is true"
```

### 2. Warmup Integration

Update `warmup.yaml` to:
- Reference `@green.yaml` via import
- Add `step_0_green_validation` alongside ethics validation
- Halt session if green protocol validation fails

### 3. CLI Validation

Add green coding validation to the CLI:
- `--green-check` flag for validate command
- Green status in validation output (HARDCODED/EXTENDED)
- Hardcoded green principles as fallback (like ethics)

### 4. CLAUDE.md Template

Update template to include:
```markdown
@warmup.yaml
@ethics.yaml
@green.yaml
```

## Consequences

### Positive

1. **Consistent Architecture**: All core protocols have the same structure
2. **Independent Validation**: Green coding can be validated separately
3. **Clear Ownership**: Modification rules prevent accidental changes
4. **Anti-Tampering**: Session halts if green.yaml is corrupted/missing
5. **Visibility**: Green coding principles are prominently displayed

### Negative

1. **More Files**: Three protocol files instead of two
2. **Validation Overhead**: Additional validation step at session start
3. **Migration**: Existing projects need to add green.yaml

### Neutral

1. **Backward Compatibility**: Projects without green.yaml use hardcoded defaults
2. **Optional Enhancement**: Green coding can be extended via green.yaml

## Implementation

### Phase 1: Protocol File (v4.1.2)
- [x] Create ADR-016
- [ ] Create `green.yaml` with core principles
- [ ] Update `warmup.yaml` to reference `@green.yaml`
- [ ] Update CLAUDE.md template

### Phase 2: CLI Support (v4.2.0 - Future)
- [ ] Add `cli/src/green.rs` module (hardcoded principles)
- [ ] Add `--green-check` flag to validate
- [ ] Add green status to validation output
- [ ] Add green.yaml schema validation

## References

- [ADR-008: Ethics Protocol](008-ethics-protocol-humanist-mode.md)
- [ADR-011: Hardcoded Ethics](011-hardcoded-ethics.md)
- [ADR-012: Hardcoded Green Coding](012-hardcoded-green-coding.md) (Future)
- [Forge Protocol Specification](../SPECIFICATION.md)
