# ADR-045: Dependency Health Protocol (STRICT)

**Status:** Accepted
**Date:** 2025-12-06
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

**GOOD CODE is a MOAT!** Dependencies must be:
1. **Secure** - Zero known vulnerabilities
2. **Fresh** - Up-to-date, including major versions
3. **License-compliant** - Compatible with project license

### The Problem

Dependencies accumulate technical debt:
- Security vulnerabilities go unpatched
- Major version updates are deferred indefinitely
- License conflicts can create legal issues

### Philosophy

**Fresh code is good code.** Outdated dependencies are a liability:
- Security patches missed
- Performance improvements missed
- Bug fixes missed
- Breaking changes compound over time

## Decision

### STRICT Enforcement

Pre-commit hooks will **FAIL** on:
1. **Any security vulnerability** (not just critical)
2. **Any outdated dependency** (including major versions)
3. **Any license incompatibility**

### LICENSE File Detection

Automatically detect project license from:
1. `LICENSE` / `LICENSE.md` / `LICENSE.txt` file
2. `Cargo.toml` license field (Rust)
3. `package.json` license field (Node)
4. `pyproject.toml` license field (Python)

### License Compatibility Matrix

| Project License | ALLOW These Deps | DENY These Deps |
|-----------------|------------------|-----------------|
| **MIT** | MIT, BSD-*, Apache-2.0, ISC, Unlicense, Zlib | GPL-*, LGPL-*, AGPL-*, MPL-* |
| **Apache-2.0** | Apache-2.0, MIT, BSD-*, ISC | GPL-*, AGPL-* |
| **GPL-3.0** | ALL (viral license) | None |
| **Proprietary** | MIT, BSD-*, Apache-2.0, ISC | GPL-*, LGPL-*, AGPL-*, Copyleft |

### Tools Per Project Type

| Type | Security | License | Freshness | License |
|------|----------|---------|-----------|---------|
| **Rust** | cargo-audit | cargo-deny | cargo-outdated | MIT/Apache-2.0 |
| **Python** | pip-audit | pip-licenses | pip (built-in) | Apache-2.0 |
| **Node** | npm audit | license-checker | npm (built-in) | ISC/MIT |
| **Go** | govulncheck | go-licenses | go (built-in) | BSD-3-Clause |
| **Flutter** | - | license_checker | dart (built-in) | MIT |

### Pre-commit Hook Template

```bash
#!/bin/bash
set -e

# === SECURITY AUDIT (FAIL on ANY vulnerability) ===
echo "Security audit..."
if command -v cargo-audit &>/dev/null; then
  cargo audit --deny warnings || { echo "SECURITY: Vulnerabilities found!"; exit 1; }
fi

# === LICENSE COMPLIANCE (FAIL on incompatible) ===
echo "License compliance..."
if [ -f "deny.toml" ] && command -v cargo-deny &>/dev/null; then
  cargo deny check licenses || { echo "LICENSE: Incompatible dependency!"; exit 1; }
fi

# === FRESHNESS CHECK (FAIL on ANY outdated) ===
echo "Freshness check..."
if command -v cargo-outdated &>/dev/null; then
  outdated=$(cargo outdated --depth 1 -R 2>/dev/null | grep -c "^[a-z]" || echo "0")
  if [ "$outdated" -gt 0 ]; then
    echo "OUTDATED: Dependencies need updates!"
    cargo outdated --depth 1 -R
    exit 1
  fi
fi
```

### `asimov doctor` Enhancement

```
$ asimov doctor

DEPENDENCY HEALTH
  License: MIT (from LICENSE file)

  Security:
    ✗ FAIL: 2 vulnerabilities
      - RUSTSEC-2024-0001: serde_yaml (HIGH)
      - RUSTSEC-2024-0002: regex (MEDIUM)

  Freshness:
    ✗ FAIL: 3 outdated
      - clap 4.5.0 → 5.0.0 (MAJOR)
      - serde 1.0.193 → 1.0.210 (patch)

  Licenses:
    ✓ PASS: All compatible with MIT
```

## Implementation

### Files Modified

- `cli/src/templates/hooks.rs` - Add security/license/freshness checks
- `cli/src/commands/doctor.rs` - Add dependency health report
- `cli/src/commands/init.rs` - Detect LICENSE file
- `docs/SPECIFICATION.md` - Dependency Health section

### License Detection

```rust
fn detect_license(dir: &Path) -> Option<String> {
    // Check LICENSE file
    for name in ["LICENSE", "LICENSE.md", "LICENSE.txt"] {
        let path = dir.join(name);
        if path.exists() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if content.contains("MIT License") { return Some("MIT".into()); }
                if content.contains("Apache License") { return Some("Apache-2.0".into()); }
                if content.contains("GNU General Public") { return Some("GPL-3.0".into()); }
            }
        }
    }

    // Check Cargo.toml
    let cargo = dir.join("Cargo.toml");
    if cargo.exists() {
        if let Ok(content) = std::fs::read_to_string(&cargo) {
            if let Some(line) = content.lines().find(|l| l.starts_with("license")) {
                // Parse license = "MIT" or license = "MIT OR Apache-2.0"
            }
        }
    }

    None
}
```

## Consequences

### Positive

1. **Zero vulnerabilities** - Security is enforced, not optional
2. **Fresh dependencies** - Major updates can't be deferred
3. **License compliance** - Legal issues prevented automatically
4. **Clear feedback** - Doctor shows exactly what needs fixing

### Negative

1. **Stricter workflow** - Can't commit with known issues
2. **Requires tools** - cargo-audit, pip-audit, etc. must be installed

### Neutral

1. **Escape hatch** - Can skip with `git commit --no-verify` (but discouraged)

## Related

- [ADR-043: Direct Coding Standards Enforcement](043-direct-coding-standards.md)
- [ADR-044: Dependency Setup](044-dependency-setup.md)

---

**Previous:** [ADR-044](044-dependency-setup.md)

---
