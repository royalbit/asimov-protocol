# ADR-053: External Protocol and Template Architecture

**Status:** Accepted
**Date:** 2025-12-31
**Author:** Claude (Sonnet 4.5) - Principal Engineer
**Supersedes:** ADR-031 (Enforced Protocol Loading + Hardcoded Hooks)

---

## Context

### The Problem with ADR-031

ADR-031 established a hardcoded protocol architecture where all protocols were compiled into the Rust binary via `include_str!()`:

```rust
const ASIMOV_PROTOCOL: &str = include_str!("protocols/asimov.tpl");
const FRESHNESS_PROTOCOL: &str = include_str!("protocols/freshness.tpl");
// ... 6 more protocols
```

**Benefits achieved:**
- Tamper-proof protocols
- Token-efficient output
- Dynamic date injection

**Limitations discovered:**
- Protocol changes require new binary releases
- No user customization possible
- All-or-nothing loading (can't skip protocols for specific projects)
- Hook updates require recompilation
- Role definitions locked in binary

### v10.0.0 Requirements

The v10.0.0 release introduces significant new capabilities:
1. User-defined roles (beyond the compiled defaults)
2. Custom templates for project-specific needs
3. Project hooks that extend beyond the hardcoded set
4. Protocol overrides for specialized use cases

These cannot be achieved with the ADR-031 architecture.

## Decision

### 1. External File Storage

All protocols, templates, hooks, and roles are stored as external files in `.asimov/`:

```
.asimov/
├── protocols/
│   ├── asimov.json        # Three Laws (ethics)
│   ├── freshness.json     # Date-aware search
│   ├── sycophancy.json    # Truth over comfort
│   ├── green.json         # Local-first computing
│   ├── sprint.json        # Session boundaries
│   ├── migrations.json    # Functional equivalence
│   └── custom/            # User-defined protocols
├── templates/
│   ├── claude-settings.json
│   ├── session-start.sh
│   ├── pre-compact.sh
│   ├── pre-commit.sh
│   └── custom/            # User-defined templates
├── hooks/
│   ├── session-start.sh   # Symlinked/copied to .claude/hooks/
│   ├── pre-compact.sh
│   └── custom/            # User-defined hooks
├── roles/
│   ├── default.yaml       # Default role configuration
│   └── custom/            # User-defined roles
├── roadmap.yaml           # Project data
└── project.yaml           # Project configuration
```

### 2. Embedded Fallback

The binary retains embedded defaults for missing files:

```rust
fn load_protocol(name: &str) -> Result<Protocol> {
    let external_path = asimov_dir.join("protocols").join(format!("{}.json", name));

    if external_path.exists() {
        // Load from external file
        load_external_protocol(&external_path)
    } else {
        // Fall back to embedded default
        load_embedded_protocol(name)
    }
}
```

**Loading hierarchy:**
1. Check `.asimov/protocols/{name}.json`
2. If missing, use embedded default
3. Validate against schema (both sources)

### 3. File Formats

| Type | Format | Rationale |
|------|--------|-----------|
| Protocols | JSON | Token-efficient, machine-readable |
| Templates | Native (sh, json) | Direct use without conversion |
| Hooks | Shell scripts | Executable, cross-platform via shebang |
| Roles | YAML | Human-readable, supports comments |
| Project config | YAML | Familiar to developers |

### 4. Validation Layer

All external files pass through validation before use:

```rust
fn validate_protocol(content: &str) -> Result<Protocol> {
    let protocol: Protocol = serde_json::from_str(content)?;

    // Schema validation
    validate_schema(&protocol)?;

    // Security checks
    check_no_executable_content(&protocol)?;
    check_no_external_urls(&protocol)?;

    Ok(protocol)
}
```

### 5. Initialization and Updates

**`asimov init`:**
- Writes all default files to `.asimov/`
- Does not overwrite existing files (preserves customization)

**`asimov update`:**
- Writes missing files only
- With `--force`: overwrites all files (reset to defaults)
- Preserves `custom/` directories always

## Consequences

### Positive

1. **Easier updates** - Protocol changes don't require binary releases
2. **User customization** - Projects can modify protocols for specific needs
3. **Role flexibility** - New roles can be added without recompilation
4. **Template extensibility** - Custom templates for specialized workflows
5. **Debugging** - External files are inspectable and modifiable
6. **Offline operation** - All files local, no network required

### Negative

1. **Larger attack surface** - External files could be tampered with
   - Mitigated by: validation layer, schema enforcement, security checks
2. **File management** - Users must maintain `.asimov/` directory
   - Mitigated by: `asimov update` auto-restores missing files
3. **Version drift** - Custom files may become incompatible with updates
   - Mitigated by: schema versioning, migration support

### Neutral

1. **Performance** - File I/O vs embedded strings (negligible difference)
2. **Binary size** - Still includes embedded defaults (unchanged)

## Migration from ADR-031

### Automatic Migration

`asimov update` from v9.x to v10.x:
1. Detects existing `.asimov/` structure
2. Writes new external protocol files
3. Preserves existing `roadmap.yaml` and `project.yaml`
4. Creates `custom/` directories for user extensions

### Manual Steps

None required. Existing projects continue to work with embedded fallbacks.

## Implementation

### Phase 1: External Protocol Loading (v10.0.0)
- Implement file-based protocol loading
- Add embedded fallback mechanism
- Validation layer for all external files

### Phase 2: Custom Protocols (v10.1.0)
- Support `custom/` directories
- Protocol composition (extend base protocols)
- Per-project protocol overrides

### Phase 3: Template and Hook Extensibility (v10.2.0)
- Custom template support
- Hook chaining (run custom hooks after defaults)
- Role inheritance system

## References

- [ADR-031: Enforced Protocol Loading](031-enforced-protocol-loading.md) - Superseded by this ADR
- [ADR-011: Hardcoded Ethics](011-hardcoded-ethics.md) - Core ethics remain validated
- [ADR-021: Protocol Directory Structure](021-protocol-directory-structure.md) - Extended by this ADR

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov)*
