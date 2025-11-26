//! Core validation logic for Forge Protocol files

use crate::error::{Error, Result};
use crate::schemas::{schema_for_file, schema_type_for_file};
use jsonschema::Validator;
use std::path::Path;

/// Validation result for a single file
#[derive(Debug)]
pub struct ValidationResult {
    pub file: String,
    pub schema_type: String,
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn success(file: String, schema_type: String) -> Self {
        Self {
            file,
            schema_type,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn failure(file: String, schema_type: String, errors: Vec<String>) -> Self {
        Self {
            file,
            schema_type,
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }
}

/// Validate a single protocol file
pub fn validate_file(path: &Path) -> Result<ValidationResult> {
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Check file exists
    if !path.exists() {
        return Err(Error::FileNotFound(path.display().to_string()));
    }

    // Determine schema type
    let schema_json =
        schema_for_file(filename).ok_or_else(|| Error::UnknownFileType(filename.to_string()))?;
    let schema_type = schema_type_for_file(filename).unwrap_or("unknown");

    // Read and parse YAML
    let content = std::fs::read_to_string(path)?;
    let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)?;

    // Convert YAML to JSON for schema validation
    let json_value = yaml_to_json(&yaml_value)?;

    // Parse schema
    let schema: serde_json::Value = serde_json::from_str(schema_json)
        .map_err(|e| Error::SchemaError(format!("Invalid schema: {}", e)))?;

    // Compile and validate
    let validator = Validator::new(&schema)
        .map_err(|e| Error::SchemaError(format!("Failed to compile schema: {}", e)))?;

    // Collect all validation errors
    let error_messages: Vec<String> = validator
        .iter_errors(&json_value)
        .map(|e| {
            let path = e.instance_path.to_string();
            if path.is_empty() {
                e.to_string()
            } else {
                format!("{}: {}", path, e)
            }
        })
        .collect();

    if error_messages.is_empty() {
        Ok(ValidationResult::success(
            path.display().to_string(),
            schema_type.to_string(),
        ))
    } else {
        Ok(ValidationResult::failure(
            path.display().to_string(),
            schema_type.to_string(),
            error_messages,
        ))
    }
}

/// Validate all protocol files in a directory
pub fn validate_directory(dir: &Path) -> Result<Vec<ValidationResult>> {
    let mut results = Vec::new();

    // Look for protocol files
    let protocol_files = ["warmup.yaml", "sprint.yaml", "roadmap.yaml"];

    for filename in &protocol_files {
        let file_path = dir.join(filename);
        if file_path.exists() {
            results.push(validate_file(&file_path)?);
        }
    }

    if results.is_empty() {
        return Err(Error::ValidationError(
            "No protocol files found (warmup.yaml, sprint.yaml, roadmap.yaml)".to_string(),
        ));
    }

    Ok(results)
}

/// Convert serde_yaml::Value to serde_json::Value
fn yaml_to_json(yaml: &serde_yaml::Value) -> Result<serde_json::Value> {
    // Serialize to string and back to handle type conversions
    let json_str = serde_json::to_string(&yaml)
        .map_err(|e| Error::ValidationError(format!("Failed to convert YAML to JSON: {}", e)))?;
    serde_json::from_str(&json_str)
        .map_err(|e| Error::ValidationError(format!("Failed to parse JSON: {}", e)))
}

/// Check if a file is a valid protocol file by name
pub fn is_protocol_file(filename: &str) -> bool {
    let name = filename.to_lowercase();
    name.contains("warmup") || name.contains("sprint") || name.contains("roadmap")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_valid_warmup() {
        let content = r#"
identity:
  project: "Test Project"
  tagline: "A test"
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
    }

    #[test]
    fn test_invalid_warmup_missing_identity() {
        let content = r#"
mission:
  problem: "No identity"
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_valid_sprint() {
        let content = r#"
sprint:
  current: "Feature work"
  status: in_progress
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
    }

    #[test]
    fn test_is_protocol_file() {
        assert!(is_protocol_file("warmup.yaml"));
        assert!(is_protocol_file("sprint.yaml"));
        assert!(is_protocol_file("roadmap.yaml"));
        assert!(is_protocol_file("my_warmup.yaml"));
        assert!(!is_protocol_file("config.yaml"));
        assert!(!is_protocol_file("random.txt"));
    }
}
