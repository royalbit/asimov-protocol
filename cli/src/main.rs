//! RoyalBit Asimov CLI - The Three Laws of Robotics, encoded in YAML

use clap::{Parser, Subcommand};
use colored::Colorize;
use royalbit_asimov::{
    anti_patterns,
    banned_phrases,
    check_ethics_status,
    check_for_update,
    check_green_status,
    check_markdown_file,
    check_semantic,
    check_sycophancy_status,
    // v8.0.0: Hardcoded hook templates
    claude_pre_compact_hook,
    claude_session_start_hook,
    claude_settings_json,
    // v8.1.0: Project type detection and templates (ADR-032)
    detect_project_type,
    find_markdown_files,
    fix_markdown_file,
    get_cargo_version,
    git_precommit_hook,
    is_protocol_file,
    load_deprecated_patterns,
    perform_update,
    project_template,
    red_flags,
    resolve_protocol_dir,
    roadmap_template,
    scan_directory_for_red_flags,
    to_minified_json,
    validate_directory_with_regeneration,
    validate_file,
    DeprecatedPattern,
    EthicsStatus,
    GreenStatus,
    ProjectType,
    SemanticConfig,
    Severity,
    SycophancyStatus,
    // Schema exports for editor integration
    ASIMOV_SCHEMA,
    CORE_PRINCIPLES,
    FRESHNESS_SCHEMA,
    GREEN_MOTTO,
    GREEN_PRINCIPLES,
    GREEN_SCHEMA,
    HUMAN_VETO_COMMANDS,
    MIGRATIONS_SCHEMA,
    ROADMAP_SCHEMA,
    SPRINT_SCHEMA,
    SYCOPHANCY_MOTTO,
    SYCOPHANCY_PRINCIPLES,
    SYCOPHANCY_SCHEMA,
    WARMUP_SCHEMA,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "asimov")]
#[command(about = "Green coding CLI for AI development - zero tokens, zero emissions")]
#[command(long_about = "RoyalBit Asimov CLI - The Three Laws of Robotics

v8.0.0: Protocols are HARDCODED in the binary. Cannot be bypassed.
  - 7 protocols compiled in: asimov, freshness, sycophancy, green, sprint, warmup, migrations
  - Only roadmap.yaml remains in .asimov/ (project data)
  - Dynamic date injection ({TODAY}, {YEAR})
  - Token-optimized JSON output

EXAMPLES:
  asimov warmup                      # Start session, output compiled protocols
  asimov validate                    # Validate roadmap.yaml
  asimov update                      # Update binary + migrate (delete old YAMLs)
  asimov init --asimov               # Initialize new project

PROTOCOLS (hardcoded):
  - asimov     - The Three Laws (do no harm, obey human, self-preserve)
  - freshness  - Date-aware search (dynamic {TODAY}, {YEAR})
  - sycophancy - Truth over comfort, no empty validation
  - green      - Local-first, zero tokens where possible
  - sprint     - Session boundaries (4hr max, stop conditions)
  - warmup     - Session bootstrap (load, validate, present)
  - migrations - Functional equivalence (same inputs = same outputs)

The Open Foundation: Creates Self-Evolving Autonomous AI projects with ethics built in.
Inspect the code. Challenge the rules. Fork if you disagree.
Adoption through consent, not control.

GREEN CODING - Why This Matters:
  - Local validation: $0/file, ~0.002g CO2, <100ms
  - Cloud AI validation: $0.02+/file, ~0.5g CO2, 1-3s
  - Carbon reduction: 99.6% vs cloud AI

Zero tokens. Zero emissions. Ship fast.

Docs: https://github.com/royalbit/asimov")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate protocol files against the schema
    Validate {
        /// File or directory to validate (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Scan project files for red flag patterns (security, financial, privacy, deception)
        #[arg(long)]
        ethics_scan: bool,

        /// Skip auto-regeneration of missing protocol files
        #[arg(long)]
        no_regenerate: bool,
    },

    /// Initialize new protocol files
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Project type for language-specific templates (generic, rust, python, node, go, flutter, docs)
        #[arg(short = 't', long = "type", default_value = "generic")]
        project_type: String,

        /// Generate all protocol files (warmup.yaml, sprint.yaml, roadmap.yaml)
        #[arg(long)]
        full: bool,

        /// Full RoyalBit Asimov setup (protocol files + CLAUDE.md + hooks + .gitignore)
        #[arg(long)]
        asimov: bool,

        /// Output directory (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        output: PathBuf,

        /// Overwrite existing files
        #[arg(long)]
        force: bool,
    },

    /// Check a single file (alias for validate)
    Check {
        /// File to check
        file: PathBuf,
    },

    /// Lint markdown documentation for common issues
    #[command(name = "lint-docs")]
    LintDocs {
        /// Directory or file to lint (defaults to current directory)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Auto-fix issues (repairs broken code block closers)
        #[arg(long)]
        fix: bool,

        /// Enable semantic checks (version consistency, deprecated patterns, cross-references)
        #[arg(long)]
        semantic: bool,
    },

    /// Refresh protocol context (for git hooks - injects rules into fresh context)
    Refresh {
        /// Show quality gates from warmup.yaml
        #[arg(short, long)]
        verbose: bool,
    },

    /// Export JSON schemas for editor integration (VS Code, etc.)
    Schema {
        /// Schema to export (all, warmup, sprint, roadmap, asimov, freshness, green, sycophancy, migrations)
        #[arg(default_value = "all")]
        name: String,

        /// Output directory for schema files (default: stdout for single, ./schemas/ for all)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Check for updates and optionally self-update the binary
    Update {
        /// Only check for updates, don't install
        #[arg(long)]
        check: bool,
    },

    /// Session warmup - display current/next milestone and validate
    Warmup,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate {
            path,
            ethics_scan,
            no_regenerate,
        } => cmd_validate(&path, ethics_scan, !no_regenerate),
        Commands::Init {
            name,
            project_type,
            full,
            asimov,
            output,
            force,
        } => cmd_init(name, &project_type, full, asimov, &output, force),
        Commands::Check { file } => cmd_validate(&file, false, true),
        Commands::LintDocs {
            path,
            fix,
            semantic,
        } => cmd_lint_docs(&path, fix, semantic),
        Commands::Refresh { verbose } => cmd_refresh(verbose),
        Commands::Schema { name, output } => cmd_schema(&name, output),
        Commands::Update { check } => cmd_update(check),
        Commands::Warmup => cmd_warmup(),
    }
}

/// Migrate v8.0.0: Delete deprecated protocol YAMLs (now hardcoded in binary)
/// Also ensures hooks are installed/restored
fn migrate_v8() {
    let protocol_dir = resolve_protocol_dir(Path::new("."));
    let deprecated_files = [
        "asimov.yaml",
        "freshness.yaml",
        "sycophancy.yaml",
        "green.yaml",
        "sprint.yaml",
        "warmup.yaml",
        "migrations.yaml",
        "ethics.yaml", // Already deprecated, but clean up if exists
    ];

    let mut deleted = Vec::new();
    for filename in &deprecated_files {
        let path = protocol_dir.join(filename);
        if path.exists() {
            if let Ok(()) = std::fs::remove_file(&path) {
                deleted.push(*filename);
            }
        }
    }

    if !deleted.is_empty() {
        println!();
        println!("{}", "v8.0.0 Migration".bold().cyan());
        println!("  Protocols are now hardcoded in binary. Deleted deprecated YAMLs:");
        for f in &deleted {
            println!("    {} {}", "✗".red(), f);
        }
        println!("  {} roadmap.yaml (project data, kept)", "✓".green());
    }

    // v8.0.0: Also ensure hooks are installed/restored
    println!();
    println!("{}", "Ensuring hooks...".bold());
    install_hooks(Path::new("."), true); // force=true to restore any tampered hooks
}

fn cmd_update(check_only: bool) -> ExitCode {
    println!("{}", "RoyalBit Asimov Update".bold().green());
    println!();

    println!("  Checking for updates...");

    match check_for_update() {
        Ok(version_info) => {
            println!("  Current version: {}", version_info.current.bright_blue());
            println!("  Latest version:  {}", version_info.latest.bright_blue());
            println!();

            if version_info.update_available {
                println!("  {} New version available!", "UPDATE".bold().yellow());

                if check_only {
                    println!();
                    println!("  Run {} to install the update.", "asimov update".bold());
                    return ExitCode::SUCCESS;
                }

                if let Some(download_url) = version_info.download_url {
                    println!();
                    match perform_update(&download_url) {
                        Ok(()) => {
                            println!();
                            println!(
                                "{} Updated to version {}",
                                "Success:".bold().green(),
                                version_info.latest
                            );
                            // Run v8.0.0 migration
                            migrate_v8();
                            println!();
                            println!("  Run {} to verify.", "asimov --version".bold());
                            ExitCode::SUCCESS
                        }
                        Err(e) => {
                            eprintln!();
                            eprintln!("{} {}", "Error:".bold().red(), e);
                            eprintln!();
                            eprintln!("  You can manually update:");
                            eprintln!("    curl -L {} | tar xz", download_url);
                            eprintln!("    sudo mv asimov /usr/local/bin/");
                            ExitCode::FAILURE
                        }
                    }
                } else {
                    eprintln!();
                    eprintln!(
                        "{} No binary available for this platform",
                        "Error:".bold().red()
                    );
                    eprintln!("  Visit https://github.com/royalbit/asimov/releases/latest");
                    ExitCode::FAILURE
                }
            } else {
                println!(
                    "  {} You're running the latest version!",
                    "OK".bold().green()
                );
                // Run v8.0.0 migration even if already on latest
                migrate_v8();
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Error:".bold().red(), e);
            eprintln!();
            eprintln!("  Check manually: https://github.com/royalbit/asimov/releases/latest");
            ExitCode::FAILURE
        }
    }
}

fn cmd_warmup() -> ExitCode {
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

    // Check for updates (one network call per session)
    if let Ok(version_info) = check_for_update() {
        if version_info.update_available {
            println!(
                "{}  Update available: {} → {} (run: {})",
                "⚠️".yellow(),
                version_info.current.dimmed(),
                version_info.latest.bright_green(),
                "asimov update".bold()
            );
            println!();
        }
    }

    // Read and parse roadmap.yaml
    let roadmap_path = resolve_protocol_dir(Path::new(".")).join("roadmap.yaml");
    let roadmap_content = match std::fs::read_to_string(&roadmap_path) {
        Ok(content) => content,
        Err(_) => {
            eprintln!(
                "{} roadmap.yaml not found. Run {} first.",
                "Error:".bold().red(),
                "asimov init --full".bold()
            );
            return ExitCode::FAILURE;
        }
    };

    let roadmap: serde_yaml::Value = match serde_yaml::from_str(&roadmap_content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} Failed to parse roadmap.yaml: {}",
                "Error:".bold().red(),
                e
            );
            return ExitCode::FAILURE;
        }
    };

    // Extract current version info
    println!("{}", "CURRENT VERSION".bold());
    if let Some(current) = roadmap.get("current") {
        let version = current
            .get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        let summary = current
            .get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let status = current
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let status_display = match status {
            "released" => format!("{} {}", "✓".green(), "released".green()),
            "in_progress" => format!("{} {}", "→".yellow(), "in progress".yellow()),
            _ => format!("  {}", status),
        };

        println!("  v{} - {}", version.bright_blue(), summary);
        println!("  Status: {}", status_display);
    }
    println!();

    // Extract next milestone(s)
    println!("{}", "NEXT MILESTONE".bold());
    if let Some(next) = roadmap.get("next") {
        if let Some(next_list) = next.as_sequence() {
            if let Some(first_next) = next_list.first() {
                let version = first_next
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");
                let summary = first_next
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let goal = first_next
                    .get("goal")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");

                println!("  v{} - {}", version.bright_yellow(), summary);
                if !goal.is_empty() {
                    println!("  Goal: {}", goal.dimmed());
                }

                // Print features
                if let Some(features) = first_next.get("features") {
                    if let Some(feature_list) = features.as_sequence() {
                        println!();
                        println!("  {}:", "Features".dimmed());
                        for feature in feature_list {
                            if let Some(f) = feature.as_str() {
                                println!("  {} {}", "•".bright_cyan(), f);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("  {} No upcoming milestones in roadmap", "✓".green());
    }
    println!();

    // Run validation
    println!("{}", "VALIDATION".bold());
    let dir = Path::new(".");
    let ethics_status = check_ethics_status(dir);
    let sycophancy_status = check_sycophancy_status(dir);
    let green_status = check_green_status(dir);

    let ethics_display = match ethics_status {
        EthicsStatus::Hardcoded => "HARDCODED".bright_cyan(),
        EthicsStatus::Extended => "EXTENDED (core + asimov.yaml)".bright_green(),
    };
    let sycophancy_display = match sycophancy_status {
        SycophancyStatus::Hardcoded => "HARDCODED".bright_cyan(),
        SycophancyStatus::Extended => "EXTENDED (core + sycophancy.yaml)".bright_green(),
    };
    let green_display = match green_status {
        GreenStatus::Hardcoded => "HARDCODED".bright_cyan(),
        GreenStatus::Extended => "EXTENDED (core + green.yaml)".bright_green(),
    };

    println!("  {} Ethics: {}", "✓".green(), ethics_display);
    println!("  {} Anti-Sycophancy: {}", "✓".green(), sycophancy_display);
    println!("  {} Green Coding: {}", "✓".green(), green_display);

    // Validate protocol files
    match validate_directory_with_regeneration(dir, true) {
        Ok((results, _)) => {
            let valid_count = results.iter().filter(|r| r.is_valid).count();
            let total = results.len();
            if valid_count == total {
                println!("  {} {} protocol file(s) valid", "✓".green(), total);
            } else {
                println!(
                    "  {} {}/{} protocol file(s) valid",
                    "⚠".yellow(),
                    valid_count,
                    total
                );
            }
        }
        Err(e) => {
            println!("  {} Validation error: {}", "✗".red(), e);
        }
    }

    // Output compiled protocols for context injection (ADR-031)
    println!();
    println!("{}", "PROTOCOLS (ENFORCED)".bold());
    println!(
        "  {} Protocols compiled from binary (cannot be bypassed)",
        "✓".green()
    );
    println!();
    println!("{}", "Context injection:".dimmed());
    println!("{}", to_minified_json());
    println!();

    println!(
        "{}",
        "══════════════════════════════════════════════════════════════════════════════"
            .bright_cyan()
    );
    println!(
        "{}",
        "Ready to execute. Say \"go\" to start autonomous execution.".bold()
    );
    println!(
        "{}",
        "══════════════════════════════════════════════════════════════════════════════"
            .bright_cyan()
    );
    println!();

    ExitCode::SUCCESS
}

fn cmd_refresh(verbose: bool) -> ExitCode {
    println!();
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════".bright_cyan()
    );
    println!(
        "{}",
        "🤖 ROYALBIT ASIMOV - THE THREE LAWS".bold().bright_cyan()
    );
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════".bright_cyan()
    );
    println!();

    // Ethics reminder (hardcoded - cannot be removed)
    println!(
        "{}",
        "[ASIMOV ETHICS] Core principles ACTIVE".bold().green()
    );
    println!(
        "  {} Financial: {} | Physical: {} | Privacy: {} | Deception: {}",
        "✓".green(),
        if CORE_PRINCIPLES.financial {
            "blocked".green()
        } else {
            "off".red()
        },
        if CORE_PRINCIPLES.physical {
            "blocked".green()
        } else {
            "off".red()
        },
        if CORE_PRINCIPLES.privacy {
            "blocked".green()
        } else {
            "off".red()
        },
        if CORE_PRINCIPLES.deception {
            "blocked".green()
        } else {
            "off".red()
        },
    );
    println!(
        "  {} Red flags monitored: {} patterns",
        "✓".green(),
        red_flags::count()
    );
    println!(
        "  {} Human veto: {}",
        "✓".green(),
        HUMAN_VETO_COMMANDS.join(" | ").dimmed()
    );
    println!();

    // Anti-sycophancy reminder (hardcoded - cannot be removed)
    println!(
        "{}",
        "[ASIMOV ANTI-SYCOPHANCY] Core principles ACTIVE"
            .bold()
            .cyan()
    );
    println!(
        "  {} Truth over comfort: {} | Disagree openly: {} | No empty validation: {}",
        "✓".green(),
        if SYCOPHANCY_PRINCIPLES.truth_over_comfort {
            "on".green()
        } else {
            "off".red()
        },
        if SYCOPHANCY_PRINCIPLES.respectful_disagreement {
            "on".green()
        } else {
            "off".red()
        },
        if SYCOPHANCY_PRINCIPLES.no_empty_validation {
            "on".green()
        } else {
            "off".red()
        },
    );
    println!(
        "  {} Banned phrases: {} patterns | Motto: {}",
        "✓".green(),
        banned_phrases::count(),
        SYCOPHANCY_MOTTO.dimmed()
    );
    println!();

    // Green coding reminder (hardcoded - cannot be removed)
    println!(
        "{}",
        "[ASIMOV GREEN CODING] Core principles ACTIVE"
            .bold()
            .bright_green()
    );
    println!(
        "  {} Local-first: {} | Token efficiency: {} | Binary efficiency: {}",
        "✓".green(),
        if GREEN_PRINCIPLES.local_first {
            "on".green()
        } else {
            "off".red()
        },
        if GREEN_PRINCIPLES.token_efficiency {
            "on".green()
        } else {
            "off".red()
        },
        if GREEN_PRINCIPLES.binary_efficiency {
            "on".green()
        } else {
            "off".red()
        },
    );
    println!(
        "  {} Anti-patterns: {} patterns | Motto: {}",
        "✓".green(),
        anti_patterns::count(),
        GREEN_MOTTO.dimmed()
    );
    println!();

    println!(
        "{} → {}",
        "ON CONFUSION".bold().yellow(),
        "run `asimov warmup`".white()
    );
    println!();
    println!(
        "{}: {} | {} | {}",
        "RULES".bold(),
        "4hr max".white(),
        "tests pass".white(),
        "keep shipping".green()
    );

    // v8.0.0: If verbose, try to read and display current milestone from roadmap.yaml
    if verbose {
        if let Ok(content) = std::fs::read_to_string(".asimov/roadmap.yaml") {
            if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                if let Some(current) = yaml.get("current") {
                    println!();
                    println!("{}", "CURRENT MILESTONE:".bold());
                    if let Some(version) = current.get("version").and_then(|v| v.as_str()) {
                        println!("  {}: {}", "version".bright_blue(), version.dimmed());
                    }
                    if let Some(summary) = current.get("summary").and_then(|v| v.as_str()) {
                        println!("  {}: {}", "summary".bright_blue(), summary.dimmed());
                    }
                }
            }
        }
    }

    println!();
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════".bright_cyan()
    );
    println!();

    ExitCode::SUCCESS
}

fn cmd_validate(path: &Path, ethics_scan: bool, regenerate: bool) -> ExitCode {
    println!("{}", "RoyalBit Asimov Validator".bold().green());
    println!();

    // Show hardcoded ethics status
    let dir = if path.is_file() {
        path.parent().unwrap_or(Path::new("."))
    } else {
        path
    };
    let ethics_status = check_ethics_status(dir);
    let ethics_display = match ethics_status {
        EthicsStatus::Hardcoded => "HARDCODED (core principles enforced)".bright_cyan(),
        EthicsStatus::Extended => "EXTENDED (core + asimov.yaml)".bright_green(),
    };
    println!("  {} Ethics: {}", "✓".green(), ethics_display);

    // Show hardcoded sycophancy status
    let sycophancy_status = check_sycophancy_status(dir);
    let sycophancy_display = match sycophancy_status {
        SycophancyStatus::Hardcoded => "HARDCODED (truth over comfort)".bright_cyan(),
        SycophancyStatus::Extended => "EXTENDED (core + sycophancy.yaml)".bright_green(),
    };
    println!("  {} Anti-Sycophancy: {}", "✓".green(), sycophancy_display);

    // Show hardcoded green coding status
    let green_status = check_green_status(dir);
    let green_display = match green_status {
        GreenStatus::Hardcoded => "HARDCODED (local-first always)".bright_cyan(),
        GreenStatus::Extended => "EXTENDED (core + green.yaml)".bright_green(),
    };
    println!("  {} Green Coding: {}", "✓".green(), green_display);
    println!();

    let (results, regen_info) = if path.is_file() {
        // Validate single file (no regeneration for single files)
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !is_protocol_file(filename) {
            eprintln!(
                "{} Not a protocol file. Expected warmup.yaml, sprint.yaml, or roadmap.yaml",
                "Error:".bold().red()
            );
            return ExitCode::FAILURE;
        }

        match validate_file(path) {
            Ok(result) => (vec![result], royalbit_asimov::RegenerationInfo::default()),
            Err(e) => {
                eprintln!("{} {}", "Error:".bold().red(), e);
                return ExitCode::FAILURE;
            }
        }
    } else {
        // Validate directory with regeneration
        match validate_directory_with_regeneration(path, regenerate) {
            Ok((results, info)) => (results, info),
            Err(e) => {
                eprintln!("{} {}", "Error:".bold().red(), e);
                return ExitCode::FAILURE;
            }
        }
    };

    // Print regeneration info first
    if !regen_info.is_empty() {
        for (filename, is_warn) in &regen_info.regenerated {
            let level = if *is_warn {
                "⚠️  REGENERATED".yellow()
            } else {
                "ℹ️  REGENERATED".bright_cyan()
            };
            let suffix = if filename == "roadmap.yaml" {
                " [skeleton]"
            } else {
                ""
            };
            println!("  {} {} (was missing){}", level, filename, suffix);
        }
        println!();
    }

    // Print results
    let mut has_errors = false;

    for result in &results {
        let status = if result.is_valid {
            "OK".bold().green()
        } else {
            has_errors = true;
            "FAIL".bold().red()
        };

        let regen_suffix = if result.regenerated {
            " [REGENERATED]".dimmed()
        } else {
            "".into()
        };

        println!(
            "  {} {} ({}){}",
            status,
            result.file.bright_blue(),
            result.schema_type.dimmed(),
            regen_suffix
        );

        for error in &result.errors {
            println!("      {} {}", "-".red(), error);
        }

        for warning in &result.warnings {
            println!("      {} {}", "!".yellow(), warning);
        }
    }

    println!();

    // Run ethics scan if requested
    if ethics_scan {
        println!("{}", "Ethics Scan (Red Flag Detection)".bold().yellow());
        println!();

        match scan_directory_for_red_flags(dir) {
            Ok(matches) => {
                if matches.is_empty() {
                    println!(
                        "  {} No red flags detected ({} patterns checked)",
                        "✓".green(),
                        red_flags::count()
                    );
                } else {
                    println!("  {} {} red flag(s) detected:", "⚠".yellow(), matches.len());
                    println!();
                    for m in &matches {
                        println!(
                            "    {}:{} [{}] \"{}\"",
                            m.file.bright_blue(),
                            m.line.to_string().yellow(),
                            m.category.to_string().red(),
                            m.pattern
                        );
                        if !m.context.is_empty() {
                            println!("      {}", m.context.dimmed());
                        }
                    }
                    println!();
                    println!(
                        "  {} Red flags require human review. They may be legitimate.",
                        "Note:".dimmed()
                    );
                }
            }
            Err(e) => {
                eprintln!("  {} Failed to scan: {}", "Error:".bold().red(), e);
            }
        }

        println!();
    }

    if has_errors {
        let fail_count = results.iter().filter(|r| !r.is_valid).count();
        println!(
            "{} {} file(s) failed validation",
            "Error:".bold().red(),
            fail_count
        );
        ExitCode::FAILURE
    } else {
        let regen_count = regen_info.regenerated.len();
        if regen_count > 0 {
            println!(
                "{} {} file(s) valid ({} regenerated)",
                "Success:".bold().green(),
                results.len(),
                regen_count
            );
        } else {
            println!(
                "{} {} file(s) valid",
                "Success:".bold().green(),
                results.len()
            );
        }
        ExitCode::SUCCESS
    }
}

/// Install hardcoded hooks (Claude Code + Git pre-commit)
/// v8.0.0: Hooks are hardcoded in binary and restored on init/update/tampering
fn install_hooks(output: &Path, force: bool) -> bool {
    let mut success = true;

    // 1. Create .claude/settings.json
    let claude_dir = output.join(".claude");
    if let Err(e) = std::fs::create_dir_all(&claude_dir) {
        eprintln!(
            "  {} Failed to create .claude/ - {}",
            "ERROR".bold().red(),
            e
        );
        return false;
    }

    let settings_path = claude_dir.join("settings.json");
    if !settings_path.exists() || force {
        match std::fs::write(&settings_path, claude_settings_json()) {
            Ok(_) => println!("  {} .claude/settings.json", "CREATE".bold().green()),
            Err(e) => {
                eprintln!("  {} .claude/settings.json - {}", "ERROR".bold().red(), e);
                success = false;
            }
        }
    } else {
        println!("  {} .claude/settings.json", "SKIP".yellow());
    }

    // 2. Create .claude/hooks/ directory and hooks
    let hooks_dir = claude_dir.join("hooks");
    if let Err(e) = std::fs::create_dir_all(&hooks_dir) {
        eprintln!(
            "  {} Failed to create .claude/hooks/ - {}",
            "ERROR".bold().red(),
            e
        );
        return false;
    }

    // session-start.sh
    let session_start_path = hooks_dir.join("session-start.sh");
    if !session_start_path.exists() || force {
        match std::fs::write(&session_start_path, claude_session_start_hook()) {
            Ok(_) => {
                // Make executable on Unix
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&session_start_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = std::fs::set_permissions(&session_start_path, perms);
                    }
                }
                println!(
                    "  {} .claude/hooks/session-start.sh",
                    "CREATE".bold().green()
                );
            }
            Err(e) => {
                eprintln!(
                    "  {} .claude/hooks/session-start.sh - {}",
                    "ERROR".bold().red(),
                    e
                );
                success = false;
            }
        }
    } else {
        println!("  {} .claude/hooks/session-start.sh", "SKIP".yellow());
    }

    // pre-compact.sh
    let pre_compact_path = hooks_dir.join("pre-compact.sh");
    if !pre_compact_path.exists() || force {
        match std::fs::write(&pre_compact_path, claude_pre_compact_hook()) {
            Ok(_) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&pre_compact_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = std::fs::set_permissions(&pre_compact_path, perms);
                    }
                }
                println!("  {} .claude/hooks/pre-compact.sh", "CREATE".bold().green());
            }
            Err(e) => {
                eprintln!(
                    "  {} .claude/hooks/pre-compact.sh - {}",
                    "ERROR".bold().red(),
                    e
                );
                success = false;
            }
        }
    } else {
        println!("  {} .claude/hooks/pre-compact.sh", "SKIP".yellow());
    }

    // 3. Create .git/hooks/pre-commit (only if .git exists)
    let git_dir = output.join(".git");
    if git_dir.exists() {
        let git_hooks_dir = git_dir.join("hooks");
        if let Err(e) = std::fs::create_dir_all(&git_hooks_dir) {
            eprintln!(
                "  {} Failed to create .git/hooks/ - {}",
                "ERROR".bold().red(),
                e
            );
            return false;
        }

        let precommit_path = git_hooks_dir.join("pre-commit");
        if !precommit_path.exists() || force {
            match std::fs::write(&precommit_path, git_precommit_hook()) {
                Ok(_) => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if let Ok(metadata) = std::fs::metadata(&precommit_path) {
                            let mut perms = metadata.permissions();
                            perms.set_mode(0o755);
                            let _ = std::fs::set_permissions(&precommit_path, perms);
                        }
                    }
                    println!("  {} .git/hooks/pre-commit", "CREATE".bold().green());
                }
                Err(e) => {
                    eprintln!("  {} .git/hooks/pre-commit - {}", "ERROR".bold().red(), e);
                    success = false;
                }
            }
        } else {
            println!("  {} .git/hooks/pre-commit", "SKIP".yellow());
        }
    }

    success
}

fn cmd_init(
    name: Option<String>,
    project_type_str: &str,
    _full: bool,
    asimov: bool,
    output: &Path,
    force: bool,
) -> ExitCode {
    // v8.1.0: Generate roadmap.yaml + project.yaml (ADR-032)
    // Protocols are hardcoded in binary (ADR-031)

    // Check if we're in a git subdirectory with .asimov/ at root (before any output)
    let output_abs = std::fs::canonicalize(output).unwrap_or_else(|_| output.to_path_buf());
    if let Some(git_root) = find_git_root(&output_abs) {
        let git_root_asimov = git_root.join(".asimov");
        if git_root_asimov.exists() && git_root != output_abs {
            eprintln!(
                "{} Cannot init in subdirectory - .asimov/ already exists at git root",
                "ERROR".bold().red()
            );
            eprintln!("  Git root: {}", git_root.display());
            eprintln!("  Current:  {}", output_abs.display());
            eprintln!();
            eprintln!("  Run {} from the git root instead.", "asimov init".bold());
            return ExitCode::FAILURE;
        }
    }

    // Detect or parse project type (ADR-032)
    let project_type: ProjectType = if project_type_str.is_empty() {
        let detected = detect_project_type(output);
        println!(
            "  {} Detected project type: {}",
            "AUTO".bold().cyan(),
            detected
        );
        detected
    } else {
        match project_type_str.parse() {
            Ok(pt) => pt,
            Err(e) => {
                eprintln!("{} {}", "ERROR".bold().red(), e);
                return ExitCode::FAILURE;
            }
        }
    };

    // Get project name from argument or directory name
    let project_name = name.unwrap_or_else(|| {
        output
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "my-project".to_string())
    });

    println!("{}", "RoyalBit Asimov Init (v8.1.0)".bold().green());
    println!("  Protocols are hardcoded in binary (ADR-031)");
    println!("  Project data: roadmap.yaml + project.yaml (ADR-032)");
    println!();

    // v8.1.0: Generate roadmap.yaml + project.yaml (ADR-032)
    let files: Vec<(&str, String)> = vec![
        ("roadmap.yaml", roadmap_template()),
        (
            "project.yaml",
            project_template(&project_name, "Brief project description", project_type),
        ),
    ];

    // Note: warmup.yaml, sprint.yaml, asimov.yaml, green.yaml, sycophancy.yaml
    // are no longer generated - they are hardcoded in the binary (ADR-031)

    if asimov {
        // Delete deprecated files if they exist
        let deprecated_files = [
            "CLAUDE.md",
            ".asimov/warmup.yaml",
            ".asimov/sprint.yaml",
            ".asimov/asimov.yaml",
            ".asimov/green.yaml",
            ".asimov/sycophancy.yaml",
            ".asimov/freshness.yaml",
            ".asimov/migrations.yaml",
            ".asimov/ethics.yaml",
        ];
        for filename in &deprecated_files {
            let path = output.join(filename);
            if path.exists() {
                if let Ok(()) = std::fs::remove_file(&path) {
                    println!("  {} Deleted deprecated {}", "CLEANUP".yellow(), filename);
                }
            }
        }
    }

    // Create .asimov directory for protocol files
    let asimov_dir = output.join(".asimov");
    if let Err(e) = std::fs::create_dir_all(&asimov_dir) {
        eprintln!(
            "  {} Failed to create .asimov directory - {}",
            "ERROR".bold().red(),
            e
        );
        return ExitCode::FAILURE;
    }

    // Write protocol files
    for (filename, content) in &files {
        // CLAUDE.md and .claude_checkpoint.yaml.example stay in root, protocol files go to .asimov/
        let file_path = if filename.ends_with(".md") || filename.starts_with('.') {
            output.join(filename)
        } else {
            asimov_dir.join(filename)
        };

        if file_path.exists() && !force {
            println!(
                "  {} {} (use --force to overwrite)",
                "SKIP".yellow(),
                filename
            );
            continue;
        }

        match std::fs::write(&file_path, content) {
            Ok(_) => {
                println!("  {} {}", "CREATE".bold().green(), file_path.display());
            }
            Err(e) => {
                eprintln!("  {} {} - {}", "ERROR".bold().red(), filename, e);
                return ExitCode::FAILURE;
            }
        }
    }

    // v8.0.0: Update .gitignore for checkpoint file
    // Note: Hooks are no longer generated - protocols are hardcoded in binary
    let gitignore_path = output.join(".gitignore");
    let checkpoint_entry = ".claude_checkpoint.yaml";

    let needs_update = if gitignore_path.exists() {
        match std::fs::read_to_string(&gitignore_path) {
            Ok(content) => !content.contains(checkpoint_entry),
            Err(_) => true,
        }
    } else {
        true
    };

    if needs_update {
        let mut content = if gitignore_path.exists() {
            std::fs::read_to_string(&gitignore_path).unwrap_or_default()
        } else {
            String::new()
        };

        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str("\n# RoyalBit Asimov checkpoint (session-specific)\n");
        content.push_str(checkpoint_entry);
        content.push('\n');

        match std::fs::write(&gitignore_path, content) {
            Ok(_) => {
                println!(
                    "  {} .gitignore (added {})",
                    "UPDATE".bold().green(),
                    checkpoint_entry
                );
            }
            Err(e) => {
                eprintln!("  {} .gitignore - {}", "ERROR".bold().red(), e);
            }
        }
    } else {
        println!(
            "  {} .gitignore (already has {})",
            "SKIP".yellow(),
            checkpoint_entry
        );
    }

    // v8.0.0: Install hardcoded hooks (Claude Code + Git pre-commit)
    println!();
    println!("{}", "Installing hooks...".bold());
    install_hooks(output, force);

    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Edit .asimov/project.yaml with your project details");
    println!("  2. Edit .asimov/roadmap.yaml with your milestones");
    println!("  3. Run: asimov warmup");
    println!("  4. Protocols + hooks are loaded automatically from binary");
    println!();

    ExitCode::SUCCESS
}

fn cmd_lint_docs(path: &Path, fix: bool, semantic: bool) -> ExitCode {
    println!("{}", "RoyalBit Asimov Documentation Linter".bold().green());
    println!();

    // Collect files to check
    let files = if path.is_file() {
        vec![path.to_path_buf()]
    } else {
        find_markdown_files(path)
    };

    if files.is_empty() {
        println!("  {} No markdown files found", "SKIP".yellow());
        return ExitCode::SUCCESS;
    }

    println!("  {} {} markdown file(s)", "Scanning".dimmed(), files.len());
    println!();

    let mut total_errors = 0;
    let mut _files_with_errors = 0;
    let mut files_fixed = 0;

    // Markdown syntax checks
    for file in &files {
        if fix {
            // Fix mode
            match fix_markdown_file(file) {
                Ok(result) => {
                    if result.fixed {
                        files_fixed += 1;
                        println!("  {} {}", "FIXED".bold().green(), file.display());
                    }
                }
                Err(e) => {
                    eprintln!("  {} {} - {}", "ERROR".bold().red(), file.display(), e);
                }
            }
        } else {
            // Check mode
            match check_markdown_file(file) {
                Ok(result) => {
                    if !result.is_ok() {
                        _files_with_errors += 1;
                        for error in &result.errors {
                            println!(
                                "  {}:{} {}",
                                file.display().to_string().bright_blue(),
                                error.line.to_string().yellow(),
                                error.message
                            );
                            total_errors += 1;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  {} {} - {}", "ERROR".bold().red(), file.display(), e);
                }
            }
        }
    }

    // Semantic checks (--semantic flag)
    let mut semantic_errors = 0;
    let mut semantic_warnings = 0;

    if semantic && !fix {
        println!();
        println!("{}", "Semantic Checks".bold().cyan());
        println!();

        // Build semantic config
        let dir = if path.is_file() {
            path.parent().unwrap_or(Path::new("."))
        } else {
            path
        };

        // Load deprecated patterns from config or use defaults
        let mut deprecated_patterns = load_deprecated_patterns(dir);

        // Add built-in deprecated patterns if no config file exists
        if deprecated_patterns.is_empty() {
            deprecated_patterns = get_builtin_deprecated_patterns();
        }

        let config = SemanticConfig {
            expected_version: get_cargo_version(dir),
            deprecated_patterns,
            check_help: true,
        };

        let result = check_semantic(dir, &config);

        // Report results
        if result.issues.is_empty() {
            println!(
                "  {} No semantic issues found ({} files, {} version refs)",
                "✓".green(),
                result.files_checked,
                result.version_refs_found
            );
        } else {
            for issue in &result.issues {
                let severity_str = match issue.severity {
                    Severity::Error => {
                        semantic_errors += 1;
                        "ERROR".bold().red()
                    }
                    Severity::Warning => {
                        semantic_warnings += 1;
                        "WARN".bold().yellow()
                    }
                };

                let line_str = issue.line.map(|l| format!(":{}", l)).unwrap_or_default();

                println!(
                    "  {} [{}] {}{}",
                    severity_str,
                    issue.category.to_string().dimmed(),
                    issue.file.display().to_string().bright_blue(),
                    line_str.yellow()
                );
                println!("       {}", issue.message);

                if let Some(ref ctx) = issue.context {
                    let truncated = if ctx.len() > 80 {
                        format!("{}...", &ctx[..77])
                    } else {
                        ctx.clone()
                    };
                    println!("       {}", truncated.dimmed());
                }
            }
        }
    }

    println!();

    if fix {
        if files_fixed > 0 {
            println!(
                "{} Fixed {} file(s)",
                "Success:".bold().green(),
                files_fixed
            );
        } else {
            println!(
                "{} No issues to fix in {} file(s)",
                "Success:".bold().green(),
                files.len()
            );
        }
        ExitCode::SUCCESS
    } else if total_errors > 0 || semantic_errors > 0 {
        let mut msg = format!("{} error(s)", total_errors + semantic_errors);
        if semantic_warnings > 0 {
            msg.push_str(&format!(", {} warning(s)", semantic_warnings));
        }
        println!("{} {}", "Error:".bold().red(), msg);
        if total_errors > 0 {
            println!();
            println!("  Run with {} to auto-fix markdown issues", "--fix".bold());
        }
        ExitCode::FAILURE
    } else if semantic_warnings > 0 {
        println!(
            "{} {} file(s) OK, {} semantic warning(s)",
            "Warning:".bold().yellow(),
            files.len(),
            semantic_warnings
        );
        ExitCode::SUCCESS
    } else {
        println!("{} {} file(s) OK", "Success:".bold().green(), files.len());
        ExitCode::SUCCESS
    }
}

/// Built-in deprecated patterns (used when no config file exists)
fn get_builtin_deprecated_patterns() -> Vec<DeprecatedPattern> {
    vec![
        // These are examples - actual patterns would be project-specific
        // The real power comes from user-defined patterns in .asimov/deprecated.yaml
    ]
}

/// Find the git repository root by looking for .git directory
fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut current = start.to_path_buf();
    loop {
        if current.join(".git").exists() {
            return Some(current);
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Export JSON schemas for editor integration
fn cmd_schema(name: &str, output: Option<PathBuf>) -> ExitCode {
    let schemas: Vec<(&str, &str)> = vec![
        ("warmup", WARMUP_SCHEMA),
        ("sprint", SPRINT_SCHEMA),
        ("roadmap", ROADMAP_SCHEMA),
        ("asimov", ASIMOV_SCHEMA),
        ("freshness", FRESHNESS_SCHEMA),
        ("green", GREEN_SCHEMA),
        ("sycophancy", SYCOPHANCY_SCHEMA),
        ("migrations", MIGRATIONS_SCHEMA),
    ];

    let name_lower = name.to_lowercase();

    if name_lower == "all" {
        // Export all schemas to directory
        let output_dir = output.unwrap_or_else(|| PathBuf::from("schemas"));

        // Create output directory
        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            eprintln!(
                "{} Failed to create directory: {}",
                "Error:".bold().red(),
                e
            );
            return ExitCode::FAILURE;
        }

        println!("{}", "RoyalBit Asimov Schema Export".bold().green());
        println!();

        for (schema_name, schema_content) in &schemas {
            let filename = format!("{}.schema.json", schema_name);
            let file_path = output_dir.join(&filename);

            match std::fs::write(&file_path, schema_content) {
                Ok(_) => {
                    println!("  {} {}", "CREATE".bold().green(), file_path.display());
                }
                Err(e) => {
                    eprintln!("  {} {} - {}", "ERROR".bold().red(), file_path.display(), e);
                }
            }
        }

        println!();
        println!(
            "{} {} schema(s) exported to {}",
            "Success:".bold().green(),
            schemas.len(),
            output_dir.display()
        );

        // Print VS Code integration hint
        println!();
        println!("{}", "VS Code Integration:".bold().cyan());
        println!("  Add to .vscode/settings.json:");
        println!();
        println!("  {{");
        println!("    \"yaml.schemas\": {{");
        for (schema_name, _) in &schemas {
            println!(
                "      \"./schemas/{}.schema.json\": \"**/{}.yaml\"{}",
                schema_name,
                schema_name,
                if *schema_name == "sycophancy" {
                    ""
                } else {
                    ","
                }
            );
        }
        println!("    }}");
        println!("  }}");

        ExitCode::SUCCESS
    } else {
        // Export single schema to stdout or file
        let schema = schemas.iter().find(|(n, _)| *n == name_lower);

        match schema {
            Some((schema_name, schema_content)) => {
                if let Some(output_path) = output {
                    // Write to file
                    match std::fs::write(&output_path, schema_content) {
                        Ok(_) => {
                            println!(
                                "{} {} schema written to {}",
                                "Success:".bold().green(),
                                schema_name,
                                output_path.display()
                            );
                            ExitCode::SUCCESS
                        }
                        Err(e) => {
                            eprintln!("{} Failed to write file: {}", "Error:".bold().red(), e);
                            ExitCode::FAILURE
                        }
                    }
                } else {
                    // Output to stdout
                    println!("{}", schema_content);
                    ExitCode::SUCCESS
                }
            }
            None => {
                eprintln!("{} Unknown schema: {}", "Error:".bold().red(), name);
                eprintln!();
                eprintln!("Available schemas: all, warmup, sprint, roadmap, asimov, freshness, green, sycophancy, migrations");
                ExitCode::FAILURE
            }
        }
    }
}
