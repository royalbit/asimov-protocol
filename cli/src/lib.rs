//! RoyalBit Asimov CLI - Creates Self-Evolving Autonomous AI projects with ethics built in
//!
//! This crate provides validation for RoyalBit Asimov files:
//! - `roadmap.yaml` - Milestone planning (WHAT to build)
//! - `project.yaml` - Project context (HOW to build) - ADR-032
//!
//! # Architecture (v8.1.0 - ADR-032)
//!
//! - **Layer 1**: Behavior protocols are hardcoded in the binary (asimov, green, sycophancy, etc.)
//! - **Layer 2**: Project data files in `.asimov/` (roadmap.yaml, project.yaml)
//!
//! # Ethics (Hardcoded)
//!
//! Core ethics are hardcoded into the binary and cannot be removed by deleting files.
//! See `ethics` module for details.
//!
//! # Example
//!
//! ```no_run
//! use royalbit_asimov::{validate_file, validate_directory};
//! use std::path::Path;
//!
//! // Validate a single file
//! let result = validate_file(Path::new("warmup.yaml")).unwrap();
//! println!("Valid: {}", result.is_valid);
//!
//! // Validate all protocol files in a directory
//! let results = validate_directory(Path::new(".")).unwrap();
//! for result in results {
//!     println!("{}: {}", result.file, if result.is_valid { "OK" } else { "FAIL" });
//! }
//! ```

pub mod commands;
pub mod error;
pub mod ethics;
pub mod green;
pub mod markdown;
pub mod protocols;
pub mod schemas;
pub mod semantic;
pub mod sycophancy;
pub mod templates;
pub mod update;
pub mod validator;

// Re-export main types
pub use error::{Error, Result};
pub use ethics::{
    check_ethics_status, red_flags, scan_directory_for_red_flags, scan_file_for_red_flags,
    CorePrinciples, EthicsStatus, RedFlagCategory, RedFlagMatch, CORE_PRINCIPLES,
    HUMAN_VETO_COMMANDS,
};
pub use green::{
    anti_patterns, best_practices, carbon, check_green_status, cost, AntiPatternCategory,
    GreenPrinciples, GreenStatus, GREEN_PRINCIPLES, MOTTO as GREEN_MOTTO,
};
pub use markdown::{
    check_file as check_markdown_file, find_markdown_files, fix_file as fix_markdown_file,
    LintError, LintResult,
};
pub use semantic::{
    check_semantic, get_cargo_version, load_deprecated_patterns, DeprecatedPattern, IssueCategory,
    SemanticConfig, SemanticIssue, SemanticResult, Severity,
};
pub use sycophancy::{
    banned_phrases, check_sycophancy_status, directives, BannedPhraseCategory,
    CorePrinciples as SycophancyPrinciples, SycophancyStatus,
    CORE_PRINCIPLES as SYCOPHANCY_PRINCIPLES, MOTTO as SYCOPHANCY_MOTTO,
};
#[allow(deprecated)]
pub use templates::{
    asimov_template,
    // v8.0.0: Hardcoded hook templates
    claude_pre_compact_hook,
    claude_session_start_hook,
    claude_settings_json,
    // v8.1.0: Project type detection (ADR-032)
    detect_project_type,
    ethics_template,
    git_precommit_hook,
    green_template,
    hook_installer_template,
    precommit_hook_template,
    // v8.1.0: Project context file (ADR-032)
    project_template,
    roadmap_template,
    sprint_template,
    sycophancy_template,
    uses_cargo_husky,
    warmup_template,
    ProjectType,
};
pub use validator::{
    check_asimov_structure, check_protocol_integrity, check_warmup_structure,
    delete_deprecated_claude_md, ensure_protocol_dir, is_protocol_file, regenerate_protocol_files,
    resolve_protocol_dir, validate_directory, validate_directory_with_options,
    validate_directory_with_regeneration, validate_file, FileSizeLimits, ProtocolCheck,
    RegenerationInfo, ValidationResult, PROTOCOL_DIR,
};

// Schema exports for editor integration (v7.2.0)
// NOTE: PROJECT_SCHEMA added in v8.1.0 (ADR-032)
pub use schemas::{
    ASIMOV_SCHEMA, FRESHNESS_SCHEMA, GREEN_SCHEMA, MIGRATIONS_SCHEMA, PROJECT_SCHEMA,
    ROADMAP_SCHEMA, SPRINT_SCHEMA, SYCOPHANCY_SCHEMA, WARMUP_SCHEMA,
};

// Update exports for self-update functionality (v7.8.0)
pub use update::{check_for_update, perform_update, VersionCheck, CURRENT_VERSION};

// Protocol exports for enforced loading (v8.0.0 - ADR-031)
// v8.14.0: Added individual protocol JSON exports
pub use protocols::{
    // v8.14.0: Individual protocol JSON files
    asimov_json,
    compile_protocols,
    exhaustive_json,
    freshness_json,
    green_json,
    inject_dates,
    migrations_json,
    sprint_json,
    sycophancy_json,
    to_minified_json,
    to_pretty_json,
    warmup_entry_json,
    CompiledProtocols,
    WarmupEntry,
    PROTOCOL_FILES,
};
