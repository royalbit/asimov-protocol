# Rust Development Guide

Standards for autonomous Rust development in Asimov Protocol projects.

## FOSS Research (Mandatory)

**BEFORE writing ANY complex code (>50 lines):**

1. Search crates.io AND web for existing solutions
2. Compare at least 3 options
3. Document the decision in code comments
4. Justify if choosing to write from scratch

### Definition of Complex

- Parsers, formatters, file format handling
- Mathematical/formula processing
- Graph/tree algorithms, serialization
- HTTP/API code, auth systems
- Anything >2 hours to write

### License Compatibility

| Status | Licenses |
|--------|----------|
| **Use freely** | MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unlicense |
| **Avoid** | GPL (any), AGPL, LGPL (copyleft conflicts with MIT) |

## Error Handling

**Use `thiserror` for rich error context:**

```rust
#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("File '{path}': {reason}")]
    FileError { path: String, reason: String },
}
```

### Rules

- `Result<T, E>` everywhere - no panics in library code
- No `unwrap()` or `expect()` in library code
- Add context with `.map_err()` when wrapping errors
- Error messages should guide user to solution

## Type-Driven Design

- Make illegal states unrepresentable
- Use newtypes for domain concepts
- Prefer enums over booleans for state
- Builder pattern for complex construction

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TableName(String);  // Newtype prevents mixing with other strings
```

## Testing

| Type | Tool | Command |
|------|------|---------|
| Unit | built-in | `cargo test` |
| Property-based | proptest | - |
| Snapshot | insta | `cargo insta review` |
| Mutation | cargo-mutants | `cargo mutants` |

### Testing Rules

- Every public function has tests
- Every error path is tested
- Edge cases: empty, null, boundary values, large inputs

## Code Quality Tools

| Tool | Purpose | Command |
|------|---------|---------|
| clippy | Linting | `cargo clippy -- -D warnings` |
| fmt | Formatting | `cargo fmt` |
| audit | Security | `cargo audit` |
| outdated | Dependencies | `cargo outdated` |
| machete | Unused deps | `cargo machete` |

## Dependency Updates

```bash
cargo outdated          # Check what needs updating
cargo update            # Update minor versions (safe)
cargo upgrade           # Update major versions (test thoroughly)
```

**After updates:** Run full test suite + clippy.

## Performance

- Profile before optimizing (`cargo bench` with criterion)
- Optimize hot paths only
- Use `&str` over `String` when possible
- Avoid `clone()` unless necessary

## Quality Gates (Non-negotiable)

```bash
cargo test              # All pass
cargo clippy -- -D warnings  # Zero warnings
cargo fmt --check       # Already formatted
```

**Nothing ships without passing all three.**
