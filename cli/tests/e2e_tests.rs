//! End-to-end tests for asimov CLI
//!
//! These tests run the actual binary and verify CLI behavior.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Get the path to the asimov binary
fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release");
    path.push("asimov");

    if !path.exists() {
        // Fall back to debug build
        path.pop();
        path.pop();
        path.push("debug");
        path.push("asimov");
    }

    path
}

// ========== Help and Version Tests ==========

#[test]
fn e2e_help_shows_usage() {
    let output = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("asimov"), "Should show program name");
    assert!(stdout.contains("validate"), "Should show validate command");
    assert!(stdout.contains("init"), "Should show init command");
}

#[test]
fn e2e_version_shows_version() {
    let output = Command::new(binary_path())
        .arg("--version")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Version should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("asimov"), "Should show program name");
}

#[test]
fn e2e_short_help_works() {
    let output = Command::new(binary_path())
        .arg("-h")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "-h should succeed");
}

// ========== Validate Command Tests ==========
// v8.16.0: validate no longer takes path arguments or validates individual files
// It runs from current directory and validates:
// - Protocol .json files against hardcoded values
// - .asimov/roadmap.yaml
// - .asimov/project.yaml (if present)

#[test]
fn e2e_validate_directory() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // v8.16.0: validate now runs from current dir, validates .asimov/roadmap.yaml
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test milestone",
    )
    .unwrap();

    // v8.16.0: validate takes no path, runs from current directory
    let output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Directory validation should pass, stdout: {stdout}, stderr: {stderr}"
    );

    // Should validate roadmap.yaml
    assert!(stdout.contains("roadmap"), "Should validate roadmap");
}

#[test]
fn e2e_validate_directory_no_protocol_files() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    // No roadmap.yaml, just an empty .asimov dir

    // v8.16.0: validate takes no args, runs from current directory
    let output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // v8.16.0: Should show missing roadmap.yaml warning
    assert!(
        stdout.contains("missing") || stdout.contains("roadmap"),
        "Should mention roadmap, got: {stdout}"
    );
}

// ========== Init Command Tests ==========

#[test]
fn e2e_init_creates_roadmap() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_creates_roadmap_and_project() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // v8.2.0: Only roadmap.yaml + project.yaml created (protocols are hardcoded in binary)
    let asimov_dir = temp_dir.path().join(".asimov");
    assert!(
        asimov_dir.join("roadmap.yaml").exists(),
        "roadmap.yaml should exist in .asimov/"
    );
}

#[test]
fn e2e_init_type_rust() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("rust")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("rust-project")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init --type rust should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_generic() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(
        output.status.success(),
        "Init --type generic should succeed"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_invalid() {
    let temp_dir = TempDir::new().unwrap();

    // v8.16.0: Invalid types should fail with error message
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("invalid_type")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    // Should fail because invalid type is not recognized
    assert!(
        !output.status.success(),
        "Init should fail with invalid type"
    );

    // Error message should mention the invalid type
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("invalid_type") || stderr.contains("Unknown project type"),
        "Error should mention invalid type"
    );
}

#[test]
fn e2e_init_skips_existing_without_force() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: Create existing roadmap.yaml in .asimov/ directory
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    let roadmap_path = asimov_dir.join("roadmap.yaml");
    fs::write(&roadmap_path, "existing content").unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Init should succeed (with skip)");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // v8.2.0: roadmap.yaml shows KEEP (project data preserved)
    assert!(
        stdout.contains("KEEP") || stdout.contains("SKIP"),
        "Should skip/keep existing file, got: {stdout}"
    );

    // Original content should be preserved
    let content = fs::read_to_string(&roadmap_path).unwrap();
    assert_eq!(content, "existing content", "Should not overwrite");
}

#[test]
fn e2e_init_force_overwrites() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: Create existing roadmap.yaml in .asimov/ directory
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    let roadmap_path = asimov_dir.join("roadmap.yaml");
    fs::write(&roadmap_path, "existing content").unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("new-project")
        .arg("--type")
        .arg("generic")
        .arg("--force")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Init --force should succeed");

    // Content should be overwritten with new roadmap template
    let content = fs::read_to_string(&roadmap_path).unwrap();
    assert!(
        content.contains("current:") || content.contains("version:"),
        "Should overwrite with roadmap template content"
    );
}

#[test]
fn e2e_init_help_shows_type_option() {
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--help")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Init help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--type"), "Should show --type option");
    assert!(stdout.contains("generic"), "Should mention generic");
    assert!(stdout.contains("rust"), "Should mention rust");
}

#[test]
fn e2e_init_type_python() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("python")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("python-project")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init --type python should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_python_alias() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("py")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(
        output.status.success(),
        "Init --type py (alias) should succeed"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_node() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("node")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("node-project")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init --type node should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_node_aliases() {
    let temp_dir = TempDir::new().unwrap();

    // Test 'js' alias
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("js")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(
        output.status.success(),
        "Init --type js (alias) should succeed"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_go() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("go")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("go-project")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init --type go should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_type_go_alias() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("golang")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(
        output.status.success(),
        "Init --type golang (alias) should succeed"
    );

    // v8.0.0: Only roadmap.yaml is created (protocols are hardcoded in binary)
    let roadmap_path = temp_dir.path().join(".asimov").join("roadmap.yaml");
    assert!(
        roadmap_path.exists(),
        "roadmap.yaml should be created in .asimov/"
    );
}

#[test]
fn e2e_init_python_generated_files_pass_validation() {
    let temp_dir = TempDir::new().unwrap();

    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("python")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: validate runs from current directory
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute validate");

    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    let stderr = String::from_utf8_lossy(&validate_output.stderr);

    assert!(
        validate_output.status.success(),
        "Python generated files should pass validation, stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_init_node_generated_files_pass_validation() {
    let temp_dir = TempDir::new().unwrap();

    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("node")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: validate runs from current directory
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute validate");

    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    let stderr = String::from_utf8_lossy(&validate_output.stderr);

    assert!(
        validate_output.status.success(),
        "Node generated files should pass validation, stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_init_go_generated_files_pass_validation() {
    let temp_dir = TempDir::new().unwrap();

    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("go")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: validate runs from current directory
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute validate");

    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    let stderr = String::from_utf8_lossy(&validate_output.stderr);

    assert!(
        validate_output.status.success(),
        "Go generated files should pass validation, stdout: {stdout}, stderr: {stderr}"
    );
}

// ========== Generated Files Validation Tests ==========

#[test]
fn e2e_init_generated_files_pass_validation() {
    let temp_dir = TempDir::new().unwrap();

    // Generate all files
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: validate runs from current directory
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute validate");

    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    let stderr = String::from_utf8_lossy(&validate_output.stderr);

    assert!(
        validate_output.status.success(),
        "Generated files should pass validation, stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_init_rust_generated_files_pass_validation() {
    let temp_dir = TempDir::new().unwrap();

    // Generate Rust files
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("rust")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: validate runs from current directory
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute validate");

    let stdout = String::from_utf8_lossy(&validate_output.stdout);
    let stderr = String::from_utf8_lossy(&validate_output.stderr);

    assert!(
        validate_output.status.success(),
        "Rust generated files should pass validation, stdout: {stdout}, stderr: {stderr}"
    );
}

// ========== Real Protocol Files Tests ==========

#[test]
fn e2e_validate_repo_protocol_files() {
    // Validate the actual protocol files in this repo
    let mut repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    repo_root.pop(); // Go up from cli/ to repo root

    // v8.16.0: validate runs from current directory
    let output = Command::new(binary_path())
        .arg("validate")
        .current_dir(&repo_root)
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Repo protocol files should be valid, stdout: {stdout}, stderr: {stderr}"
    );
}

// ========== Lint-Docs Semantic Tests ==========

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

// ========== v8.14.0: Forge/External Path Tests ==========

#[test]
fn e2e_validate_external_path_creates_asimov_dir() {
    let temp_dir = TempDir::new().unwrap();

    // v8.16.0: validate runs from current dir
    let output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // v8.16.0: validate now shows protocol files status
    assert!(
        stdout.contains("PROTOCOL") || stdout.contains("ROADMAP"),
        "Should show validation output, got: {stdout}"
    );
}

#[test]
fn e2e_validate_external_path_no_regenerate() {
    // v8.16.0: --no-regenerate flag was removed, skip this test
    // validate now just validates, doesn't regenerate
}

#[test]
fn e2e_validate_external_path_with_existing_roadmap() {
    let temp_dir = TempDir::new().unwrap();

    // Create .asimov/roadmap.yaml manually
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    let roadmap_content = r#"current:
  version: "1.0.0"
  status: in_progress
  summary: "Test milestone"
"#;
    fs::write(asimov_dir.join("roadmap.yaml"), roadmap_content).unwrap();

    // v8.16.0: validate runs from current dir
    let output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Validate should succeed, stdout: {stdout}, stderr: {stderr}"
    );
    assert!(stdout.contains("roadmap"), "Should validate roadmap.yaml");
}

#[test]
fn e2e_validate_external_path_ethics_scan() {
    let temp_dir = TempDir::new().unwrap();

    // Create .asimov/roadmap.yaml
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test",
    )
    .unwrap();

    // Create a file that might trigger ethics scan
    fs::write(
        temp_dir.path().join("test.rs"),
        "// TODO: add crypto wallet",
    )
    .unwrap();

    // v8.16.0: validate runs from current directory
    let output = Command::new(binary_path())
        .arg("validate")
        .arg("--ethics-scan")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show ethics scan section
    assert!(
        stdout.contains("Ethics Scan") || stdout.contains("Red Flag"),
        "Should show ethics scan results"
    );
}

#[test]
fn e2e_init_output_creates_files_in_target() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-forge")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // All files should be in target directory
    let asimov_dir = temp_dir.path().join(".asimov");
    assert!(asimov_dir.exists(), ".asimov/ should be in target");
    assert!(
        asimov_dir.join("roadmap.yaml").exists(),
        "roadmap.yaml should be in target/.asimov/"
    );
    assert!(
        asimov_dir.join("project.yaml").exists(),
        "project.yaml should be in target/.asimov/"
    );

    // Claude hooks should be in target directory
    let claude_dir = temp_dir.path().join(".claude");
    assert!(claude_dir.exists(), ".claude/ should be in target");
    assert!(
        claude_dir.join("settings.json").exists(),
        "settings.json should be in target/.claude/"
    );
    assert!(
        claude_dir.join("hooks").join("session-start.sh").exists(),
        "session-start.sh should be in target/.claude/hooks/"
    );

    // .gitignore should be in target directory
    let gitignore = temp_dir.path().join(".gitignore");
    assert!(gitignore.exists(), ".gitignore should be in target");
}

#[test]
fn e2e_init_output_with_force_overwrites_target() {
    let temp_dir = TempDir::new().unwrap();

    // Create existing roadmap
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    fs::write(asimov_dir.join("roadmap.yaml"), "existing: content").unwrap();

    // Run init with --force
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--force")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Init --force should succeed");

    // Roadmap should be overwritten with template
    let content = fs::read_to_string(asimov_dir.join("roadmap.yaml")).unwrap();
    assert!(
        content.contains("current:"),
        "Should contain template content"
    );
}

#[test]
fn e2e_lint_docs_external_path() {
    let temp_dir = TempDir::new().unwrap();

    // Create markdown files in target
    let docs_dir = temp_dir.path().join("docs");
    fs::create_dir_all(&docs_dir).unwrap();
    fs::write(docs_dir.join("README.md"), "# Test\n\nValid content.\n").unwrap();
    fs::write(
        temp_dir.path().join("CHANGELOG.md"),
        "# Changelog\n\n## v1.0.0\n",
    )
    .unwrap();

    let output = Command::new(binary_path())
        .arg("lint-docs")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "lint-docs should succeed, stdout: {stdout}, stderr: {stderr}"
    );
    assert!(
        stdout.contains("2 markdown file"),
        "Should find 2 markdown files"
    );
}

#[test]
fn e2e_lint_docs_external_path_with_fix() {
    let temp_dir = TempDir::new().unwrap();

    // Create markdown file with fixable issues (unclosed code block)
    let test_md = temp_dir.path().join("test.md");
    fs::write(&test_md, "# Test\n\n```rust\nlet x = 1;\n").unwrap();

    let output = Command::new(binary_path())
        .arg("lint-docs")
        .arg("--fix")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show fixed files
    assert!(output.status.success(), "lint-docs --fix should succeed");
    assert!(
        stdout.contains("FIXED") || stdout.contains("Success"),
        "Should fix or succeed"
    );
}

#[test]
fn e2e_warmup_runs_from_project_dir() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize the project
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: Run warmup with --verbose for full output
    let output = Command::new(binary_path())
        .arg("warmup")
        .arg("--verbose")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Warmup should succeed when run from project dir, stdout: {stdout}, stderr: {stderr}"
    );
    assert!(
        stdout.contains("ROYALBIT ASIMOV"),
        "Should show warmup banner"
    );
    assert!(
        stdout.contains("PROTOCOLS"),
        "Should show protocols section"
    );
}

#[test]
fn e2e_warmup_simple_output() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize the project
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // v8.16.0: warmup without --verbose shows simple output
    let output = Command::new(binary_path())
        .arg("warmup")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Warmup should succeed, stdout: {stdout}, stderr: {stderr}"
    );
    // Simple output should show milestone info and JSON
    assert!(
        stdout.contains("MILESTONE") || stdout.contains("{"),
        "Should show milestone or JSON, got: {stdout}"
    );
}

#[test]
fn e2e_doctor_runs_from_project_dir() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize the project
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // Run doctor FROM the project directory
    let output = Command::new(binary_path())
        .arg("doctor")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Doctor should succeed when run from project dir, stdout: {stdout}, stderr: {stderr}"
    );
    assert!(
        stdout.contains("ASIMOV - DOCTOR"),
        "Should show doctor banner"
    );
    assert!(
        stdout.contains(".asimov/") || stdout.contains("directory exists"),
        "Should check .asimov directory"
    );
}

#[test]
fn e2e_refresh_runs_from_any_dir() {
    // v8.16.0: Refresh regenerates protocol .json files from hardcoded values
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("refresh")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Refresh should succeed, stdout: {stdout}, stderr: {stderr}"
    );
    assert!(
        stdout.contains("REFRESH") || stdout.contains("REGENERAT"),
        "Should show refresh/regeneration output, got: {stdout}"
    );
}

#[test]
fn e2e_validate_warns_on_missing_roadmap() {
    let temp_dir = TempDir::new().unwrap();

    // Create .asimov directory but no roadmap.yaml
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // v8.16.0: validate runs from current directory, warns about missing roadmap
    let output = Command::new(binary_path())
        .arg("validate")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should warn about missing roadmap (v8.16.0 validate doesn't regenerate)
    assert!(
        stdout.contains("missing") || stdout.contains("roadmap"),
        "Should mention missing roadmap, got: {stdout}"
    );
}
