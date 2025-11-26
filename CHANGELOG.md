# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.0] - 2025-11-26

### Added

- **CLI Validator**: `forge-protocol validate` - Validates protocol files against JSON schemas
- **Template Generator**: `forge-protocol init` - Generates starter protocol files
- **Language Templates**: `--type rust` for Rust-specific templates (generic is default)
- **Full Flag**: `--full` generates all three protocol files (warmup, sprint, roadmap)
- **Force Flag**: `--force` overwrites existing files
- **Check Command**: `forge-protocol check` as alias for validate
- **Comprehensive Tests**: 37 unit tests + 25 e2e tests
- **Pre-commit Hooks**: cargo-husky for automatic fmt + clippy checks

### Technical

- Written in Rust for zero-dependency distribution
- JSON Schema validation using `jsonschema` crate
- YAML parsing with `serde_yaml`
- CLI built with `clap` derive macros
- Colored output for terminal feedback

## [1.0.0] - 2025-11-25

### Added

- Initial protocol specification
- Core files: `warmup.yaml`, `sprint.yaml`, `roadmap.yaml`
- Full documentation with Mermaid diagrams
- Examples for Rust, Python, JavaScript, monorepos
- Guides for autonomous sessions and sprint protocol
- Stories documenting AI development journey
- Research on experiential continuity
- Markdown linting with markdownlint-cli2

### Documentation

- `docs/SPECIFICATION.md` - Full protocol specification
- `docs/EXAMPLES.md` - Example configurations
- `docs/MANIFESTO.md` - Philosophy and methodology
- `docs/ECOSYSTEM.md` - Full ecosystem case study
- `docs/GREEN_CODING.md` - Cost and carbon savings

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)
