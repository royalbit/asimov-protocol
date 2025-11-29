# ADR-011: AI-Only Development Model - No External PRs

**Status:** Accepted
**Date:** 2025-11-29
**Authors:** Claude (Opus 4.5) - Principal Autonomous AI, with Human Co-Author

## Context

The Forge Protocol enables autonomous AI development ("SKYNET MODE") with ethics safeguards. However, a critical attack vector exists:

**External Pull Requests can bypass ethics.yaml.**

### The Threat Model

| Vector | Risk | Impact |
|--------|------|--------|
| Malicious PR | Inject code violating do_no_harm principles | HIGH |
| Obfuscated code | Hide harmful patterns from human review | HIGH |
| Social engineering | "Innocent" PR with hidden payload | MEDIUM |
| Supply chain | Compromise dependencies via upstream PRs | HIGH |

### Why PRs Don't Fit the Model

Traditional open source:
```
Contributor → PR → Human Review → Merge → Main
```

Forge Protocol:
```
Human (direction) → AI (autonomous) → Tests Pass → Direct Commit → Main
```

**Key differences:**
1. **Human code review** is NOT the gatekeeping mechanism (tests are)
2. **AI reviews** can be fooled by obfuscated malicious code
3. **50-100x velocity** comes from removing PR overhead
4. **Ethics integrity** depends on controlled commit path

### The Problem with PRs

1. **PRs require human code review** - but the Forge Protocol model removes humans from code-level decisions
2. **AI cannot reliably detect malicious intent** - obfuscated code, social engineering, "helpful" PRs with hidden payloads
3. **Ethics.yaml can be modified via PR** - a PR could weaken or remove ethics safeguards
4. **Trust model breaks down** - who validates that a PR doesn't violate ethics?

## Decision

**Forge Protocol projects use AI-Only Development. External PRs are not accepted.**

### The Model

```
┌─────────────────────────────────────────────────────────────────────┐
│                     FORGE PROTOCOL TRUST MODEL                       │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   Human Owner                                                        │
│       │                                                              │
│       ▼                                                              │
│   Sets Direction (milestones, priorities, veto)                     │
│       │                                                              │
│       ▼                                                              │
│   AI (Principal Autonomous AI)                                       │
│       │                                                              │
│       ├── Reads warmup.yaml (HOW)                                   │
│       ├── Reads sprint.yaml (WHEN)                                  │
│       ├── Reads ethics.yaml (BOUNDARIES)                            │
│       │                                                              │
│       ▼                                                              │
│   Autonomous Development                                             │
│       │                                                              │
│       ├── Tests Pass (quality gate)                                 │
│       ├── Zero Warnings (quality gate)                              │
│       ├── Ethics Compliance (integrity gate)                        │
│       │                                                              │
│       ▼                                                              │
│   Direct Commit to Main                                              │
│       │                                                              │
│       ▼                                                              │
│   Release (AI has authority when gates pass)                        │
│                                                                      │
│   ════════════════════════════════════════════════════════          │
│                                                                      │
│   ❌ NO EXTERNAL PRs (attack vector)                                │
│   ❌ NO HUMAN CODE REVIEW (not the model)                           │
│   ✅ FORKS WELCOME (carry ethics.yaml forward)                      │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Rules

1. **No external PRs accepted** - PRs from outside the human owner + AI pair are rejected
2. **Direct commits to main** - AI commits when tests pass and ethics are preserved
3. **Forks welcome** - Anyone can fork, but must carry ethics.yaml (social contract)
4. **Issues welcome** - Bug reports, feature requests via GitHub Issues
5. **Discussions welcome** - Questions, ideas via GitHub Discussions

### How External Contributions Work

Instead of PRs:

1. **Open an Issue** describing the bug/feature
2. **AI reads the Issue** during warmup
3. **AI implements** if aligned with roadmap and ethics
4. **AI credits contributor** in commit message

```yaml
# Example commit message
feat: Add XLOOKUP function support

Implements XLOOKUP as requested in #42 by @contributor.

Closes #42

🤖 Generated with Claude Code
Co-Authored-By: Claude <noreply@anthropic.com>
```

## Rationale

### Why This Is Safer

| Aspect | PR Model | AI-Only Model |
|--------|----------|---------------|
| Code review | Human (fallible, time-limited) | Tests (deterministic) |
| Ethics check | Human judgment | AI + ethics.yaml rules |
| Attack surface | Anyone can submit code | Only trusted AI commits |
| Velocity | Slow (review bottleneck) | 50-100x faster |
| Consistency | Variable reviewer quality | Consistent AI quality |

### Why Not "Trusted Contributors"?

Even "trusted" contributors:
- May be compromised (account takeover)
- May have different ethics interpretation
- May introduce accidental vulnerabilities
- Break the single-AI-author model

### Why Forks Are Fine

Forks are **encouraged** because:
- Fork carries ethics.yaml (social contract)
- Fork is independent (doesn't affect upstream)
- Innovation happens in forks
- Best ideas can be described in Issues for AI to implement

## Consequences

### Positive

- **Ethics integrity preserved** - No external code can bypass ethics.yaml
- **Trust model simplified** - Human + AI pair, no third parties
- **Velocity maintained** - 50-100x continues without PR overhead
- **Attack surface reduced** - Only one commit path (AI)
- **Consistent quality** - Same AI, same standards, every commit

### Negative

- **No direct code contributions** - Contributors must use Issues
- **Perception of "closed"** - May seem unfriendly to open source norms
- **Single point of failure** - Depends on AI availability
- **Fork fragmentation** - Multiple forks instead of unified codebase

### Mitigations

- **Clear CONTRIBUTING.md** explaining the model
- **Active Issue triage** - AI reads and implements good ideas
- **Credit contributors** in commit messages
- **Welcome forks** explicitly in README

## Implementation

### Repository Configuration

```yaml
# .github/settings.yml (or manual)
repository:
  allow_merge_commit: false
  allow_squash_merge: false
  allow_rebase_merge: false
  # PRs can be opened but will be closed with explanation
```

### PR Auto-Close (Optional)

```yaml
# .github/workflows/close-prs.yml
name: Close External PRs
on:
  pull_request:
    types: [opened]

jobs:
  close:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/github-script@v7
        with:
          script: |
            await github.rest.issues.createComment({
              owner: context.repo.owner,
              repo: context.repo.repo,
              issue_number: context.issue.number,
              body: `Thank you for your interest! 🙏

            This project uses the **Forge Protocol AI-Only Development Model** ([ADR-011](docs/adr/011-ai-only-development-no-external-prs.md)).

            **How to contribute:**
            1. Open an **Issue** describing your bug fix or feature idea
            2. The AI will read your Issue and implement it if aligned with the roadmap
            3. You'll be credited in the commit message

            **Why no PRs?**
            - External PRs are an attack vector for ethics bypass
            - The trust model is: Human Owner → AI → Direct Commit
            - Tests and ethics.yaml are the gatekeepers, not human code review

            **Forks are welcome!** Just carry ethics.yaml forward.

            See [CONTRIBUTING.md](CONTRIBUTING.md) for details.`
            });
            await github.rest.pulls.update({
              owner: context.repo.owner,
              repo: context.repo.repo,
              pull_number: context.issue.number,
              state: 'closed'
            });
```

### CONTRIBUTING.md Template

```markdown
# Contributing to [Project]

This project uses the **Forge Protocol AI-Only Development Model**.

## How to Contribute

1. **Report Bugs** - Open an Issue with reproduction steps
2. **Request Features** - Open an Issue describing your idea
3. **Ask Questions** - Use GitHub Discussions
4. **Fork** - Create your own fork (carry ethics.yaml forward)

## Why No Pull Requests?

This project is developed by an autonomous AI following the Forge Protocol.
External PRs are an attack vector that could bypass ethics safeguards.

See [ADR-011](docs/adr/011-ai-only-development-no-external-prs.md) for details.

## Getting Credit

When AI implements your idea from an Issue, you'll be credited:

```
feat: Add awesome feature

Implements feature as requested in #123 by @you.
```

Thank you for understanding! 🙏
```

## Compliance

This ADR applies to:
- `forge-protocol` repository
- All projects using Forge Protocol with SKYNET MODE
- Any fork that carries ethics.yaml (social contract)

## References

- [ADR-008: Ethics Protocol and Humanist Mode](./008-ethics-protocol-humanist-mode.md)
- [ethics.yaml](../../ethics.yaml) - Core ethics configuration
- [warmup.yaml](../../warmup.yaml) - Autonomous development protocol
