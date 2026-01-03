//! Init command tests

use super::binary_path;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

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
