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

/// Validate all protocol files in a directory
pub fn validate_directory(dir: &Path) -> Result<Vec<ValidationResult>> {
    let mut results = Vec::new();

    // Look for protocol files
    let protocol_files = [
        "warmup.yaml",
        "sprint.yaml",
        "roadmap.yaml",
        "ethics.yaml",
        ".claude_checkpoint.yaml",
    ];

    for filename in &protocol_files {
        let file_path = dir.join(filename);
        if file_path.exists() {
            results.push(validate_file(&file_path)?);
        }
    }

    // Also validate CLAUDE.md if present (size check only, not schema)
    if let Some(claude_md_result) = validate_claude_md(dir) {
        results.push(claude_md_result);
    }

    if results.is_empty() {
        return Err(Error::ValidationError(
            "No protocol files found (warmup.yaml, sprint.yaml, roadmap.yaml, ethics.yaml)"
                .to_string(),
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
    let is_yaml = name.ends_with(".yaml") || name.ends_with(".yml");
    is_yaml
        && (name.contains("warmup")
            || name.contains("sprint")
            || name.contains("roadmap")
            || name.contains("ethics")
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

/// Structure validation for warmup.yaml (v3.2.0 Anti-Hallucination)
/// Validates that self_healing.on_confusion exists for recovery
pub fn check_warmup_structure(content: &str) -> (Vec<String>, Vec<String>) {
    let errors = Vec::new();
    let mut warnings = Vec::new();

    // Parse YAML to check structure
    let yaml: serde_yaml::Value = match serde_yaml::from_str(content) {
        Ok(v) => v,
        Err(_) => return (errors, warnings), // YAML parsing errors handled elsewhere
    };

    // Check for self_healing section
    let has_self_healing = yaml.get("self_healing").is_some();
    let has_on_confusion = yaml
        .get("self_healing")
        .and_then(|sh| sh.get("on_confusion"))
        .is_some();

    if !has_self_healing {
        warnings.push(
            "warmup.yaml missing 'self_healing' section. Recommended for context recovery."
                .to_string(),
        );
    } else if !has_on_confusion {
        warnings.push(
            "warmup.yaml self_healing missing 'on_confusion'. Recommended for AI recovery guidance."
                .to_string(),
        );
    }

    // Check position of on_confusion (should be in first ~100 lines for quick access)
    let on_confusion_line = content
        .lines()
        .enumerate()
        .find(|(_, line)| line.trim().starts_with("on_confusion:"))
        .map(|(i, _)| i + 1);

    if let Some(line) = on_confusion_line {
        if line > 100 {
            warnings.push(format!(
                "warmup.yaml 'on_confusion' at line {} - consider moving earlier for faster context recovery.",
                line
            ));
        }
    }

    (errors, warnings)
}

/// Validate CLAUDE.md file for self-healing protocol (ADR-007)
/// Returns warnings if the file is too large or missing
pub fn validate_claude_md(dir: &Path) -> Option<ValidationResult> {
    let claude_md_path = dir.join("CLAUDE.md");

    if !claude_md_path.exists() {
        // CLAUDE.md is optional but recommended for SKYNET MODE
        return None;
    }

    let content = match std::fs::read_to_string(&claude_md_path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let line_count = content.lines().count();
    let limits = FileSizeLimits::CLAUDE_MD;

    let mut result = ValidationResult::success(
        claude_md_path.display().to_string(),
        "claude-md".to_string(),
    );

    if line_count > limits.hard_lines {
        result = result.with_warning(format!(
            "CLAUDE.md has {} lines, exceeds hard limit of {} lines. Must be ultra-short to survive compaction.",
            line_count, limits.hard_lines
        ));
    } else if line_count > limits.soft_lines {
        result = result.with_warning(format!(
            "CLAUDE.md has {} lines, exceeds recommended {} lines. Keep it ultra-short for self-healing.",
            line_count, limits.soft_lines
        ));
    }

    Some(result)
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
metadata:
  current_version: "1.0.0"
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
metadata:
  current_version: "1.0.0"
  last_updated: "2025-01-01"
  philosophy: "Ship fast"

current:
  version: "1.0.0"
  status: released
  date: "2025-01-01"
  summary: "Initial release"
  highlights:
    - "Feature one"
    - "Feature two"

next:
  version: "1.1.0"
  status: planned
  summary: "Next release"
  features:
    - "New feature"

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

        // Create all three protocol files
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
            "metadata:\n  current_version: '1.0.0'",
        )
        .unwrap();

        let results = validate_directory(temp_dir.path()).unwrap();
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

        let results = validate_directory(temp_dir.path()).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].schema_type, "warmup");
    }

    #[test]
    fn test_validate_directory_no_protocol_files() {
        let temp_dir = TempDir::new().unwrap();

        std::fs::write(temp_dir.path().join("config.yaml"), "key: value").unwrap();

        let result = validate_directory(temp_dir.path());
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

    // ========== Warmup Structure Validation Tests (v3.2.0) ==========

    #[test]
    fn test_warmup_structure_with_self_healing() {
        let content = r#"
identity:
  project: Test
self_healing:
  on_confusion: "Re-read warmup.yaml"
"#;
        let (errors, warnings) = check_warmup_structure(content);
        assert!(errors.is_empty());
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_warmup_structure_missing_self_healing() {
        let content = r#"
identity:
  project: Test
"#;
        let (errors, warnings) = check_warmup_structure(content);
        assert!(errors.is_empty());
        assert!(
            warnings.iter().any(|w| w.contains("self_healing")),
            "Should warn about missing self_healing: {:?}",
            warnings
        );
    }

    #[test]
    fn test_warmup_structure_missing_on_confusion() {
        let content = r#"
identity:
  project: Test
self_healing:
  checkpoint: true
"#;
        let (errors, warnings) = check_warmup_structure(content);
        assert!(errors.is_empty());
        assert!(
            warnings.iter().any(|w| w.contains("on_confusion")),
            "Should warn about missing on_confusion: {:?}",
            warnings
        );
    }

    #[test]
    fn test_warmup_structure_on_confusion_late_position() {
        // Generate content with on_confusion after line 100
        let mut lines: Vec<String> = Vec::new();
        lines.push("identity:".to_string());
        lines.push("  project: Test".to_string());
        for i in 0..100 {
            lines.push(format!("  field_{}: value", i));
        }
        lines.push("self_healing:".to_string());
        lines.push("  on_confusion: 'too late'".to_string());

        let content = lines.join("\n");
        let (errors, warnings) = check_warmup_structure(&content);
        assert!(errors.is_empty());
        assert!(
            warnings.iter().any(|w| w.contains("line")),
            "Should warn about late on_confusion position: {:?}",
            warnings
        );
    }
}
