//! JSON Schema definitions for RoyalBit Asimov files
//!
//! Schemas are embedded as string constants and compiled at runtime.

mod asimov;
mod freshness;
mod green;
mod migrations;
mod project;
mod roadmap;
mod sprint;
mod sycophancy;
mod warmup;

// NOTE: ethics.rs removed - asimov.yaml is the canonical ethics source (ADR-031)
// NOTE: checkpoint.rs removed - deprecated by hooks in ADR-032

pub use asimov::ASIMOV_SCHEMA;
pub use freshness::FRESHNESS_SCHEMA;
pub use green::GREEN_SCHEMA;
pub use migrations::MIGRATIONS_SCHEMA;
pub use project::PROJECT_SCHEMA;
pub use roadmap::ROADMAP_SCHEMA;
pub use sprint::SPRINT_SCHEMA;
pub use sycophancy::SYCOPHANCY_SCHEMA;
pub use warmup::WARMUP_SCHEMA;

/// Determine which schema to use based on filename
/// NOTE: ethics.yaml is no longer supported - use asimov.yaml (ADR-031)
/// NOTE: checkpoint.yaml is deprecated - use hooks instead (ADR-032)
pub fn schema_for_file(filename: &str) -> Option<&'static str> {
    let name = filename.to_lowercase();
    if name.contains("warmup") {
        Some(WARMUP_SCHEMA)
    } else if name.contains("sprint") {
        Some(SPRINT_SCHEMA)
    } else if name.contains("roadmap") {
        Some(ROADMAP_SCHEMA)
    } else if name.contains("asimov") {
        Some(ASIMOV_SCHEMA)
    } else if name.contains("freshness") {
        Some(FRESHNESS_SCHEMA)
    } else if name.contains("migrations") {
        Some(MIGRATIONS_SCHEMA)
    } else if name.contains("green") {
        Some(GREEN_SCHEMA)
    } else if name.contains("sycophancy") {
        Some(SYCOPHANCY_SCHEMA)
    } else if name.contains("project") {
        Some(PROJECT_SCHEMA)
    } else {
        None
    }
}

/// Get the schema type name for display
/// NOTE: ethics.yaml is no longer supported - use asimov.yaml (ADR-031)
/// NOTE: checkpoint.yaml is deprecated - use hooks instead (ADR-032)
pub fn schema_type_for_file(filename: &str) -> Option<&'static str> {
    let name = filename.to_lowercase();
    if name.contains("warmup") {
        Some("warmup")
    } else if name.contains("sprint") {
        Some("sprint")
    } else if name.contains("roadmap") {
        Some("roadmap")
    } else if name.contains("asimov") {
        Some("asimov")
    } else if name.contains("freshness") {
        Some("freshness")
    } else if name.contains("migrations") {
        Some("migrations")
    } else if name.contains("green") {
        Some("green")
    } else if name.contains("sycophancy") {
        Some("sycophancy")
    } else if name.contains("project") {
        Some("project")
    } else {
        None
    }
}
