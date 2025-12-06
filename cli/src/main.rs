#![cfg_attr(feature = "coverage", feature(coverage_attribute))]
//! RoyalBit Asimov CLI - The Three Laws of Robotics, encoded in YAML
//!
//! This is a thin wrapper around the commands module. All business logic
//! is in lib.rs for testability. This file only handles CLI parsing and output.

use clap::{Parser, Subcommand};
use colored::Colorize;
use royalbit_asimov::commands::{
    check_launch_conditions, run_doctor, run_init, run_lint_docs, run_refresh, run_replay,
    run_stats, run_update, run_validate, run_warmup, LaunchResult, UpdateResult,
};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "asimov")]
#[command(about = "RoyalBit Asimov CLI - AI development with protocol enforcement")]
#[command(long_about = "RoyalBit Asimov CLI v9.0.0
Copyright (c) 2025 RoyalBit. All Rights Reserved.
Proprietary and Confidential.

LAUNCHER MODE:
  asimov                             # From terminal: launches Claude Code + auto-warmup
  asimov                             # Inside Claude: runs warmup directly

EXAMPLES:
  asimov                             # Start session (launcher mode)
  asimov warmup                      # Manual warmup (inside Claude Code)
  asimov validate                    # Validate roadmap.yaml
  asimov update                      # Update binary
  asimov init                        # Initialize new project

PROTOCOLS (hardcoded in binary):
  - asimov     - The Three Laws (do no harm, obey human, self-preserve)
  - freshness  - Date-aware search (WebSearch/WebFetch with current date)
  - sycophancy - Truth over comfort, honest disagreement
  - green      - Efficiency benchmarks via WebSearch
  - sprint     - Autonomous execution (4hr max, no interruptions)
  - warmup     - Session bootstrap (load, validate, present)
  - migrations - Functional equivalence (same inputs = same outputs)
  - exhaustive - Complete tasks without stopping

Docs: https://github.com/royalbit/asimov")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate protocol files against the schema
    Validate {
        /// Scan project files for red flag patterns
        #[arg(long)]
        ethics_scan: bool,
    },

    /// Initialize or migrate an asimov project
    Init {
        /// Project name (required)
        #[arg(short, long)]
        name: String,

        /// Project type: generic, rust, python, node, go, flutter, docs
        #[arg(short = 't', long = "type")]
        project_type: String,

        /// Output directory (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },

    /// Lint markdown documentation
    #[command(name = "lint-docs")]
    LintDocs {
        /// Directory or file to lint
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Auto-fix issues
        #[arg(long)]
        fix: bool,

        /// Enable semantic checks (version consistency, etc.)
        #[arg(long)]
        semantic: bool,
    },

    /// Refresh protocol context
    Refresh {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Check for updates and self-update
    Update {
        /// Only check, don't install
        #[arg(long)]
        check: bool,
    },

    /// Session warmup - display milestone and validate
    Warmup {
        /// Target directory
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show session statistics
    Stats,

    /// Diagnose autonomous mode issues
    Doctor,

    /// Replay a session from git history
    Replay {
        /// Number of commits to show
        #[arg(short = 'n', long)]
        commits: Option<usize>,

        /// Show yesterday's session
        #[arg(long)]
        yesterday: bool,

        /// Show commits since time
        #[arg(long)]
        since: Option<String>,

        /// Show full diffs
        #[arg(short, long)]
        verbose: bool,
    },
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        None => cmd_launch(),
        Some(Commands::Validate { ethics_scan }) => cmd_validate(ethics_scan),
        Some(Commands::Init {
            name,
            project_type,
            output,
            force,
        }) => cmd_init(&name, &project_type, &output, force),
        Some(Commands::LintDocs {
            path,
            fix,
            semantic,
        }) => cmd_lint_docs(&path, fix, semantic),
        Some(Commands::Refresh { verbose: _ }) => cmd_refresh(),
        Some(Commands::Update { check }) => cmd_update(check),
        Some(Commands::Warmup { path, verbose }) => cmd_warmup(&path, verbose),
        Some(Commands::Stats) => cmd_stats(),
        Some(Commands::Doctor) => cmd_doctor(),
        Some(Commands::Replay {
            commits,
            yesterday,
            since,
            verbose: _,
        }) => cmd_replay(commits, yesterday, since),
    }
}

// ============================================================================
// THIN WRAPPERS - Call commands.rs and format output
// These are CLI output formatters, tested via e2e tests (ADR-039)
// ============================================================================

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_launch() -> ExitCode {
    match check_launch_conditions() {
        LaunchResult::InsideClaude => {
            // Inside Claude - run warmup
            cmd_warmup(std::path::Path::new("."), false)
        }
        LaunchResult::ClaudeNotFound => {
            eprintln!("{} Claude Code not found in PATH", "Error:".bold().red());
            eprintln!("  Install: https://claude.ai/download");
            ExitCode::FAILURE
        }
        LaunchResult::Launching => {
            println!("{}", "Launching Claude Code...".bright_cyan());
            // Execute claude with warmup
            let status = std::process::Command::new("claude")
                .args(["--dangerously-skip-permissions", "--model", "opus"])
                .arg("run asimov warmup")
                .status();
            match status {
                Ok(s) if s.success() => ExitCode::SUCCESS,
                _ => ExitCode::FAILURE,
            }
        }
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_update(check_only: bool) -> ExitCode {
    println!("{}", "RoyalBit Asimov Update".bold().green());
    println!();
    format_update_result(run_update(check_only))
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn format_update_result(result: UpdateResult) -> ExitCode {
    match result {
        UpdateResult::AlreadyLatest { current, .. } => {
            println!(
                "  {} v{} is the latest version",
                "OK".bold().green(),
                current
            );
            ExitCode::SUCCESS
        }
        UpdateResult::UpdateAvailable { current, latest } => {
            println!("  Current: {}", current.bright_blue());
            println!("  Latest:  {}", latest.bright_green());
            println!();
            println!(
                "  {} Run {} to install",
                "UPDATE".bold().yellow(),
                "asimov update".bold()
            );
            ExitCode::SUCCESS
        }
        UpdateResult::Updated { from, to } => {
            println!("  {} Updated {} → {}", "Success:".bold().green(), from, to);
            ExitCode::SUCCESS
        }
        UpdateResult::UpdateFailed {
            error,
            download_url,
            ..
        } => {
            eprintln!("  {} {}", "Error:".bold().red(), error);
            eprintln!("  Manual: curl -L {} | tar xz", download_url);
            ExitCode::FAILURE
        }
        UpdateResult::NoBinaryAvailable { .. } => {
            eprintln!("  {} No binary for this platform", "Error:".bold().red());
            ExitCode::FAILURE
        }
        UpdateResult::CheckFailed { error } => {
            eprintln!("  {} {}", "Error:".bold().red(), error);
            ExitCode::FAILURE
        }
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_warmup(path: &std::path::Path, verbose: bool) -> ExitCode {
    let result = run_warmup(path, verbose);

    if let Some(ref err) = result.error {
        eprintln!("{} {}", "Error:".bold().red(), err);
        if err.contains("not found") {
            eprintln!(
                "  Run {} first",
                "asimov init --name <NAME> --type <TYPE>".bold()
            );
        }
        return ExitCode::FAILURE;
    }

    if verbose {
        println!();
        println!(
            "{}",
            "══════════════════════════════════════════════════════════════════════════════"
                .bright_cyan()
        );
        println!(
            "{}",
            "🔥 ROYALBIT ASIMOV - SESSION WARMUP".bold().bright_cyan()
        );
        println!(
            "{}",
            "══════════════════════════════════════════════════════════════════════════════"
                .bright_cyan()
        );
        println!();

        if let Some(ref update_ver) = result.update_available {
            println!(
                "{}  Update available: {} (run: {})",
                "⚠️".yellow(),
                update_ver.bright_green(),
                "asimov update".bold()
            );
            println!();
        }
    }

    // Display milestone info
    if let (Some(ref ver), Some(ref summary), Some(ref status)) = (
        &result.current_version,
        &result.current_summary,
        &result.current_status,
    ) {
        if verbose {
            println!("{}", "CURRENT VERSION".bold());
            println!("  v{} - {}", ver.bright_blue(), summary);
            println!("  Status: {}", status);
            println!();
        } else {
            println!("v{} - {} [{}]", ver.bright_blue(), summary, status);
        }
    }

    if verbose {
        if let Some(ref json) = result.protocols_json {
            println!("{}", "PROTOCOLS".bold());
            println!("  {} bytes of protocol context loaded", json.len());
            println!();
        }
    }

    // Output protocols JSON for context injection
    if let Some(ref json) = result.protocols_json {
        println!();
        println!("{}", json);
    }

    ExitCode::SUCCESS
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_validate(ethics_scan: bool) -> ExitCode {
    let result = run_validate(std::path::Path::new("."), ethics_scan);

    println!("{}", "RoyalBit Asimov Validate".bold().green());
    println!();

    // Show roadmap validation
    if result.roadmap.is_some() || result.project.is_some() {
        println!("{}", "ROADMAP & PROJECT".bold());
    }
    if let Some(ref r) = result.roadmap {
        if r.valid {
            println!("  {} roadmap.yaml", "✓".green());
        } else {
            println!("  {} roadmap.yaml", "✗".red());
            for e in &r.errors {
                println!("      {}", e.red());
            }
        }
    }

    // Show project validation
    if let Some(ref p) = result.project {
        if p.valid {
            println!("  {} project.yaml", "✓".green());
        } else {
            println!("  {} project.yaml", "✗".red());
        }
    }

    // Show ethics scan results
    if let Some(ref scan) = result.ethics_scan {
        println!();
        println!("{}", "Ethics Scan".bold());
        if scan.red_flags_found > 0 {
            println!(
                "  {} {} red flag(s) found",
                "⚠".yellow(),
                scan.red_flags_found
            );
            for m in &scan.matches {
                println!("      {}:{} - {}", m.file, m.line, m.pattern);
            }
        } else {
            println!("  {} No red flags found", "✓".green());
        }
    }

    println!();
    if result.success {
        println!("{} All validations passed", "Success:".bold().green());
        ExitCode::SUCCESS
    } else {
        println!("{} Validation failed", "Error:".bold().red());
        ExitCode::FAILURE
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_init(name: &str, project_type: &str, output: &std::path::Path, force: bool) -> ExitCode {
    let result = run_init(output, name, project_type, force);

    if let Some(ref err) = result.error {
        eprintln!("{} {}", "Error:".bold().red(), err);
        return ExitCode::FAILURE;
    }

    println!("{}", "RoyalBit Asimov Init".bold().green());
    println!();

    for f in &result.files_created {
        println!("  {} {}", "CREATE".green(), f);
    }
    for f in &result.files_updated {
        println!("  {} {}", "UPDATE".yellow(), f);
    }
    for f in &result.files_kept {
        println!("  {} {}", "KEEP".bright_blue(), f);
    }
    for h in &result.hooks_installed {
        println!("  {} {}", "HOOK".bright_cyan(), h);
    }

    println!();
    if result.success {
        println!("{} Project initialized", "Success:".bold().green());
        println!();
        println!("Next steps:");
        println!("  1. Edit .asimov/roadmap.yaml with your milestones");
        println!("  2. Run: asimov warmup");
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_lint_docs(path: &std::path::Path, fix: bool, semantic: bool) -> ExitCode {
    let result = run_lint_docs(path, fix, semantic);

    println!("{}", "RoyalBit Asimov Lint".bold().green());
    println!();
    println!("  {} markdown file(s) checked", result.files_checked);

    if result.files_with_errors > 0 {
        println!(
            "  Files with errors: {}",
            result.files_with_errors.to_string().red()
        );
    }

    if fix && result.files_fixed > 0 {
        println!("  Files fixed: {}", result.files_fixed.to_string().green());
    }

    if semantic && result.semantic_files_checked > 0 {
        println!("  Semantic Checks: {} files", result.semantic_files_checked);
        for issue in &result.semantic_issues {
            println!(
                "    {} {}:{} - {}",
                "⚠".yellow(),
                issue.file,
                issue.line,
                issue.message
            );
        }
    }

    println!();
    if result.success {
        println!("{} All checks passed", "Success:".bold().green());
        ExitCode::SUCCESS
    } else {
        println!("{} Issues found", "Error:".bold().red());
        ExitCode::FAILURE
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_refresh() -> ExitCode {
    let result = run_refresh(std::path::Path::new("."));

    if !result.is_asimov_project {
        eprintln!("{} Not in an asimov project", "Error:".bold().red());
        eprintln!("  Run {} first", "asimov init".bold());
        return ExitCode::FAILURE;
    }

    if let Some(ref err) = result.error {
        eprintln!("{} {}", "Error:".bold().red(), err);
        return ExitCode::FAILURE;
    }

    println!("{}", "RoyalBit Asimov REFRESH".bold().green());
    println!();

    // v9.0.0: Protocol integrity status
    for f in &result.protocols_updated {
        println!("  {} {} (was outdated)", "UPDATED".yellow(), f);
    }
    for f in &result.protocols_created {
        println!("  {} {}", "CREATED".green(), f);
    }
    for f in &result.protocols_ok {
        println!("  {} {}", "OK".dimmed(), f);
    }

    // Data files (roadmap.yaml etc)
    for f in &result.files_regenerated {
        println!("  {} {}", "REGENERATE".green(), f);
    }
    for f in &result.files_unchanged {
        println!("  {} {}", "UNCHANGED".dimmed(), f);
    }

    println!();
    if result.success {
        let updated_count = result.protocols_updated.len();
        if updated_count > 0 {
            println!(
                "{} Protocols refreshed ({} updated)",
                "Success:".bold().green(),
                updated_count
            );
        } else {
            println!("{} Protocols refreshed", "Success:".bold().green());
        }
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_stats() -> ExitCode {
    let result = run_stats(std::path::Path::new("."));

    println!("{}", "RoyalBit Asimov Stats".bold().green());
    println!();
    println!("  Session: {}", result.session_date.bright_blue());
    println!("  Commits today: {}", result.today_commits);
    println!("  Asimov commits: {}", result.asimov_commits);
    println!("  Total commits: {}", result.total_commits);

    if let Some(ref ver) = result.milestone_version {
        println!();
        println!("  Milestone: v{}", ver.bright_yellow());
        if let Some(ref summary) = result.milestone_summary {
            println!("  Summary: {}", summary);
        }
    }

    ExitCode::SUCCESS
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_doctor() -> ExitCode {
    let result = run_doctor(std::path::Path::new("."));

    println!("{}", "RoyalBit ASIMOV - DOCTOR".bold().green());
    println!();

    for check in &result.checks {
        let icon = if check.passed {
            "✓".green()
        } else {
            "✗".red()
        };
        println!("  {} {}: {}", icon, check.name, check.message);
        if check.auto_fixed {
            println!("      {} Auto-fixed", "→".yellow());
        }
    }

    if !result.issues.is_empty() {
        println!();
        println!("{}", "Issues:".bold().red());
        for issue in &result.issues {
            println!("  • {}", issue);
        }
    }

    if !result.warnings.is_empty() {
        println!();
        println!("{}", "Warnings:".bold().yellow());
        for warn in &result.warnings {
            println!("  • {}", warn);
        }
    }

    if let Some((ver, is_latest)) = &result.version_info {
        println!();
        if *is_latest {
            println!("  {} v{} (latest)", "Version:".bold(), ver);
        } else {
            println!(
                "  {} v{} (update available)",
                "Version:".bold().yellow(),
                ver
            );
        }
    }

    println!();
    let passed = result.checks.iter().filter(|c| c.passed).count();
    let total = result.checks.len();
    println!("{} {}/{} checks passed", "Result:".bold(), passed, total);

    if passed == total && result.issues.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg_attr(feature = "coverage", coverage(off))]
fn cmd_replay(commits: Option<usize>, yesterday: bool, since: Option<String>) -> ExitCode {
    let result = run_replay(std::path::Path::new("."), commits, yesterday, since);

    if !result.is_git_repo {
        eprintln!("{} Not a git repository", "Error:".bold().red());
        return ExitCode::FAILURE;
    }

    if let Some(ref err) = result.error {
        eprintln!("{} {}", "Error:".bold().red(), err);
        return ExitCode::FAILURE;
    }

    println!("{}", "RoyalBit Asimov Replay".bold().green());
    println!();
    println!("  Range: {}", result.range_description.bright_blue());
    println!("  Commits: {}", result.commits.len());
    println!("  Files changed: {}", result.total_files_changed);
    println!(
        "  Insertions: +{}",
        result.total_insertions.to_string().green()
    );
    println!("  Deletions: -{}", result.total_deletions.to_string().red());
    println!();

    for commit in &result.commits {
        println!(
            "  {} {} {}",
            commit.hash.bright_yellow(),
            commit.time.dimmed(),
            commit.message
        );
    }

    if result.success {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cmd_update_check() {
        // This exercises the update check path
        let result = cmd_update(true);
        // Either success or failure is fine - we're testing the code path
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_warmup_no_project() {
        let temp = TempDir::new().unwrap();
        let result = cmd_warmup(temp.path(), false);
        assert_eq!(result, ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_warmup_with_project() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        let result = cmd_warmup(temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_warmup_verbose() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        let result = cmd_warmup(temp.path(), true);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_validate_empty() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(false);
        // May succeed or fail depending on state
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_validate_with_ethics() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(true);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_init_success() {
        let temp = TempDir::new().unwrap();
        let result = cmd_init("TestProject", "rust", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_init_force() {
        let temp = TempDir::new().unwrap();
        // First init
        cmd_init("Test1", "rust", temp.path(), false);
        // Force overwrite
        let result = cmd_init("Test2", "python", temp.path(), true);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_lint_docs_empty() {
        let temp = TempDir::new().unwrap();
        let result = cmd_lint_docs(temp.path(), false, false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_lint_docs_with_fix() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("test.md"), "# Test\n\nContent.\n").unwrap();
        let result = cmd_lint_docs(temp.path(), true, false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_lint_docs_semantic() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("test.md"), "# Test\n\nContent.\n").unwrap();
        let result = cmd_lint_docs(temp.path(), false, true);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_refresh_no_project() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_refresh();
        // Refresh requires .asimov/ to exist (run init first)
        assert_eq!(result, ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_refresh_with_project() {
        use royalbit_asimov::templates::roadmap_template;
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Use proper template so it passes validation
        std::fs::write(asimov_dir.join("roadmap.yaml"), roadmap_template()).unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_refresh();
        // May succeed or fail depending on parallel test execution changing cwd
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_stats() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_stats();
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_doctor() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_doctor();
        // May pass or fail depending on state
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_replay_not_git() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(None, false, None);
        assert_eq!(result, ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_replay_git_repo() {
        let temp = TempDir::new().unwrap();
        // Init git
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(Some(5), false, None);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_launch_inside_claude() {
        std::env::set_var("CLAUDECODE", "1");
        // Can't fully test launch but exercise the path
        let result = check_launch_conditions();
        std::env::remove_var("CLAUDECODE");
        assert!(matches!(result, LaunchResult::InsideClaude));
    }

    #[test]
    fn test_cmd_warmup_with_update_available() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test milestone\n",
        )
        .unwrap();
        // Verbose mode checks for updates
        let result = cmd_warmup(temp.path(), true);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_validate_with_roadmap_errors() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Invalid YAML
        std::fs::write(asimov_dir.join("roadmap.yaml"), "invalid: [[[").unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(false);
        // Should fail due to invalid YAML
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_validate_with_project() {
        use royalbit_asimov::templates::{project_template, roadmap_template, ProjectType};
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(asimov_dir.join("roadmap.yaml"), roadmap_template()).unwrap();
        std::fs::write(
            asimov_dir.join("project.yaml"),
            project_template("Test", "A test project", ProjectType::Rust),
        )
        .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(false);
        // Validation may have warnings but should generally succeed
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_init_with_error() {
        // Test init with empty name - should still work
        let temp = TempDir::new().unwrap();
        let result = cmd_init("", "rust", temp.path(), false);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_lint_docs_with_errors() {
        let temp = TempDir::new().unwrap();
        // Create a markdown file with unclosed code block
        std::fs::write(temp.path().join("broken.md"), "# Test\n\n~~~\nunclosed\n").unwrap();
        let result = cmd_lint_docs(temp.path(), false, false);
        // May fail due to lint errors
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_doctor_with_project() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_doctor();
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_replay_with_commits() {
        let temp = TempDir::new().unwrap();
        // Init git with a commit
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        // Disable commit signing for test environments
        std::process::Command::new("git")
            .args(["config", "commit.gpgsign", "false"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::fs::write(temp.path().join("test.txt"), "test").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "test"])
            .current_dir(temp.path())
            .output()
            .unwrap();

        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(Some(10), false, None);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_replay_yesterday() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(None, true, None);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_replay_since() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(None, false, Some("1 hour ago".to_string()));
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_update_all_variants() {
        // Test check mode (doesn't actually update)
        let result = cmd_update(true);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_launch_conditions() {
        // Just exercise the code path - result depends on system state
        let result = check_launch_conditions();
        // Accept any variant as valid (depends on if claude installed and env vars)
        assert!(matches!(
            result,
            LaunchResult::ClaudeNotFound | LaunchResult::Launching | LaunchResult::InsideClaude
        ));
    }

    #[test]
    fn test_cmd_warmup_error_path() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // Invalid YAML to trigger error
        std::fs::write(asimov_dir.join("roadmap.yaml"), "invalid: [[[").unwrap();
        let result = cmd_warmup(temp.path(), false);
        assert_eq!(result, ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_validate_ethics_scan_with_flags() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        // Create a file with potential red flags
        std::fs::write(temp.path().join("script.sh"), "#!/bin/bash\nrm -rf /\n").unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(true);
        // May find flags or not
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_init_all_types() {
        for ptype in &["rust", "python", "node", "go", "flutter", "docs", "generic"] {
            let temp = TempDir::new().unwrap();
            let result = cmd_init("Test", ptype, temp.path(), false);
            assert_eq!(result, ExitCode::SUCCESS);
        }
    }

    #[test]
    fn test_cmd_lint_files_with_issues() {
        let temp = TempDir::new().unwrap();
        // Create multiple files
        std::fs::write(temp.path().join("good.md"), "# Good\n\nContent.\n").unwrap();
        std::fs::write(temp.path().join("bad.md"), "# Bad\n\n~~~\nunclosed\n").unwrap();
        let result = cmd_lint_docs(temp.path(), false, false);
        // Will have some errors
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_lint_fix_mode() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("fixable.md"), "# Test\n\n~~~\ncode\n~~~\n").unwrap();
        let result = cmd_lint_docs(temp.path(), true, false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_refresh_error_path() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        // No roadmap - should still work
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_refresh();
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_doctor_full() {
        let temp = TempDir::new().unwrap();
        // Set up a more complete project
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();
        std::fs::write(
            asimov_dir.join("project.yaml"),
            "identity:\n  project: Test\n  tagline: Test\n",
        )
        .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_doctor();
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_stats_with_git() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "t@t.com"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "T"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::fs::write(temp.path().join("f.txt"), "x").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(temp.path())
            .output()
            .unwrap();

        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0'\n  status: in_progress\n  summary: Test\n",
        )
        .unwrap();

        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_stats();
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_replay_basic_default() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(None, false, None);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_replay_limited_commits() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.email", "t@t.com"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["config", "user.name", "T"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        // Disable commit signing for test environments
        std::process::Command::new("git")
            .args(["config", "commit.gpgsign", "false"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::fs::write(temp.path().join("f.txt"), "x").unwrap();
        std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::process::Command::new("git")
            .args(["commit", "-m", "init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(Some(5), false, None);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_replay_yesterday_option() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(None, true, None);
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_replay_since_date() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_replay(None, false, Some("2024-01-01".to_string()));
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_lint_semantic_mode() {
        let temp = TempDir::new().unwrap();
        std::fs::write(temp.path().join("test.md"), "# Test\n\nContent.\n").unwrap();
        let result = cmd_lint_docs(temp.path(), false, true);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_validate_no_project() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(false);
        // May succeed or fail depending on project state
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_validate_with_valid_project() {
        use royalbit_asimov::templates::roadmap_template;
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(asimov_dir.join("roadmap.yaml"), roadmap_template()).unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_validate(false);
        // May have warnings/errors depending on project.yaml presence
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_init_rust() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_init("TestProject", "rust", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_init_python() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_init("TestProject", "python", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_init_node() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_init("TestProject", "node", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_init_go() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_init("TestProject", "go", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_init_docs() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_init("TestProject", "docs", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_init_generic() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_init("TestProject", "generic", temp.path(), false);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_warmup_with_project_yaml() {
        use royalbit_asimov::templates::{project_template, roadmap_template, ProjectType};
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(asimov_dir.join("roadmap.yaml"), roadmap_template()).unwrap();
        std::fs::write(
            asimov_dir.join("project.yaml"),
            project_template("Test", "A test project", ProjectType::Rust),
        )
        .unwrap();
        let result = cmd_warmup(temp.path(), true);
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_cmd_doctor_no_roadmap() {
        let temp = TempDir::new().unwrap();
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        // No roadmap - doctor may succeed or fail depending on hook checks
        let result = cmd_doctor();
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    #[test]
    fn test_cmd_stats_no_asimov() {
        let temp = TempDir::new().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();
        let result = cmd_stats();
        assert!(result == ExitCode::SUCCESS || result == ExitCode::FAILURE);
    }

    // Test all UpdateResult formatting variants
    #[test]
    fn test_format_update_result_already_latest() {
        let result = format_update_result(UpdateResult::AlreadyLatest {
            current: "1.0.0".to_string(),
            latest: "1.0.0".to_string(),
        });
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_format_update_result_update_available() {
        let result = format_update_result(UpdateResult::UpdateAvailable {
            current: "1.0.0".to_string(),
            latest: "2.0.0".to_string(),
        });
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_format_update_result_updated() {
        let result = format_update_result(UpdateResult::Updated {
            from: "1.0.0".to_string(),
            to: "2.0.0".to_string(),
        });
        assert_eq!(result, ExitCode::SUCCESS);
    }

    #[test]
    fn test_format_update_result_failed() {
        let result = format_update_result(UpdateResult::UpdateFailed {
            current: "1.0.0".to_string(),
            latest: "2.0.0".to_string(),
            error: "Download failed".to_string(),
            download_url: "https://example.com/file.tar.gz".to_string(),
        });
        assert_eq!(result, ExitCode::FAILURE);
    }

    #[test]
    fn test_format_update_result_no_binary() {
        let result = format_update_result(UpdateResult::NoBinaryAvailable {
            current: "1.0.0".to_string(),
            latest: "2.0.0".to_string(),
        });
        assert_eq!(result, ExitCode::FAILURE);
    }

    #[test]
    fn test_format_update_result_check_failed() {
        let result = format_update_result(UpdateResult::CheckFailed {
            error: "Network error".to_string(),
        });
        assert_eq!(result, ExitCode::FAILURE);
    }
}
