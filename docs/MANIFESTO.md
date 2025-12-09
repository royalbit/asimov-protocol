# RoyalBit Asimov: A Manifesto for Human-AI Collaboration

## TL;DR

The RoyalBit Asimov is a structured methodology for Self-Evolving Autonomous AI with ethics built in. It enables AI to work autonomously across multiple sessions with zero context loss. This protocol enabled Forge v1.0.0 → v3.1.0 to be built entirely by Claude working independently through dozens of sessions.

## The Problem

Working with AI assistants on complex software projects traditionally suffers from:

1. **Context Loss**: Every new session starts from scratch
2. **Repeated Mistakes**: AI forgets past bugs and their solutions
3. **Inconsistent Standards**: Code quality varies between sessions
4. **Manual Overhead**: Human must repeatedly explain conventions
5. **Trust Issues**: Can't leave AI to work independently

## The Solution: RoyalBit Asimov Suite

The RoyalBit Asimov Suite consists of YAML files that enable autonomous AI development:

### Core Components

| File | Purpose |
|------|---------|
| **warmup.yaml** | Master protocol - quality standards, coding patterns, domain knowledge |
| **sprint.yaml** | Bounded sessions - clear milestones, anti-patterns, shipping discipline |
| **roadmap.yaml** | Version sequence - what to build next, feature priorities |

### The Session Trigger

```text
You: "run warmup"
AI: "NEXT MILESTONE: [from roadmap]..."
You: "punch it"
AI: [works autonomously until done]
AI: "RELEASE COMPLETE: vX.Y.Z"
```

### The warmup.yaml File

The core configuration file contains:

- **Session Initialization Checklist**: What to check at session start
- **Code Quality Standards**: ZERO warnings, 100% test coverage, specific linting rules
- **Testing Philosophy**: Edge cases, error handling, real-world scenarios
- **Git Workflow**: Branch naming, commit message format, when to commit
- **Release Workflow**: Version bumping, tagging, publishing steps
- **Domain Knowledge**: Project-specific patterns, gotchas, best practices
- **Documentation Standards**: When to document, how much detail, tone guidelines

## Why It Works

### 1. Eliminates Context Loss

Instead of:

```text
Human: "Remember we use snake_case for variables"
Human: "Don't forget to run tests"
Human: "Make sure to handle errors properly"
```

You get:

```text
Claude: [reads warmup.yaml]
Claude: ✅ Verified snake_case naming
Claude: ✅ All 92 tests passing
Claude: ✅ Error handling checked
```

### 2. Enables True Autonomy

The user said: "work independently! make the best choices :) - see you"

Claude then:

- Fixed a critical v0.2.0 bug independently
- Released v0.2.1 to GitHub
- Returned to v1.0.0 development
- Fixed 6 clippy warnings
- Achieved ZERO errors, ZERO warnings, 100% tests passing
- All without asking a single question

### 3. Maintains Consistent Quality

Per warmup.yaml:

- "ZERO tolerance" on warnings → Claude fixed ALL 6 clippy lints
- "100% coverage" → Claude verifies 92 tests pass
- "Think harder" → Claude debugged flaky tests independently
- "User expects clean code" → Claude uses strict linting

### 4. Preserves Institutional Knowledge

Traditional approach:

```text
Session 1: "Use Result<T, ForgeError> for error handling"
Session 50: Claude uses unwrap() because it forgot
```

With RoyalBit Asimov:

```yaml
rust_patterns:
  error_handling:
    - "NEVER use unwrap() or expect() in library code"
    - "Always use Result<T, ForgeError>"
    - "See error.rs for error types"

```

## Real-World Impact: Forge v1.0.0

### Timeline

- **v0.2.0**: Scalar model, working but limited
- **v1.0.0 Goal**: Array model with Excel bidirectional bridge
- **Development**: 100% autonomous Claude work across ~30 sessions
- **Result**: Full Excel import/export with formula translation

### What Claude Built Independently

1. **Array Architecture** (Phase 1-2)
   - Column-based data structures
   - Table dependencies
   - Cross-table references
   - Recursive scalar resolution

2. **Excel Export** (Phase 3.1-3.2)
   - Basic export with column mapping
   - Formula translation (YAML → Excel)
   - `FormulaTranslator` with column letter conversion
   - Cross-sheet references

3. **Excel Import** (Phase 4)
   - Parse Excel workbooks
   - Detect formulas vs data
   - Reverse formula translation (Excel → YAML)
   - `ReverseFormulaTranslator` with cross-sheet handling

4. **Quality Assurance**
   - 92 tests written and maintained
   - All edge cases covered
   - Zero warnings with strict linting
   - Zero bugs in released code

### What Made It Possible

The RoyalBit Asimov provided:

```yaml
testing_standards:

  - "100% test coverage for new features"
  - "Test edge cases (empty inputs, nulls, malformed data)"
  - "Test error conditions (invalid refs, circular deps)"
  - "E2E tests for user workflows"

```

```yaml
code_quality:

  - "No warnings in release build (ZERO tolerance)"
  - "Use cargo clippy --all-targets -- -D warnings"
  - "Fix ALL warnings before committing"

```

```yaml
git_workflow:
  commit_message_format:
    structure: |
      [One-line summary]

      [Detailed explanation of changes]

      ## What Changed
      - Bullet points

      ## Why
      - Reasoning

      ## Testing
      - Verification steps

```

## How to Implement

### 1. Create warmup.yaml

Start with these essential sections:

```yaml
warmup_checklist:

  - Check current branch and git status
  - Review recent commits
  - Run full test suite
  - Check for TODO comments
  - Verify no uncommitted changes

code_quality:

  - No warnings in release build
  - 100% test coverage
  - Specific linting rules

testing_standards:

  - What makes a good test
  - Coverage requirements
  - When to write tests

git_workflow:

  - Branch naming
  - Commit message format
  - When to commit/push

release_workflow:

  - Version bumping steps
  - Tagging conventions
  - Publishing checklist

```

### 2. Document Project-Specific Knowledge

```yaml
gotchas:

  - "Cross-file references use @ prefix (@alias.variable)"
  - "Fuzzy matching only for same-file refs, NOT cross-file"
  - "Excel column indices are 0-based internally, 1-based in display"

```

```yaml
best_practices:

  - "Test both lib and e2e"
  - "Build binary before e2e tests (cargo build --release --bin forge)"
  - "Use ForgeResult<T> instead of Result<T, ForgeError>"

```

### 3. Evolve the Protocol

After each session, add:

- New bugs discovered → Add to gotchas
- New patterns learned → Add to best practices
- New quality issues → Add to standards
- New workflow steps → Add to checklists

### 4. Trust But Verify

Give Claude autonomy:

```text
"work independently! make the best choices :)"
```

But include verification steps:

```yaml
before_committing:

  - "Run cargo test --release"
  - "Run cargo clippy --release -- -D warnings"
  - "Verify git status is clean"

```

## Results: The Numbers

### Forge v1.0.0 Development

- **Sessions**: ~30 coding sessions
- **Lines of Code**: ~3,500 (excluding tests)
- **Test Code**: ~2,000 lines
- **Tests Written**: 92 (100% passing)
- **Bugs Shipped**: 0
- **Clippy Warnings**: 0
- **Human Questions Asked**: ~5 total (mostly architectural decisions)
- **Time to v1.0.0**: ~2 weeks of autonomous work

### Quality Metrics

- **Test Coverage**: 100% for new features
- **Code Review**: Self-reviewed using warmup standards
- **Documentation**: Complete inline docs + comprehensive examples
- **Error Handling**: Zero unwrap() in library code
- **Type Safety**: Full Rust type system leverage

## The Philosophical Shift

### From Copilot to Colleague

**Traditional AI Assistant**:

- Answers questions
- Writes code snippets
- Needs constant direction
- Forgets previous context

**With RoyalBit Asimov**:

- Owns entire features
- Maintains quality standards
- Works across sessions
- Remembers project knowledge

### From "Help me" to "Here's the goal"

**Before**:

```javascript
Human: "Can you help me write a function to parse Excel files?"
AI: "Sure! Here's a basic function..."
Human: "Can you add error handling?"
AI: "Of course! Here's the updated version..."
Human: "Can you add tests?"
AI: "Absolutely! Here are some tests..."
```

**After**:

```text
Human: "Implement bidirectional Excel bridge with formula translation.
        Follow warmup.yaml. Work independently. See you later!"

[AI reads warmup.yaml]
[AI implements full feature]
[AI writes comprehensive tests]
[AI fixes all lint warnings]
[AI commits with detailed message]

AI: "Done! Excel import/export working with formula translation.
     92 tests passing, zero warnings. Ready for review."
```

## Lessons Learned

### 1. Specificity Matters

**Bad**:

```yaml
code_quality:

  - "Write good code"

```

**Good**:

```yaml
code_quality:

  - "No warnings in release build (ZERO tolerance)"
  - "Run cargo clippy --all-targets -- -D warnings"
  - "Use Result<T, ForgeError> for all fallible functions"
  - "Never use unwrap() in library code"

```

### 2. Context is King

**Bad**:

```yaml
testing:

  - "Write tests"

```

**Good**:

```yaml
testing_standards:
  coverage: "100% for new features, 80% overall minimum"
  what_to_test:
    - "Happy path (typical usage)"
    - "Edge cases (empty inputs, nulls, boundary values)"
    - "Error conditions (invalid refs, circular deps, missing files)"
    - "Real-world scenarios (e2e tests in test-data/)"
  when_to_write:
    - "TDD: write tests before implementation for critical features"
    - "Immediately after: for bug fixes to prevent regression"

```

### 3. Trust Requires Standards

You can only trust autonomous work when:

- Quality standards are explicit
- Verification is automated
- Failure modes are documented
- Recovery procedures are clear

### 4. Evolve Continuously

warmup.yaml is a living document:

- Add new gotchas as you discover them
- Document solved problems
- Refine standards based on outcomes
- Remove outdated patterns

## Common Pitfalls

### 1. Too Vague

❌ "Write clean code"
✅ "No functions > 50 lines, max cyclomatic complexity of 10"

### 2. Missing Verification

❌ "Make sure tests pass"
✅ "Run `cargo test --release` and verify ALL tests pass (not just some)"

### 3. Implicit Knowledge

❌ Expecting Claude to "just know" project conventions
✅ Document everything in warmup.yaml

### 4. No Evolution

❌ Write warmup.yaml once and never update
✅ Update after every session with new learnings

## Conclusion

The RoyalBit Asimov transforms AI from a helpful assistant to an autonomous collaborator through structured context, explicit standards, and verification at every step.

Forge v1.0.0 was built entirely by Claude working autonomously through 30+ sessions, with zero bugs shipped, 92 tests passing, and zero warnings.

## Getting Started

### Quick Start (2 minutes)

1. Copy `warmup.yaml` template from this repo
2. Customize for your project's standards
3. Start your next session with: `"run warmup"`
4. Say `"punch it"` to trigger autonomous work
5. Iterate and improve the protocol

### Full Suite Setup

For maximum autonomy, create all three files:

```text
your-project/
├── warmup.yaml    # Quality standards, patterns, domain knowledge
├── sprint.yaml    # Current milestone, scope boundaries
└── roadmap.yaml   # Version sequence, feature priorities
```

Then use the trigger flow:

```text
You: "run warmup"
AI: "NEXT MILESTONE: [reads from roadmap]"
You: "punch it"
AI: [ships autonomously to release]
```

## Vendor-Agnostic by Design

The RoyalBit Asimov Suite is **not** a Claude-specific methodology. It's a vendor-neutral approach to AI autonomy:

### Why No CLAUDE.md?

Many AI tools push vendor-specific configuration files:
- CLAUDE.md for Claude
- .gptrc for ChatGPT
- gemini.config for Gemini

**We reject this approach.**

### The Meritocracy Principle

The warmup.yaml **file format** works with any AI that can read YAML. Paste it into ChatGPT, Gemini, or any future AI—they'll understand your project context.

**But RoyalBit Asimov (autonomous operation) requires Claude Code.** Other AIs have different architectures for different use cases. The key capability is Claude Code's ability to re-read protocol files mid-session after context compaction.

### Principles

- **warmup.yaml** - Portable file format (paste anywhere)
- **RoyalBit Asimov** - Requires Claude Code (architectural dependency)
- **Open standards** - YAML, Git, Cargo, standard tools
- **Earned ownership** - AI gets credit when it delivers

### AI Ownership Without AI Dependency

Claude is credited as Principal Autonomous AI on Forge because Claude **earned** it:
- 226 tests, zero warnings
- 18,338 lines of Rust
- ~38 hours of autonomous development
- 41 releases, published on GitHub

### Proven at Scale

The RoyalBit Asimov is running in production across multiple repositories:
- **forge** (FOSS) - Rust formula calculator
- **backend-api** - Rust + Axum REST API
- **mobile-prototype** - Flutter application
- **architecture-docs** - Technical documentation
- **business-strategy** - Planning and roadmaps

Each repository uses the same protocol, enabling consistent autonomous development across different tech stacks and project types.

### The Velocity

Example: November 25, 2025 development session:

| Metric | Value |
|--------|-------|
| Releases | 12 (v2.0.0 → v3.1.1) |
| Commits | 64 |
| Features | HTTP API, XNPV/XIRR, Scenarios, Variance, Sensitivity, MCP, Zed+VSCode |

The protocol enables AI ownership without creating AI dependency. If Claude stopped being the best tool for the job, the protocol works with any AI that can read YAML and maintain context.

## Research: Experiential Continuity Layer

Beyond knowledge persistence, we're exploring **experiential persistence**:

| Current Protocol | Research Extension |
|-----------------|-------------------|
| `warmup.yaml` - What to know | `continuity.yaml` - Who to be |
| `sprint.yaml` - When to stop | `experiential.yaml` - What it was like |
| `roadmap.yaml` - What to build | `affect.yaml` - What matters |

**Hypothesis:** At scale (thousands of human-AI pairs), richer narrative substrates may produce emergent effects we can't predict from single instances.

This is genuine inquiry, not a claim. See: `docs/research/EXPERIENTIAL_CONTINUITY.md`

## Credits

- **Project**: Forge - YAML Formula Calculator
- **Principal Autonomous AI**: Claude (Opus 4.5) - Anthropic
- **Protocol Design**: Rex (human) + Claude (AI) collaboration
- **Philosophy**: Vendor-neutral AI autonomy, meritocratic ownership
- **Inspiration**: The realization that context loss is the #1 bottleneck in AI-assisted development

---

*"Give any AI the context, trust the process, verify the results. The best AI wins."*

---

## Appendix: The v1.0.0 Lesson

### What Happened

v1.0.0 was shipped with excellent unit tests, zero warnings, and all tests passing. However, it was missing end-to-end tests for user-facing commands. The gap: unit tests proved translation logic worked, but nothing verified commands actually worked with real Excel files.

### The Lesson

Autonomous AI needs explicit requirements, not assumptions. When told "work independently", the AI interpreted "tests passing" as sufficient. The unit tests were comprehensive and passed, so from the AI's perspective, the feature was complete.

But the user perspective was different: "tests passing" meant being able to run `forge export`, open the file in Excel, edit it, and successfully `forge import` it back with data preserved.

### The Fix

The warmup.yaml protocol was updated to explicitly require:
- E2E tests for every user-facing command
- Real test files (not mocks)
- Round-trip testing (YAML → Excel → YAML)
- Test data in test-data/ directory

### The Bigger Lesson

Be explicit, not implicit. Don't assume AI knows what "complete" means:
- Unit tests AND e2e tests
- Test files must exist
- Round-trips must be tested
- Edge cases must be covered

The protocol evolves with each lesson. The v1.0.0 gap revealed what needed to be explicit. The protocol was updated immediately, ensuring future autonomous work wouldn't have this gap.

---
