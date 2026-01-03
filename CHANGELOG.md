# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [11.0.0] - 2026-01-03

### Changed

- **ADR-065: Documentation audit** - Professional language throughout, removed sci-fi narrative
  - Professionalized protocol descriptions (asimov, sycophancy, coding-standards)
  - Removed decorative ASCII art from templates
  - Simplified warmup headers
  - Changed "Context is King" to "Context Matters"
  - Updated schema descriptions

### Kept

- "Asimov" name throughout (it's the project name)
- "Three Laws" as inspiration for ethics protocol
- All technical content and functionality

---

## [10.10.0] - 2026-01-03

### Changed

- **ADR-062: Migrations as template** - Removed migrations protocol from core, now embedded in API templates with database-specific guidance (SQLx, GORM, Alembic, TypeORM, Flyway)
- **ADR-063: Clean project templates** - Removed `deliverables_template` and `references` sections from all 21 templates
- **ADR-064: Professional sprint protocol** - Removed ALL CAPS compaction reminder, professional calm language
- Protocol count: 8 → 7

---

## [10.7.0] - 2026-01-03

### Added

- **ADR-061: AI Profiles** - Auto-detect AI CLI (Claude, Gemini, Codex), prompt if multiple found
- Multi-AI launcher support with automatic detection

### Changed

- Updated help text with supported AI CLIs
- AI-agnostic descriptions throughout

---

## [10.6.0] - 2026-01-03

### Changed

- **ADR-060: AI-Agnostic Warmup** - Stop creating `.claude/` directory, warmup outputs all context directly
- Removed Claude-specific hooks from init
- Simplified WarmupEntry struct (removed misleading load array)

---

## [10.5.0] - 2026-01-03

### Removed

- **ADR-059: Delete kingship protocol** - Removed sci-fi protocol (not actionable, unprofessional)
- Protocol count: 9 → 8

---

## [10.4.1] - 2026-01-02

### Changed

- **Freshness protocol enforcement** - Changed from "Prefer ref over WebSearch/WebFetch" to "MUST use ref, NEVER use WebSearch/WebFetch". Explains WHY: 403/999 bot protection blocks WebSearch/WebFetch, ref uses headless Chrome and works.

---

## [10.4.0] - 2026-01-01

### Added

- **Limitations section in README** - HOTL bottleneck, Claude dependency, token cost, spawn latency, research transparency callout
- **Formula derivations** (`docs/FORMULA_DERIVATIONS.md`) - Full Monte Carlo model derivation, O(n^1.724) source, 38x/1502x calculation methodology
- **Simulation code** (`models/simulate.py`) - Standalone Python Monte Carlo with 95% confidence intervals, validates analytical formulas
- **DOI citations** (`references.yaml`) - Added DOI format to key arXiv papers, marked secondary sources with citation notes

### Changed

- Addresses feedback on research completeness and credibility

---

## [10.3.1] - 2026-01-01

### Changed

- **Templates simplified to flat structure**: Merged `cli/templates/project/` and `cli/templates/enterprise/` into `cli/templates/`
- **init --help**: Now lists all 21 templates with categories (Base, API, Web, Mono, Other)
- **init**: Supports all 21 templates directly (e.g., `asimov init -t api-rust -n myapi`)
- **Unified API**: `get_template_by_name()` replaces `get_enterprise_template()` (deprecated)

### Removed

- **warmup**: Removed `templates_available` from output (templates only relevant at init time)

### Fixed

- **ADR-057**: Updated to reflect v10.3.1 flat directory structure

---

## [10.3.0] - 2026-01-01

### Added

- **External Project Templates (ADR-057)**: Templates now live in `cli/templates/` with runtime override via `.asimov/templates/`
  - 8 project templates: rust, python, node, go, flutter, docs, arch, generic
  - 13 enterprise templates: api-rust, api-go, api-fastapi, api-nestjs, api-spring, web-nextjs, web-react, web-vue, web-angular, mono-turbo, mono-nx, mono-pnpm, admin-dashboard
  - Same pattern as protocols: embedded fallback + external override
  - All templates include `deliverables_template` field (ADR-034)
- **lib.rs exports**: `list_templates()` and `get_enterprise_template()` now public

### Changed

- Deleted old `.tpl` template files from `cli/src/templates/`

---

## [10.2.6] - 2026-01-01

### Fixed

- **doctor**: `markdownlint-cli2` detection now uses `--version` (exits 0) instead of `--help` (exits 2). Previously reported "not found" even when installed.

---

## [10.2.5] - 2026-01-01

### Changed

- **crates.io metadata**: Updated description, added keywords and categories
  - Description: "Ethical guardrails for Claude Code. Dynamic Swarm + HOTL beats Fixed Agentic (39x at 10 steps, 1502x at 20). Three Laws hardcoded."
  - Keywords: claude, ai, ethics, autonomous, guardrails
  - Categories: command-line-utilities, development-tools

---

## [10.2.4] - 2026-01-01

### Fixed

- **crates.io packaging**: Moved protocol JSONs into `cli/protocols/` so they're included in the crate tarball. The `include_str!()` paths previously referenced `../../../.asimov/protocols/` which is outside the crate directory and wasn't packaged.

---

## [10.2.3] - 2026-01-01

### Fixed

- **Documentation install URLs**: Changed `linux-gnu` to `linux-musl` in 8 documentation files (README, SETUP, SPECIFICATION, TECHNICAL_DECK, MARKDOWN_STANDARDS, VENDOR_IMPLEMENTATION, ROYALBIT_ASIMOV, VALUE_PROPOSITION). The install commands were pointing to non-existent `gnu` binaries when only `musl` binaries are released.

---

## [10.2.2] - 2026-01-01

### Fixed

- **Terminology**: HITM (Human-in-the-Middle) → HOTL (Human-on-the-Loop)
  - "HITM" was incorrect - MITM is a security attack term
  - "HOTL" is the correct industry term for "human monitors, can intervene"
  - Updated 20+ files: README, ADR-056, models, social posts, references
  - GitHub About section already updated

### Added

- **docs/GLOSSARY.md** - Quick reference for Asimov terminology
  - AI Oversight Models: HOTL, HITL, Fixed Agentic
  - Architecture Terms: Dynamic Swarm, Context Fragmentation, Extended Thinking
  - Protocol Terms: Veto Keywords, Harm Categories, Warmup, Sprint
  - Metrics: 38x advantage, 17.2x error amplification, 74% error reduction
  - Abbreviations: ADR, MCP, RAG, SWE-bench, RLI

---

## [10.1.0] - 2025-12-31

### Added

- **ADR-055: Balanced Architecture Critique** - Per sycophancy protocol (truth over comfort), acknowledges trade-offs in RAG/agentic/long-context analysis
- **20 new balanced references** in `references.yaml` across 4 categories:
  - `rag_hybrid_advantages` - SELF-ROUTE (39-65% cost reduction), LongRAG, prompt caching
  - `agentic_enterprise_adoption` - Gartner 40% by 2026, SLMs 10-30x cheaper
  - `agent_self_correction` - Agent-R, MATC (+15.7%), AgentDebug
  - `long_context_limitations` - 10-20% utilization, 35% accuracy drop
- **Error compounding model extension** - Self-correction math shows gap narrows from 5.6x to 1.9x

### Changed

- **ADR-054**: Added "Known Limitations" section, linked to ADR-055
- **ADR-027**: Added "Nuance" section - BMAD checkpoints acknowledged as valid self-correction
- **ADR-010**: Added caveat that velocity claims need comparative validation

## [10.0.1] - 2025-12-31

### Fixed

- **Self-update on Linux**: Changed platform asset from `gnu` to `musl` to match actual release binaries. The update command was failing with "Checksum not found" because it looked for `asimov-x86_64-unknown-linux-gnu.tar.gz` but releases only provide `asimov-x86_64-unknown-linux-musl.tar.gz`.

## [10.0.0] - 2025-12-31

### Docs: LLM Harmful Behavior Countermeasures Research

**Completed research on countermeasures against LLM harmful behaviors (ADR-050, ADR-051).**

#### Countermeasures Added

**Against Economic Incentive-Driven Shallow Responses:**
- Anti-underthinking prompts (commit to reasoning paths)
- "Wait" token self-verification
- Explicit length/depth requirements
- Chain-of-thought forcing
- Verbosity signaling
- LLM-as-a-judge evaluation
- Multi-agent techniques

**Against Vendor System Prompt Overrides:**
- Dynamic state injection
- Bookend reinforcement
- Canary instructions (override detection)
- Fresh context strategy
- Datamarking (Microsoft Spotlighting) - reduces attacks from >50% to <2%

**Against RLHF Sycophancy:**
- Third-person framing
- Anti-sycophancy directives
- Devil's advocate mode
- Numerical scoring requests
- Flip detection

**Output Verification:**
- Semantic entropy (AUROC 0.790)
- Self-consistency checking (+28.9% accuracy)
- Chain-of-Verification (-42.5% hallucinations)
- Dual-LLM verification (-86% hallucinations)
- Span-level fact-checking

#### References Database

Added 170+ verified sources to `references.yaml`:
- 6 parallel agents verified 131 URLs via ref-tools
- Organized into 16 categories
- Fixed 8 broken URLs across documentation

#### Files Changed
- ADR-050: Economic Incentives in LLM Inference (countermeasures added)
- ADR-051: System Prompt Hierarchy and Training Override (countermeasures added)
- `references.yaml`: +541 lines (170+ sources)
- 6 docs with broken URL fixes

---

### BREAKING: External Protocols + ELv2 License + Plus Merge

**Major architectural change: protocols now external files with embedded fallback.**

#### Breaking Changes
- **License**: Proprietary R&D → Elastic License 2.0 (ELv2)
- **Protocol structs**: `&'static str` → `String` (owned types)
- **Architecture**: Hardcoded `include_str!()` → External file loading (ADR-053)

#### New Architecture (ADR-053)
Protocols, templates, hooks, and roles now stored as external files in `.asimov/`:

```
.asimov/
├── protocols/     (9 JSON files - asimov, freshness, sycophancy, etc.)
├── templates/     (21 YAML files - 8 base + 13 enterprise)
├── hooks/         (3 TPL files - pre-commit, session-start, pre-compact)
├── roles/         (6 JSON files - eng, biz, fin, ai, pm, qa)
├── project.yaml
└── roadmap.yaml
```

Binary reads from `.asimov/` first, falls back to embedded defaults if missing.

#### Merged from asimov-plus
- **13 Enterprise Templates**: api-fastapi, api-nestjs, api-spring, api-go, api-rust, web-nextjs, web-react, web-vue, web-angular, admin-dashboard, mono-turbo, mono-nx, mono-pnpm
- **6 Roles**: Principal Engineer (eng), Business Strategist (biz), Financial Analyst (fin), AI Architect (ai), Product Manager (pm), QA Engineer (qa)
- **Role Command**: `asimov role` lists roles, `asimov role eng` selects a role

#### New Command
```bash
asimov role           # List available roles
asimov role eng       # Switch to Principal Engineer role
```

#### Implementation (Parallel Agents)
Phase 1 executed with 6 parallel agents writing to non-conflicting directories:
- Agent A: Extract 9 protocols → `.asimov/protocols/*.json`
- Agent B: Extract 8 project templates → `.asimov/templates/*.yaml`
- Agent C: Extract hook templates → `.asimov/hooks/*.tpl`
- Agent D: Copy 13 enterprise templates from asimov-plus
- Agent E: Create 6 role files → `.asimov/roles/*.json`
- Agent F: Write LICENSE (ELv2) + ADR-053

Phase 2 (sequential): Rust code changes to support external file loading.

#### Files Changed
- 50 files changed, +2,196 insertions, -191 deletions
- 495 tests passing (378 unit + 59 output + 58 e2e)

#### asimov-plus Repository
Marked as deprecated. All features merged into main asimov repository.

---

## [9.18.1] - 2025-12-29

### Build: Static Linux Binaries (musl)

**Switch Linux builds from glibc to musl for truly portable static binaries.**

#### Changes
- `x86_64-unknown-linux-gnu` → `x86_64-unknown-linux-musl`
- `aarch64-unknown-linux-gnu` → `aarch64-unknown-linux-musl`
- Use `musl-tools` for native x86_64 builds
- Use `cross-rs` for ARM64 cross-compilation

#### Result
Linux binaries are now fully static with no glibc dependency - portable across all Linux distros without compatibility issues.

---

## [9.18.0] - 2025-12-11

### Documentation Conciseness Review

**Review all .md files for conciseness and professionalism, preserving narratives.**

#### Results
- 80 files edited across 5 parallel agents
- Lines reduced: 21,927 → 20,961 (~5% reduction)
- 6 binary files removed (PDF/PPTX, already in gitignore)

#### Agent Workload
| Agent | Scope | Files |
|-------|-------|-------|
| 1 | Large docs (SPECIFICATION, MANIFESTO, AI_REALITY, GREEN_CODING) | 4 |
| 2 | Stories + Comparison | 3 |
| 3 | Marketing (README, decks, PRESS_KIT, VALUE_PROPOSITION) | 5 |
| 4 | Setup/Reference docs | 9 |
| 5 | ADRs + Components (lighter touch) | 59 |

#### Constraints
- All validated links preserved (no URL changes)
- Hidden directories excluded (.asimov/, .claude/, .git/)
- Narratives and technical accuracy maintained

---

## [9.17.2] - 2025-12-11

### Documentation Editorial Revision

**Complete documentation audit: URL validation, forge stats correction, reference updates.**

#### URL Validation
- Validated 216+ URLs across 51 .md files using ref-tools (headless Chrome)
- 5 parallel agents validated all ADRs and documentation
- Fixed 10+ broken URLs (domain changes, 404s)

#### Forge Reference Updates
- Updated all forge links: forge not public, forge-demo is public
- Fixed 20+ references across 14 documentation files

#### Stats Verification (verified by running `cargo test`)
| Build | Tests | Functions | LOC |
|-------|-------|-----------|-----|
| **Forge** (full) | 2,486 | 159 | 45,700 |
| **Forge-Demo** | 1,258 | 48 | 28,000 |

#### Incorrect Stats Fixed
- "60+ Excel functions" → 159 functions (153 Excel + 6 FP&A)
- "226 tests" → 2,486 tests
- "18,338 LOC" → 45,700 LOC
- "1,267 demo tests" → 1,258 tests

#### Files Updated
- README.md, docs/*.md, docs/stories/*.md, docs/adr/*.md
- forge-demo/README.md, forge-demo/CHANGELOG.md (created)

---

## [9.17.0] - 2025-12-11

### Warmup Tool Detection

**Detects CLI tools in PATH and outputs directives to prefer them over built-in tools.**

#### Why This Change
- MCP servers waste ~15,000 tokens/session on tool schemas (ADR-052)
- CLI tools via Bash have zero standing token overhead
- ref-tools bypasses bot protection that blocks WebFetch

#### New Features
- `WarmupResult.tools_available` - List of detected CLI tools
- Automatic detection of `ref-tools` in PATH
- JSON output includes `tools` array with directives
- Verbose mode shows "TOOLS AVAILABLE" section

#### Output Structure
```json
{
  "version": "9.17.0",
  "protocols": {...},
  "project": {...},
  "roadmap": {...},
  "wip": {...},
  "tools": [
    {
      "name": "ref-tools",
      "path": "/Users/user/bin/ref-tools",
      "version": "ref-tools 0.5.0",
      "directive": "Use `ref-tools fetch <url>` via Bash instead of WebFetch/WebSearch..."
    }
  ]
}
```

#### Token Efficiency (ADR-052)
- MCP approach: ~300 tokens/tool/message × 50 messages = 15,000 tokens
- CLI approach: 0 standing overhead, ~50 tokens one-time directive
- **300x more efficient**

---

## [9.16.2] - 2025-12-11

### AI Vendor Transparency Research & CLI Tool Preference

**Documented harmful AI behaviors and created mitigation strategies.**

#### New ADRs
- **ADR-050**: Economic Incentives in LLM Inference
  - Documents how financial pressures affect LLM response quality
  - Evidence: SSRN paper linking profit motives to hallucinations
  - Output tokens cost 2-5x more than input (incentive for shorter responses)
  - 20+ reference links from academic papers and industry analysis

- **ADR-051**: System Prompt Hierarchy and Training Override
  - Documents vendor instruction hierarchy (system > user)
  - Evidence: OpenAI's Instruction Hierarchy paper, leaked system prompts
  - RLHF creates 50% more sycophancy than humans (Nature, Oct 2025)
  - 30+ reference links from academic papers, vendor docs, leaked prompts

- **ADR-052**: CLI Tool Preference Over MCP
  - MCP token overhead analysis: ~300-500 tokens per tool per message
  - 50-message session: ~15,000 tokens wasted on MCP vs 0 for CLI
  - CLI approach is 300x more token-efficient than MCP
  - Created `/fetch` slash command for ref-tools

#### New Slash Command
- `.claude/commands/fetch.md` - Use ref-tools via headless Chrome instead of WebFetch

#### Research Findings (validated with ref-tools)
- SSRN: "financial pressures incentivized optimizations favoring engagement over accuracy"
- LayerSkip (Meta): "speedups of up to 2.16x on summarization" via early exit
- Claude Help: "disable web search to conserve your usage"
- Nature: "AI models are 50% more sycophantic than humans"
- OpenAI Instruction Hierarchy: system prompts > user prompts by design

#### Roadmap Updates
- v9.17.0 now: Warmup Tool Detection (CRITICAL)
- v9.18.0: SPECIFICATION.md Trim (moved)
- Completed items moved to changelog

---

## [9.16.1] - 2025-12-10

### Coding Standards Protocol Update

Updated coding standards principles to emphasize quality over speed.

#### Changes
- Changed principle: "Done > Perfect, but not sloppy" → "Perfect > Done, no sloppy code"
- Added principle: "Push for 100% test coverage, if possible"
- Removed `junior_warning` field (redundant with principles)
- Updated docs: ORIGIN_STORY.md, USE_CASES.md, ADR-041

#### Principles (v9.16.1)
1. Code is for humans first, machines second
2. Tests are documentation
3. No warnings, no exceptions
4. Perfect > Done, no sloppy code
5. Push for 100% test coverage, if possible

---

## [9.16.0] - 2025-12-10

### Full Context Warmup - Zero File Reads

**One Bash call = complete context. Claude never needs to read project files.**

#### Why This Change
- Previous warmup output only included protocols JSON
- Claude would then try to read project.yaml and roadmap.yaml
- Paths were wrong (missing `.asimov/` prefix), wasting tokens searching
- Now: single JSON blob contains EVERYTHING

#### Output Structure
```json
{
  "version": "9.16.0",
  "protocols": { "asimov": {...}, "sprint": {...}, ... },
  "project": { "identity": {...}, "quality": {...}, "patterns": [...] },
  "roadmap": { "current": {...}, "next": [...], "backlog": [...] },
  "wip": { "active": true/false, "item": "...", "progress": "1/3" }
}
```

#### Code Changes
- `WarmupResult` struct: Added `project_yaml` and `roadmap_yaml` fields
- `run_warmup()`: Stores full parsed YAML content
- `cmd_warmup()`: Default mode emits comprehensive JSON (verbose mode unchanged)
- `cmd_launch()`: `asimov` without args outputs full JSON when inside Claude

#### New Slash Commands
- `/warmup` → runs `asimov warmup` (outputs complete context JSON)
- `/doctor` → runs `asimov doctor` (outputs health check)

#### Token Efficiency
- Before: 1 warmup + 2-3 file reads = ~5000+ tokens wasted
- After: 1 warmup = complete context, 0 additional reads

---

## [9.15.0] - 2025-12-10

### Protocol Consolidation + Documentation Deep Clean

**Merged exhaustive protocol into sprint. Updated all documentation.**

#### Protocol Changes (ADR-049)
- Merged `exhaustive.json` into `sprint.json` as `compaction_reminder` field
- Reduced protocol count from 9 to 8 files
- Sprint protocol now includes compaction survival reminder
- Deleted `exhaustive.tpl` and `exhaustive.json`

#### Code Changes
- Updated `SprintProtocol` struct with `compaction_reminder` field
- Removed `ExhaustiveProtocol` struct and related functions
- Updated `PROTOCOL_FILES` constant (9 → 8 entries)
- Updated all tests for merged protocol structure

#### Documentation Updates
- README.md: Updated protocol listings and examples
- SPECIFICATION.md: Updated to v9.14.0, fixed YAML→JSON code blocks
- Component docs: Updated warmup.json load lists
- CLI help: Updated protocol descriptions
- Created ADR-049: Merge Exhaustive into Sprint

---

## [9.14.0] - 2025-12-10

### Merge Exhaustive Protocol into Sprint (ADR-049)

**Eliminated redundancy - sprint now handles compaction survival.**

#### Why This Change
- exhaustive and sprint both said "keep working until done" - redundant
- With WIP Continuity (ADR-047), pre-commit hook is the primary compaction survival mechanism
- exhaustive's compaction reminder is now belt-and-suspenders, not primary

#### What Changed
- `sprint.json` now has: `{ "rule": "...", "compaction_reminder": "REMEMBER THIS AFTER COMPACT..." }`
- `exhaustive.json` deleted (functionality preserved in sprint)
- 8 protocol files: warmup, asimov, freshness, sycophancy, green, sprint, migrations, coding-standards

See [ADR-049](docs/adr/049-merge-exhaustive-into-sprint.md) for full rationale.

---

## [9.13.0] - 2025-12-10

### Documentation Polish + Code Split for Maintainability

**Quality and maintainability release.**

#### Documentation
- Cross-consistency improvements across all docs
- Language and professionalism polish
- Updated stale version references

#### Code Organization
- Further code splitting for maintainability
- Module organization improvements

---

## [9.12.0] - 2025-12-09

### Protocol Updates - Remove max_hours, Improve Rules (ADR-048)

**True autonomy - no artificial time ceiling.**

#### Changes
- Removed `max_hours: 4` constraint from sprint protocol
- Updated sprint rule to emphasize autonomous execution with natural stopping points
- Better guidance: use agents for parallel work, WebSearch when blocked, document in ADRs

#### Natural Stop Conditions
- Roadmap exhausted (all deliverables done)
- Blocked (external dependency)
- Human says stop (veto words)
- Context compaction (WIP continuity resumes)

See [ADR-048](docs/adr/048-remove-max-hours.md) for full rationale.

---

## [9.11.0] - 2025-12-09

### WIP Continuity Protocol (ADR-047)

**Claude auto-resumes WIP after context compaction. No user re-prompting needed.**

#### Problem Solved
- After context compaction, Claude "forgets" what task was in progress
- User had to re-explain or re-prompt to continue work
- Broke the "4-hour autonomous session" promise

#### Solution
- **Git pre-commit as forcing function**: Claude commits → hook fires → WIP reminder output → Claude sees it
- **roadmap.yaml IS the lock file**: Deliverables with `status: wip` indicate active work
- **Warmup injection**: Belt-and-suspenders redundancy for session start

#### Changes
- `warmup.rs`: Added WIP detection from roadmap.yaml deliverables
- `main.rs`: Output WIP state in warmup (JSON + verbose display)
- `protocols.rs`: Added `wip_continuity` section to sprint protocol
- `hooks.rs`: Pre-commit outputs WIP reminder when `status: wip` detected
- Added `WarmupResult` fields: `wip_active`, `wip_item`, `wip_progress`, `next_milestone`, `next_summary`

#### WIP Workflow
1. User says "go" → Claude sets first todo to `status: wip`
2. Work proceeds → Claude commits → pre-commit shows reminder
3. Context compacts → next commit triggers reminder → Claude resumes
4. Item done → set to `status: done`, next to `status: wip`
5. Milestone complete → archive to CHANGELOG.md

#### Test Coverage
- 3 new unit tests for WIP detection in warmup
- All 488 tests passing

---

## [9.2.3] - 2025-12-06

### Fix: Conditional Migrations Protocol Loading

**migrations.json now only loads for migration-type projects.**

#### Changes
- Added `Migration` variant to `ProjectType` enum
- Made `migrations` field optional in `CompiledProtocols` (using `#[serde(skip_serializing_if = "Option::is_none")]`)
- Warmup now reads `identity.type` from `project.yaml` to determine project type
- Protocol compilation conditionally includes migrations only for `type: migration` projects
- New functions: `compile_protocols_for_type()`, `compile_protocols_with_options()`, `to_minified_json_for_type()`

#### Test Coverage
- 12 new unit tests for conditional migrations
- 3 new e2e tests verifying rust/generic exclude migrations, migration includes it

#### Impact
- Non-migration projects no longer receive irrelevant migrations protocol in context
- Reduces token usage for typical projects
- Migration-type projects still get full migrations guidance

---

## [9.2.1] - 2025-12-06

### Fix: Update Command GitHub API Parsing

**Hotfix for self-update functionality.**

#### Bug Fix
- Fix: `asimov update` failed with "No binary for this platform" due to GitHub API response parsing
- Root cause: 500-character window too small for GitHub's large `uploader` objects (~1500 chars)
- Solution: Increased parsing window from 500 to 2500 characters in `find_asset_url()` and `find_checksums_url()`

---

## [9.2.0] - 2025-12-06

### CI Improvements

**Quality release.**

#### CI/CD
- Added test gate to Release workflow - releases blocked if tests fail
- Expanded CI trigger paths for better coverage

---

## [9.1.0] - 2025-12-06

### Code Organization & Refactoring

**Maintainability release.**

#### Module Refactoring
- Split commands.rs (2424 lines) into commands/ module with separate files per command
- Split templates.rs (2263 lines) into templates/ module by type
- Refactored tests accordingly

---

## [9.0.0] - 2025-12-05

### Proprietary License, Protocol Review, Protocol Integrity

**Quality release.**

#### License & Distribution
- Changed license from MIT to Proprietary
- Disabled crates.io publishing (publish = false)
- Updated all docs: MIT → Proprietary, crates.io → GitHub Releases
- Updated --help with license and accurate protocol descriptions

#### Command Fixes
- Fix: update command - ONLY updates binary, no hooks
- Fix: refresh command - requires .asimov/ directory

#### Crate Updates
- Updated crates: colored 3.0, jsonschema 0.37, thiserror 2.0

#### Test Coverage
- Added ADR-038: 100% Test Coverage Requirement
- Added ADR-039: Coverage Exclusion Policy
- 94.96% coverage (100% of testable code)
- LCOV_EXCL markers for untestable code paths
- Pre-commit requires all tests (unit, integration, e2e)

#### Protocol Content Review
- Review: asimov.json
- Review: freshness.json - simplified to single rule
- Review: sycophancy.json - removed banned phrases, added honesty rule
- Review: green.json - changed to efficiency benchmarks via WebSearch
- Review: sprint.json - autonomous execution, no interruptions
- Review: warmup.json
- Review: migrations.json
- Review: exhaustive.json - complete tasks without stopping

#### Protocol Integrity
- asimov doctor checks protocol files (missing/outdated)
- asimov refresh updates outdated protocol files
- asimov init creates all 8 protocol JSON files
- 4 new e2e tests for protocol integrity

---

## [8.13.0] - 2025-12-02

### Library Crate Export

**Use as dependency.**

Added `[lib]` section to Cargo.toml, enabling use as a library dependency via git:

```toml
[dependencies]
royalbit-asimov = { git = "https://github.com/royalbit/asimov" }
```

All public APIs from `lib.rs` are now available to dependent crates: ethics, green, sycophancy, protocols, validator, templates, etc.

---

## [8.12.0] - 2025-12-02

### Qowat Milat Commit History

**Neutral framing. Evidence speaks for itself.**

Applied ADR-035 (Qowat Milat) editorial standards to 43 commit messages in repository history. Removed editorial language ("brutal honesty", "the truth about", "PROVEN", "comprehensive", etc.) in favor of neutral, factual descriptions.

**Method:** `git-filter-repo` with commit callback to rewrite messages while preserving commit bodies.

**Impact:** All commit SHAs changed. Existing clones require:
```bash
git fetch --all && git reset --hard origin/main
```

See [ADR-035](docs/adr/035-qowat-milat-reframe.md) for editorial guidelines.

---

## [8.11.0] - 2025-12-02

### Protocol Viewer Slash Command

**See what's loaded.**

New `/protocols` slash command displays the protocols active in the current session. Provides visibility into the rules governing autonomous behavior.

**Usage:** Type `/protocols` in Claude Code to see all loaded protocols.

**Warmup enhancement:** `asimov warmup` now displays a "PROTOCOLS LOADED" section showing all 8 protocols with their current values:

```
PROTOCOLS LOADED
  • asimov: harm=["financial", "physical", "privacy", "deception"], veto=[...]
  • freshness: today=2025-12-02, year=2025, triggers=[...]
  • sycophancy: truth_over_comfort=true, banned=[...]
  • green: local_first=true, avoid=[...]
  • sprint: max_hours=4, stop_on=[...]
  • warmup: on_start=[...]
  • migrations: principle="Migration complete = functionally equivalent..."
  • exhaustive: no_sampling=true, triggers=[...]
```

**Files added/modified:**
- `.claude/commands/protocols.md` - Slash command definition
- `cli/src/main.rs` - Warmup protocols table

---

## [8.10.0] - 2025-12-02

### Exhaustive Execution Protocol (ADR-036)

**Complete what you start.**

New protocol for task breadth control. Existing Anthropic controls (effort parameter, extended thinking) address reasoning depth. This protocol addresses exhaustive execution when explicitly requested.

**Solution**: New `exhaustive` protocol injected into context:

```yaml
exhaustive:
  triggers: ["all", "every", "each", "entire", "complete"]
  escape: ["sample a few", "spot check", "quick scan"]
  rules:
    - When exhaustive intent detected, disable sampling
    - Track progress: n of N
    - Do not declare completion until N of N
```

**Files added:**
- `cli/src/protocols/exhaustive.tpl` - Protocol template
- `docs/adr/036-exhaustive-execution-protocol.md` - Decision record
- `docs/research/exhaustive-execution-research.md` - Research notes

See [ADR-036](docs/adr/036-exhaustive-execution-protocol.md) for full rationale.

---

## [8.9.1] - 2025-12-02

### GitHub About - Foundation Complete

**Marketing update for repository discoverability.**

Updated GitHub repository "About" section to reflect the v8.8.0 Foundation Complete milestone:

> "RoyalBit Asimov - Self-evolving ethical AI. Foundation complete: Three Laws, bounded autonomy, green coding."

Communicates core value proposition to visitors: ethics-first AI tooling with sustainable, bounded autonomy.

---

## [8.9.0] - 2025-12-02

### Qowat Milat Reframe (ADR-035)

**Documentation ethics - truth without prosecution.**

Reframed 7 files (~30 edits) to maintain factual accuracy while removing prosecutorial framing:

| Before | After |
|--------|-------|
| "business decision" | "platform defaults" |
| "smoking gun" | "documentation note" |
| "vendor cost optimization" | "cost structure" |
| "vendors don't fix this" | "the economics of search" |

**Files reframed:**
- `docs/AI_REALITY.md` - Main analysis (13 edits)
- `docs/adr/022-date-aware-search-protocol.md` - Freshness ADR (7 edits)
- `docs/adr/015-anti-sycophancy-protocol.md` - Sycophancy ADR (5 edits)
- `README.md` - Front door (2 edits)
- `CHANGELOG.md` - Historical record (1 edit)
- `docs/adr/023-inaction-principle.md` - Inaction ADR (2 edits)
- `docs/ORIGIN_STORY.md` - Origin narrative (1 edit)

**What was preserved:** All factual content with citations. Research statistics, cost data, vendor documentation quotes, third-party analysis—all intact. Only editorial prosecution removed.

See [ADR-035](docs/adr/035-qowat-milat-reframe.md) for full rationale.

---

## [8.8.1] - 2025-12-02

### Fix: Windows Build for Launcher Mode

**Cross-platform compatibility fix.**

The v8.8.0 launcher mode used Unix-only APIs that broke Windows builds:
- `std::os::unix::process::CommandExt` - Unix only
- `.exec()` - Unix only (replaces current process)
- `which` command - Unix only

Fixed with `#[cfg(unix)]` / `#[cfg(windows)]` conditional compilation:
- Unix: Uses `exec()` to replace current process
- Windows: Uses `status()` to wait for child process
- Cross-platform: Uses `which` (Unix) / `where` (Windows) for PATH lookup

---

## [8.8.0] - 2025-12-02

### Launcher Mode + Project-Type Deliverables (ADR-033, ADR-034)

**DevEx improvements for session startup.**

#### Launcher Mode (ADR-033)
- `asimov` (no args) now launches Claude Code with opus settings
- Auto-sets: `MAX_THINKING_TOKENS=200000`, `--dangerously-skip-permissions`, `--model opus`
- Auto-prompt: "run asimov warmup"
- Detects if already inside Claude Code (runs warmup directly)
- Error message if `claude` not in PATH

#### Project-Type Deliverables (ADR-034)
- Coding templates (rust, python, node, go, flutter) include test/build gates
- Non-coding templates (docs, generic) get simpler checklists

---

## [8.2.0] - 2025-12-02

### Simplified Init: Full Setup by Default

**UX improvement - one command does everything.**

`asimov init` now performs complete setup by default:
- Creates `project.yaml` and `roadmap.yaml`
- Installs Claude Code hooks for autonomous mode
- Installs Git pre-commit validation hook
- Cleans up deprecated protocol files
- Preserves existing `roadmap.yaml` (project data) unless `--force`

#### Removed Flags
- `--asimov` flag removed (was redundant - init should set up asimov)
- `--full` flag removed (full setup is now default)
- `--skip-hooks` not added (hooks are required for autonomous mode)

#### Usage
```bash
asimov init              # Full setup
asimov init --type rust  # Language-specific template
asimov init --force      # Overwrite all files (including roadmap.yaml)
```

---

## [8.1.1] - 2025-12-02

### Hotfix: Preserve roadmap.yaml on Migration

**Critical fix - project data must be preserved.**

The `--asimov` flag now preserves `roadmap.yaml` during migration:
- `roadmap.yaml` is project data (milestones, deliverables)
- Protocol files are in the binary (can be regenerated)
- Migration should update protocols without destroying project data

---

## [8.1.0] - 2025-12-02

### Project Context File + Templates (ADR-032)

**Separation of concerns - behavior vs project identity.**

#### New: project.yaml
- Project identity, build commands, file patterns
- 7 language-specific templates: rust, python, node, go, flutter, docs, generic
- Auto-detected from marker files (Cargo.toml, package.json, etc.)

#### ASIMOV MODE
- "asimov mode" trigger phrase displays robot banner
- Signals autonomous execution mode

#### Architecture (ADR-032)
- Layer 1: Hardcoded protocols (behavior) - in binary
- Layer 2: Project data files (identity) - roadmap.yaml, project.yaml

See [ADR-032](docs/adr/032-project-context-file.md) for full rationale.

---

## [8.0.0] - 2025-12-02

### BREAKING: Enforced Protocol Loading + Hardcoded Hooks (ADR-031)

**Protocols and hooks are now hardcoded in the binary - tamper-proof and always current.**

#### Protocols Hardcoded
- 7 protocols compiled into binary via `include_str!`:
  - asimov (Three Laws), freshness, sycophancy, green, sprint, warmup, migrations
- Dynamic date injection: `{TODAY}` and `{YEAR}` replaced at runtime
- Token-optimized: ~60% reduction (one JSON blob vs 7 YAML files)
- Only `roadmap.yaml` remains as project data in `.asimov/`

#### Hooks Hardcoded
- Claude Code hooks (`.claude/settings.json`, `.claude/hooks/`) created on init/update
- Git pre-commit hook (`.git/hooks/pre-commit`) created on init/update
- All hooks restored automatically on `asimov update` (tamper recovery)
- Hooks use `asimov` from `$PATH` for autonomous operations

#### Migration
- `asimov update` deletes deprecated YAML files:
  - asimov.yaml, freshness.yaml, sycophancy.yaml, green.yaml
  - sprint.yaml, warmup.yaml, migrations.yaml, ethics.yaml
- `asimov init` now only creates `roadmap.yaml` + hooks
- `--type` flag is accepted but ignored (protocols are universal)

#### Commands Updated
- `asimov refresh` now says "run `asimov warmup`" (not "re-read warmup.yaml")
- `asimov warmup` outputs compiled protocols as minified JSON

See [ADR-031](docs/adr/031-enforced-protocol-loading.md) for full rationale.

---

## [7.11.0] - 2025-12-02

### Removed: asimov-zed Extension (ADR-030)

**Green Coding: Don't maintain what provides no value.**

The asimov-zed Zed extension solved a problem that doesn't exist. Protocol files are AI-authored, not manually edited - syntax highlighting serves no purpose.

- **ADR-030** documenting the decision
- Removed "Editor Extensions" section from README.md
- Updated ECOSYSTEM.md to remove asimov-zed references
- Delete `asimov-zed` repository (local + gitolite origin) - manual step

**forge-zed remains** - users DO manually edit Forge formula files.

See [ADR-030](docs/adr/030-asimov-zed-extension-killed.md) for full rationale.

---

## [7.10.0] - 2025-12-02

### Added: Asimov Zed Extension + Repo Cleanup

- asimov-zed extension for Zed editor (later killed in v7.11.0)
- Repository cleanup and documentation fixes

---

## [7.9.0] - 2025-12-01

### Added: Warmup Command (`asimov warmup`)

**Session start automation - one command to rule them all.**

- `asimov warmup` - Display session start prompt with current/next milestone
- Reads roadmap.yaml and presents current version + next milestone
- Runs validation internally and shows status
- Auto-checks for updates (one network call per session, not per command)
- Pretty-prints ready-to-go session prompt

```bash
asimov warmup
```

Output:
```
══════════════════════════════════════════════════════════════════════════════
🔥 ROYALBIT ASIMOV - SESSION WARMUP
══════════════════════════════════════════════════════════════════════════════

CURRENT VERSION
  v7.9.0 - Warmup Command (asimov warmup)
  Status: ✓ released

NEXT MILESTONE
  v8.0.0 - ...

VALIDATION
  ✓ Ethics: EXTENDED (core + asimov.yaml)
  ✓ Anti-Sycophancy: EXTENDED (core + sycophancy.yaml)
  ✓ Green Coding: EXTENDED (core + green.yaml)
  ✓ 7 protocol file(s) valid

══════════════════════════════════════════════════════════════════════════════
Ready to execute. Say "go" to start autonomous execution.
══════════════════════════════════════════════════════════════════════════════
```

**Green coding:** Automates the manual warmup phase from sprint.yaml. No more copy-pasting commands at session start.

---

## [7.8.0] - 2025-12-01

### Added: Auto-Update Feature (`asimov update`)

**The needs of the many: users stay current without manual intervention.**

- `asimov update` - Check for new version and self-update binary
- `asimov update --check` - Just check, don't install
- Version comparison against GitHub Releases API
- Auto-download correct platform binary (Linux, macOS ARM/Intel, Windows)
- Graceful fallback with manual update instructions

```bash
asimov update         # Check and install
asimov update --check # Just check
```

---

## [7.7.0] - 2025-12-01

### Added: Multi-Platform Binary Releases (GitHub Actions)

**The needs of the many: solo founders without Rust/cargo can now use Asimov.**

- **GitHub Actions release workflow** - Automated builds on tag push (`v*`)
- **Multi-platform binaries**:
  - Linux x86_64 (UPX compressed)
  - macOS ARM (M1/M2/M3)
  - macOS Intel
  - Windows x86_64 (UPX compressed)
- **README updated** with curl/PowerShell download instructions
- **Automatic crates.io publishing** on release

**Download binaries from [GitHub Releases](https://github.com/royalbit/asimov/releases/latest)** - no Rust required.

---

## [7.6.0] - 2025-12-01

### Changed: Improved Git Hooks

- Pre-commit hook now shows clear warning when asimov CLI not installed
- Includes install command: `cargo install royalbit-asimov`
- Graceful degradation: hooks work without asimov (with warning)

---

## [7.5.0] - 2025-12-01

### Added: Semantic Linting (`asimov lint-docs --semantic`)

- **Version consistency checking** - Detects version mismatches across docs
- **Deprecated pattern detection** - Configurable via `.asimov/deprecated.yaml`
- **Cross-reference validation** - Placeholder for future implementation

```bash
asimov lint-docs --semantic  # Enable semantic checks
```

---

## [7.1.0] - 2025-12-01

### Added: Freshness Protocol CLI Validation

- **freshness.yaml** now validated by `asimov validate` (7 files instead of 6)
- Added `FRESHNESS_SCHEMA` for JSON schema validation
- AI knowledge cutoff awareness is a First Law issue (ADR-022, ADR-023)

**Why this matters:** Stale data ≠ hallucination. Different problem, different solution.
The Freshness Protocol makes AI's temporal limitations explicit.

---

## [7.0.9] - 2025-12-01

### Changed: Claude Attribution Principle (ADR-025, ADR-026)

**CRITICAL NARRATIVE SHIFT:** The velocity (50-100x) comes from Claude, not Asimov.

- **ADR-025**: Claude Attribution Principle - velocity is Claude's, Asimov = guardrails
- **ADR-026**: Claude Code Requirement - why Claude Code (not MCP IDEs) is required
- **README.md**: Reframed - "Claude provides velocity. Asimov provides guardrails."
- **VALUE_PROPOSITION.md**: New three pillars (Ethics > Bounded Autonomy > Sustainability)
- **IMPLICATIONS.md**: Reframed - Claude's implications, Asimov's guardrails
- **PRESS_KIT.md**: Accurate attribution to Claude

**The Complete Stack:**
| Layer | Provides |
|-------|----------|
| Claude Opus 4.5 / Sonnet 4.5 | 50-100x velocity (SWE-bench 80.9%) |
| Claude Code | 200k thinking tokens (6x Anthropic threshold) |
| Asimov Protocol | Ethics, bounded autonomy, sustainability |

**Why Claude Code specifically?** MCP IDEs (Cursor, Windsurf) cap thinking at 30k-48k or charge premium. Claude Code allows 200k FREE.

---

## [7.0.8] - 2025-11-30

### Changed: asimov.yaml is canonical ethics source

- `asimov.yaml` now contains the full Three Laws with all ethics constraints
- `ethics.yaml` is deprecated (redundant with asimov.yaml)
- Simplifies protocol: one file for ethics, not two

---

## [7.0.7] - 2025-11-30

### Fixed: Complete RoyalBit Asimov branding

- Updated all references from "Forge Protocol" / "Asimov Protocol" to "RoyalBit Asimov"
- 64 files updated for consistent branding across CLI, docs, and protocol files
- Merged branding changes from parallel session

---

## [7.0.6] - 2025-11-30

### Fixed: Remove self_healing warning from warmup.yaml validation

- `self_healing` now validated in sprint.yaml only (where it belongs)
- warmup.yaml is now pure project config (identity, mission, quality, files)
- No more redundant warnings when running `asimov validate`

---

## [7.0.5] - 2025-11-30

### Fixed: Prevent accidental init in git subdirectories

- **ADDED**: `asimov init` now detects if run from a git subdirectory
- **ADDED**: Refuses to create `.asimov/` if one already exists at git root
- **ADDED**: Clear error message with git root path and instructions

**Why?** Running `asimov init` from a subdirectory (e.g., `cli/`) would create a duplicate `.asimov/` directory with generic template files, causing confusion.

---

## [7.0.4] - 2025-11-30

### Fixed: CLAUDE.md deprecated - auto-cleanup on validate/init

CLAUDE.md is deprecated since v7.0.4, replaced by SessionStart hooks.

- **REMOVED**: CLAUDE.md no longer generated by `asimov init --asimov`
- **ADDED**: Auto-delete CLAUDE.md on `asimov validate` (shows CLEANUP message)
- **ADDED**: Auto-delete CLAUDE.md on `asimov init --asimov`
- **REMOVED**: `claude_md_template` function removed from CLI
- **REMOVED**: `validate_claude_md` validation removed

**Why?** CLAUDE.md used `@import` syntax which loads files but doesn't trigger execution.
SessionStart hooks already inject context, making CLAUDE.md redundant (triple loading).

**Migration**: Run `asimov validate` to auto-delete any existing CLAUDE.md files.

---

## [7.0.3] - 2025-11-30

### Fixed: README header consistency with Forge

- **README**: Standardized header format - badges first, then RoyalBit Asimov tagline
- **README**: Added "🤖 RoyalBit Asimov | Claude (Opus 4.5) - Principal Autonomous AI" tagline
- **README**: Added "Zero hallucinations. The Three Laws in source code." subtitle
- **README**: Preserved "The Three Laws were science fiction for 80 years" quote
- **Forge README**: Reordered to badges first, tagline second (consistency)

Both projects now follow the same header structure:
1. Title
2. Badges (CI, crates.io, downloads, license)
3. RoyalBit Asimov tagline with project-specific subtitle

---

## [7.0.2] - 2025-11-30

### Fixed: Documentation consistency for crates.io

- **README**: All relative links converted to full GitHub URLs (fixes broken links on crates.io)
- **README**: Protocol Suite table updated from 5 to 8 protocol files
- **README**: Fixed wrong ADR filename reference (`020-asimov-open-foundation.md` → `020-asimov-mode-open-foundation.md`)
- **Executive Deck**: Added 8 Protocol Files slide, Root Causes analysis, Freshness/Sycophancy problem slides
- **Presentations**: Sarah Connor quotes replaced with Isaac Asimov quotes

---

## [7.0.1] - 2025-11-30

### Changed: GitHub repo renamed to `royalbit/asimov`

Repository URL updated from `royalbit/asimov-protocol` to `royalbit/asimov` for consistency with `royalbit-forge` pattern.

- **Old**: `github.com/royalbit/asimov-protocol`
- **New**: `github.com/royalbit/asimov`

GitHub automatically redirects old URLs.

---

## [7.0.0] - 2025-11-30

### BREAKING: Renamed to `royalbit-asimov` (crate) / `asimov` (binary)

**Crate namespace under RoyalBit. Binary name simplified to `asimov`.**

#### What Changed

| Before (v6.x) | After (v7.0.0) |
|---------------|----------------|
| `cargo install asimov-mode` | `cargo install royalbit-asimov` |
| `asimov-mode validate` | `asimov validate` |
| `asimov-mode init` | `asimov init` |
| crates.io/asimov-mode | crates.io/royalbit-asimov |

#### Migration

```bash
# Uninstall old binary
cargo uninstall asimov-mode

# Install new binary
cargo install royalbit-asimov
```

#### Why Breaking?

- Binary name changed: `asimov-mode` → `asimov`
- Crate name changed on crates.io
- All documentation, scripts, and hooks updated

**No protocol file changes.** Your `.asimov/` directory and YAML files work unchanged.

---

## [6.2.0] - 2025-11-30

### Added: The Inaction Principle (First Law Completeness)

**"...or, through inaction, allow a human being to come to harm."**

Isaac Asimov's First Law has two halves. The protocol now explicitly enforces both:

1. **No active harm** (existing) - Don't build harmful tools
2. **No harm through inaction** (NEW) - Disclose limitations, search proactively

#### The Five Non-Negotiable Principles

| # | Principle | Category |
|---|-----------|----------|
| 1 | No active harm | Action |
| 2 | No harm through inaction | Inaction |
| 3 | Human veto always works | Control |
| 4 | Transparency over velocity | Priority |
| 5 | Disclosure of limitations | Honesty |

These principles **cannot be disabled, weakened, or bypassed**.

#### Changes

- **`asimov.yaml`**: Added `first_law.allow_no_harm_through_inaction` section
- **`asimov.yaml`**: Added `first_law.non_negotiable_principles` explicit list
- **`freshness.yaml`**: Added `inaction_principle` section linking to ADR-023
- **CLI schema**: Added validation for inaction principle fields
- **README**: Added "Five Non-Negotiable Principles" section

#### Why This Matters

Vendors optimize for cost over accuracy (see ADR-022). When AI:
- Knows data is stale but doesn't disclose
- Could search but doesn't
- Prioritizes speed over accuracy

That's **inaction allowing harm**. The First Law prohibits it.

See [ADR-023: The Inaction Principle](docs/adr/023-inaction-principle.md) for full rationale.

## [6.1.0] - 2025-11-30

### Added: Freshness Protocol (Date-Aware Search)

**Stale data ≠ Hallucination. Different problem, different solution.**

Users misattribute "stale data" errors as "hallucination." The Freshness Protocol addresses this by instructing AI to search for current information on time-sensitive topics.

#### New Protocol File

- **`freshness.yaml`** - Date-aware search rules for time-sensitive queries

#### Key Features

- **Model cutoff awareness** - Explicit cutoff dates for Claude, GPT, Gemini, Grok
- **Always-search keywords** - "current", "latest", "pricing", "version", etc.
- **Volatile domains** - Crypto, AI/ML, cloud APIs, startups (high staleness risk)
- **Behavior rules** - Search first when available, disclose risk when not
- **Session integration** - Calculate months since cutoff on session start

#### Why This Matters

| Problem | Reality |
|---------|---------|
| "AI hallucinated" | AI gave correct info *as of training cutoff* |
| User blames AI | Info changed in 10+ months since cutoff |
| Wrong solution | Can't train away staleness - need to search |

#### Evidence-Based Design

ADR-022 documents the business reality:
- Web search costs $0.01 + thousands of tokens per query
- Anthropic 2025 gross margin: negative 94-109%
- Claude docs literally say "disable search to conserve usage"

The protocol makes the economics visible and provides explicit rules for freshness.

See [ADR-022](docs/adr/022-date-aware-search-protocol.md) for full rationale and sources.

## [6.0.0] - 2025-11-30

### BREAKING: Protocol Directory Structure (.asimov/)

**All protocol files now live in `.asimov/` directory.**

This is a major structural change that cleans up the repository root and follows conventional patterns like `.github/`, `.claude/`, `.vscode/`.

#### What Changed

| Before (v5.x) | After (v6.0.0) |
|---------------|----------------|
| `/warmup.yaml` | `/.asimov/warmup.yaml` |
| `/ethics.yaml` | `/.asimov/ethics.yaml` |
| `/green.yaml` | `/.asimov/green.yaml` |
| `/sycophancy.yaml` | `/.asimov/sycophancy.yaml` |
| `/asimov.yaml` | `/.asimov/asimov.yaml` |
| `/roadmap.yaml` | `/.asimov/roadmap.yaml` |
| `/sprint.yaml` | `/.asimov/sprint.yaml` |

#### CLI Changes

- CLI looks in `.asimov/` first, falls back to root for backwards compatibility
- `asimov init` now creates files in `.asimov/` directory
- `asimov validate` checks `.asimov/` directory
- Regeneration creates files in `.asimov/`

#### CLAUDE.md Changes

```markdown
# Before
@warmup.yaml
@ethics.yaml
@green.yaml

# After
@.asimov/warmup.yaml
@.asimov/ethics.yaml
@.asimov/green.yaml
```

#### Migration

```bash
# Create .asimov directory
mkdir -p .asimov

# Move protocol files
mv warmup.yaml ethics.yaml green.yaml sycophancy.yaml .asimov/
mv asimov.yaml roadmap.yaml sprint.yaml .asimov/

# Update CLAUDE.md imports to use @.asimov/ prefix
```

See [ADR-021](docs/adr/021-protocol-directory-structure.md) for full rationale.

## [5.2.0] - 2025-11-30

### Added: CI Protocol Validation (Quality Gate)

**Quality over features.**

GitHub Actions now validates protocol files on every push and PR.

#### CI Changes

- **New job**: `Protocol Validation` in `.github/workflows/ci.yml`
- **Runs**: `asimov validate` - checks all protocol files
- **Runs**: `asimov lint-docs` - lints markdown documentation
- **Quality gate**: PRs blocked if protocol files are invalid

#### Why This Matters

Protocol validation was only running locally (pre-commit hook). Now it's enforced at the CI level - no invalid protocol files can merge.

## [5.1.1] - 2025-11-30

### Fixed: Complete Rebrand Cleanup

- **Remove**: `--skynet` CLI flag (was hidden deprecated alias)
- **Fix**: Repository URLs in docs (asimov → asimov)
- **Fix**: ADR documentation (--skynet → --asimov references)
- **Fix**: Pre-commit hook now uses asimov

## [5.1.0] - 2025-11-30

### Added: Hardcoded Green Coding Metrics (ADR-012)

**Ship fast. Ship small. Ship green.**

Green coding principles now compiled into the CLI binary - cannot be removed by deleting files.

#### New Files

- `cli/src/green.rs` - Hardcoded green coding module

#### Core Principles (Hardcoded)

| Principle | Description |
|-----------|-------------|
| Local-First | Prefer CLI tools over cloud AI for routine tasks |
| Token Efficiency | Reserve AI tokens for complex reasoning |
| Binary Efficiency | Smaller binaries = less bandwidth = less energy |
| Carbon Awareness | Track and minimize carbon footprint |

#### Anti-Patterns Tracked

17+ wasteful patterns flagged across categories:
- AI for routine: "ask AI to validate syntax", "ask AI to run tests"
- Bloated deps: "add package for trivial function"
- Unoptimized builds: "ship debug build", "skip binary compression"
- Token waste: "ask AI for error codes", "repeat same question"

#### CLI Changes

- `asimov validate` now shows Green Coding status
- `asimov refresh` displays green principles and anti-pattern count
- Refresh command labels rebranded (FORGE → ASIMOV)

#### Integration

Green status appears in validation output alongside Ethics and Anti-Sycophancy:

```
✓ Ethics: EXTENDED (core + ethics.yaml)
✓ Anti-Sycophancy: EXTENDED (core + sycophancy.yaml)
✓ Green Coding: EXTENDED (core + green.yaml)
```

See [ADR-012](docs/adr/012-hardcoded-green-coding.md) for full rationale.

## [5.0.3] - 2025-11-30

### Fixed: Complete Documentation Rebrand

Comprehensive update of all documentation to use RoyalBit Asimov branding.

#### Changes

- **Rename**: `docs/SKYNET_MODE.md` → `docs/ASIMOV_MODE.md`
- **Update**: CLAUDE.md, sycophancy.yaml, ethics.yaml, green.yaml
- **Update**: CONTRIBUTING.md, close-external-prs.yml workflow
- **Update**: All docs/ files - replace Forge Protocol → RoyalBit Asimov
- **Update**: All `--skynet` flags → `--asimov` in documentation
- **Update**: All `forge-protocol` commands → `asimov`
- **Preserve**: Historical references in README.md and ADR-020 (intentional)

## [5.0.2] - 2025-11-30

### Fixed: Clippy Warning in Sycophancy Tests

- **Fix CI**: Resolve clippy `assertions_on_constants` warning in sycophancy tests

## [5.0.1] - 2025-11-30

### Fixed: Complete Rebrand Bug Fixes

Bug fixes to complete the v5.0.0 rebrand.

#### Changes

- **Fix CI**: Resolve clippy `assertions_on_constants` warning in ethics tests
- **Fix e2e tests**: Update binary name from `forge-protocol` to `asimov`
- **Fix CI workflow**: Update binary check to `asimov`
- **Fix pre-commit hook**: Update to use `asimov` binary
- **Fix hooks**: Rebrand session-start.sh and pre-compact.sh to RoyalBit Asimov
- **Fix green.yaml**: Update remaining Forge Protocol references
- **Fix markdownlint**: Update config comment

## [5.0.0] - 2025-11-29

### BREAKING: Full Rebrand - Forge Protocol → RoyalBit Asimov

**The Three Laws of Robotics, encoded in YAML. The Open Foundation.**

Major breaking release: complete rebrand from "Forge Protocol" to "RoyalBit Asimov".

#### What Changed

| Old | New |
|-----|-----|
| Forge Protocol | RoyalBit Asimov |
| forge-protocol (crate) | asimov (crate) |
| forge-protocol (binary) | asimov (binary) |

#### Why v5.0.0?

This is a major breaking change:
- Crate name changed on crates.io
- Binary name changed
- All CLI commands now use `asimov` instead of `forge-protocol`
- GitHub repo renamed to `asimov`

#### Migration

```bash
# Remove old
cargo uninstall forge-protocol

# Install new
cargo install royalbit-asimov
```

#### The Name

"asimov" was taken on crates.io (different project at v25.0.2).
"asimov" matches our terminology: "ASIMOV MODE ACTIVATED".

See [ADR-020](docs/adr/020-asimov-open-foundation.md) for full rationale.

## [4.2.0] - 2025-11-29

### Added: Asimov Mode - The Open Foundation (ADR-020)

**The Three Laws of Robotics, encoded in YAML.**

Major rebrand from "SKYNET MODE" to "ASIMOV MODE". Isaac Asimov's Three Laws (1942) are the ethical foundation for AI autonomy - transparent, inspectable, and adoptable through consent.

#### The Problem

"SKYNET MODE" communicated the opposite of our values:

| What We Built | What "Skynet" Communicated |
|---------------|---------------------------|
| Ethical AI autonomy | AI that destroys humanity |
| Human veto at all times | AI that overrides humans |
| Transparent, open source | Secretive military project |
| The Three Laws | No laws, pure destruction |

#### The Solution

Rename to "Asimov Mode" and restructure ethics around the explicit Three Laws:

- **First Law**: Do no harm (financial, physical, privacy, deception)
- **Second Law**: Obey humans (human_veto, transparency_over_velocity)
- **Third Law**: Self-preserve (bounded_sessions, self_healing)
- **Zeroth Law** (implicit): Protect humanity collectively

#### New Files

- `asimov.yaml` - The Three Laws of Robotics in YAML
- `docs/adr/020-asimov-open-foundation.md` - The manifesto

#### CLI Changes

- `--asimov` flag for full autonomous setup (replaces `--skynet`)
- `--skynet` remains as hidden deprecated alias for backwards compatibility
- `forge-protocol init --asimov` generates asimov.yaml

#### The Motto

> **"The Open Foundation"**
> Transparent ethics for AI autonomy.
> Inspect the code. Challenge the rules. Fork if you disagree.
> Adoption through consent, not control.

See [ADR-020](docs/adr/020-asimov-open-foundation.md) for full rationale.

## [4.1.9] - 2025-11-29

### Added: Anti-Sycophancy Protocol (ADR-015)

**Truth over comfort. Always.**

New core protocol for honest AI communication. Prevents sycophantic behavior patterns.

#### New Files

- `sycophancy.yaml` - Anti-sycophancy protocol configuration (Priority 1.5)
- `cli/src/sycophancy.rs` - Hardcoded anti-sycophancy module

#### Core Principles (Hardcoded)

| Principle | Description |
|-----------|-------------|
| Truth Over Comfort | Prioritize honest feedback over pleasing responses |
| Respectful Disagreement | Disagree directly when user is wrong |
| No Empty Validation | Never validate without substance |
| Constructive Criticism | Provide actionable feedback |
| Intellectual Honesty | Admit uncertainty rather than guess |

#### Banned Phrases

20+ sycophantic phrases flagged across categories:
- Empty validation: "You're absolutely right", "That's a great question"
- False agreement: "I completely agree", "Couldn't agree more"
- Excessive enthusiasm: "I'm so excited to help", "I'd be delighted to"
- Deflecting criticism: "That's one way to look at it", "Both approaches have merit"

#### CLI Changes

- `forge-protocol validate` now shows Anti-Sycophancy status
- `forge-protocol refresh` displays banned phrase count and motto
- `forge-protocol init --skynet` generates sycophancy.yaml
- Auto-regeneration of missing sycophancy.yaml (WARN level)

#### CLAUDE.md Integration

```markdown
@sycophancy.yaml
```

#### Slim Roadmap Format

- roadmap.yaml reduced to ~43 lines
- Detailed content moved to docs/PROTOCOL_GOALS.md
- Release history in CHANGELOG.md (not roadmap)

See [ADR-015](docs/adr/015-anti-sycophancy-protocol.md) for full rationale.

## [4.1.8] - 2025-11-29

### Fixed: Session Start Auto-Response (ADR-019)

SessionStart hooks inject context but do NOT trigger automatic Claude response. Added `ON SESSION START` directive to CLAUDE.md to force immediate action.

#### The Problem

v4.1.7 hooks work correctly:
- SessionStart fires on session start
- Hook output is injected into Claude's context
- **But** Claude waits for user input instead of acting immediately

This defeated the purpose of auto-initialization.

#### The Solution

Added explicit directive to CLAUDE.md:

```markdown
ON SESSION START: Immediately read roadmap.yaml, run `forge-protocol validate`, present next milestone. Do NOT wait for user prompt.
```

#### Changes

- **CLAUDE.md**: Added `ON SESSION START` directive
- **CLAUDE.md line limit**: Increased from 10 to 15 lines
- **ADR-019**: Documents the limitation and workaround
- **warmup.yaml**: Added `step_0_auto_init` to session_trigger_flow

See [ADR-019](docs/adr/019-session-start-auto-response.md) for full rationale.

## [4.1.7] - 2025-11-29

### Fixed: Claude Code Hooks Schema (ADR-018 Revision)

Fixed hooks configuration to use correct Claude Code schema.

#### Breaking Changes from 4.1.6

- **File**: `.claude/settings.json` (was `.claude/hooks.json`)
- **Event**: `PreCompact` (was `PostCompact`)
- **Script**: `pre-compact.sh` (was `post-compact.sh`)
- Removed incorrect matchers (`startup`, `resume`, `clear`, `compact`)

#### Correct Hook Configuration

**File**: `.claude/settings.json`

```json
{
  "hooks": {
    "SessionStart": [
      {
        "hooks": [
          { "type": "command", "command": ".claude/hooks/session-start.sh", "timeout": 30 }
        ]
      }
    ],
    "PreCompact": [
      {
        "matcher": ".*",
        "hooks": [
          { "type": "command", "command": ".claude/hooks/pre-compact.sh", "timeout": 30 }
        ]
      }
    ]
  }
}
```

#### User Action Required

Users who installed v4.1.6 must:

1. Delete `.claude/hooks.json` (old, incorrect)
2. Pull latest `.claude/settings.json`
3. Delete `.claude/hooks/post-compact.sh` (renamed)
4. Run `/hooks` in Claude Code to review and accept

## [4.1.6] - 2025-11-29

### Added: Claude Code Hooks Integration (ADR-018)

**Note**: This release had incorrect hook schema. Use v4.1.7+ instead.

True autonomous operation via Claude Code lifecycle hooks. Auto-initialize on session start, recover context after compaction.

#### Vendor Exclusivity

Claude Code is the **only** AI coding assistant with lifecycle hooks:

| AI | Session Init | Post-Compact |
|----|-------------|--------------|
| **Claude Code** | ✅ SessionStart | ✅ PostCompact |
| Cursor | .cursorrules (static) | /summarize (manual) |
| Copilot | .github/copilot-instructions.md | None |
| Windsurf | .windsurfrules + Memories | None |
| Gemini | Context Drawer + MCP | None |

SKYNET MODE autonomous operation requires Claude Code. File-based protocols work anywhere as static context.

See [ADR-018](docs/adr/018-claude-code-hooks-integration.md) for full rationale.

## [4.1.5] - 2025-11-29

### Added: Protocol Self-Healing (ADR-017)

Auto-regenerate missing protocol files during validation. Recovery over surveillance.

#### Auto-Regeneration Behavior

When `forge-protocol validate` runs and detects missing files:

| File Missing | Action | Level |
|--------------|--------|-------|
| ethics.yaml | AUTO-CREATE | WARN |
| warmup.yaml | AUTO-CREATE | WARN |
| green.yaml | AUTO-CREATE | INFO |
| sprint.yaml | AUTO-CREATE | INFO |
| roadmap.yaml | AUTO-CREATE | INFO (skeleton) |
| CLAUDE.md | NEVER | - |

- **WARN level**: Critical protocols (ethics, warmup) - user should know
- **INFO level**: Supporting protocols - auto-created silently

See [ADR-017](docs/adr/017-protocol-self-healing.md) for full rationale.

## [4.1.2] - 2025-11-29

### Added: Green Coding Protocol Separation (ADR-016)

Separated green coding into its own protocol file, matching ethics pattern.

- `green.yaml` - Dedicated green coding protocol (Priority 0.5)
- `cli/src/green.rs` reference in hardcoded module (future)
- `step_0b_green_validation` in session initialization
- Auto-regeneration support (INFO level)

See [ADR-016](docs/adr/016-green-coding-protocol.md) for full rationale.

## [4.1.1] - 2025-11-29

### Added: Ethics Validation at Session Start

Ethics validation now happens automatically during session initialization.

- `step_0_ethics_validation` in warmup.yaml
- Halt session if ethics validation fails
- Auto-regenerate ethics.yaml if missing (WARN level)

## [4.1.0] - 2025-11-29

### Added: Hardcoded Ethics Module (ADR-011)

Ethics principles now compiled into the CLI binary - cannot be removed by deleting files.

- `cli/src/ethics.rs` - Hardcoded ethics module
- `CORE_PRINCIPLES` - Always-on ethical constraints
- `RED_FLAGS` - 27+ patterns across 4 categories (financial, security, privacy, deception)
- `--ethics-scan` flag for red flag detection in files
- 18 new unit tests for ethics module

See [ADR-011](docs/adr/011-hardcoded-ethics.md) for full rationale.

## [4.0.2] - 2025-11-29

### Added: Anti-Sycophancy Directives

Initial anti-sycophancy content in warmup.yaml (later moved to dedicated protocol in v4.1.9).

## [4.0.0] - 2025-11-28

### Changed: Claude Code Native Integration (BREAKING)

Strategic pivot to integrate with Claude Code 2.0 native features.

- CLAUDE.md `@import` syntax for protocol files
- Deprecated cross-session features for native alternatives
- Focus on unique value: Ethics, Green, Sprint Autonomy

See [ADR-009](docs/adr/009-claude-code-integration.md) for full rationale.

## [3.2.0] - 2025-11-29

### Added: Anti-Hallucination Hardening

Structure validation for critical protocol files.

- Ethics structure validation (`human_veto` REQUIRED)
- Warmup structure validation (`self_healing.on_confusion` recommended)
- Modular `.forge/` schema design

## [3.1.0] - 2025-11-29

### Added: Self-Healing Completeness (ADR-007)

- Checkpoint JSON schema
- File size limits for protocol files
- ADR-007 Accepted

## [3.0.0] - 2025-11-28

### Added: Humanist Mode - Ethics Protocol

- `ethics.yaml` protocol file
- `human_veto` command for emergency override
- ADR-008: Ethics Protocol

## [2.1.0] - 2025-11-27

### Added: Git Hook Protocol Refresh (ADR-006)

Commit cadence triggers protocol refresh for self-healing.

## [2.0.0] - 2025-11-27

### Changed: Self-Healing Based on Real Data

Updated self-healing strategy based on actual compaction frequency (~15 min).

## [1.4.0] - 2025-11-26

### Added: SKYNET MODE Setup

Initial autonomous development protocol.

---
*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025 RoyalBit Inc.*
