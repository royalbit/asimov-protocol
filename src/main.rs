#![cfg_attr(feature = "coverage", feature(coverage_attribute))]
//! RoyalBit Asimov CLI - The Three Laws of Robotics, encoded in YAML
//!
//! This is a thin wrapper around the commands module. All business logic
//! is in lib.rs for testability. This file only handles CLI parsing and output.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

mod output;
use output::{
    cmd_doctor, cmd_init, cmd_launch, cmd_lint_docs, cmd_refresh, cmd_replay, cmd_role, cmd_stats,
    cmd_update, cmd_validate, cmd_warmup,
};

#[derive(Parser)]
#[command(name = "asimov")]
#[command(about = "RoyalBit Asimov CLI - AI development with protocol enforcement")]
#[command(long_about = "RoyalBit Asimov CLI v11.0.0
Copyright (c) 2025 RoyalBit Inc.
Licensed under Elastic License 2.0 (ELv2).

LAUNCHER MODE (AI-agnostic, v10.7.0 ADR-061):
  asimov                             # Auto-detect AI CLI, launch + warmup
  asimov                             # Inside AI session: runs warmup directly

SUPPORTED AI CLIs:
  - Claude Code (claude)             # https://claude.ai/download
  - Gemini CLI (gemini)              # https://cloud.google.com/gemini-cli
  - Codex CLI (codex)                # https://github.com/openai/codex

EXAMPLES:
  asimov                             # Start session (auto-detect AI)
  asimov warmup                      # Manual warmup (inside AI session)
  asimov role                        # List available roles
  asimov role eng                    # Switch to Principal Engineer role
  asimov validate                    # Validate roadmap.yaml
  asimov update                      # Update binary
  asimov init                        # Initialize new project

PROTOCOLS (7 total, loaded from .asimov/protocols/ with embedded fallback):
  - asimov     - The Three Laws (do no harm, obey human, self-preserve)
  - freshness  - Date-aware search (WebSearch/WebFetch with current date)
  - sycophancy - Truth over comfort, honest disagreement
  - green      - Efficiency benchmarks via WebSearch
  - sprint     - Autonomous execution + compaction survival (run until done)
  - warmup     - Session bootstrap (load, validate, present)
  - coding-standards - Human-readable code (RFC2119 compliance)

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

        /// Template type. Base: rust, python, node, go, flutter, docs, arch, generic.
        /// API: api-rust, api-go, api-fastapi, api-nestjs, api-spring.
        /// Web: web-nextjs, web-react, web-vue, web-angular.
        /// Mono: mono-turbo, mono-nx, mono-pnpm. Other: admin-dashboard.
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

    /// Refresh protocol context and migrate project files
    Refresh {
        /// Show verbose output
        #[arg(short, long)]
        verbose: bool,

        /// Auto-accept template defaults without prompting
        #[arg(short, long)]
        yes: bool,

        /// Show what would change without writing
        #[arg(long)]
        dry_run: bool,
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

    /// Switch or list available roles (v10.0.0)
    Role {
        /// Role code (eng, biz, fin, ai, pm, qa). Omit to list all.
        code: Option<String>,
    },

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
        Some(Commands::Refresh {
            verbose,
            yes,
            dry_run,
        }) => cmd_refresh(verbose, yes, dry_run),
        Some(Commands::Update { check }) => cmd_update(check),
        Some(Commands::Warmup { path, verbose }) => cmd_warmup(&path, verbose),
        Some(Commands::Stats) => cmd_stats(),
        Some(Commands::Doctor) => cmd_doctor(),
        Some(Commands::Role { code }) => cmd_role(code.as_deref()),
        Some(Commands::Replay {
            commits,
            yesterday,
            since,
            verbose: _,
        }) => cmd_replay(commits, yesterday, since),
    }
}
