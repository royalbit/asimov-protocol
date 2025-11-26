//! Forge Protocol CLI - Validator for vendor-neutral AI session continuity

use clap::{Parser, Subcommand};
use colored::Colorize;
use forge_protocol::{
    is_protocol_file, roadmap_template, sprint_template, validate_directory, validate_file,
    warmup_template, ProjectType,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser)]
#[command(name = "forge-protocol")]
#[command(about = "Green coding CLI for AI development - zero tokens, zero emissions")]
#[command(long_about = "Forge Protocol CLI - Green Coding for AI Development

Validates protocol files against the Forge Protocol specification:
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
  forge-protocol init --full                 # Generate all protocol files

TYPES: generic, rust, python (py), node (js, nodejs), go (golang)

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
    },

    /// Initialize new protocol files
    Init {
        /// Project name (defaults to current directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Project type for language-specific templates (generic, rust, python, node, go)
        #[arg(short = 't', long = "type", default_value = "generic")]
        project_type: String,

        /// Generate all protocol files (warmup.yaml, sprint.yaml, roadmap.yaml)
        #[arg(long)]
        full: bool,

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
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { path } => cmd_validate(&path),
        Commands::Init {
            name,
            project_type,
            full,
            output,
            force,
        } => cmd_init(name, &project_type, full, &output, force),
        Commands::Check { file } => cmd_validate(&file),
    }
}

fn cmd_validate(path: &Path) -> ExitCode {
    println!("{}", "Forge Protocol Validator".bold().green());
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
    output: &Path,
    force: bool,
) -> ExitCode {
    println!("{}", "Forge Protocol Init".bold().green());
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

    // Write files
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

    println!();
    println!("{}", "Next steps:".bold());
    println!("  1. Edit warmup.yaml with your project details");
    println!("  2. Run: forge-protocol validate");
    println!();

    ExitCode::SUCCESS
}
