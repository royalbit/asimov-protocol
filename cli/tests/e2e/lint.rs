//! Lint-docs tests

use super::binary_path;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn e2e_lint_docs_semantic_flag_help() {
    let output = Command::new(binary_path())
        .arg("lint-docs")
        .arg("--help")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "lint-docs --help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("--semantic"),
        "Help should show --semantic flag"
    );
    assert!(
        stdout.contains("version consistency"),
        "Should describe semantic checks"
    );
}

#[test]
fn e2e_lint_docs_semantic_runs() {
    let temp_dir = TempDir::new().unwrap();

    // Create a test markdown file
    let test_md = temp_dir.path().join("test.md");
    fs::write(&test_md, "# Test\n\nVersion: 1.0.0\n").unwrap();

    let output = Command::new(binary_path())
        .arg("lint-docs")
        .arg("--semantic")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should run and show semantic checks section
    assert!(
        stdout.contains("Semantic Checks"),
        "Should show semantic checks section, stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_lint_docs_semantic_detects_deprecated() {
    let temp_dir = TempDir::new().unwrap();

    // Create .asimov directory with deprecated.yaml
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // Create deprecated patterns config
    let deprecated_config = r#"
deprecated:
  - pattern: "old-pattern"
    replacement: "new-pattern"
    reason: "test deprecation"
"#;
    fs::write(asimov_dir.join("deprecated.yaml"), deprecated_config).unwrap();

    // Create a markdown file that uses the deprecated pattern
    let test_md = temp_dir.path().join("docs.md");
    fs::write(&test_md, "# Docs\n\nUse old-pattern for this.\n").unwrap();

    let output = Command::new(binary_path())
        .arg("lint-docs")
        .arg("--semantic")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect the deprecated pattern
    assert!(
        stdout.contains("old-pattern") || stdout.contains("deprecated"),
        "Should detect deprecated pattern, stdout: {stdout}"
    );
}

#[test]
fn e2e_lint_docs_without_semantic_skips_checks() {
    let temp_dir = TempDir::new().unwrap();

    // Create a test markdown file
    let test_md = temp_dir.path().join("test.md");
    fs::write(&test_md, "# Test\n\nSome content.\n").unwrap();

    let output = Command::new(binary_path())
        .arg("lint-docs")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Without --semantic, should NOT show semantic checks
    assert!(
        !stdout.contains("Semantic Checks"),
        "Should NOT show semantic checks without flag"
    );
}
