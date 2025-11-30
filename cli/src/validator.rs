//! Core validation logic for Asimov Protocol files

use crate::error::{Error, Result};
use crate::schemas::{schema_for_file, schema_type_for_file};
use colored::Colorize;
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
    pub regenerated: bool,
}

impl ValidationResult {
    pub fn success(file: String, schema_type: String) -> Self {
        Self {
            file,
            schema_type,
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            regenerated: false,
        }
    }

    pub fn failure(file: String, schema_type: String, errors: Vec<String>) -> Self {
        Self {
            file,
            schema_type,
            is_valid: false,
            errors,
            warnings: Vec::new(),
            regenerated: false,
        }
    }

    /// Add a warning to the validation result
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add multiple warnings to the validation result
    pub fn with_warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings.extend(warnings);
        self
    }

    /// Mark this result as regenerated
    pub fn with_regenerated(mut self) -> Self {
        self.regenerated = true;
        self
    }
}

/// File size limits for self-healing protocol (ADR-007)
pub struct FileSizeLimits {
    /// Soft limit (triggers warning)
    pub soft_lines: usize,
    /// Hard limit (triggers error)
    pub hard_lines: usize,
}

impl FileSizeLimits {
    pub const CHECKPOINT: FileSizeLimits = FileSizeLimits {
        soft_lines: 20,
        hard_lines: 30,
    };

    pub const CLAUDE_MD: FileSizeLimits = FileSizeLimits {
        soft_lines: 10,
        hard_lines: 15,
    };

    pub const WARMUP: FileSizeLimits = FileSizeLimits {
        soft_lines: 200,
        hard_lines: 500,
    };
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

    let mut result = if error_messages.is_empty() {
        ValidationResult::success(path.display().to_string(), schema_type.to_string())
    } else {
        ValidationResult::failure(
            path.display().to_string(),
            schema_type.to_string(),
            error_messages,
        )
    };

    // Add size warnings based on file type (ADR-007)
    let line_count = content.lines().count();
    let size_warnings = check_file_size(schema_type, line_count);
    result = result.with_warnings(size_warnings);

    // Structure validation for ethics.yaml (v3.2.0 Anti-Hallucination)
    if schema_type == "ethics" {
        let structure_errors = check_ethics_structure(&content);
        if !structure_errors.is_empty() {
            // Ethics structure errors are CRITICAL - fail validation
            result.is_valid = false;
            result.errors.extend(structure_errors);
        }
    }

    // Structure validation for warmup.yaml (v3.2.0 Anti-Hallucination)
    if schema_type == "warmup" {
        let (structure_errors, structure_warnings) = check_warmup_structure(&content);
        if !structure_errors.is_empty() {
            result.is_valid = false;
            result.errors.extend(structure_errors);
        }
        result = result.with_warnings(structure_warnings);
    }

    Ok(result)
}

/// Check file size against limits and return warnings (ADR-007)
fn check_file_size(schema_type: &str, line_count: usize) -> Vec<String> {
    let mut warnings = Vec::new();

    let limits = match schema_type {
        "checkpoint" => Some(FileSizeLimits::CHECKPOINT),
        "warmup" => Some(FileSizeLimits::WARMUP),
        _ => None,
    };

    if let Some(limits) = limits {
        if line_count > limits.hard_lines {
            warnings.push(format!(
                "File has {} lines, exceeds hard limit of {} lines. Consider trimming.",
                line_count, limits.hard_lines
            ));
        } else if line_count > limits.soft_lines {
            warnings.push(format!(
                "File has {} lines, exceeds recommended {} lines. Consider trimming.",
                line_count, limits.soft_lines
            ));
        }
    }

    warnings
}

/// Regeneration info returned when files are auto-created
#[derive(Debug, Default)]
pub struct RegenerationInfo {
    /// Files that were regenerated (filename, is_warn_level)
    pub regenerated: Vec<(String, bool)>,
}

impl RegenerationInfo {
    pub fn is_empty(&self) -> bool {
        self.regenerated.is_empty()
    }
}

/// The protocol directory name (v6.0.0+)
pub const PROTOCOL_DIR: &str = ".asimov";

/// Resolve the protocol directory for a given base directory.
/// Returns `.asimov/` if it exists, otherwise returns the base directory.
/// This provides backwards compatibility with pre-6.0.0 installations.
pub fn resolve_protocol_dir(base_dir: &Path) -> std::path::PathBuf {
    let asimov_dir = base_dir.join(PROTOCOL_DIR);
    if asimov_dir.exists() && asimov_dir.is_dir() {
        asimov_dir
    } else {
        base_dir.to_path_buf()
    }
}

/// Ensure the .asimov directory exists, creating it if necessary.
/// Returns the path to the .asimov directory.
pub fn ensure_protocol_dir(base_dir: &Path) -> Result<std::path::PathBuf> {
    let asimov_dir = base_dir.join(PROTOCOL_DIR);
    if !asimov_dir.exists() {
        std::fs::create_dir_all(&asimov_dir).map_err(|e| {
            Error::ValidationError(format!("Failed to create .asimov directory: {}", e))
        })?;
    }
    Ok(asimov_dir)
}

/// Validate all protocol files in a directory
pub fn validate_directory(dir: &Path) -> Result<Vec<ValidationResult>> {
    validate_directory_with_options(dir, true)
}

/// Validate all protocol files in a directory with regeneration control
pub fn validate_directory_with_options(
    dir: &Path,
    regenerate: bool,
) -> Result<Vec<ValidationResult>> {
    let (results, _info) = validate_directory_internal(dir, regenerate)?;
    Ok(results)
}

/// Validate all protocol files in a directory, returning regeneration info
pub fn validate_directory_with_regeneration(
    dir: &Path,
    regenerate: bool,
) -> Result<(Vec<ValidationResult>, RegenerationInfo)> {
    validate_directory_internal(dir, regenerate)
}

/// Internal implementation that returns both results and regeneration info
fn validate_directory_internal(
    base_dir: &Path,
    regenerate: bool,
) -> Result<(Vec<ValidationResult>, RegenerationInfo)> {
    use crate::templates::{
        ethics_template, green_template, roadmap_template, sprint_template, sycophancy_template,
        warmup_template, ProjectType,
    };

    // Resolve protocol directory (.asimov/ or root for backwards compatibility)
    let protocol_dir = resolve_protocol_dir(base_dir);

    let mut results = Vec::new();
    let mut regen_info = RegenerationInfo::default();

    // Required protocol files with their templates and warn level
    // (filename, template_fn, is_warn_level)
    #[allow(clippy::type_complexity)]
    let required_files: Vec<(&str, Box<dyn Fn() -> String>, bool)> = vec![
        ("ethics.yaml", Box::new(ethics_template), true), // WARN - Priority 0
        (
            "warmup.yaml",
            Box::new(|| warmup_template("project", ProjectType::Generic)),
            true,
        ), // WARN
        ("green.yaml", Box::new(green_template), false),  // INFO - Priority 0.5
        ("sycophancy.yaml", Box::new(sycophancy_template), true), // WARN - Priority 1.5
        ("sprint.yaml", Box::new(sprint_template), false), // INFO
        ("roadmap.yaml", Box::new(roadmap_template), false), // INFO
    ];

    // Check and regenerate missing required files
    for (filename, template_fn, is_warn) in &required_files {
        let file_path = protocol_dir.join(filename);
        if !file_path.exists() && regenerate {
            // Ensure .asimov directory exists before regenerating
            ensure_protocol_dir(base_dir)?;
            let regen_path = base_dir.join(PROTOCOL_DIR).join(filename);
            // Regenerate the file
            let content = template_fn();
            if let Err(e) = std::fs::write(&regen_path, &content) {
                return Err(Error::ValidationError(format!(
                    "Failed to regenerate {}: {}",
                    filename, e
                )));
            }
            regen_info
                .regenerated
                .push((filename.to_string(), *is_warn));
        }
    }

    // Re-resolve protocol dir in case we just created .asimov/
    let protocol_dir = resolve_protocol_dir(base_dir);

    // Look for protocol files (including optional ones)
    let protocol_files = [
        "warmup.yaml",
        "sprint.yaml",
        "roadmap.yaml",
        "ethics.yaml",
        "green.yaml",
        "sycophancy.yaml",
        ".claude_checkpoint.yaml",
    ];

    for filename in &protocol_files {
        let file_path = protocol_dir.join(filename);
        if file_path.exists() {
            let mut result = validate_file(&file_path)?;
            // Mark as regenerated if it was just created
            if regen_info.regenerated.iter().any(|(f, _)| f == *filename) {
                result = result.with_regenerated();
            }
            results.push(result);
        }
    }

    // CLAUDE.md is deprecated - delete if found
    delete_deprecated_claude_md(base_dir);

    if results.is_empty() {
        return Err(Error::ValidationError(
            "No protocol files found in .asimov/ or root (warmup.yaml, sprint.yaml, roadmap.yaml, ethics.yaml, sycophancy.yaml)"
                .to_string(),
        ));
    }

    Ok((results, regen_info))
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
    let is_yaml = name.ends_with(".yaml") || name.ends_with(".yml");
    is_yaml
        && (name.contains("warmup")
            || name.contains("sprint")
            || name.contains("roadmap")
            || name.contains("ethics")
            || name.contains("green")
            || name.contains("sycophancy")
            || name.contains("checkpoint"))
}

/// Structure validation for ethics.yaml (v3.2.0 Anti-Hallucination)
/// Validates that critical sections like human_veto exist
/// Returns errors (not warnings) for missing required sections
pub fn check_ethics_structure(content: &str) -> Vec<String> {
    let mut errors = Vec::new();

    // Parse YAML to check structure
    let yaml: serde_yaml::Value = match serde_yaml::from_str(content) {
        Ok(v) => v,
        Err(_) => return errors, // YAML parsing errors handled elsewhere
    };

    // human_veto is REQUIRED - Priority 0 for ethics
    if yaml.get("human_veto").is_none() {
        errors.push(
            "CRITICAL: ethics.yaml missing 'human_veto' section. Human override capability is required."
                .to_string(),
        );
    }

    // core_principles should exist
    if yaml.get("core_principles").is_none() {
        errors.push("ethics.yaml missing 'core_principles' section.".to_string());
    }

    errors
}

/// Structure validation for warmup.yaml (v7.0.6)
/// Note: self_healing is now validated in sprint.yaml, not warmup.yaml
/// warmup.yaml contains only project-specific configuration
pub fn check_warmup_structure(_content: &str) -> (Vec<String>, Vec<String>) {
    let errors = Vec::new();
    let warnings = Vec::new();

    // v7.0.6: self_healing moved to sprint.yaml
    // warmup.yaml now contains only project-specific config:
    // - identity, mission, environment, quality, files

    (errors, warnings)
}

/// Delete CLAUDE.md if it exists - deprecated since v7.1.0
/// CLAUDE.md was replaced by SessionStart hooks which inject context directly.
/// The @import syntax in CLAUDE.md didn't trigger execution, making it redundant.
pub fn delete_deprecated_claude_md(dir: &Path) {
    let claude_md_path = dir.join("CLAUDE.md");

    if claude_md_path.exists() {
        match std::fs::remove_file(&claude_md_path) {
            Ok(_) => {
                eprintln!(
                    "  {} Deleted deprecated CLAUDE.md (replaced by SessionStart hooks)",
                    "CLEANUP".yellow()
                );
            }
            Err(e) => {
                eprintln!("  {} Failed to delete CLAUDE.md: {}", "WARN".yellow(), e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    // ========== warmup.yaml Tests ==========

    #[test]
    fn test_valid_warmup_minimal() {
        let content = r#"
identity:
  project: "Test Project"
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
        assert_eq!(result.schema_type, "warmup");
    }

    #[test]
    fn test_valid_warmup_full() {
        let content = r#"
identity:
  project: "Test Project"
  tagline: "A test project"
  version: "1.0.0"
  philosophy: "Test all the things"

mission:
  problem: "Need to test"
  solution: "Write tests"
  principles:
    - "Test first"
    - "Test often"

files:
  source:
    - "src/ - Source code"
  docs:
    - "README.md - Docs"

session:
  start:
    - "Read warmup.yaml"
  during:
    - "Track progress"
  end:
    - "Run tests"

quality:
  tests: "All must pass"

style:
  code:
    - "Be consistent"
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
        assert!(
            result.errors.iter().any(|e| e.contains("identity")),
            "Should mention missing identity: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_invalid_warmup_missing_project() {
        let content = r#"
identity:
  tagline: "No project name"
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid);
        assert!(
            result.errors.iter().any(|e| e.contains("project")),
            "Should mention missing project: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_invalid_warmup_empty_project() {
        let content = r#"
identity:
  project: ""
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid, "Empty project should fail");
    }

    // ========== sprint.yaml Tests ==========

    #[test]
    fn test_valid_sprint_minimal() {
        let content = r#"
sprint:
  current: "Feature work"
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
        assert_eq!(result.schema_type, "sprint");
    }

    #[test]
    fn test_valid_sprint_full() {
        let content = r#"
sprint:
  current: "Feature work"
  started: "2025-01-01"
  status: in_progress
  tasks:
    - "[ ] Task one"
    - "[x] Task two"
  completed:
    - "[x] Done task"
  blockers:
    - "Waiting for review"
  next_up:
    - "Next task"
  notes: "Some notes here"
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
    }

    #[test]
    fn test_valid_sprint_all_statuses() {
        for status in ["planned", "in_progress", "blocked", "done"] {
            let content = format!(
                r#"
sprint:
  current: "Test"
  status: {}
"#,
                status
            );
            let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
            write!(file, "{}", content).unwrap();

            let result = validate_file(file.path()).unwrap();
            assert!(result.is_valid, "Status '{}' should be valid", status);
        }
    }

    #[test]
    fn test_invalid_sprint_bad_status() {
        let content = r#"
sprint:
  current: "Test"
  status: invalid_status
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_invalid_sprint_missing_current() {
        let content = r#"
sprint:
  status: in_progress
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid);
    }

    // ========== roadmap.yaml Tests ==========

    #[test]
    fn test_valid_roadmap_minimal() {
        let content = r#"
current:
  version: "1.0.0"
  status: planned
  summary: "First milestone"
"#;
        let mut file = NamedTempFile::with_suffix("_roadmap.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
        assert_eq!(result.schema_type, "roadmap");
    }

    #[test]
    fn test_valid_roadmap_full() {
        let content = r#"
current:
  version: "1.0.0"
  status: in_progress
  summary: "Current milestone"
  goal: "CORE_VALUE"
  adr: "docs/adr/001-example.md"
  deliverables:
    - "[ ] Feature one"
    - "[ ] Feature two"

next:
  - version: "1.1.0"
    summary: "Next milestone"
    goal: "ANOTHER_VALUE"
  - version: "1.2.0"
    summary: "Future milestone"

backlog:
  - "Future idea one"
  - "Future idea two"
"#;
        let mut file = NamedTempFile::with_suffix("_roadmap.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
    }

    #[test]
    fn test_valid_roadmap_all_statuses() {
        for status in ["planned", "in_progress", "released"] {
            let content = format!(
                r#"
current:
  version: "1.0.0"
  status: {}
  summary: "Test milestone"
"#,
                status
            );
            let mut file = NamedTempFile::with_suffix("_roadmap.yaml").unwrap();
            write!(file, "{}", content).unwrap();

            let result = validate_file(file.path()).unwrap();
            assert!(result.is_valid, "Status '{}' should be valid", status);
        }
    }

    // ========== Error Handling Tests ==========

    #[test]
    fn test_file_not_found() {
        let result = validate_file(Path::new("/nonexistent/path/warmup.yaml"));
        assert!(result.is_err());
        match result {
            Err(Error::FileNotFound(_)) => (),
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn test_unknown_file_type() {
        let content = "key: value";
        let mut file = NamedTempFile::with_suffix(".yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path());
        assert!(result.is_err());
        match result {
            Err(Error::UnknownFileType(_)) => (),
            _ => panic!("Expected UnknownFileType error"),
        }
    }

    #[test]
    fn test_malformed_yaml() {
        let content = r#"
identity:
  project: "Test
  unclosed: [
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path());
        assert!(result.is_err());
        match result {
            Err(Error::YamlError(_)) => (),
            _ => panic!("Expected YamlError"),
        }
    }

    // ========== Directory Validation Tests ==========

    #[test]
    fn test_validate_directory_with_all_files() {
        let temp_dir = TempDir::new().unwrap();

        // Create all protocol files (without regeneration)
        std::fs::write(
            temp_dir.path().join("warmup.yaml"),
            "identity:\n  project: Test",
        )
        .unwrap();
        std::fs::write(
            temp_dir.path().join("sprint.yaml"),
            "sprint:\n  current: Work",
        )
        .unwrap();
        std::fs::write(
            temp_dir.path().join("roadmap.yaml"),
            "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test milestone",
        )
        .unwrap();

        // Use no-regenerate to only validate existing files
        let results = validate_directory_with_options(temp_dir.path(), false).unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_valid));
    }

    #[test]
    fn test_validate_directory_warmup_only() {
        let temp_dir = TempDir::new().unwrap();

        std::fs::write(
            temp_dir.path().join("warmup.yaml"),
            "identity:\n  project: Test",
        )
        .unwrap();

        // Use no-regenerate to only validate existing warmup.yaml
        let results = validate_directory_with_options(temp_dir.path(), false).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].schema_type, "warmup");
    }

    #[test]
    fn test_validate_directory_no_protocol_files() {
        let temp_dir = TempDir::new().unwrap();

        std::fs::write(temp_dir.path().join("config.yaml"), "key: value").unwrap();

        // Use no-regenerate - should fail because no protocol files exist
        let result = validate_directory_with_options(temp_dir.path(), false);
        assert!(result.is_err());
        match result {
            Err(Error::ValidationError(msg)) => {
                assert!(msg.contains("No protocol files"));
            }
            _ => panic!("Expected ValidationError"),
        }
    }

    // ========== is_protocol_file Tests ==========

    #[test]
    fn test_is_protocol_file() {
        // Valid protocol files (.yaml)
        assert!(is_protocol_file("warmup.yaml"));
        assert!(is_protocol_file("sprint.yaml"));
        assert!(is_protocol_file("roadmap.yaml"));
        assert!(is_protocol_file("ethics.yaml"));
        assert!(is_protocol_file("green.yaml"));
        assert!(is_protocol_file("sycophancy.yaml"));
        assert!(is_protocol_file("WARMUP.yaml"));
        assert!(is_protocol_file("SPRINT.YAML"));
        assert!(is_protocol_file("my_warmup.yaml"));
        assert!(is_protocol_file("project_sprint.yaml"));
        assert!(is_protocol_file("roadmap_v2.yaml"));

        // Valid protocol files (.yml)
        assert!(is_protocol_file("warmup.yml"));
        assert!(is_protocol_file("sprint.yml"));
        assert!(is_protocol_file("roadmap.yml"));

        // Invalid files - wrong extension
        assert!(!is_protocol_file("warmup.json"));
        assert!(!is_protocol_file("sprint.txt"));
        assert!(!is_protocol_file("warmup")); // No extension

        // Invalid files - wrong name
        assert!(!is_protocol_file("config.yaml"));
        assert!(!is_protocol_file("random.txt"));
        assert!(!is_protocol_file(""));
    }

    // ========== ValidationResult Tests ==========

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success("test.yaml".to_string(), "warmup".to_string());
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_result_failure() {
        let errors = vec!["Error 1".to_string(), "Error 2".to_string()];
        let result =
            ValidationResult::failure("test.yaml".to_string(), "warmup".to_string(), errors);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 2);
    }

    // ========== Ethics Structure Validation Tests (v3.2.0) ==========

    #[test]
    fn test_ethics_structure_valid() {
        let content = r#"
core_principles:
  status: REQUIRED
human_veto:
  command: "stop"
"#;
        let errors = check_ethics_structure(content);
        assert!(errors.is_empty(), "Expected no errors: {:?}", errors);
    }

    #[test]
    fn test_ethics_structure_missing_human_veto() {
        let content = r#"
core_principles:
  status: REQUIRED
"#;
        let errors = check_ethics_structure(content);
        assert!(!errors.is_empty());
        assert!(
            errors.iter().any(|e| e.contains("human_veto")),
            "Should mention missing human_veto: {:?}",
            errors
        );
    }

    #[test]
    fn test_ethics_structure_missing_core_principles() {
        let content = r#"
human_veto:
  command: "stop"
"#;
        let errors = check_ethics_structure(content);
        assert!(!errors.is_empty());
        assert!(
            errors.iter().any(|e| e.contains("core_principles")),
            "Should mention missing core_principles: {:?}",
            errors
        );
    }

    #[test]
    fn test_ethics_validation_fails_without_human_veto() {
        let content = r#"
core_principles:
  status: REQUIRED
red_flags:
  financial:
    - "crypto wallet"
"#;
        let mut file = NamedTempFile::with_suffix("_ethics.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid, "Ethics without human_veto should fail");
        assert!(
            result.errors.iter().any(|e| e.contains("human_veto")),
            "Error should mention human_veto: {:?}",
            result.errors
        );
    }

    // ========== Warmup Structure Validation Tests (v7.0.6) ==========

    #[test]
    fn test_warmup_structure_no_self_healing_required() {
        // v7.0.6: self_healing moved to sprint.yaml, warmup.yaml is project config only
        let content = r#"
identity:
  project: Test
  tagline: "Test project"
quality:
  test: "cargo test"
"#;
        let (errors, warnings) = check_warmup_structure(content);
        assert!(errors.is_empty());
        assert!(
            warnings.is_empty(),
            "Should not warn - self_healing is now in sprint.yaml"
        );
    }

    #[test]
    fn test_warmup_structure_no_warnings_for_any_content() {
        // v7.0.6: warmup.yaml validation is minimal - just project config
        let content = r#"
identity:
  project: Test
mission:
  problem: "Test problem"
  solution: "Test solution"
"#;
        let (errors, warnings) = check_warmup_structure(content);
        assert!(errors.is_empty());
        assert!(warnings.is_empty());
    }

    // ========== Auto-Regeneration Tests (v4.1.5) ==========

    #[test]
    fn test_regeneration_info_empty() {
        let info = RegenerationInfo::default();
        assert!(info.is_empty());
    }

    #[test]
    fn test_regeneration_info_not_empty() {
        let mut info = RegenerationInfo::default();
        info.regenerated.push(("ethics.yaml".to_string(), true));
        assert!(!info.is_empty());
    }

    #[test]
    fn test_validate_directory_regenerates_missing_files() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Start with empty directory
        let (results, info) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // Should have regenerated all required files
        assert!(!info.is_empty(), "Should have regenerated files");
        assert!(
            info.regenerated.iter().any(|(f, _)| f == "ethics.yaml"),
            "Should regenerate ethics.yaml"
        );
        assert!(
            info.regenerated.iter().any(|(f, _)| f == "warmup.yaml"),
            "Should regenerate warmup.yaml"
        );
        assert!(
            info.regenerated.iter().any(|(f, _)| f == "green.yaml"),
            "Should regenerate green.yaml"
        );
        assert!(
            info.regenerated.iter().any(|(f, _)| f == "sycophancy.yaml"),
            "Should regenerate sycophancy.yaml"
        );
        assert!(
            info.regenerated.iter().any(|(f, _)| f == "sprint.yaml"),
            "Should regenerate sprint.yaml"
        );
        assert!(
            info.regenerated.iter().any(|(f, _)| f == "roadmap.yaml"),
            "Should regenerate roadmap.yaml"
        );

        // All results should be valid
        assert!(
            results.iter().all(|r| r.is_valid),
            "All regenerated files should be valid"
        );
    }

    #[test]
    fn test_validate_directory_no_regenerate_flag() {
        let temp_dir = tempfile::tempdir().unwrap();

        // With regenerate=false, should fail because no files exist
        let result = validate_directory_with_regeneration(temp_dir.path(), false);
        assert!(
            result.is_err(),
            "Should error when no files exist and regeneration disabled"
        );
    }

    #[test]
    fn test_validate_directory_marks_regenerated_results() {
        let temp_dir = tempfile::tempdir().unwrap();

        let (results, _) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // All results should be marked as regenerated
        assert!(
            results.iter().all(|r| r.regenerated),
            "All results should be marked as regenerated"
        );
    }

    #[test]
    fn test_validate_directory_existing_files_not_regenerated() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create .asimov directory and warmup.yaml file manually
        let asimov_dir = temp_dir.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        let warmup_content = r#"
identity:
  project: "Test"
"#;
        std::fs::write(asimov_dir.join("warmup.yaml"), warmup_content).unwrap();

        let (results, info) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // warmup.yaml should NOT be in regenerated list
        assert!(
            !info.regenerated.iter().any(|(f, _)| f == "warmup.yaml"),
            "Existing warmup.yaml should not be regenerated"
        );

        // warmup.yaml result should NOT be marked as regenerated
        let warmup_result = results.iter().find(|r| r.file.contains("warmup.yaml"));
        assert!(
            warmup_result.is_some(),
            "Should have warmup.yaml in results"
        );
        assert!(
            !warmup_result.unwrap().regenerated,
            "Existing file should not be marked as regenerated"
        );
    }

    #[test]
    fn test_regeneration_warn_levels() {
        let temp_dir = tempfile::tempdir().unwrap();

        let (_, info) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // ethics.yaml, warmup.yaml, and sycophancy.yaml should have warn level = true
        for (filename, is_warn) in &info.regenerated {
            match filename.as_str() {
                "ethics.yaml" | "warmup.yaml" | "sycophancy.yaml" => {
                    assert!(*is_warn, "{} should have WARN level", filename);
                }
                "green.yaml" | "sprint.yaml" | "roadmap.yaml" => {
                    assert!(!*is_warn, "{} should have INFO level", filename);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_validation_result_with_regenerated() {
        let result = ValidationResult::success("test.yaml".to_string(), "warmup".to_string())
            .with_regenerated();
        assert!(result.regenerated);
    }
}
