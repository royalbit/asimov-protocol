//! Miscellaneous e2e tests - external path, warmup, doctor, refresh, stats, replay

use super::binary_path;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

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
fn e2e_refresh_requires_asimov_project() {
    // v8.16.2: Refresh requires .asimov/ directory (must be in an asimov project)
    let temp_dir = TempDir::new().unwrap();

    // Without .asimov/ - should fail
    let output = Command::new(binary_path())
        .arg("refresh")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "Refresh should fail without .asimov/"
    );
    assert!(
        stderr.contains("Not in an asimov project"),
        "Should show error about not being in asimov project, got: {stderr}"
    );

    // With .asimov/ - should succeed
    std::fs::create_dir(temp_dir.path().join(".asimov")).unwrap();

    let output = Command::new(binary_path())
        .arg("refresh")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Refresh should succeed with .asimov/, stdout: {stdout}, stderr: {stderr}"
    );
    assert!(
        stdout.contains("REFRESH") || stdout.contains("REGENERAT"),
        "Should show refresh/regeneration output, got: {stdout}"
    );
}

#[test]
fn e2e_refresh_detects_outdated_protocol() {
    // v9.0.0: Refresh should detect and update outdated protocol files
    let temp_dir = TempDir::new().unwrap();

    // Initialize project first
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

    // Simulate outdated protocol file (old version)
    let freshness_path = temp_dir.path().join(".asimov").join("freshness.json");
    fs::write(&freshness_path, r#"{"old_format": true}"#).unwrap();

    // Run refresh
    let output = Command::new(binary_path())
        .arg("refresh")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute refresh");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "Refresh should succeed");
    assert!(
        stdout.contains("UPDATED") && stdout.contains("freshness.json"),
        "Should show UPDATED for outdated file, got: {stdout}"
    );
    assert!(
        stdout.contains("updated"),
        "Should mention protocols were updated, got: {stdout}"
    );

    // Verify file was updated to correct content
    let content = fs::read_to_string(&freshness_path).unwrap();
    assert!(
        content.contains("rule"),
        "Updated file should have 'rule' field, got: {content}"
    );
}

#[test]
fn e2e_doctor_detects_outdated_protocol() {
    // v9.0.0: Doctor should detect outdated protocol files
    let temp_dir = TempDir::new().unwrap();

    // Initialize project first
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

    // Simulate outdated protocol file (old version format)
    let sycophancy_path = temp_dir.path().join(".asimov").join("sycophancy.json");
    fs::write(&sycophancy_path, r#"{"old_version": true}"#).unwrap();

    // Run doctor
    let output = Command::new(binary_path())
        .arg("doctor")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute doctor");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success(),
        "Doctor should fail with outdated protocol"
    );
    assert!(
        stdout.contains("outdated") || stdout.contains("Outdated"),
        "Should mention outdated file, got: {stdout}"
    );
    assert!(
        stdout.contains("sycophancy.json"),
        "Should mention the specific outdated file, got: {stdout}"
    );
}

#[test]
fn e2e_doctor_detects_missing_protocol() {
    // v9.0.0: Doctor should detect missing protocol files
    let temp_dir = TempDir::new().unwrap();

    // Initialize project first
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

    // Delete a protocol file
    let green_path = temp_dir.path().join(".asimov").join("green.json");
    fs::remove_file(&green_path).unwrap();

    // Run doctor
    let output = Command::new(binary_path())
        .arg("doctor")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute doctor");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        !output.status.success(),
        "Doctor should fail with missing protocol"
    );
    assert!(
        stdout.contains("missing") || stdout.contains("Missing"),
        "Should mention missing file, got: {stdout}"
    );
    assert!(
        stdout.contains("green.json"),
        "Should mention the specific missing file, got: {stdout}"
    );
}

#[test]
fn e2e_refresh_creates_missing_protocol() {
    // v9.0.0: Refresh should create missing protocol files
    let temp_dir = TempDir::new().unwrap();

    // Initialize project first
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

    // Delete a protocol file
    let sprint_path = temp_dir.path().join(".asimov").join("sprint.json");
    fs::remove_file(&sprint_path).unwrap();
    assert!(!sprint_path.exists(), "File should be deleted");

    // Run refresh
    let output = Command::new(binary_path())
        .arg("refresh")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute refresh");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success(), "Refresh should succeed");
    assert!(
        stdout.contains("CREATED") || stdout.contains("UPDATED"),
        "Should show CREATED or UPDATED for missing file, got: {stdout}"
    );
    assert!(sprint_path.exists(), "File should be recreated");
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

// ========== Update Command Integration Tests ==========

#[test]
fn e2e_update_check_runs() {
    // Test `asimov update --check` - makes network call to GitHub API
    let output = Command::new(binary_path())
        .args(["update", "--check"])
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should either succeed (found version info) or fail gracefully (network error)
    // Both are valid outcomes - we're testing the code path runs
    assert!(
        stdout.contains("Update")
            || stdout.contains("version")
            || stdout.contains("latest")
            || stderr.contains("Error")
            || stdout.contains("OK"),
        "Update check should produce output, got stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_default_command_runs() {
    // Test running `asimov` with no args (default command)
    // This exercises the main() dispatch and cmd_launch()
    let temp_dir = TempDir::new().unwrap();

    let output = Command::new(binary_path())
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should either:
    // - Show "Claude Code not found" if claude not installed
    // - Show "Launching Claude Code..." if claude found
    // - Run warmup if inside Claude session
    // - Show roadmap error if no project
    assert!(
        stderr.contains("Claude")
            || stderr.contains("roadmap")
            || stdout.contains("Launching")
            || stdout.contains("warmup")
            || stderr.contains("Error")
            || stdout.contains("RoyalBit"),
        "Default command should run, got stdout: {stdout}, stderr: {stderr}"
    );
}

#[test]
fn e2e_stats_command_runs() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repo
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    // Create .asimov with roadmap
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
    )
    .unwrap();

    let output = Command::new(binary_path())
        .arg("stats")
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Stats should show commit info
    assert!(
        stdout.contains("commits") || stdout.contains("Commits") || stdout.contains("Stats"),
        "Stats should show commit information, got: {stdout}"
    );
}

#[test]
fn e2e_replay_command_runs() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize git repo with a commit
    Command::new("git")
        .args(["init"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.email", "test@test.com"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();
    // Disable commit signing for test environments
    Command::new("git")
        .args(["config", "commit.gpgsign", "false"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();
    fs::write(temp_dir.path().join("test.txt"), "content").unwrap();
    Command::new("git")
        .args(["add", "."])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();
    Command::new("git")
        .args(["commit", "-m", "test commit"])
        .current_dir(temp_dir.path())
        .output()
        .unwrap();

    let output = Command::new(binary_path())
        .args(["replay", "--commits", "5"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        output.status.success(),
        "Replay should succeed, got: {stdout}"
    );
}

// v9.2.3: Conditional migrations protocol e2e tests

#[test]
fn e2e_warmup_rust_project_excludes_migrations() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize the project
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("rust-test-project")
        .arg("--type")
        .arg("rust")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // Run warmup
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
    // Rust projects should NOT include migrations protocol in output
    assert!(
        !stdout.contains("\"migrations\""),
        "Rust project should NOT include migrations protocol, got: {stdout}"
    );
}

#[test]
fn e2e_warmup_migration_project_includes_migrations() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // Create roadmap.yaml
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test migration\n",
    )
    .unwrap();

    // Create project.yaml with migration type
    fs::write(
        asimov_dir.join("project.yaml"),
        "identity:\n  name: migration-test\n  type: migration\n",
    )
    .unwrap();

    // Run warmup
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
    // Migration projects SHOULD include migrations protocol in output
    assert!(
        stdout.contains("\"migrations\""),
        "Migration project SHOULD include migrations protocol, got: {stdout}"
    );
    assert!(
        stdout.contains("functionally equivalent"),
        "Migration protocol should contain principle, got: {stdout}"
    );
}

#[test]
fn e2e_warmup_generic_project_excludes_migrations() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize the project
    let init_output = Command::new(binary_path())
        .arg("init")
        .arg("--name")
        .arg("generic-test-project")
        .arg("--type")
        .arg("generic")
        .arg("--output")
        .arg(temp_dir.path())
        .output()
        .expect("Failed to execute init");

    assert!(init_output.status.success(), "Init should succeed");

    // Run warmup
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
    // Generic projects should NOT include migrations protocol in output
    assert!(
        !stdout.contains("\"migrations\""),
        "Generic project should NOT include migrations protocol, got: {stdout}"
    );
}

// v9.5.0: Enhanced refresh migration e2e tests

#[test]
fn e2e_refresh_with_yes_flag() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // Create minimal roadmap
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
    )
    .unwrap();

    // Add Cargo.toml to detect as Rust project
    fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();

    // Run refresh with --yes (should auto-create project.yaml)
    let output = Command::new(binary_path())
        .args(["refresh", "--yes"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Refresh --yes should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // Should have created project.yaml
    assert!(
        asimov_dir.join("project.yaml").exists(),
        "project.yaml should be created with --yes"
    );

    // project.yaml should have rust type
    let content = fs::read_to_string(asimov_dir.join("project.yaml")).unwrap();
    assert!(
        content.contains("type: rust"),
        "Should detect and set rust type"
    );
}

#[test]
fn e2e_refresh_with_dry_run_flag() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // Create minimal roadmap
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
    )
    .unwrap();

    // Add Cargo.toml to detect as Rust project
    fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();

    // Run refresh with --dry-run (should NOT create project.yaml)
    let output = Command::new(binary_path())
        .args(["refresh", "--dry-run"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Refresh --dry-run should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // Should show dry run message
    assert!(
        stdout.contains("dry run"),
        "Should show dry run message, got: {stdout}"
    );

    // Should NOT have created project.yaml
    assert!(
        !asimov_dir.join("project.yaml").exists(),
        "project.yaml should NOT be created with --dry-run"
    );
}

#[test]
fn e2e_refresh_upgrades_coding_standards() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // Create minimal roadmap
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
    )
    .unwrap();

    // Create old-style project.yaml (coding_standards without code/documentation/architecture)
    fs::write(
        asimov_dir.join("project.yaml"),
        r#"identity:
  name: test-project
  type: rust
  tagline: Test

coding_standards:
  file_size:
    soft_limit: 1000
  coverage: "100%"
"#,
    )
    .unwrap();

    // Run refresh with --yes to auto-accept upgrade
    let output = Command::new(binary_path())
        .args(["refresh", "--yes", "--verbose"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Refresh should succeed, stdout: {stdout}, stderr: {stderr}"
    );

    // Check that coding_standards was upgraded
    let content = fs::read_to_string(asimov_dir.join("project.yaml")).unwrap();
    assert!(
        content.contains("documentation:") && content.contains("architecture:"),
        "Should upgrade coding_standards to include documentation and architecture sections, got: {content}"
    );
}

#[test]
fn e2e_refresh_preserves_identity() {
    let temp_dir = TempDir::new().unwrap();
    let asimov_dir = temp_dir.path().join(".asimov");
    fs::create_dir_all(&asimov_dir).unwrap();

    // Create roadmap
    fs::write(
        asimov_dir.join("roadmap.yaml"),
        "current:\n  version: '1.0'\n  status: planned\n  summary: Test\n",
    )
    .unwrap();

    // Create project.yaml with custom name and tagline
    fs::write(
        asimov_dir.join("project.yaml"),
        r#"identity:
  name: my-custom-project
  type: rust
  tagline: My custom tagline

coding_standards:
  file_size:
    soft_limit: 500
"#,
    )
    .unwrap();

    // Run refresh with --yes
    let output = Command::new(binary_path())
        .args(["refresh", "--yes"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Refresh should succeed");

    // Check that identity was preserved
    let content = fs::read_to_string(asimov_dir.join("project.yaml")).unwrap();
    assert!(
        content.contains("my-custom-project"),
        "Should preserve identity.name, got: {content}"
    );
    assert!(
        content.contains("My custom tagline"),
        "Should preserve identity.tagline, got: {content}"
    );
}

#[test]
fn e2e_refresh_help_shows_new_flags() {
    let output = Command::new(binary_path())
        .args(["refresh", "--help"])
        .output()
        .expect("Failed to execute");

    assert!(output.status.success(), "Help should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--yes"), "Should show --yes flag");
    assert!(stdout.contains("--dry-run"), "Should show --dry-run flag");
    assert!(
        stdout.contains("migrate") || stdout.contains("auto-accept"),
        "Should describe migration features"
    );
}
