//! JSON Schema definitions for Forge Protocol files
//!
//! Schemas are embedded as string constants and compiled at runtime.

mod roadmap;
mod sprint;
mod warmup;

pub use roadmap::ROADMAP_SCHEMA;
pub use sprint::SPRINT_SCHEMA;
pub use warmup::WARMUP_SCHEMA;

/// Determine which schema to use based on filename
pub fn schema_for_file(filename: &str) -> Option<&'static str> {
    let name = filename.to_lowercase();
    if name.contains("warmup") {
        Some(WARMUP_SCHEMA)
    } else if name.contains("sprint") {
        Some(SPRINT_SCHEMA)
    } else if name.contains("roadmap") {
        Some(ROADMAP_SCHEMA)
    } else {
        None
    }
}

/// Get the schema type name for display
pub fn schema_type_for_file(filename: &str) -> Option<&'static str> {
    let name = filename.to_lowercase();
    if name.contains("warmup") {
        Some("warmup")
    } else if name.contains("sprint") {
        Some("sprint")
    } else if name.contains("roadmap") {
        Some("roadmap")
    } else {
        None
    }
}
