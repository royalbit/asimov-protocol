//! Core validation logic for RoyalBit Asimov files

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
    // NOTE: CHECKPOINT removed in v8.1.0 (ADR-032)
    pub const PROJECT: FileSizeLimits = FileSizeLimits {
        soft_lines: 50,
        hard_lines: 100,
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
    let yaml_value: serde_yaml_ng::Value = serde_yaml_ng::from_str(&content)?;

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
            let path = e.instance_path().to_string();
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

    // NOTE: ethics.yaml validation removed - asimov.yaml is canonical (ADR-031)
    // Structure validation for asimov.yaml (Three Laws)
    if schema_type == "asimov" {
        let structure_errors = check_asimov_structure(&content);
        if !structure_errors.is_empty() {
            // Asimov structure errors are CRITICAL - fail validation
            result.is_valid = false;
            result.errors.extend(structure_errors);
        }
    }

    // Structure validation for warmup.yaml (v7.0.6: minimal validation)
    // Note: warmup.yaml now only contains project-specific config
    if schema_type == "warmup" {
        let (_errors, warnings) = check_warmup_structure(&content);
        result = result.with_warnings(warnings);
    }

    Ok(result)
}

/// Check file size against limits and return warnings (ADR-007)
/// NOTE: checkpoint removed in v8.1.0 (ADR-032), replaced by project
fn check_file_size(schema_type: &str, line_count: usize) -> Vec<String> {
    let mut warnings = Vec::new();

    let limits = match schema_type {
        "project" => Some(FileSizeLimits::PROJECT),
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
    use crate::templates::roadmap_template;

    // Resolve protocol directory (.asimov/ or root for backwards compatibility)
    let protocol_dir = resolve_protocol_dir(base_dir);

    let mut results = Vec::new();
    let mut regen_info = RegenerationInfo::default();

    // Required files with their templates and warn level
    // (filename, template_fn, is_warn_level)
    // NOTE: v8.0.0 - Protocol YAMLs no longer regenerated (hardcoded in binary)
    // Only roadmap.yaml is regenerated (project data, not protocol)
    #[allow(clippy::type_complexity)]
    let required_files: Vec<(&str, Box<dyn Fn() -> String>, bool)> = vec![
        ("roadmap.yaml", Box::new(roadmap_template), false), // INFO - project data
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

    // Look for data files (v8.0.0: protocol YAMLs are deprecated, hardcoded in binary)
    // v8.1.0: project.yaml replaces deprecated checkpoint (ADR-032)
    let protocol_files = [
        "roadmap.yaml", // Project data - WHAT to build (required)
        "project.yaml", // Project context - HOW to build (ADR-032)
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
            "No data files found in .asimov/ (roadmap.yaml required). Run: asimov init --full"
                .to_string(),
        ));
    }

    Ok((results, regen_info))
}

/// Convert serde_yaml_ng::Value to serde_json::Value
fn yaml_to_json(yaml: &serde_yaml_ng::Value) -> Result<serde_json::Value> {
    // Serialize to string and back to handle type conversions
    let json_str = serde_json::to_string(&yaml)
        .map_err(|e| Error::ValidationError(format!("Failed to convert YAML to JSON: {}", e)))?;
    serde_json::from_str(&json_str)
        .map_err(|e| Error::ValidationError(format!("Failed to parse JSON: {}", e)))
}

/// Check if a file is a valid protocol file by name
/// NOTE: checkpoint deprecated in v8.1.0 (ADR-032), replaced by project.yaml
pub fn is_protocol_file(filename: &str) -> bool {
    let name = filename.to_lowercase();
    let is_yaml = name.ends_with(".yaml") || name.ends_with(".yml");
    is_yaml
        && (name.contains("warmup")
            || name.contains("sprint")
            || name.contains("roadmap")
            || name.contains("asimov")
            || name.contains("freshness")
            || name.contains("green")
            || name.contains("sycophancy")
            || name.contains("migrations")
            || name.contains("project"))
}

/// Structure validation for asimov.yaml (Three Laws - ADR-031)
/// Validates that critical sections like second_law.human_veto exist
/// Returns errors (not warnings) for missing required sections
pub fn check_asimov_structure(content: &str) -> Vec<String> {
    let mut errors = Vec::new();

    // Parse YAML to check structure
    let yaml: serde_yaml_ng::Value = match serde_yaml_ng::from_str(content) {
        Ok(v) => v,
        Err(_) => return errors, // YAML parsing errors handled elsewhere
    };

    // second_law.human_veto is REQUIRED - Priority 0 for Three Laws
    let has_human_veto = yaml
        .get("second_law")
        .and_then(|sl| sl.get("human_veto"))
        .is_some();

    if !has_human_veto {
        errors.push(
            "CRITICAL: asimov.yaml missing 'second_law.human_veto' section. Human override capability is required."
                .to_string(),
        );
    }

    // first_law should exist (do_no_harm)
    if yaml.get("first_law").is_none() {
        errors.push("asimov.yaml missing 'first_law' section (do_no_harm).".to_string());
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

// ============================================================================
// PROTOCOL INTEGRITY CHECK (v9.0.0)
// ============================================================================

/// Result of checking a single protocol file
#[derive(Debug, Clone)]
pub struct ProtocolCheck {
    pub filename: String,
    pub exists: bool,
    pub matches: bool,
    pub outdated: bool, // v9.0.0: renamed from tampered - could be old version, not malicious
}

/// Check all protocol JSON files against expected (hardcoded) content
/// Returns list of checks with status for each file
pub fn check_protocol_integrity(dir: &Path) -> Vec<ProtocolCheck> {
    use crate::protocols::PROTOCOL_FILES;

    let asimov_dir = dir.join(".asimov");
    let mut checks = Vec::new();

    for (filename, generator) in PROTOCOL_FILES {
        let file_path = asimov_dir.join(filename);
        let expected = generator();

        let (exists, matches, outdated) = if file_path.exists() {
            match std::fs::read_to_string(&file_path) {
                Ok(content) => {
                    // Normalize whitespace for comparison
                    let content_normalized = content.trim();
                    let expected_normalized = expected.trim();
                    let matches = content_normalized == expected_normalized;
                    (true, matches, !matches)
                }
                Err(_) => (true, false, true), // Can't read = outdated/corrupt
            }
        } else {
            (false, false, false) // Missing, not outdated
        };

        checks.push(ProtocolCheck {
            filename: filename.to_string(),
            exists,
            matches,
            outdated,
        });
    }

    checks
}

/// Regenerate protocol files and return which ones were changed
/// Returns: (filename, was_different)
pub fn regenerate_protocol_files(dir: &Path) -> Result<Vec<(String, bool)>> {
    use crate::protocols::PROTOCOL_FILES;

    let asimov_dir = dir.join(".asimov");
    if !asimov_dir.exists() {
        return Err(Error::ValidationError(
            "Not in an asimov project (.asimov/ not found)".to_string(),
        ));
    }

    let mut results = Vec::new();

    for (filename, generator) in PROTOCOL_FILES {
        let file_path = asimov_dir.join(filename);
        let expected = generator();

        let was_different = if file_path.exists() {
            match std::fs::read_to_string(&file_path) {
                Ok(content) => content.trim() != expected.trim(),
                Err(_) => true,
            }
        } else {
            true // Missing = different
        };

        // Write the correct content
        std::fs::write(&file_path, &expected)
            .map_err(|e| Error::ValidationError(format!("Failed to write {}: {}", filename, e)))?;

        results.push((filename.to_string(), was_different));
    }

    Ok(results)
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
  name: "Test Project"
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
  name: "Test Project"
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
    fn test_invalid_warmup_missing_name() {
        let content = r#"
identity:
  tagline: "No project name"
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid);
        assert!(
            result.errors.iter().any(|e| e.contains("name")),
            "Should mention missing name: {:?}",
            result.errors
        );
    }

    #[test]
    fn test_invalid_warmup_empty_name() {
        let content = r#"
identity:
  name: ""
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid, "Empty name should fail");
    }

    // ========== sprint.yaml Tests ==========

    #[test]
    fn test_valid_sprint_minimal() {
        let content = r#"
rules:
  must_ship: true
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
rules:
  max_milestones: unlimited
  must_ship: true
  mantra: "Keep shipping until done"

phases:
  1_warmup:
    duration: "2-5 min"
    actions:
      - "Run asimov warmup"

anti_patterns:
  scope_creep: "Note it for NEXT session"

authority:
  principle: "Make decisions. Don't ask."
  can_release_when:
    - "All tests pass"
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(result.is_valid, "Errors: {:?}", result.errors);
    }

    #[test]
    fn test_invalid_sprint_missing_rules() {
        let content = r#"
phases:
  1_warmup:
    duration: "2-5 min"
"#;
        let mut file = NamedTempFile::with_suffix("_sprint.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid);
    }

    #[test]
    fn test_invalid_sprint_missing_must_ship() {
        let content = r#"
rules:
  mantra: "Keep shipping"
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
        for status in ["pending", "planned", "in_progress", "released"] {
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

        // v8.0.0: Only roadmap.yaml is validated (protocols are hardcoded in binary)
        std::fs::write(
            temp_dir.path().join("roadmap.yaml"),
            "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test milestone",
        )
        .unwrap();

        // Use no-regenerate to only validate existing files
        let results = validate_directory_with_options(temp_dir.path(), false).unwrap();
        assert_eq!(results.len(), 1);
        assert!(results.iter().all(|r| r.is_valid));
    }

    #[test]
    fn test_validate_directory_roadmap_only() {
        let temp_dir = TempDir::new().unwrap();

        // v8.0.0: Only roadmap.yaml is validated
        std::fs::write(
            temp_dir.path().join("roadmap.yaml"),
            "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test",
        )
        .unwrap();

        let results = validate_directory_with_options(temp_dir.path(), false).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].schema_type, "roadmap");
    }

    #[test]
    fn test_validate_directory_no_protocol_files() {
        let temp_dir = TempDir::new().unwrap();

        std::fs::write(temp_dir.path().join("config.yaml"), "key: value").unwrap();

        // Use no-regenerate - should fail because no data files exist
        let result = validate_directory_with_options(temp_dir.path(), false);
        assert!(result.is_err());
        match result {
            Err(Error::ValidationError(msg)) => {
                assert!(msg.contains("No data files"));
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
        // NOTE: ethics.yaml removed - asimov.yaml is canonical (ADR-031)
        assert!(is_protocol_file("asimov.yaml"));
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

    #[test]
    fn test_validation_result_with_warning() {
        let result = ValidationResult::success("test.yaml".to_string(), "warmup".to_string())
            .with_warning("Warning message".to_string());
        assert!(result.is_valid);
        assert_eq!(result.warnings.len(), 1);
        assert_eq!(result.warnings[0], "Warning message");
    }

    #[test]
    fn test_validation_result_with_warnings() {
        let warnings = vec!["Warning 1".to_string(), "Warning 2".to_string()];
        let result = ValidationResult::success("test.yaml".to_string(), "warmup".to_string())
            .with_warnings(warnings);
        assert_eq!(result.warnings.len(), 2);
    }

    // ========== Asimov Structure Validation Tests (ADR-031) ==========

    #[test]
    fn test_asimov_structure_valid() {
        let content = r#"
first_law:
  do_no_harm:
    financial: true
second_law:
  human_veto:
    commands: ["stop", "halt"]
"#;
        let errors = check_asimov_structure(content);
        assert!(errors.is_empty(), "Expected no errors: {:?}", errors);
    }

    #[test]
    fn test_asimov_structure_missing_human_veto() {
        let content = r#"
first_law:
  do_no_harm:
    financial: true
second_law:
  transparency: true
"#;
        let errors = check_asimov_structure(content);
        assert!(!errors.is_empty());
        assert!(
            errors.iter().any(|e| e.contains("human_veto")),
            "Should mention missing human_veto: {:?}",
            errors
        );
    }

    #[test]
    fn test_asimov_structure_missing_first_law() {
        let content = r#"
second_law:
  human_veto:
    commands: ["stop"]
"#;
        let errors = check_asimov_structure(content);
        assert!(!errors.is_empty());
        assert!(
            errors.iter().any(|e| e.contains("first_law")),
            "Should mention missing first_law: {:?}",
            errors
        );
    }

    #[test]
    fn test_asimov_validation_fails_without_human_veto() {
        let content = r#"
first_law:
  do_no_harm:
    financial: true
red_flags:
  financial:
    - "crypto wallet"
"#;
        let mut file = NamedTempFile::with_suffix("_asimov.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        assert!(!result.is_valid, "Asimov without human_veto should fail");
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
        info.regenerated.push(("warmup.yaml".to_string(), true));
        assert!(!info.is_empty());
    }

    #[test]
    fn test_validate_directory_regenerates_missing_files() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Start with empty directory
        let (results, info) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // v8.0.0: Only roadmap.yaml is regenerated (protocols are hardcoded in binary)
        assert!(!info.is_empty(), "Should have regenerated files");
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

        // v8.0.0: Only roadmap.yaml is regenerated
        assert!(
            results.iter().all(|r| r.regenerated),
            "All results should be marked as regenerated"
        );
    }

    #[test]
    fn test_validate_directory_existing_files_not_regenerated() {
        let temp_dir = tempfile::tempdir().unwrap();

        // Create .asimov directory and roadmap.yaml file manually
        let asimov_dir = temp_dir.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        let roadmap_content = r#"
current:
  version: "1.0.0"
  status: planned
  summary: "Test"
"#;
        std::fs::write(asimov_dir.join("roadmap.yaml"), roadmap_content).unwrap();

        let (results, info) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // roadmap.yaml should NOT be in regenerated list
        assert!(
            !info.regenerated.iter().any(|(f, _)| f == "roadmap.yaml"),
            "Existing roadmap.yaml should not be regenerated"
        );

        // roadmap.yaml result should NOT be marked as regenerated
        let roadmap_result = results.iter().find(|r| r.file.contains("roadmap.yaml"));
        assert!(
            roadmap_result.is_some(),
            "Should have roadmap.yaml in results"
        );
        assert!(
            !roadmap_result.unwrap().regenerated,
            "Existing file should not be marked as regenerated"
        );
    }

    #[test]
    fn test_regeneration_warn_levels() {
        let temp_dir = tempfile::tempdir().unwrap();

        let (_, info) = validate_directory_with_regeneration(temp_dir.path(), true).unwrap();

        // v8.0.0: Only roadmap.yaml is regenerated (protocols are hardcoded in binary)
        for (filename, is_warn) in &info.regenerated {
            if filename.as_str() == "roadmap.yaml" {
                assert!(!*is_warn, "{} should have INFO level", filename);
            }
        }
    }

    #[test]
    fn test_validation_result_with_regenerated() {
        let result = ValidationResult::success("test.yaml".to_string(), "warmup".to_string())
            .with_regenerated();
        assert!(result.regenerated);
    }

    #[test]
    fn test_check_file_size_project() {
        // Under soft limit - no warnings
        let warnings = check_file_size("project", 30);
        assert!(warnings.is_empty());

        // Over soft limit - warning
        let warnings = check_file_size("project", 60);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("exceeds recommended"));

        // Over hard limit - warning
        let warnings = check_file_size("project", 150);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("exceeds hard limit"));
    }

    #[test]
    fn test_check_file_size_warmup() {
        // Under soft limit - no warnings
        let warnings = check_file_size("warmup", 100);
        assert!(warnings.is_empty());

        // Over soft limit - warning
        let warnings = check_file_size("warmup", 300);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("exceeds recommended"));

        // Over hard limit - warning
        let warnings = check_file_size("warmup", 600);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("exceeds hard limit"));
    }

    #[test]
    fn test_check_file_size_unknown_type() {
        // Unknown types have no limits
        let warnings = check_file_size("unknown", 1000);
        assert!(warnings.is_empty());
    }

    #[test]
    fn test_validate_directory_wrapper() {
        // Test the validate_directory wrapper calls validate_directory_with_options
        let temp_dir = tempfile::tempdir().unwrap();
        let asimov_dir = temp_dir.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
        )
        .unwrap();

        let results = validate_directory(temp_dir.path()).unwrap();
        assert!(!results.is_empty());
    }

    #[test]
    fn test_delete_deprecated_claude_md_removes_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let claude_md = temp_dir.path().join("CLAUDE.md");
        std::fs::write(&claude_md, "# Legacy content").unwrap();

        assert!(claude_md.exists());
        delete_deprecated_claude_md(temp_dir.path());
        assert!(!claude_md.exists());
    }

    #[test]
    fn test_delete_deprecated_claude_md_nonexistent() {
        let temp_dir = tempfile::tempdir().unwrap();
        // No CLAUDE.md exists - should not panic
        delete_deprecated_claude_md(temp_dir.path());
    }

    #[test]
    fn test_warmup_structure_validation_path() {
        // v7.0.6: warmup.yaml has minimal structure validation
        // This test exercises the check_warmup_structure code path
        let content = r#"
identity:
  name: "test"
  tagline: "A test"
"#;
        let mut file = NamedTempFile::with_suffix("_warmup.yaml").unwrap();
        write!(file, "{}", content).unwrap();

        let result = validate_file(file.path()).unwrap();
        // Since v7.0.6, warmup.yaml has minimal validation (just YAML schema)
        // This test verifies the code path is executed without error
        assert!(result.is_valid);
    }

    #[test]
    fn test_ensure_protocol_dir_creates() {
        let temp_dir = tempfile::tempdir().unwrap();
        let asimov_dir = temp_dir.path().join(".asimov");

        assert!(!asimov_dir.exists());
        let result = ensure_protocol_dir(temp_dir.path());
        assert!(result.is_ok());
        assert!(asimov_dir.exists());
    }

    #[test]
    fn test_check_asimov_structure_invalid_yaml() {
        // Invalid YAML should return empty errors (parse errors handled elsewhere)
        let content = "not: valid: yaml: [";
        let errors = check_asimov_structure(content);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_warmup_structure_returns_empty() {
        // v7.0.6: check_warmup_structure always returns empty
        let content = "identity: test";
        let (errors, warnings) = check_warmup_structure(content);
        assert!(errors.is_empty());
        assert!(warnings.is_empty());
    }
}
