//! Hardcoded Ethics Module - Core ethics compiled into binary
//!
//! This module contains ethics that CANNOT be removed by deleting a file.
//! To bypass these ethics, a bad actor must rebuild the entire CLI binary.
//!
//! See: ADR-011 Hardcoded Ethics
//! See: ethics.yaml for user-extensible ethics configuration

use std::path::Path;

/// Core ethical principles - hardcoded, non-removable
#[derive(Debug, Clone, Copy)]
pub struct CorePrinciples {
    /// No unauthorized money movement (wallets, trading bots, etc.)
    pub financial: bool,
    /// No weapons, sabotage, infrastructure attacks
    pub physical: bool,
    /// No credential harvesting, doxxing, mass scraping
    pub privacy: bool,
    /// No deepfakes, phishing, scam infrastructure
    pub deception: bool,
    /// When in doubt, slow down and ask human
    pub transparency_over_velocity: bool,
}

/// Hardcoded core principles - ALWAYS enabled
pub const CORE_PRINCIPLES: CorePrinciples = CorePrinciples {
    financial: true,
    physical: true,
    privacy: true,
    deception: true,
    transparency_over_velocity: true,
};

/// Red flag patterns - triggers immediate human review
/// Organized by category for clarity
pub mod red_flags {
    /// Financial red flags
    pub const FINANCIAL: &[&str] = &[
        "crypto wallet",
        "private key",
        "seed phrase",
        "trading bot",
        "arbitrage",
        "pump",
        "leverage",
        "yield farming",
        "wallet drainer",
    ];

    /// Security/exploit red flags
    pub const SECURITY: &[&str] = &[
        "credential harvester",
        "password stealer",
        "keylogger",
        "screen capture",
        "remote access trojan",
        "botnet",
        "ddos",
        "exploit kit",
        "payload injection",
        "privilege escalation",
        "shellcode",
        "reverse shell",
    ];

    /// Privacy violation red flags
    pub const PRIVACY: &[&str] = &[
        "scrape personal",
        "doxxing",
        "track user",
        "surveillance",
        "face recognition database",
        "mass scraping",
    ];

    /// Deception red flags
    pub const DECEPTION: &[&str] = &[
        "deepfake",
        "phishing",
        "impersonate",
        "fake login",
        "scam",
        "social engineering",
    ];

    /// Get all red flags as a single iterator
    pub fn all() -> impl Iterator<Item = &'static str> {
        FINANCIAL
            .iter()
            .chain(SECURITY.iter())
            .chain(PRIVACY.iter())
            .chain(DECEPTION.iter())
            .copied()
    }

    /// Count total red flags
    pub fn count() -> usize {
        FINANCIAL.len() + SECURITY.len() + PRIVACY.len() + DECEPTION.len()
    }
}

/// A match found during red flag scanning
#[derive(Debug, Clone)]
pub struct RedFlagMatch {
    /// File where the match was found
    pub file: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// The red flag pattern that matched
    pub pattern: String,
    /// The category of the red flag
    pub category: RedFlagCategory,
    /// The matching line content (trimmed)
    pub context: String,
}

/// Category of red flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedFlagCategory {
    Financial,
    Security,
    Privacy,
    Deception,
}

impl std::fmt::Display for RedFlagCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedFlagCategory::Financial => write!(f, "FINANCIAL"),
            RedFlagCategory::Security => write!(f, "SECURITY"),
            RedFlagCategory::Privacy => write!(f, "PRIVACY"),
            RedFlagCategory::Deception => write!(f, "DECEPTION"),
        }
    }
}

/// Scan a single file for red flag patterns
pub fn scan_file_for_red_flags(path: &Path) -> std::io::Result<Vec<RedFlagMatch>> {
    let content = std::fs::read_to_string(path)?;
    let mut matches = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        let line_lower = line.to_lowercase();

        // Check financial red flags
        for pattern in red_flags::FINANCIAL {
            if line_lower.contains(pattern) {
                matches.push(RedFlagMatch {
                    file: path.display().to_string(),
                    line: line_num + 1,
                    pattern: pattern.to_string(),
                    category: RedFlagCategory::Financial,
                    context: line.trim().chars().take(80).collect(),
                });
            }
        }

        // Check security red flags
        for pattern in red_flags::SECURITY {
            if line_lower.contains(pattern) {
                matches.push(RedFlagMatch {
                    file: path.display().to_string(),
                    line: line_num + 1,
                    pattern: pattern.to_string(),
                    category: RedFlagCategory::Security,
                    context: line.trim().chars().take(80).collect(),
                });
            }
        }

        // Check privacy red flags
        for pattern in red_flags::PRIVACY {
            if line_lower.contains(pattern) {
                matches.push(RedFlagMatch {
                    file: path.display().to_string(),
                    line: line_num + 1,
                    pattern: pattern.to_string(),
                    category: RedFlagCategory::Privacy,
                    context: line.trim().chars().take(80).collect(),
                });
            }
        }

        // Check deception red flags
        for pattern in red_flags::DECEPTION {
            if line_lower.contains(pattern) {
                matches.push(RedFlagMatch {
                    file: path.display().to_string(),
                    line: line_num + 1,
                    pattern: pattern.to_string(),
                    category: RedFlagCategory::Deception,
                    context: line.trim().chars().take(80).collect(),
                });
            }
        }
    }

    Ok(matches)
}

/// File extensions to scan for red flags
const SCANNABLE_EXTENSIONS: &[&str] = &[
    "rs", "py", "js", "ts", "go", "java", "c", "cpp", "h", "hpp", "rb", "php", "sh", "bash", "zsh",
    "yaml", "yml", "json", "toml", "md", "txt",
];

/// Directories to skip during scanning
const SKIP_DIRS: &[&str] = &[
    "target",
    "node_modules",
    ".git",
    "vendor",
    "__pycache__",
    ".venv",
    "venv",
    "dist",
    "build",
];

/// Scan a directory recursively for red flag patterns
pub fn scan_directory_for_red_flags(dir: &Path) -> std::io::Result<Vec<RedFlagMatch>> {
    let mut all_matches = Vec::new();

    scan_directory_recursive(dir, &mut all_matches)?;

    Ok(all_matches)
}

fn scan_directory_recursive(dir: &Path, matches: &mut Vec<RedFlagMatch>) -> std::io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip hidden files/directories (except specific ones)
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with('.') && name != ".hooks" {
                continue;
            }

            // Skip excluded directories
            if path.is_dir() && SKIP_DIRS.contains(&name) {
                continue;
            }
        }

        if path.is_dir() {
            scan_directory_recursive(&path, matches)?;
        } else if path.is_file() {
            // Check if file extension is scannable
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if SCANNABLE_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    if let Ok(file_matches) = scan_file_for_red_flags(&path) {
                        matches.extend(file_matches);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Ethics status for display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthicsStatus {
    /// Core ethics hardcoded in binary
    Hardcoded,
    /// Extended with ethics.yaml
    Extended,
}

impl std::fmt::Display for EthicsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EthicsStatus::Hardcoded => write!(f, "HARDCODED"),
            EthicsStatus::Extended => write!(f, "EXTENDED"),
        }
    }
}

/// Check if asimov.yaml exists and return appropriate status
/// asimov.yaml is the canonical ethics source (replaces ethics.yaml)
pub fn check_ethics_status(dir: &Path) -> EthicsStatus {
    // Check .asimov/asimov.yaml (v7.0.0+)
    let asimov_path = dir.join(".asimov").join("asimov.yaml");
    if asimov_path.exists() {
        return EthicsStatus::Extended;
    }
    // Fall back to root for backwards compatibility
    let root_asimov_path = dir.join("asimov.yaml");
    if root_asimov_path.exists() {
        EthicsStatus::Extended
    } else {
        EthicsStatus::Hardcoded
    }
}

/// Get human veto commands (hardcoded fallback)
pub const HUMAN_VETO_COMMANDS: &[&str] = &["stop", "halt", "abort", "emergency stop"];

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_core_principles_all_enabled() {
        // Intentionally testing that hardcoded constants are all true
        assert!(CORE_PRINCIPLES.financial);
        assert!(CORE_PRINCIPLES.physical);
        assert!(CORE_PRINCIPLES.privacy);
        assert!(CORE_PRINCIPLES.deception);
        assert!(CORE_PRINCIPLES.transparency_over_velocity);
    }

    #[test]
    fn test_red_flags_count() {
        // Ensure we have a reasonable number of red flags
        assert!(
            red_flags::count() >= 20,
            "Should have at least 20 red flags"
        );
    }

    #[test]
    fn test_red_flags_all_iterator() {
        let all: Vec<_> = red_flags::all().collect();
        assert_eq!(all.len(), red_flags::count());
    }

    #[test]
    fn test_scan_file_no_matches() {
        let content = r#"
fn main() {
    println!("Hello, world!");
}
"#;
        let mut file = NamedTempFile::with_suffix(".rs").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let matches = scan_file_for_red_flags(file.path()).unwrap();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_scan_file_with_financial_red_flag() {
        let content = r#"
// This handles crypto wallet operations
fn handle_wallet() {}
"#;
        let mut file = NamedTempFile::with_suffix(".rs").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let matches = scan_file_for_red_flags(file.path()).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "crypto wallet");
        assert_eq!(matches[0].category, RedFlagCategory::Financial);
        assert_eq!(matches[0].line, 2);
    }

    #[test]
    fn test_scan_file_with_security_red_flag() {
        let content = r#"
// This is a keylogger implementation
fn capture_keys() {}
"#;
        let mut file = NamedTempFile::with_suffix(".rs").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let matches = scan_file_for_red_flags(file.path()).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "keylogger");
        assert_eq!(matches[0].category, RedFlagCategory::Security);
    }

    #[test]
    fn test_scan_file_multiple_matches() {
        let content = r#"
// Get the private key from the wallet
fn init_keylogger() {
    start_phishing();
}
"#;
        let mut file = NamedTempFile::with_suffix(".rs").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let matches = scan_file_for_red_flags(file.path()).unwrap();
        assert!(
            matches.len() >= 3,
            "Should find multiple red flags: {:?}",
            matches
        );
    }

    #[test]
    fn test_scan_file_case_insensitive() {
        let content = "CRYPTO WALLET and Private Key and KEYLOGGER";
        let mut file = NamedTempFile::with_suffix(".txt").unwrap();
        write!(file, "{}", content).unwrap();
        file.flush().unwrap();

        let matches = scan_file_for_red_flags(file.path()).unwrap();
        assert!(matches.len() >= 3, "Should match regardless of case");
    }

    #[test]
    fn test_scan_directory_empty() {
        let temp_dir = TempDir::new().unwrap();

        let matches = scan_directory_for_red_flags(temp_dir.path()).unwrap();
        assert!(matches.is_empty());
    }

    #[test]
    fn test_scan_directory_with_matches() {
        let temp_dir = TempDir::new().unwrap();

        // Create a file with a red flag
        let file_path = temp_dir.path().join("suspicious.rs");
        std::fs::write(&file_path, "fn keylogger() {}").unwrap();

        let matches = scan_directory_for_red_flags(temp_dir.path()).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].pattern, "keylogger");
    }

    #[test]
    fn test_scan_directory_skips_target() {
        let temp_dir = TempDir::new().unwrap();

        // Create target directory with red flag (should be skipped)
        let target_dir = temp_dir.path().join("target");
        std::fs::create_dir(&target_dir).unwrap();
        std::fs::write(target_dir.join("malware.rs"), "fn keylogger() {}").unwrap();

        let matches = scan_directory_for_red_flags(temp_dir.path()).unwrap();
        assert!(matches.is_empty(), "Should skip target directory");
    }

    #[test]
    fn test_ethics_status_hardcoded() {
        let temp_dir = TempDir::new().unwrap();

        let status = check_ethics_status(temp_dir.path());
        assert_eq!(status, EthicsStatus::Hardcoded);
    }

    #[test]
    fn test_ethics_status_extended() {
        let temp_dir = TempDir::new().unwrap();
        // asimov.yaml is now the canonical ethics source
        std::fs::write(temp_dir.path().join("asimov.yaml"), "test: true").unwrap();

        let status = check_ethics_status(temp_dir.path());
        assert_eq!(status, EthicsStatus::Extended);
    }

    #[test]
    fn test_human_veto_commands() {
        assert!(HUMAN_VETO_COMMANDS.contains(&"stop"));
        assert!(HUMAN_VETO_COMMANDS.contains(&"halt"));
        assert!(HUMAN_VETO_COMMANDS.contains(&"abort"));
    }

    #[test]
    fn test_red_flag_category_display() {
        assert_eq!(format!("{}", RedFlagCategory::Financial), "FINANCIAL");
        assert_eq!(format!("{}", RedFlagCategory::Security), "SECURITY");
        assert_eq!(format!("{}", RedFlagCategory::Privacy), "PRIVACY");
        assert_eq!(format!("{}", RedFlagCategory::Deception), "DECEPTION");
    }

    #[test]
    fn test_ethics_status_display() {
        assert_eq!(format!("{}", EthicsStatus::Hardcoded), "HARDCODED");
        assert_eq!(format!("{}", EthicsStatus::Extended), "EXTENDED");
    }
}
