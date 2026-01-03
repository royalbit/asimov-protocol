//! Markdown linting for RoyalBit Asimov projects
//!
//! Checks for common markdown issues, especially the code block closer bug
//! where blocks are closed with ```lang instead of just ```

use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Result of linting a single markdown file
#[derive(Debug)]
pub struct LintResult {
    pub file: PathBuf,
    pub errors: Vec<LintError>,
    pub fixed: bool,
}

/// A single lint error
#[derive(Debug)]
pub struct LintError {
    pub line: usize,
    pub message: String,
}

impl LintResult {
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Find all markdown files in a directory, excluding common ignore patterns
pub fn find_markdown_files(dir: &Path) -> Vec<PathBuf> {
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
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .map(|ext| ext == "md" || ext == "markdown")
                    .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Check a single markdown file for code block closer issues
pub fn check_file(path: &Path) -> Result<LintResult, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let errors = check_content(&content);

    Ok(LintResult {
        file: path.to_path_buf(),
        errors,
        fixed: false,
    })
}

/// Check markdown content for code block closer issues
fn check_content(content: &str) -> Vec<LintError> {
    let mut errors = Vec::new();
    let mut in_block = false;
    let mut block_fence: Option<&str> = None; // "```" or "~~~"

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Check for backtick fence
        let is_backtick = trimmed.starts_with("```");
        // Check for tilde fence
        let is_tilde = trimmed.starts_with("~~~");

        if !in_block {
            // Not in a block - check for opening fence
            if is_backtick {
                in_block = true;
                block_fence = Some("```");
            } else if is_tilde {
                in_block = true;
                block_fence = Some("~~~");
            }
        } else {
            // Inside a block - only matching fence type can close it
            match block_fence {
                Some("```") if is_backtick => {
                    // Closing backtick block - should be just ```
                    if trimmed != "```" {
                        let lang = &trimmed[3..];
                        errors.push(LintError {
                            line: line_num + 1,
                            message: format!(
                                "Code block closed with '```{}' instead of '```'",
                                lang
                            ),
                        });
                    }
                    in_block = false;
                    block_fence = None;
                }
                Some("~~~") if is_tilde => {
                    // Closing tilde block - should be just ~~~
                    if trimmed != "~~~" {
                        let suffix = &trimmed[3..];
                        errors.push(LintError {
                            line: line_num + 1,
                            message: format!(
                                "Code block closed with '~~~{}' instead of '~~~'",
                                suffix
                            ),
                        });
                    }
                    in_block = false;
                    block_fence = None;
                }
                _ => {
                    // Other fence type inside block is just content
                }
            }
        }
    }

    errors
}

/// Fix code block closer issues in a file
pub fn fix_file(path: &Path) -> Result<LintResult, std::io::Error> {
    let content = fs::read_to_string(path)?;
    let (fixed_content, errors_found) = fix_content(&content);

    if errors_found > 0 {
        fs::write(path, &fixed_content)?;
    }

    Ok(LintResult {
        file: path.to_path_buf(),
        errors: vec![], // After fix, no errors remain
        fixed: errors_found > 0,
    })
}

/// Fix code block closer issues in content
fn fix_content(content: &str) -> (String, usize) {
    let mut fixed_lines = Vec::new();
    let mut in_block = false;
    let mut block_fence: Option<&str> = None;
    let mut errors_fixed = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        // Check for backtick fence
        let is_backtick = trimmed.starts_with("```");
        // Check for tilde fence
        let is_tilde = trimmed.starts_with("~~~");

        if !in_block {
            // Not in a block - check for opening fence
            if is_backtick {
                in_block = true;
                block_fence = Some("```");
            } else if is_tilde {
                in_block = true;
                block_fence = Some("~~~");
            }
            fixed_lines.push(line.to_string());
        } else {
            // Inside a block
            match block_fence {
                Some("```") if is_backtick => {
                    // Closing backtick block
                    if trimmed != "```" {
                        // Fix: replace with just ```
                        let indent = line.len() - line.trim_start().len();
                        fixed_lines.push(format!("{}```", " ".repeat(indent)));
                        errors_fixed += 1;
                    } else {
                        fixed_lines.push(line.to_string());
                    }
                    in_block = false;
                    block_fence = None;
                }
                Some("~~~") if is_tilde => {
                    // Closing tilde block
                    if trimmed != "~~~" {
                        // Fix: replace with just ~~~
                        let indent = line.len() - line.trim_start().len();
                        fixed_lines.push(format!("{}~~~", " ".repeat(indent)));
                        errors_fixed += 1;
                    } else {
                        fixed_lines.push(line.to_string());
                    }
                    in_block = false;
                    block_fence = None;
                }
                _ => {
                    // Content inside block
                    fixed_lines.push(line.to_string());
                }
            }
        }
    }

    (fixed_lines.join("\n"), errors_fixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_valid_content() {
        let content = r#"# Test

```rust
let x = 1;
```

Some text.
"#;
        let errors = check_content(content);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_check_invalid_closer() {
        let content = r#"# Test

```text
some content
```text

More text.
"#;
        let errors = check_content(content);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("```text"));
    }

    #[test]
    fn test_fix_content() {
        let content = r#"# Test

```text
some content
```text

More text.
"#;
        let (fixed, count) = fix_content(content);
        assert_eq!(count, 1);
        assert!(fixed.contains("```\n\nMore"));
    }

    #[test]
    fn test_nested_fences() {
        // ~~~ block containing ``` example should not trigger error
        let content = r#"# Test

~~~
```text
content
```text
~~~

Done.
"#;
        let errors = check_content(content);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_lint_result_is_ok() {
        let result = LintResult {
            file: PathBuf::from("test.md"),
            errors: vec![],
            fixed: false,
        };
        assert!(result.is_ok());

        let result_with_error = LintResult {
            file: PathBuf::from("test.md"),
            errors: vec![LintError {
                line: 1,
                message: "test error".to_string(),
            }],
            fixed: false,
        };
        assert!(!result_with_error.is_ok());
    }

    #[test]
    fn test_check_file() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let content = "# Valid\n\n```rust\ncode\n```\n";
        let mut file = NamedTempFile::with_suffix(".md").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let result = check_file(file.path()).unwrap();
        assert!(result.is_ok());
        assert!(!result.fixed);
    }

    #[test]
    fn test_fix_file_with_error() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let content = "# Test\n\n```rust\ncode\n```text\n\nMore.\n";
        let mut file = NamedTempFile::with_suffix(".md").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let result = fix_file(file.path()).unwrap();
        assert!(result.fixed);
        assert!(result.errors.is_empty()); // After fix, no errors remain

        // Verify the file was actually fixed
        let fixed_content = std::fs::read_to_string(file.path()).unwrap();
        assert!(fixed_content.contains("```\n\nMore"));
    }

    #[test]
    fn test_fix_file_no_errors() {
        use std::io::Write;
        use tempfile::NamedTempFile;

        let content = "# Valid\n\n```rust\ncode\n```\n";
        let mut file = NamedTempFile::with_suffix(".md").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let result = fix_file(file.path()).unwrap();
        assert!(!result.fixed);
    }

    #[test]
    fn test_check_tilde_with_suffix() {
        // Test the ~~~suffix error case
        let content = r#"# Test

~~~bash
code
~~~sh

Done.
"#;
        let errors = check_content(content);
        assert_eq!(errors.len(), 1);
        assert!(errors[0].message.contains("~~~sh"));
    }

    #[test]
    fn test_fix_tilde_with_suffix() {
        // Test fixing ~~~ blocks with suffix
        let content = "# Test\n\n~~~bash\ncode\n~~~sh\n\nDone.\n";
        let (fixed, count) = fix_content(content);
        assert_eq!(count, 1);
        assert!(fixed.contains("~~~\n\nDone"));
    }

    #[test]
    fn test_fix_tilde_clean() {
        // Clean tilde block should not be modified
        let content = "# Test\n\n~~~\ncode\n~~~\n\nDone.\n";
        let (fixed, count) = fix_content(content);
        assert_eq!(count, 0);
        assert_eq!(fixed, content.trim_end());
    }

    #[test]
    fn test_tilde_block_matching() {
        // Tilde blocks should match correctly
        let content = r#"# Test

~~~rust
let x = 1;
~~~

Done.
"#;
        let errors = check_content(content);
        assert!(errors.is_empty());
    }
}
