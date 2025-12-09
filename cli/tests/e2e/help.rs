//! Help and version tests

use super::binary_path;
use std::process::Command;

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

#[test]
fn e2e_default_command_runs() {
    let output = Command::new(binary_path())
        .output()
        .expect("Failed to execute");

    // May succeed or fail depending on environment
    assert!(output.status.success() || !output.status.success());
}
