# ADR-040: Code Organization - Human-Readable File Sizes

**Status:** Accepted
**Date:** 2025-12-06
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

RoyalBit follows Rust official guidelines **strictly**. The Rust Book states:

> "As a project grows, you **should** organize code by splitting it into multiple modules and then multiple files."
> — [The Rust Book, Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

### RFC 2119 Interpretation

In technical documentation, **"should"** means:

| Term | Meaning |
|------|---------|
| **MUST** | Absolute requirement |
| **SHOULD** | Recommended - valid reasons may exist to ignore, but implications must be understood |
| **MAY** | Optional |

### RoyalBit Philosophy

> "Without these rules, you are a JUNIOR programmer - but super fast one... that creates tons of ugly, unmaintainable code for humans at high velocity!"

Speed without discipline is high-velocity technical debt. For RoyalBit projects:

1. **Human readability** - Code is written for humans first, machines second
2. **Beautiful code** - Well-formatted, linted, organized
3. **Maintainability** - Easy to understand, modify, and extend
4. **OCD-level quality** - No "good enough", only "right"

### Forge Precedent

The Forge project (sister project) established this standard in ADR-007:

> "For this project, 'should' becomes 'must'."
> "Human-readable, beautiful, well-formatted code is a core value."

Asimov must match this standard.

### Current State (Asimov)

Asimov **already follows good practices** with modular structure:

```
cli/src/
├── commands/          ✅ Already split by command
│   ├── doctor.rs
│   ├── init.rs
│   ├── validate.rs
│   └── ...
├── templates/         ✅ Already split by concern
│   ├── protocols.rs
│   ├── warmup.rs
│   └── ...
├── main.rs           ⚠️  1,390 lines (borderline)
└── validator.rs      ⚠️  1,307 lines (borderline)
```

| File | Lines | Status |
|------|-------|--------|
| `main.rs` | 1,390 | Borderline - evaluate |
| `validator.rs` | 1,307 | Borderline - evaluate |
| `update.rs` | 925 | Acceptable |
| `semantic.rs` | 703 | Acceptable |
| `ethics.rs` | 632 | Acceptable |
| All others | < 500 | Ideal |

## Decision

**For RoyalBit projects, Rust's "should" becomes "must".**

We adopt a **1,000-line soft limit** per file as a project standard.

### Thresholds

| Threshold | Action |
|-----------|--------|
| **< 500 lines** | Ideal - no action needed |
| **500-1,000 lines** | Acceptable - monitor for growth |
| **1,000-1,500 lines** | Evaluate - consider splitting if logical boundaries exist |
| **> 1,500 lines** | Split required - find natural module boundaries |

### Splitting Criteria

Files **should** be split when:

1. **Logical boundaries exist** - Distinct sections with clear responsibilities
2. **Navigation is impaired** - Hard to find code within the file
3. **Section comments exist** - `// ═══ SECTION NAME ═══` indicates natural split points
4. **Test organization** - Tests for a section could live with that section

Files should **NOT** be split when:

1. **Tight coupling** - Code is genuinely interdependent
2. **Splitting adds complexity** - Traits/fn pointers needed just for organization
3. **No logical boundaries** - The code is one cohesive unit

### Test Organization

Per Rust convention, unit tests stay with code:

```rust
// In the same file as the code being tested
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() { ... }
}
```

When splitting a file, tests move WITH their associated code to the new module.

## Implementation

### Current Status: GOOD

Asimov already follows the modular pattern:
- ✅ `commands/` - Split by command handler
- ✅ `templates/` - Split by concern
- ✅ `protocols/` - Separate module
- ⚠️ `main.rs` - 1,390 lines (borderline, monitor)
- ⚠️ `validator.rs` - 1,307 lines (borderline, monitor)

### Future Work (if files grow)

If `main.rs` exceeds 1,500 lines:
```
cli/src/
├── main.rs        (~100 lines) - entry point only
├── cli.rs         - clap argument definitions
├── app.rs         - application orchestration
└── ...
```

If `validator.rs` exceeds 1,500 lines:
```
cli/src/validator/
├── mod.rs         - public interface
├── yaml.rs        - YAML validation
├── schema.rs      - schema validation
├── rules.rs       - validation rules
└── ...
```

## Rationale

### Why Follow "Should" Strictly?

1. **Consistency** - Arbitrary exceptions lead to inconsistent codebase
2. **Quality culture** - High standards attract quality contributors
3. **Future-proofing** - Smaller files are easier to refactor
4. **Code review** - PRs touching smaller files are easier to review
5. **Self-respect** - We are not junior programmers

### The Junior Programmer Problem

Without standards:
```
AI velocity + no standards = mountains of technical debt
Fast code + ugly code = unmaintainable code
Many lines + poor organization = confused humans
```

With standards:
```
AI velocity + strict standards = high-quality, fast delivery
Fast code + beautiful code = maintainable code
Many lines + proper organization = readable code
```

## Consequences

### Positive

1. **Human-readable codebase** - Easy to navigate and understand
2. **Better code reviews** - Smaller, focused changes
3. **Clear responsibilities** - Each file has one job
4. **Easier testing** - Tests colocated with focused code
5. **Professional quality** - Not "fast junior" output

### Negative

1. **More files** - Directory navigation required
2. **Import management** - More `use` statements
3. **Vigilance required** - Monitor file growth

### Metrics

| Metric | Current | Target |
|--------|---------|--------|
| Files > 1,500 lines | 0 | 0 |
| Files > 1,000 lines | 2 | 0 (long-term) |
| Avg file size | ~400 lines | < 500 lines |

## Related

- [Forge ADR-007: Code Organization](https://github.com/royalbit/forge/blob/main/docs/architecture/ADR-007-CODE-ORGANIZATION.md)
- [ADR-038: 100% Test Coverage](038-100-percent-test-coverage.md)
- [The Rust Book - Managing Growing Projects](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [The Rust Book - Separating Modules](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
- [RFC 2119 - Key Words](https://www.rfc-editor.org/rfc/rfc2119)

---

**Previous:** [ADR-039](039-coverage-exclusion-policy.md)

---
