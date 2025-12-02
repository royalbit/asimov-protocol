//! Semantic linting for RoyalBit Asimov projects
//!
//! Cross-consistency checking for documentation and code:
//! - Version consistency across files
//! - Deprecated pattern detection
//! - Cross-reference validation

use crate::markdown::find_markdown_files;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

/// Result of semantic linting
#[derive(Debug, Default)]
pub struct SemanticResult {
    pub issues: Vec<SemanticIssue>,
    pub files_checked: usize,
    pub version_refs_found: usize,
    pub deprecated_matches: usize,
}

impl SemanticResult {
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count()
    }
}

/// A single semantic issue
#[derive(Debug)]
pub struct SemanticIssue {
    pub file: PathBuf,
    pub line: Option<usize>,
    pub category: IssueCategory,
    pub severity: Severity,
    pub message: String,
    pub context: Option<String>,
}

/// Issue categories
#[derive(Debug, Clone, PartialEq)]
pub enum IssueCategory {
    VersionMismatch,
    DeprecatedPattern,
    HelpDocMismatch,
}

impl std::fmt::Display for IssueCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueCategory::VersionMismatch => write!(f, "version"),
            IssueCategory::DeprecatedPattern => write!(f, "deprecated"),
            IssueCategory::HelpDocMismatch => write!(f, "help-doc"),
        }
    }
}

/// Issue severity
#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
    Error,
    Warning,
}

/// Configuration for semantic linting
#[derive(Debug, Default)]
pub struct SemanticConfig {
    /// Patterns to flag as deprecated (case-insensitive by default)
    pub deprecated_patterns: Vec<DeprecatedPattern>,
    /// Expected version (from Cargo.toml or explicit)
    pub expected_version: Option<String>,
    /// Check --help output against docs
    pub check_help: bool,
}

/// A deprecated pattern to detect
#[derive(Debug, Clone)]
pub struct DeprecatedPattern {
    pub pattern: String,
    pub replacement: Option<String>,
    pub reason: Option<String>,
    pub case_sensitive: bool,
}

impl DeprecatedPattern {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: pattern.to_string(),
            replacement: None,
            reason: None,
            case_sensitive: false,
        }
    }

    pub fn with_replacement(mut self, replacement: &str) -> Self {
        self.replacement = Some(replacement.to_string());
        self
    }

    pub fn with_reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_string());
        self
    }
}

/// Run semantic checks on a directory
pub fn check_semantic(dir: &Path, config: &SemanticConfig) -> SemanticResult {
    let mut result = SemanticResult::default();

    // Find all markdown files
    let md_files = find_markdown_files(dir);
    result.files_checked = md_files.len();

    // Also check source files for version consistency
    let source_files = find_source_files(dir);

    // 1. Version consistency check
    if let Some(expected) = &config.expected_version {
        check_version_consistency(dir, expected, &md_files, &source_files, &mut result);
    }

    // 2. Deprecated pattern detection
    if !config.deprecated_patterns.is_empty() {
        check_deprecated_patterns(&md_files, &config.deprecated_patterns, &mut result);
    }

    // 3. Help/doc consistency check
    if config.check_help {
        check_help_doc_consistency(dir, &mut result);
    }

    result
}

/// Find source files (Rust, Python, etc.) for version checking
fn find_source_files(dir: &Path) -> Vec<PathBuf> {
    use walkdir::WalkDir;

    let ignore_dirs = [
        "node_modules",
        "target",
        "vendor",
        ".git",
        "__pycache__",
        "venv",
    ];

    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !ignore_dirs.contains(&name.as_ref())
        })
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && {
                let ext = e.path().extension().and_then(|e| e.to_str());
                matches!(ext, Some("rs") | Some("py") | Some("js") | Some("ts"))
            }
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Check version consistency across files
fn check_version_consistency(
    _dir: &Path,
    expected: &str,
    md_files: &[PathBuf],
    source_files: &[PathBuf],
    result: &mut SemanticResult,
) {
    // Regex patterns for version references
    let version_patterns = [
        // YAML: version: "X.Y.Z" or version: X.Y.Z
        r#"version:\s*["']?(\d+\.\d+\.\d+)["']?"#,
        // Markdown: Version X.Y.Z or vX.Y.Z
        r"(?i)version\s+(\d+\.\d+\.\d+)",
        r"(?i)\bv(\d+\.\d+\.\d+)\b",
        // In text: RoyalBit Asimov X.Y.Z
        r"(?i)asimov\s+(\d+\.\d+\.\d+)",
    ];

    let regexes: Vec<Regex> = version_patterns
        .iter()
        .filter_map(|p| Regex::new(p).ok())
        .collect();

    // Check all files
    let all_files: Vec<&PathBuf> = md_files.iter().chain(source_files.iter()).collect();

    for file in all_files {
        if let Ok(content) = fs::read_to_string(file) {
            for (line_num, line) in content.lines().enumerate() {
                for re in &regexes {
                    if let Some(caps) = re.captures(line) {
                        if let Some(ver) = caps.get(1) {
                            let found_version = ver.as_str();
                            result.version_refs_found += 1;

                            if found_version != expected {
                                result.issues.push(SemanticIssue {
                                    file: file.clone(),
                                    line: Some(line_num + 1),
                                    category: IssueCategory::VersionMismatch,
                                    severity: Severity::Warning,
                                    message: format!(
                                        "Version mismatch: found '{}', expected '{}'",
                                        found_version, expected
                                    ),
                                    context: Some(line.trim().to_string()),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Check for deprecated patterns in markdown files
fn check_deprecated_patterns(
    files: &[PathBuf],
    patterns: &[DeprecatedPattern],
    result: &mut SemanticResult,
) {
    for file in files {
        if let Ok(content) = fs::read_to_string(file) {
            for (line_num, line) in content.lines().enumerate() {
                for dp in patterns {
                    let matches = if dp.case_sensitive {
                        line.contains(&dp.pattern)
                    } else {
                        line.to_lowercase().contains(&dp.pattern.to_lowercase())
                    };

                    if matches {
                        result.deprecated_matches += 1;

                        let mut message = format!("Deprecated pattern: '{}'", dp.pattern);
                        if let Some(ref replacement) = dp.replacement {
                            message.push_str(&format!(" → use '{}' instead", replacement));
                        }
                        if let Some(ref reason) = dp.reason {
                            message.push_str(&format!(" ({})", reason));
                        }

                        result.issues.push(SemanticIssue {
                            file: file.clone(),
                            line: Some(line_num + 1),
                            category: IssueCategory::DeprecatedPattern,
                            severity: Severity::Warning,
                            message,
                            context: Some(line.trim().to_string()),
                        });
                    }
                }
            }
        }
    }
}

/// Check --help output against documentation
/// Note: This is a placeholder for future implementation.
/// Running cargo from within the process is problematic.
fn check_help_doc_consistency(_dir: &Path, _result: &mut SemanticResult) {
    // TODO: Implement help/doc consistency checking
    // Options:
    // 1. Parse Cargo.toml version and compare with README
    // 2. Run asimov --version as subprocess (requires installed binary)
    // 3. Extract version from source and compare
    //
    // For now, version consistency is handled by check_version_consistency
}

/// Load deprecated patterns from a config file or warmup.yaml
pub fn load_deprecated_patterns(dir: &Path) -> Vec<DeprecatedPattern> {
    let mut patterns = Vec::new();

    // Try to load from .asimov/deprecated.yaml
    let deprecated_file = dir.join(".asimov").join("deprecated.yaml");
    if deprecated_file.exists() {
        if let Ok(content) = fs::read_to_string(&deprecated_file) {
            if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                if let Some(deprecated) = yaml.get("deprecated") {
                    if let Some(arr) = deprecated.as_sequence() {
                        for item in arr {
                            if let Some(map) = item.as_mapping() {
                                let pattern = map.get("pattern").and_then(|v| v.as_str());
                                let replacement = map.get("replacement").and_then(|v| v.as_str());
                                let reason = map.get("reason").and_then(|v| v.as_str());

                                if let Some(p) = pattern {
                                    let mut dp = DeprecatedPattern::new(p);
                                    if let Some(r) = replacement {
                                        dp = dp.with_replacement(r);
                                    }
                                    if let Some(r) = reason {
                                        dp = dp.with_reason(r);
                                    }
                                    patterns.push(dp);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    patterns
}

/// Get version from Cargo.toml
pub fn get_cargo_version(dir: &Path) -> Option<String> {
    // Look for Cargo.toml in common locations
    let cargo_paths = [dir.join("Cargo.toml"), dir.join("cli").join("Cargo.toml")];

    // Create regex once outside the loop
    let re = Regex::new(r#"^version\s*=\s*["'](\d+\.\d+\.\d+)["']"#).ok()?;

    for cargo_path in cargo_paths {
        if cargo_path.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_path) {
                for line in content.lines() {
                    if let Some(caps) = re.captures(line) {
                        return caps.get(1).map(|m| m.as_str().to_string());
                    }
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_deprecated_pattern_detection() {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.md");
        fs::write(&test_file, "Use ethics.yaml for configuration.\n").unwrap();

        let patterns = vec![DeprecatedPattern::new("ethics.yaml")
            .with_replacement("asimov.yaml")
            .with_reason("deprecated in v7.0.8")];

        let config = SemanticConfig {
            deprecated_patterns: patterns,
            ..Default::default()
        };

        let result = check_semantic(temp.path(), &config);
        assert_eq!(result.deprecated_matches, 1);
        assert_eq!(result.issues.len(), 1);
        assert!(result.issues[0].message.contains("asimov.yaml"));
    }

    #[test]
    fn test_version_mismatch() {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("README.md");
        fs::write(&test_file, "# Project v7.3.0\n\nVersion: 7.3.0\n").unwrap();

        let config = SemanticConfig {
            expected_version: Some("7.5.0".to_string()),
            ..Default::default()
        };

        let result = check_semantic(temp.path(), &config);
        assert!(result.version_refs_found >= 1);
        assert!(!result.issues.is_empty());
    }
}
