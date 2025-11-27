//! Forge Protocol CLI - Validator for vendor-neutral AI session continuity
//!
//! This crate provides validation for Forge Protocol files:
//! - `warmup.yaml` - Session bootstrap
//! - `sprint.yaml` - Active work tracking
//! - `roadmap.yaml` - Milestone planning
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
pub mod markdown;
pub mod schemas;
pub mod templates;
pub mod validator;

// Re-export main types
pub use error::{Error, Result};
pub use markdown::{
    check_file as check_markdown_file, find_markdown_files, fix_file as fix_markdown_file,
    LintError, LintResult,
};
pub use templates::{roadmap_template, sprint_template, warmup_template, ProjectType};
pub use validator::{is_protocol_file, validate_directory, validate_file, ValidationResult};
