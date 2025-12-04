# ADR-038: 100% Test Coverage Requirement

**Status:** Accepted
**Date:** 2025-12-04
**Author:** Claude Opus 4.5 (Principal Autonomous AI)

---

## Context

RoyalBit Asimov is a protocol enforcement tool that validates YAML configurations, manages development sessions, and ensures quality gates are met. As infrastructure tooling, correctness is critical - bugs can cause:

1. **False validation passes** - Invalid configs marked as valid
2. **Silent failures** - Hooks not triggering when expected
3. **Protocol drift** - Hardcoded values diverging from expected behavior
4. **Session corruption** - Checkpoint/state management errors

### The Problem

Untested code is unknown code. Any line without test coverage is a potential bug waiting to happen.

```
80% coverage = 20% untested
20% of 3000 lines = 600 untested lines
600 lines × 0.1% bug rate = 0.6 bugs lurking
```

For a protocol enforcement tool, even one bug undermines trust in the entire system.

### Forge Precedent

The Forge project (sister project) established this standard in ADR-004:

> "**NO EXCEPTIONS. ZERO. NONE.**
> If code cannot be tested, it must be refactored until it CAN be tested."

Forge achieved 100% coverage with 542+ tests. Asimov must match this standard.

## Decision

**100% line coverage and 100% branch coverage required. Build MUST fail if not met.**

### Enforcement Mechanism

```
┌─────────────────────────────────────────────────────────────────┐
│                    COVERAGE ENFORCEMENT                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  make coverage          Run coverage, fail if < 100%            │
│  make coverage-report   Generate detailed HTML report           │
│  make coverage-ci       CI mode: strict 100% enforcement        │
│                                                                 │
│  Pre-commit Hook:                                               │
│  ┌─────────┐  ┌─────────┐  ┌──────────┐  ┌─────────┐           │
│  │  Test   │──│  Lint   │──│ Coverage │──│ Commit  │           │
│  └─────────┘  └─────────┘  └──────────┘  └─────────┘           │
│                               │                                 │
│                               ▼                                 │
│                         < 100%? FAIL                            │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Coverage Requirements

| Metric | Requirement | Rationale |
|--------|-------------|-----------|
| **Line coverage** | 100% | Every code path must be tested |
| **Branch coverage** | 100% | All conditional branches tested |
| **Function coverage** | 100% | No untested functions |

### Tool: cargo-llvm-cov

```bash
# Install
cargo install cargo-llvm-cov

# Run with enforcement
cargo llvm-cov --fail-under-lines 100

# Generate HTML report
cargo llvm-cov --html
```

### Makefile Targets

```makefile
coverage:
	@cargo llvm-cov --fail-under-lines 100 --ignore-filename-regex '(tests/|test_)'

coverage-report:
	@cargo llvm-cov --html --ignore-filename-regex '(tests/|test_)' --output-dir coverage-report

coverage-ci:
	@cargo llvm-cov --fail-under-lines 100 --ignore-filename-regex '(tests/|test_)' --lcov --output-path lcov.info
```

## Rationale

### Why 100%?

1. **Protocol tools must be trustworthy**
   - If `asimov validate` says "valid", it MUST be valid
   - Untested validation logic = unknown behavior

2. **Self-hosting integrity**
   - Asimov validates itself
   - Bugs in asimov could cause false confidence

3. **Quality gate enforcement**
   - We enforce quality gates on users
   - We must meet or exceed those gates ourselves

4. **Technical debt prevention**
   - Untested code is unmaintainable code
   - 100% coverage forces good design (testable = modular)

### Why Not 80%?

The "80% is good enough" argument fails for infrastructure tools:

```
80% coverage = 20% untested
20% untested in validator = 20% unknown validation behavior
Unknown validation behavior = UNACCEPTABLE for protocol tool
```

## Consequences

### Positive

1. **Zero validation bugs in production** - Every line tested
2. **Confident refactoring** - Tests catch regressions immediately
3. **Self-documenting** - Tests show expected behavior
4. **Credibility** - We practice what we preach

### Negative

1. **More test code** - ~1:1 ratio of test code to source
2. **Slower CI** - Coverage analysis adds time
3. **Strict enforcement** - Can't merge with < 100%

### Implementation

1. **Entry points (main.rs)**: Extract ALL logic into testable library functions
2. **Network code (update.rs)**: Mock HTTP calls with test fixtures
3. **File I/O**: Use trait-based abstraction for testability
4. **CLI parsing**: Test argument combinations exhaustively

## Exceptions

**NO EXCEPTIONS.**

If code cannot be tested, it must be refactored until it CAN be tested:

1. **Extract pure functions** - Move ALL logic out of I/O code
2. **Dependency injection** - Mock ALL external dependencies
3. **Thin entry points** - main() should only call library code

The build WILL FAIL if coverage is below 100%. This is non-negotiable.

## Monitoring

Coverage is tracked and must never decrease:

```
PR Merge Requirements:
✓ All tests pass
✓ Zero warnings (clippy)
✓ 100% line coverage
✓ 100% branch coverage
```

---

## Related

- [Forge ADR-004: 100% Test Coverage](https://github.com/royalbit/forge/blob/main/docs/architecture/ADR-004-100-PERCENT-TEST-COVERAGE.md)
- [ADR-011: Hardcoded Ethics](011-hardcoded-ethics.md)

---

**Previous:** [ADR-037](037-remove-check-schema-commands.md)
