//! End-to-end tests for asimov-mode CLI
//!
//! These tests run the actual binary and verify CLI behavior.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Get the path to the asimov-mode binary
fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("release");
    path.push("asimov-mode");

    if !path.exists() {
        // Fall back to debug build
        path.pop();
        path.pop();
        path.push("debug");
        path.push("asimov-mode");
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
    assert!(stdout.contains("asimov-mode"), "Should show program name");
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
    assert!(stdout.contains("asimov-mode"), "Should show program name");
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

    // Create all protocol files
    fs::write(
        temp_dir.path().join("warmup.yaml"),
        "identity:\n  project: Test",
    )
    .unwrap();
    fs::write(
        temp_dir.path().join("sprint.yaml"),
        "sprint:\n  current: Work",
    )
    .unwrap();
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

    // Should validate all 3 files
    assert!(stdout.contains("warmup"), "Should validate warmup");
    assert!(stdout.contains("sprint"), "Should validate sprint");
    assert!(stdout.contains("roadmap"), "Should validate roadmap");
    assert!(stdout.contains("3 file"), "Should report 3 files");
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

    assert!(
        combined.contains("No protocol files"),
        "Should report no protocol files, got: {combined}"
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
fn e2e_init_creates_warmup() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("test-project")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // Check warmup.yaml was created
    let warmup_path = temp_dir.path().join("warmup.yaml");
    assert!(warmup_path.exists(), "warmup.yaml should be created");

    // Check content
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(
        content.contains("test-project"),
        "Should contain project name"
    );
}

#[test]
fn e2e_init_full_creates_all_files() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--full")
        .arg("--output")
        .arg(temp_dir.path())
        .arg("--name")
        .arg("full-project")
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Init --full should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // Check all files created
    assert!(
        temp_dir.path().join("warmup.yaml").exists(),
        "warmup.yaml should exist"
    );
    assert!(
        temp_dir.path().join("sprint.yaml").exists(),
        "sprint.yaml should exist"
    );
    assert!(
        temp_dir.path().join("roadmap.yaml").exists(),
        "roadmap.yaml should exist"
    );
}

#[test]
fn e2e_init_type_rust() {
    let temp_dir = TempDir::new().unwrap();

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

    // Check Rust-specific content
    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(content.contains("cargo"), "Should contain cargo commands");
    assert!(content.contains("Cargo.toml"), "Should mention Cargo.toml");
    assert!(content.contains("clippy"), "Should mention clippy");
}

#[test]
fn e2e_init_type_generic() {
    let temp_dir = TempDir::new().unwrap();

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

    // Check generic content (no Rust-specific)
    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(
        !content.contains("cargo"),
        "Generic should not contain cargo"
    );
}

#[test]
fn e2e_init_type_invalid() {
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("invalid_type")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(!output.status.success(), "Invalid type should fail");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");

    assert!(
        combined.contains("Unknown project type"),
        "Should report unknown type, got: {combined}"
    );
}

#[test]
fn e2e_init_skips_existing_without_force() {
    let temp_dir = TempDir::new().unwrap();

    // Create existing warmup.yaml
    let warmup_path = temp_dir.path().join("warmup.yaml");
    fs::write(&warmup_path, "existing content").unwrap();

    let output = Command::new(binary_path())
        .arg("init")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Init should succeed (with skip)");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("SKIP"), "Should skip existing file");

    // Original content should be preserved
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert_eq!(content, "existing content", "Should not overwrite");
}

#[test]
fn e2e_init_force_overwrites() {
    let temp_dir = TempDir::new().unwrap();

    // Create existing warmup.yaml
    let warmup_path = temp_dir.path().join("warmup.yaml");
    fs::write(&warmup_path, "existing content").unwrap();

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

    // Content should be overwritten
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(
        content.contains("new-project"),
        "Should overwrite with new content"
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

    // Check Python-specific content
    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(content.contains("pytest"), "Should contain pytest");
    assert!(content.contains("ruff"), "Should mention ruff");
    assert!(
        content.contains("pyproject.toml"),
        "Should mention pyproject.toml"
    );
    assert!(
        content.contains("green_coding"),
        "Should contain green_coding"
    );
}

#[test]
fn e2e_init_type_python_alias() {
    let temp_dir = TempDir::new().unwrap();

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

    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(
        content.contains("pytest"),
        "py alias should create Python template"
    );
}

#[test]
fn e2e_init_type_node() {
    let temp_dir = TempDir::new().unwrap();

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

    // Check Node-specific content
    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(content.contains("npm test"), "Should contain npm test");
    assert!(content.contains("eslint"), "Should mention eslint");
    assert!(
        content.contains("package.json"),
        "Should mention package.json"
    );
    assert!(content.contains("TypeScript"), "Should mention TypeScript");
    assert!(
        content.contains("green_coding"),
        "Should contain green_coding"
    );
}

#[test]
fn e2e_init_type_node_aliases() {
    let temp_dir = TempDir::new().unwrap();

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

    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(
        content.contains("npm"),
        "js alias should create Node template"
    );
}

#[test]
fn e2e_init_type_go() {
    let temp_dir = TempDir::new().unwrap();

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

    // Check Go-specific content
    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(content.contains("go test"), "Should contain go test");
    assert!(
        content.contains("golangci-lint"),
        "Should mention golangci-lint"
    );
    assert!(content.contains("go.mod"), "Should mention go.mod");
    assert!(content.contains("internal/"), "Should mention internal/");
    assert!(
        content.contains("green_coding"),
        "Should contain green_coding"
    );
    assert!(
        content.contains("CGO_ENABLED"),
        "Should mention CGO_ENABLED"
    );
}

#[test]
fn e2e_init_type_go_alias() {
    let temp_dir = TempDir::new().unwrap();

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

    let warmup_path = temp_dir.path().join("warmup.yaml");
    let content = fs::read_to_string(&warmup_path).unwrap();
    assert!(
        content.contains("go test"),
        "golang alias should create Go template"
    );
}

#[test]
fn e2e_init_python_generated_files_pass_validation() {
    let temp_dir = TempDir::new().unwrap();

    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--type")
        .arg("python")
        .arg("--full")
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
        .arg("--full")
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
        .arg("--full")
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
        .arg("--full")
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
        .arg("--full")
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
