//! Forge Protocol CLI - Validator for vendor-neutral AI session continuity

use clap::{Parser, Subcommand};
use colored::Colorize;
use forge_protocol::{
    check_ethics_status, check_markdown_file, checkpoint_template, claude_md_template,
    ethics_template, find_markdown_files, fix_markdown_file, hook_installer_template,
    is_protocol_file, precommit_hook_template, red_flags, roadmap_template,
    scan_directory_for_red_flags, sprint_template, uses_cargo_husky, validate_directory,
    validate_file, warmup_template, EthicsStatus, ProjectType, CORE_PRINCIPLES,
    HUMAN_VETO_COMMANDS,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "forge-protocol")]
#[command(about = "Green coding CLI for AI development - zero tokens, zero emissions")]
#[command(long_about = "Forge Protocol CLI - Ethical AI Development

Validates protocol files against the Forge Protocol specification:
  - ethics.yaml  - Humanist Mode safeguards (required in SKYNET)
  - warmup.yaml  - Session bootstrap (required)
  - sprint.yaml  - Active work tracking (optional)
  - roadmap.yaml - Milestone planning (optional)

EXAMPLES:
  forge-protocol validate                    # Validate all protocol files in cwd
  forge-protocol validate warmup.yaml        # Validate specific file
  forge-protocol init                        # Generate starter warmup.yaml (generic)
  forge-protocol init --type rust            # Generate Rust-specific warmup.yaml
  forge-protocol init --type python          # Generate Python-specific warmup.yaml
  forge-protocol init --type node            # Generate Node.js-specific warmup.yaml
  forge-protocol init --type go              # Generate Go-specific warmup.yaml
  forge-protocol init --type flutter         # Generate Flutter-specific warmup.yaml
  forge-protocol init --type docs            # Generate docs/architecture warmup.yaml
  forge-protocol init --full                 # Generate all protocol files
  forge-protocol init --skynet               # Full SKYNET MODE setup

TYPES: generic, rust, python (py), node (js), go (golang), flutter (dart), docs (arch)

SKYNET MODE (--skynet): Full autonomous session setup
  - ethics.yaml (Humanist Mode - required, cannot opt out)
  - All protocol files (warmup.yaml, sprint.yaml, roadmap.yaml)
  - CLAUDE.md (auto-loaded by Claude Code)
  - Pre-commit hooks (.hooks/ or cargo-husky for Rust)
  - .gitignore update (exclude checkpoint file)

GREEN CODING - Why This Matters:
  - Local validation: $0/file, ~0.002g CO2, <100ms
  - Cloud AI validation: $0.02+/file, ~0.5g CO2, 1-3s
  - Team savings: $1,000-7,300/year (10-person team)
  - Carbon reduction: 99.6% vs cloud AI
  - ESG compliance: Supports corporate sustainability goals

Every project initialized with forge-protocol is a green-coding project.
Zero tokens. Zero emissions. Ship fast.

Docs: https://github.com/royalbit/forge-protocol")]
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

        /// Full SKYNET MODE setup (protocol files + CLAUDE.md + hooks + .gitignore)
        #[arg(long)]
        skynet: bool,

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
    },

    /// Refresh protocol context (for git hooks - injects rules into fresh context)
    Refresh {
        /// Show quality gates from warmup.yaml
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path, ethics_scan } => cmd_validate(&path, ethics_scan),
        Commands::Init {
            name,
            project_type,
            full,
            skynet,
            output,
            force,
        } => cmd_init(name, &project_type, full, skynet, &output, force),
        Commands::Check { file } => cmd_validate(&file, false),
        Commands::LintDocs { path, fix } => cmd_lint_docs(&path, fix),
        Commands::Refresh { verbose } => cmd_refresh(verbose),
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
        "🤖 SKYNET MODE - PROTOCOL REFRESH".bold().bright_cyan()
    );
    println!(
        "{}",
        "═══════════════════════════════════════════════════════════════════════".bright_cyan()
    );
    println!();

    // Ethics reminder (hardcoded - cannot be removed)
    println!("{}", "[FORGE ETHICS] Core principles ACTIVE".bold().green());
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

    println!(
        "{} → {}",
        "ON CONFUSION".bold().yellow(),
        "re-read warmup.yaml".white()
    );
    println!();
    println!(
        "{}: {} | {} | {} | {}",
        "RULES".bold(),
        "4hr max".white(),
        "1 milestone".white(),
        "tests pass".white(),
        "ship it".green()
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

fn cmd_validate(path: &Path, ethics_scan: bool) -> ExitCode {
    println!("{}", "Forge Protocol Validator".bold().green());
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
        EthicsStatus::Extended => "EXTENDED (core + ethics.yaml)".bright_green(),
    };
    println!("  {} Ethics: {}", "✓".green(), ethics_display);
    println!();

    let results = if path.is_file() {
        // Validate single file
        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !is_protocol_file(filename) {
            eprintln!(
                "{} Not a protocol file. Expected warmup.yaml, sprint.yaml, or roadmap.yaml",
                "Error:".bold().red()
            );
            return ExitCode::FAILURE;
        }

        match validate_file(path) {
            Ok(result) => vec![result],
            Err(e) => {
                eprintln!("{} {}", "Error:".bold().red(), e);
                return ExitCode::FAILURE;
            }
        }
    } else {
        // Validate directory
        match validate_directory(path) {
            Ok(results) => results,
            Err(e) => {
                eprintln!("{} {}", "Error:".bold().red(), e);
                return ExitCode::FAILURE;
            }
        }
    };

    // Print results
    let mut has_errors = false;

    for result in &results {
        let status = if result.is_valid {
            "OK".bold().green()
        } else {
            has_errors = true;
            "FAIL".bold().red()
        };

        println!(
            "  {} {} ({})",
            status,
            result.file.bright_blue(),
            result.schema_type.dimmed()
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
        println!(
            "{} {} file(s) valid",
            "Success:".bold().green(),
            results.len()
        );
        ExitCode::SUCCESS
    }
}

fn cmd_init(
    name: Option<String>,
    project_type_str: &str,
    full: bool,
    skynet: bool,
    output: &Path,
    force: bool,
) -> ExitCode {
    // --skynet implies --full
    let full = full || skynet;

    println!("{}", "Forge Protocol Init".bold().green());
    if skynet {
        println!("{}", "  SKYNET MODE enabled".bold().cyan());
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

    if skynet {
        files.push(("CLAUDE.md", claude_md_template(&project_name, project_type)));
        files.push(("ethics.yaml", ethics_template()));
        // Generate example checkpoint file (will be gitignored)
        files.push((
            ".claude_checkpoint.yaml.example",
            checkpoint_template("Initial milestone"),
        ));
    }

    // Write protocol files
    for (filename, content) in &files {
        let file_path = output.join(filename);

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

    // SKYNET MODE: Generate hooks and update .gitignore
    if skynet {
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
            content.push_str("\n# SKYNET MODE checkpoint (session-specific)\n");
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
    if skynet {
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
        println!("  2. Run: forge-protocol validate");
    }
    println!();

    ExitCode::SUCCESS
}

fn cmd_lint_docs(path: &Path, fix: bool) -> ExitCode {
    println!("{}", "Forge Protocol Documentation Linter".bold().green());
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
    let mut files_with_errors = 0;
    let mut files_fixed = 0;

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
                        files_with_errors += 1;
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
    } else if total_errors > 0 {
        println!(
            "{} {} error(s) in {} file(s)",
            "Error:".bold().red(),
            total_errors,
            files_with_errors
        );
        println!();
        println!("  Run with {} to auto-fix", "--fix".bold());
        ExitCode::FAILURE
    } else {
        println!("{} {} file(s) OK", "Success:".bold().green(), files.len());
        ExitCode::SUCCESS
    }
}
