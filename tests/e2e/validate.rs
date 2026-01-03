//! Validate command tests

use super::binary_path;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

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
