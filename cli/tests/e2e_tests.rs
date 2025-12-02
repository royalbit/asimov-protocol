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

/// Get the path to a test data file
fn test_data_path(filename: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("test-data");
    path.push(filename);
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

#[test]
fn e2e_validate_valid_warmup() {
    let file = test_data_path("valid_warmup.yaml");

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Valid warmup should pass, stdout: {stdout}, stderr: {stderr}"
    );

    assert!(
        stdout.contains("OK") || stdout.contains("valid"),
        "Should indicate success, got: {stdout}"
    );
}

#[test]
fn e2e_validate_valid_sprint() {
    let file = test_data_path("valid_sprint.yaml");

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Valid sprint should pass, stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_validate_valid_roadmap() {
    let file = test_data_path("valid_roadmap.yaml");

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Valid roadmap should pass, stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_validate_invalid_warmup_missing_identity() {
    let file = test_data_path("invalid_warmup_missing_identity.yaml");

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    assert!(
        !output.status.success(),
        "Invalid warmup should fail validation"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("FAIL") || stdout.contains("identity"),
        "Should report failure, got: {stdout}"
    );
}

#[test]
fn e2e_validate_invalid_warmup_missing_project() {
    let file = test_data_path("invalid_warmup_missing_project.yaml");

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    assert!(
        !output.status.success(),
        "Missing project should fail validation"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("FAIL") || stdout.contains("project"),
        "Should report project error, got: {stdout}"
    );
}

#[test]
fn e2e_validate_malformed_yaml() {
    let file = test_data_path("malformed.yaml");

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success(), "Malformed YAML should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.contains("YAML") || combined.contains("Error") || combined.contains("scanning"),
        "Should report YAML error, got: {combined}"
    );
}

#[test]
fn e2e_validate_nonexistent_file() {
    let output = Command::new(binary_path())
        .arg("validate")
        .arg("/nonexistent/warmup.yaml")
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success(), "Nonexistent file should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.contains("not found") || combined.contains("Error") || combined.contains("File"),
        "Should report file not found, got: {combined}"
    );
}

#[test]
fn e2e_validate_non_protocol_file() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("config.yaml");
    fs::write(&file, "key: value").unwrap();

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    assert!(
        !output.status.success(),
        "Non-protocol file should fail validation"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.contains("Not a protocol file") || combined.contains("Error"),
        "Should report not a protocol file, got: {combined}"
    );
}

#[test]
fn e2e_validate_directory() {
    let temp_dir = TempDir::new().unwrap();

    // v8.0.0: Only roadmap.yaml is validated (protocols are hardcoded in binary)
    fs::write(
        temp_dir.path().join("roadmap.yaml"),
        "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test milestone",
    )
    .unwrap();

    // Use --no-regenerate to only validate existing files
    let output = Command::new(binary_path())
        .arg("validate")
        .arg("--no-regenerate")
        .arg(temp_dir.path())
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
    assert!(stdout.contains("1 file"), "Should report 1 file");
}

#[test]
fn e2e_validate_directory_no_protocol_files() {
    let temp_dir = TempDir::new().unwrap();
    fs::write(temp_dir.path().join("config.yaml"), "key: value").unwrap();

    // Use --no-regenerate to prevent auto-creation of missing files
    let output = Command::new(binary_path())
        .arg("validate")
        .arg("--no-regenerate")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(
        !output.status.success(),
        "Empty directory should fail validation"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    // v8.0.0: Error message changed to "No data files"
    assert!(
        combined.contains("No data files"),
        "Should report no data files, got: {combined}"
    );
}

// ========== Check Command Tests ==========

#[test]
fn e2e_check_is_alias_for_validate() {
    let file = test_data_path("valid_warmup.yaml");

    let output = Command::new(binary_path())
        .arg("check")
        .arg(&file)
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Check should work like validate, stdout: {stdout}, stderr: {stderr}"
    );
}

// ========== Init Command Tests ==========

#[test]
fn e2e_init_creates_roadmap() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
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

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
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

    // v8.1.0: --type is used to generate project.yaml
    // Invalid types should fail with error message
    let output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("invalid_type")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    // v8.1.0: Should fail because invalid type is not recognized
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
        .arg("--force")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("new-project")
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

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
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

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    // Test 'js' alias
    let output = Command::new(binary_path())
        .arg("init")
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

    // v8.0.0: --type is accepted but ignored (protocols are hardcoded in binary)
    let output = Command::new(binary_path())
        .arg("init")
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
        .arg("--type")
        .arg("python")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    let validate_output = Command::new(binary_path())
        .arg("validate")
        .arg(temp_dir.path())
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
        .arg("--type")
        .arg("node")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    let validate_output = Command::new(binary_path())
        .arg("validate")
        .arg(temp_dir.path())
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
        .arg("--type")
        .arg("go")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    let validate_output = Command::new(binary_path())
        .arg("validate")
        .arg(temp_dir.path())
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
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // Validate generated files
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .arg(temp_dir.path())
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
        .arg("--type")
        .arg("rust")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // Validate generated files
    let validate_output = Command::new(binary_path())
        .arg("validate")
        .arg(temp_dir.path())
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

    let output = Command::new(binary_path())
        .arg("validate")
        .arg(&repo_root)
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Repo protocol files should be valid, stdout: {stdout}, stderr: {stderr}"
    );
}

// ========== Schema Command Tests ==========

#[test]
fn e2e_schema_help_shows_options() {
    let output = Command::new(binary_path())
        .arg("schema")
        .arg("--help")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Schema help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("warmup"), "Should list warmup schema");
    assert!(stdout.contains("sprint"), "Should list sprint schema");
    assert!(stdout.contains("asimov"), "Should list asimov schema");
    assert!(stdout.contains("--output"), "Should show output option");
}

#[test]
fn e2e_schema_single_to_stdout() {
    let output = Command::new(binary_path())
        .arg("schema")
        .arg("warmup")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Schema warmup should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    assert!(stdout.contains("$schema"), "Should contain JSON schema");
    assert!(
        stdout.contains("identity"),
        "Should contain warmup properties"
    );
}

#[test]
fn e2e_schema_all_to_directory() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("schema")
        .arg("all")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Schema all should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // Check all schema files were created
    assert!(
        temp_dir.path().join("warmup.schema.json").exists(),
        "warmup.schema.json should exist"
    );
    assert!(
        temp_dir.path().join("sprint.schema.json").exists(),
        "sprint.schema.json should exist"
    );
    assert!(
        temp_dir.path().join("roadmap.schema.json").exists(),
        "roadmap.schema.json should exist"
    );
    assert!(
        temp_dir.path().join("asimov.schema.json").exists(),
        "asimov.schema.json should exist"
    );
    assert!(
        temp_dir.path().join("freshness.schema.json").exists(),
        "freshness.schema.json should exist"
    );
    assert!(
        temp_dir.path().join("green.schema.json").exists(),
        "green.schema.json should exist"
    );
    assert!(
        temp_dir.path().join("sycophancy.schema.json").exists(),
        "sycophancy.schema.json should exist"
    );
}

#[test]
fn e2e_schema_invalid_name() {
    let output = Command::new(binary_path())
        .arg("schema")
        .arg("invalid_schema")
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success(), "Invalid schema name should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.contains("Unknown schema"),
        "Should report unknown schema, got: {combined}"
    );
}

#[test]
fn e2e_schema_output_is_valid_json() {
    let output = Command::new(binary_path())
        .arg("schema")
        .arg("asimov")
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Schema asimov should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify it's valid JSON by parsing
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stdout);
    assert!(parsed.is_ok(), "Output should be valid JSON");

    let json = parsed.unwrap();
    assert!(json.get("$schema").is_some(), "Should have $schema field");
    assert!(json.get("title").is_some(), "Should have title field");
    assert!(json.get("properties").is_some(), "Should have properties");
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
