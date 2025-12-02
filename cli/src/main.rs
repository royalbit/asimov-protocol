//! RoyalBit Asimov CLI - The Three Laws of Robotics, encoded in YAML

use clap::{Parser, Subcommand};
use colored::Colorize;
use royalbit_asimov::{
    anti_patterns,
    asimov_template,
    banned_phrases,
    check_ethics_status,
    check_for_update,
    check_green_status,
    check_markdown_file,
    check_semantic,
    check_sycophancy_status,
    checkpoint_template,
    find_markdown_files,
    fix_markdown_file,
    get_cargo_version,
    green_template,
    hook_installer_template,
    is_protocol_file,
    load_deprecated_patterns,
    perform_update,
    precommit_hook_template,
    red_flags,
    roadmap_template,
    scan_directory_for_red_flags,
    sprint_template,
    sycophancy_template,
    uses_cargo_husky,
    validate_directory_with_regeneration,
    validate_file,
    warmup_template,
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

Validates protocol files against the RoyalBit Asimov specification:
  - asimov.yaml  - The Three Laws (required)
  - warmup.yaml  - Session bootstrap (required)
  - sprint.yaml  - Active work tracking (optional)
  - roadmap.yaml - Milestone planning (optional)

EXAMPLES:
  asimov validate                    # Validate all protocol files in cwd
  asimov validate warmup.yaml        # Validate specific file
  asimov init                        # Generate starter warmup.yaml (generic)
  asimov init --type rust            # Generate Rust-specific warmup.yaml
  asimov init --type python          # Generate Python-specific warmup.yaml
  asimov init --type node            # Generate Node.js-specific warmup.yaml
  asimov init --type go              # Generate Go-specific warmup.yaml
  asimov init --type flutter         # Generate Flutter-specific warmup.yaml
  asimov init --type docs            # Generate docs/architecture warmup.yaml
  asimov init --full                 # Generate all protocol files
  asimov init --asimov               # Full RoyalBit Asimov setup

TYPES: generic, rust, python (py), node (js), go (golang), flutter (dart), docs (arch)

RoyalBit Asimov (--asimov): Full autonomous session setup with The Three Laws
  - asimov.yaml (The Three Laws - required, cannot opt out)
  - All protocol files (warmup.yaml, sprint.yaml, roadmap.yaml)
  - Pre-commit hooks (.hooks/ or cargo-husky for Rust)
  - .gitignore update (exclude checkpoint file)

The Open Foundation: Creates Self-Evolving Autonomous AI projects with ethics built in.
Inspect the code. Challenge the rules. Fork if you disagree.
Adoption through consent, not control.

GREEN CODING - Why This Matters:
  - Local validation: $0/file, ~0.002g CO2, <100ms
  - Cloud AI validation: $0.02+/file, ~0.5g CO2, 1-3s
  - Team savings: $1,000-7,300/year (10-person team)
  - Carbon reduction: 99.6% vs cloud AI
  - ESG compliance: Supports corporate sustainability goals

Every project initialized with asimov is a green-coding project.
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
    }
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
        "re-read warmup.yaml".white()
    );
    println!();
    println!(
        "{}: {} | {} | {}",
        "RULES".bold(),
        "4hr max".white(),
        "tests pass".white(),
        "keep shipping".green()
    );

    // If verbose, try to read and display quality gates from warmup.yaml
    if verbose {
        if let Ok(content) = std::fs::read_to_string("warmup.yaml") {
            if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&content) {
                if let Some(quality) = yaml.get("quality") {
                    println!();
                    println!("{}", "QUALITY GATES:".bold());
                    if let Some(map) = quality.as_mapping() {
                        for (key, value) in map {
                            if let (Some(k), Some(v)) = (key.as_str(), value.as_str()) {
                                println!("  {}: {}", k.bright_blue(), v.dimmed());
                            }
                        }
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

fn cmd_init(
    name: Option<String>,
    project_type_str: &str,
    full: bool,
    asimov: bool,
    output: &Path,
    force: bool,
) -> ExitCode {
    // --asimov implies --full
    let full = full || asimov;

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

    println!("{}", "RoyalBit Asimov Init".bold().green());
    if asimov {
        println!("{}", "  RoyalBit Asimov - The Three Laws".bold().cyan());
    }
    println!();

    // Parse project type
    let project_type: ProjectType = match project_type_str.parse() {
        Ok(pt) => pt,
        Err(e) => {
            eprintln!("{} {}", "Error:".bold().red(), e);
            return ExitCode::FAILURE;
        }
    };

    // Determine project name
    let project_name = name.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "my-project".to_string())
    });

    println!("  Project: {}", project_name.bright_blue());
    println!("  Type: {}", project_type.to_string().bright_yellow());
    println!();

    // Files to generate
    let mut files: Vec<(&str, String)> =
        vec![("warmup.yaml", warmup_template(&project_name, project_type))];

    if full {
        files.push(("sprint.yaml", sprint_template()));
        files.push(("roadmap.yaml", roadmap_template()));
    }

    if asimov {
        files.push(("asimov.yaml", asimov_template()));
        files.push(("green.yaml", green_template()));
        files.push(("sycophancy.yaml", sycophancy_template()));
        // Generate example checkpoint file (will be gitignored)
        files.push((
            ".claude_checkpoint.yaml.example",
            checkpoint_template("Initial milestone"),
        ));

        // Delete deprecated CLAUDE.md if it exists
        let claude_md_path = output.join("CLAUDE.md");
        if claude_md_path.exists() {
            if let Err(e) = std::fs::remove_file(&claude_md_path) {
                eprintln!(
                    "  {} Failed to delete deprecated CLAUDE.md: {}",
                    "WARN".yellow(),
                    e
                );
            } else {
                println!(
                    "  {} Deleted deprecated CLAUDE.md (replaced by SessionStart hooks)",
                    "CLEANUP".yellow()
                );
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

    // RoyalBit Asimov: Generate hooks and update .gitignore
    if asimov {
        println!();

        // For Rust projects, show cargo-husky instructions instead of creating hooks
        if uses_cargo_husky(project_type) {
            println!("{}", "  Hooks (Rust/cargo-husky):".dimmed());
            println!(
                "    {}",
                "Add to Cargo.toml [dev-dependencies]:".bright_black()
            );
            println!(
                "    {}",
                "cargo-husky = { version = \"1\", features = [\"precommit-hook\", \"run-cargo-clippy\", \"run-cargo-fmt\"] }".bright_black()
            );
            println!("    {}", "Then run: cargo test".bright_black());
        } else {
            // Create .hooks directory and files
            let hooks_dir = output.join(".hooks");
            if let Err(e) = std::fs::create_dir_all(&hooks_dir) {
                eprintln!(
                    "  {} Failed to create .hooks directory - {}",
                    "ERROR".bold().red(),
                    e
                );
                return ExitCode::FAILURE;
            }

            // Pre-commit hook
            let precommit_path = hooks_dir.join("pre-commit");
            if !precommit_path.exists() || force {
                let hook_content = precommit_hook_template(project_type);
                match std::fs::write(&precommit_path, hook_content) {
                    Ok(_) => {
                        // Make executable on Unix
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let _ = std::fs::set_permissions(
                                &precommit_path,
                                std::fs::Permissions::from_mode(0o755),
                            );
                        }
                        println!("  {} {}", "CREATE".bold().green(), precommit_path.display());
                    }
                    Err(e) => {
                        eprintln!(
                            "  {} {} - {}",
                            "ERROR".bold().red(),
                            precommit_path.display(),
                            e
                        );
                    }
                }
            } else {
                println!(
                    "  {} {} (use --force to overwrite)",
                    "SKIP".yellow(),
                    precommit_path.display()
                );
            }

            // Install script
            let install_path = hooks_dir.join("install.sh");
            if !install_path.exists() || force {
                let install_content = hook_installer_template();
                match std::fs::write(&install_path, install_content) {
                    Ok(_) => {
                        #[cfg(unix)]
                        {
                            use std::os::unix::fs::PermissionsExt;
                            let _ = std::fs::set_permissions(
                                &install_path,
                                std::fs::Permissions::from_mode(0o755),
                            );
                        }
                        println!("  {} {}", "CREATE".bold().green(), install_path.display());
                    }
                    Err(e) => {
                        eprintln!(
                            "  {} {} - {}",
                            "ERROR".bold().red(),
                            install_path.display(),
                            e
                        );
                    }
                }
            }
        }

        // Update .gitignore
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
    }

    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Edit warmup.yaml with your project details");
    if asimov {
        if !uses_cargo_husky(project_type) {
            println!("  2. Install hooks: ./.hooks/install.sh");
            println!("  3. Launch: claude --dangerously-skip-permissions");
            println!("  4. Start: run warmup → punch it");
        } else {
            println!("  2. Add cargo-husky to Cargo.toml and run: cargo test");
            println!("  3. Launch: claude --dangerously-skip-permissions");
            println!("  4. Start: run warmup → punch it");
        }
    } else {
        println!("  2. Run: asimov validate");
    }
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
