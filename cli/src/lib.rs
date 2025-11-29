//! Forge Protocol CLI - Validator for vendor-neutral AI session continuity
//!
//! This crate provides validation for Forge Protocol files:
//! - `warmup.yaml` - Session bootstrap
//! - `sprint.yaml` - Active work tracking
//! - `roadmap.yaml` - Milestone planning
//! - `.claude_checkpoint.yaml` - Session state for self-healing
//!
//! # Ethics (Hardcoded)
//!
//! Core ethics are hardcoded into the binary and cannot be removed by deleting files.
//! See `ethics` module for details.
//!
//! # Example
//!
//! ```no_run
//! use forge_protocol::{validate_file, validate_directory};
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

pub mod error;
pub mod ethics;
pub mod markdown;
pub mod schemas;
pub mod templates;
pub mod validator;

// Re-export main types
pub use error::{Error, Result};
pub use ethics::{
    check_ethics_status, red_flags, scan_directory_for_red_flags, scan_file_for_red_flags,
    CorePrinciples, EthicsStatus, RedFlagCategory, RedFlagMatch, CORE_PRINCIPLES,
    HUMAN_VETO_COMMANDS,
};
pub use markdown::{
    check_file as check_markdown_file, find_markdown_files, fix_file as fix_markdown_file,
    LintError, LintResult,
};
pub use templates::{
    checkpoint_template, claude_md_template, ethics_template, hook_installer_template,
    precommit_hook_template, roadmap_template, sprint_template, uses_cargo_husky, warmup_template,
    ProjectType,
};
pub use validator::{
    check_ethics_structure, check_warmup_structure, is_protocol_file, validate_claude_md,
    validate_directory, validate_file, FileSizeLimits, ValidationResult,
};
