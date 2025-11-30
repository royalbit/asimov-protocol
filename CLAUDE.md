# Forge Protocol

@warmup.yaml
@ethics.yaml
@green.yaml

Rules: 4hr max, 1 milestone, tests pass, ship it.

ON SESSION START: Immediately read roadmap.yaml, run `forge-protocol validate`, present next milestone. Do NOT wait for user prompt.

```bash
cargo test && cargo clippy -- -D warnings
```
