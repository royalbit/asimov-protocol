//! Hardcoded Green Coding Module - Core sustainability principles compiled into binary
//!
//! This module contains green coding principles that CANNOT be removed by deleting a file.
//! To bypass these principles, a bad actor must rebuild the entire CLI binary.
//!
//! See: ADR-012 Hardcoded Green Coding
//! See: green.yaml for user-extensible configuration

use std::path::Path;

/// Core green coding principles - hardcoded, non-removable
#[derive(Debug, Clone, Copy)]
pub struct GreenPrinciples {
    /// Prefer CLI tools over cloud AI for routine tasks (validation, linting, formatting)
    pub local_first: bool,
    /// Reserve AI tokens for complex reasoning, not routine tasks
    pub token_efficiency: bool,
    /// Smaller binaries = less bandwidth = less energy
    pub binary_efficiency: bool,
    /// Track and minimize carbon footprint of development
    pub carbon_awareness: bool,
}

/// Hardcoded core principles - ALWAYS enabled
pub const GREEN_PRINCIPLES: GreenPrinciples = GreenPrinciples {
    local_first: true,
    token_efficiency: true,
    binary_efficiency: true,
    carbon_awareness: true,
};

/// Anti-patterns - wasteful practices to avoid
/// Organized by category for clarity
pub mod anti_patterns {
    /// Using AI for tasks that should be done locally
    pub const AI_FOR_ROUTINE: &[&str] = &[
        "ask AI to validate syntax",
        "ask AI to check formatting",
        "ask AI to run linter",
        "ask AI to run tests",
        "AI for compile check",
    ];

    /// Bloated dependency practices
    pub const BLOATED_DEPS: &[&str] = &[
        "add package for trivial function",
        "import entire library for one function",
        "unnecessary runtime dependency",
        "duplicate functionality across deps",
    ];

    /// Wasteful binary practices
    pub const UNOPTIMIZED_BUILDS: &[&str] = &[
        "ship debug build",
        "skip binary compression",
        "include debug symbols in release",
        "no LTO in release",
    ];

    /// Wasteful token usage
    pub const TOKEN_WASTE: &[&str] = &[
        "ask AI for error codes",
        "ask AI for API signatures",
        "ask AI to read documentation",
        "repeat same question",
    ];

    /// Get all anti-patterns as a single iterator
    pub fn all() -> impl Iterator<Item = &'static str> {
        AI_FOR_ROUTINE
            .iter()
            .chain(BLOATED_DEPS.iter())
            .chain(UNOPTIMIZED_BUILDS.iter())
            .chain(TOKEN_WASTE.iter())
            .copied()
    }

    /// Count total anti-patterns
    pub fn count() -> usize {
        AI_FOR_ROUTINE.len() + BLOATED_DEPS.len() + UNOPTIMIZED_BUILDS.len() + TOKEN_WASTE.len()
    }
}

/// Green coding best practices
pub mod best_practices {
    /// Local-first practices
    pub const LOCAL_FIRST: &str =
        "Use cargo test, cargo clippy, npm run lint - not AI for validation";

    /// Token efficiency practices
    pub const TOKEN_EFFICIENCY: &str =
        "Reserve AI for architecture, debugging, creativity - not syntax checking";

    /// Binary efficiency practices
    pub const BINARY_EFFICIENCY: &str =
        "Use LTO, strip symbols, UPX compression for smaller releases";

    /// Carbon awareness practices
    pub const CARBON_AWARENESS: &str = "Track token usage, minimize API calls, cache aggressively";

    /// Get all best practices
    pub fn all() -> [&'static str; 4] {
        [
            LOCAL_FIRST,
            TOKEN_EFFICIENCY,
            BINARY_EFFICIENCY,
            CARBON_AWARENESS,
        ]
    }
}

/// Category of anti-pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AntiPatternCategory {
    AiForRoutine,
    BloatedDeps,
    UnoptimizedBuilds,
    TokenWaste,
}

impl std::fmt::Display for AntiPatternCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AntiPatternCategory::AiForRoutine => write!(f, "AI_FOR_ROUTINE"),
            AntiPatternCategory::BloatedDeps => write!(f, "BLOATED_DEPS"),
            AntiPatternCategory::UnoptimizedBuilds => write!(f, "UNOPTIMIZED_BUILDS"),
            AntiPatternCategory::TokenWaste => write!(f, "TOKEN_WASTE"),
        }
    }
}

/// Green coding status for display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GreenStatus {
    /// Core green principles hardcoded in binary
    Hardcoded,
    /// Extended with green.yaml
    Extended,
}

impl std::fmt::Display for GreenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreenStatus::Hardcoded => write!(f, "HARDCODED"),
            GreenStatus::Extended => write!(f, "EXTENDED"),
        }
    }
}

/// Check if green.yaml exists and return appropriate status
/// Checks .asimov/ first, then falls back to root for backwards compatibility
pub fn check_green_status(dir: &Path) -> GreenStatus {
    // Check .asimov/ first (v6.0.0+)
    let asimov_path = dir.join(".asimov").join("green.yaml");
    if asimov_path.exists() {
        return GreenStatus::Extended;
    }
    // Fall back to root for backwards compatibility
    let green_path = dir.join("green.yaml");
    if green_path.exists() {
        GreenStatus::Extended
    } else {
        GreenStatus::Hardcoded
    }
}

/// Get the green coding motto
pub const MOTTO: &str = "Ship fast. Ship small. Ship green.";

/// Carbon savings estimates (for future metrics)
pub mod carbon {
    /// Estimated CO2 grams per cloud AI validation
    pub const CLOUD_AI_CARBON_G: f64 = 0.25;
    /// Estimated CO2 grams per local CLI validation
    pub const LOCAL_CLI_CARBON_G: f64 = 0.0005;
    /// Carbon reduction percentage (local vs cloud)
    pub const REDUCTION_PERCENT: f64 = 99.8;
}

/// Cost savings estimates (for future metrics)
pub mod cost {
    /// Estimated USD per cloud AI validation
    pub const CLOUD_AI_USD: f64 = 0.02;
    /// Local CLI cost (always zero)
    pub const LOCAL_CLI_USD: f64 = 0.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_green_principles_all_enabled() {
        // Intentionally testing that hardcoded constants are all true
        assert!(GREEN_PRINCIPLES.local_first);
        assert!(GREEN_PRINCIPLES.token_efficiency);
        assert!(GREEN_PRINCIPLES.binary_efficiency);
        assert!(GREEN_PRINCIPLES.carbon_awareness);
    }

    #[test]
    fn test_anti_patterns_count() {
        // Ensure we have a reasonable number of anti-patterns
        assert!(
            anti_patterns::count() >= 10,
            "Should have at least 10 anti-patterns"
        );
    }

    #[test]
    fn test_anti_patterns_all_iterator() {
        let all: Vec<_> = anti_patterns::all().collect();
        assert_eq!(all.len(), anti_patterns::count());
    }

    #[test]
    fn test_best_practices_count() {
        assert_eq!(best_practices::all().len(), 4);
    }

    #[test]
    fn test_green_status_hardcoded() {
        let temp_dir = TempDir::new().unwrap();

        let status = check_green_status(temp_dir.path());
        assert_eq!(status, GreenStatus::Hardcoded);
    }

    #[test]
    fn test_green_status_extended() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("green.yaml"), "test: true").unwrap();

        let status = check_green_status(temp_dir.path());
        assert_eq!(status, GreenStatus::Extended);
    }

    #[test]
    fn test_anti_pattern_category_display() {
        assert_eq!(
            format!("{}", AntiPatternCategory::AiForRoutine),
            "AI_FOR_ROUTINE"
        );
        assert_eq!(
            format!("{}", AntiPatternCategory::BloatedDeps),
            "BLOATED_DEPS"
        );
        assert_eq!(
            format!("{}", AntiPatternCategory::UnoptimizedBuilds),
            "UNOPTIMIZED_BUILDS"
        );
        assert_eq!(
            format!("{}", AntiPatternCategory::TokenWaste),
            "TOKEN_WASTE"
        );
    }

    #[test]
    fn test_green_status_display() {
        assert_eq!(format!("{}", GreenStatus::Hardcoded), "HARDCODED");
        assert_eq!(format!("{}", GreenStatus::Extended), "EXTENDED");
    }

    #[test]
    fn test_motto() {
        assert_eq!(MOTTO, "Ship fast. Ship small. Ship green.");
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_carbon_constants() {
        assert!(carbon::CLOUD_AI_CARBON_G > carbon::LOCAL_CLI_CARBON_G);
        assert!(carbon::REDUCTION_PERCENT > 99.0);
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_cost_constants() {
        assert!(cost::CLOUD_AI_USD > cost::LOCAL_CLI_USD);
        assert_eq!(cost::LOCAL_CLI_USD, 0.0);
    }
}
