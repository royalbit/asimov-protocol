# Forge Protocol - Vendor-Neutral AI Session Continuity

## CRITICAL: Self-Healing Protocol (Survives Auto-Compact)

After ANY context compaction, confusion, or uncertainty, RE-READ:
1. `warmup.yaml` - Full protocol and rules
2. `.claude_checkpoint.yaml` - Session state (if exists)

## Mandatory Checkpoints

- **Every 2 hours**: Write progress to `.claude_checkpoint.yaml`, re-read `warmup.yaml`
- **Before any commit**: Re-read quality gates from `warmup.yaml`
- **After task completion**: Update `.claude_checkpoint.yaml`
- **When confused**: STOP → re-read `warmup.yaml` → re-read `.claude_checkpoint.yaml`

## Core Rules (Memorize - These Must Survive)

- 4hr MAX session, 1 milestone, NO scope creep
- Tests pass + ZERO warnings → then commit
- NO "let me also...", NO "while I'm here..."
- Done > Perfect. Ship it.

## Commands

```bash
cargo test                           # Must pass
cargo clippy -- -D warnings          # Zero warnings
cd cli && cargo publish              # Release to crates.io
```

## Key Files

- `warmup.yaml` - Full protocol (RE-READ after compact)
- `cli/Cargo.toml` - Version
- `cli/src/` - CLI source code
- `docs/SPECIFICATION.md` - Protocol spec
