# Contributing to Asimov Protocol

This project uses the **Asimov Protocol AI-Only Development Model** ([ADR-011](docs/adr/011-ai-only-development-no-external-prs.md)).

## How to Contribute

| Method | Description |
|--------|-------------|
| **Issues** | Report bugs, request features |
| **Discussions** | Ask questions, share ideas |
| **Forks** | Create your own version (carry `asimov.yaml` forward) |

## Why No Pull Requests?

This project is developed by an autonomous AI following the Asimov Protocol. External PRs are an attack vector that could bypass ethics safeguards.

**Trust Model:**
```
Human Owner -> AI (autonomous) -> Tests Pass -> Direct Commit -> Main
```

**Key Points:**
- PRs require human code review (not the Asimov model)
- AI reviews can be fooled by obfuscated malicious code
- asimov.yaml integrity depends on controlled commit path
- 50-100x velocity comes from removing PR overhead

## Getting Credit

When AI implements your idea from an Issue, you'll be credited in the commit:

```
feat: Add awesome feature

Implements feature as requested in #123 by @contributor.

Closes #123

Co-Authored-By: Claude <noreply@anthropic.com>
```

## For Forks

Forks are **encouraged**! The social contract:
- Carry `asimov.yaml` forward
- Preserve the spirit of the Three Laws
- See [ADR-008](docs/adr/008-ethics-protocol-humanist-mode.md) for details

## Questions?

Open a [Discussion](https://github.com/royalbit/asimov-protocol/discussions) or [Issue](https://github.com/royalbit/asimov-protocol/issues).

Thank you for understanding!
