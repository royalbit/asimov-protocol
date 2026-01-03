//! Error types for the RoyalBit Asimov validator

use thiserror::Error;

/// Result type for RoyalBit Asimov operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during validation
#[derive(Error, Debug)]
pub enum Error {
    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Failed to read file: {0}")]
    ReadError(#[from] std::io::Error),

    #[error("Invalid YAML: {0}")]
    YamlError(#[from] serde_yaml_ng::Error),

    #[error("Schema validation failed: {0}")]
    SchemaError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unknown file type: {0}. Expected warmup.yaml, sprint.yaml, or roadmap.yaml")]
    UnknownFileType(String),
}
