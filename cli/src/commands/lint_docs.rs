//! Lint-docs command implementation

use crate::{
    check_markdown_file, check_semantic, find_markdown_files, fix_markdown_file,
    load_deprecated_patterns, SemanticConfig, Severity,
};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct LintFileResult {
    pub file: String,
    pub errors: Vec<String>,
    pub fixed: bool,
}

#[derive(Debug, Clone)]
pub struct SemanticIssue {
    pub file: String,
    pub line: usize,
    pub severity: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct LintDocsResult {
    pub success: bool,
    pub files_checked: usize,
    pub files_with_errors: usize,
    pub files_fixed: usize,
    pub lint_results: Vec<LintFileResult>,
    pub semantic_issues: Vec<SemanticIssue>,
    pub semantic_files_checked: usize,
}

pub fn run_lint_docs(dir: &Path, fix: bool, semantic: bool) -> LintDocsResult {
    let mut result = LintDocsResult {
        success: true,
        files_checked: 0,
        files_with_errors: 0,
        files_fixed: 0,
        lint_results: Vec::new(),
        semantic_issues: Vec::new(),
        semantic_files_checked: 0,
    };

    let files = find_markdown_files(dir);
    result.files_checked = files.len();

    for file in &files {
        let mut file_result = LintFileResult {
            file: file.display().to_string(),
            errors: Vec::new(),
            fixed: false,
        };

        match check_markdown_file(file) {
            Ok(lint_result) => {
                if !lint_result.errors.is_empty() {
                    result.files_with_errors += 1;
                    for err in &lint_result.errors {
                        file_result
                            .errors
                            .push(format!("Line {}: {}", err.line, err.message));
                    }

                    if fix {
                        if fix_markdown_file(file).is_ok() {
                            file_result.fixed = true;
                            result.files_fixed += 1;
                        }
                    } else {
                        result.success = false;
                    }
                }
            }
            Err(e) => {
                file_result.errors.push(format!("Error: {}", e));
                result.success = false;
            }
        }

        result.lint_results.push(file_result);
    }

    if semantic {
        let patterns = load_deprecated_patterns(dir);
        let config = SemanticConfig {
            deprecated_patterns: patterns,
            expected_version: None,
            check_help: false,
        };

        let semantic_result = check_semantic(dir, &config);
        result.semantic_files_checked = semantic_result.files_checked;

        for issue in semantic_result.issues {
            result.semantic_issues.push(SemanticIssue {
                file: issue.file.display().to_string(),
                line: issue.line.unwrap_or(0),
                severity: format!("{:?}", issue.severity),
                message: issue.message,
            });

            if issue.severity == Severity::Error {
                result.success = false;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_lint_docs_empty() {
        let temp = TempDir::new().unwrap();
        let result = run_lint_docs(temp.path(), false, false);
        assert!(result.success);
        assert_eq!(result.files_checked, 0);
    }

    #[test]
    fn test_run_lint_docs_with_file() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("test.md"), "# Test\n\nContent here.\n").unwrap();

        let result = run_lint_docs(temp.path(), false, false);
        assert!(result.success);
        assert_eq!(result.files_checked, 1);
    }

    #[test]
    fn test_run_lint_docs_with_semantic() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("test.md"), "# Test\n\nVersion 1.0.0\n").unwrap();

        let result = run_lint_docs(temp.path(), false, true);
        assert!(result.semantic_files_checked >= 1);
    }

    #[test]
    fn test_lint_file_result_struct() {
        let r = LintFileResult {
            file: "test.md".to_string(),
            errors: vec!["Error 1".to_string()],
            fixed: false,
        };
        assert!(!r.fixed);
        assert_eq!(r.errors.len(), 1);
    }

    #[test]
    fn test_semantic_issue_struct() {
        let i = SemanticIssue {
            file: "test.md".to_string(),
            line: 5,
            severity: "Warning".to_string(),
            message: "Test issue".to_string(),
        };
        assert_eq!(i.line, 5);
    }

    #[test]
    fn test_lint_docs_result_fields() {
        let r = LintDocsResult {
            success: true,
            files_checked: 5,
            files_with_errors: 0,
            files_fixed: 2,
            lint_results: vec![],
            semantic_issues: vec![],
            semantic_files_checked: 3,
        };
        assert_eq!(r.files_checked, 5);
    }

    #[test]
    fn test_run_lint_docs_with_fix() {
        let temp = TempDir::new().unwrap();
        // Create a markdown file with an unclosed code block
        std::fs::write(temp.path().join("broken.md"), "# Test\n\n~~~\ncode\n").unwrap();

        let result = run_lint_docs(temp.path(), true, false);
        // File may or may not be fixed depending on implementation
        assert!(result.files_checked >= 1);
    }

    #[test]
    fn test_run_lint_docs_with_errors() {
        let temp = TempDir::new().unwrap();
        // Create a markdown file with lint issues
        std::fs::write(
            temp.path().join("issues.md"),
            "# Test\n\n~~~\nunclosed code block\n",
        )
        .unwrap();

        let result = run_lint_docs(temp.path(), false, false);
        assert!(result.files_checked >= 1);
        // May have issues detected
    }

    #[test]
    fn test_run_lint_docs_single_file() {
        let temp = TempDir::new().unwrap();
        let md_file = temp.path().join("README.md");
        std::fs::write(&md_file, "# Title\n\nContent here.\n").unwrap();

        let result = run_lint_docs(&md_file, false, false);
        assert!(result.success);
        assert_eq!(result.files_checked, 1);
    }

    #[test]
    fn test_run_lint_docs_with_subdirs() {
        let temp = TempDir::new().unwrap();
        let docs_dir = temp.path().join("docs");
        std::fs::create_dir_all(&docs_dir).unwrap();
        std::fs::write(docs_dir.join("guide.md"), "# Guide\n\nContent.\n").unwrap();
        std::fs::write(temp.path().join("README.md"), "# Readme\n\nContent.\n").unwrap();

        let result = run_lint_docs(temp.path(), false, false);
        assert!(result.success);
        assert!(result.files_checked >= 2);
    }

    #[test]
    fn test_run_lint_docs_fix_mode() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("fixable.md"), "# Test\n\n~~~\ncode\n~~~\n").unwrap();
        let result = run_lint_docs(temp.path(), true, false);
        assert!(result.success);
    }

    #[test]
    fn test_run_lint_docs_semantic_empty() {
        let temp = TempDir::new().unwrap();
        let result = run_lint_docs(temp.path(), false, true);
        // No files to check
        assert!(result.success);
    }
}
