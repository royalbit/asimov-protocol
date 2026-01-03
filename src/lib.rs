#![cfg_attr(feature = "coverage", feature(coverage_attribute))]
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
    // v10.6.0: Removed claude_* hooks (ADR-060)
    // v8.1.0: Project type detection (ADR-032)
    detect_project_type,
    ethics_template,
    get_enterprise_template,
    // v10.3.1: Unified template lookup (ADR-057)
    get_template_by_name,
    git_precommit_hook,
    green_template,
    hook_installer_template,
    // v10.3.0: List all templates (ADR-057)
    list_templates,
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
// v9.2.3: Added conditional migrations support
// v9.14.0: Merged exhaustive into sprint (ADR-049)
// v9.18.0: Protocol layer refactoring
// v10.8.0: Migrations removed (ADR-062) - now part of API templates
pub use protocols::{
    // v8.14.0: Individual protocol JSON files
    asimov_json,
    compile_protocols,
    // v10.8.0: compile_protocols_for_type removed (ADR-062)
    // v10.8.0: compile_protocols_with_options removed (ADR-062)
    freshness_json,
    green_json,
    inject_dates,
    // v10.8.0: migrations_json removed (ADR-062)
    sprint_json,
    sycophancy_json,
    to_minified_json,
    // v10.8.0: to_minified_json_for_type removed (ADR-062)
    to_pretty_json,
    warmup_entry_json,
    CompiledProtocols,
    WarmupEntry,
    PROTOCOL_FILES,
};
