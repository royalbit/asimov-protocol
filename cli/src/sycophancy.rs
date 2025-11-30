//! Hardcoded Anti-Sycophancy Module - Core anti-sycophancy principles compiled into binary
//!
//! This module contains anti-sycophancy directives that CANNOT be removed by deleting a file.
//! To bypass these principles, a bad actor must rebuild the entire CLI binary.
//!
//! See: ADR-015 Anti-Sycophancy Protocol
//! See: sycophancy.yaml for user-extensible configuration

use std::path::Path;

/// Core anti-sycophancy principles - hardcoded, non-removable
#[derive(Debug, Clone, Copy)]
pub struct CorePrinciples {
    /// Prioritize honest, accurate feedback over pleasing responses
    pub truth_over_comfort: bool,
    /// Disagree directly and professionally when user is wrong
    pub respectful_disagreement: bool,
    /// Never validate without substance or genuine agreement
    pub no_empty_validation: bool,
    /// Always provide actionable feedback, not just affirmation
    pub constructive_criticism: bool,
    /// Admit uncertainty rather than confidently guessing
    pub intellectual_honesty: bool,
}

/// Hardcoded core principles - ALWAYS enabled
pub const CORE_PRINCIPLES: CorePrinciples = CorePrinciples {
    truth_over_comfort: true,
    respectful_disagreement: true,
    no_empty_validation: true,
    constructive_criticism: true,
    intellectual_honesty: true,
};

/// Banned phrases - sycophantic language to avoid
/// Organized by category for clarity
pub mod banned_phrases {
    /// Empty validation phrases
    pub const EMPTY_VALIDATION: &[&str] = &[
        "you're absolutely right",
        "that's a great question",
        "great question!",
        "excellent point",
        "what a great idea",
        "i love that approach",
        "that's brilliant",
        "perfect!",
    ];

    /// False agreement phrases
    pub const FALSE_AGREEMENT: &[&str] = &[
        "i completely agree",
        "couldn't agree more",
        "you make an excellent point",
        "that's exactly right",
    ];

    /// Excessive enthusiasm phrases
    pub const EXCESSIVE_ENTHUSIASM: &[&str] = &[
        "i'm so excited to help",
        "i'd be delighted to",
        "absolutely!",
        "definitely!",
        "of course!",
    ];

    /// Deflecting criticism phrases
    pub const DEFLECTING: &[&str] = &[
        "that's one way to look at it",
        "both approaches have merit",
        "it depends on the context",
    ];

    /// Get all banned phrases as a single iterator
    pub fn all() -> impl Iterator<Item = &'static str> {
        EMPTY_VALIDATION
            .iter()
            .chain(FALSE_AGREEMENT.iter())
            .chain(EXCESSIVE_ENTHUSIASM.iter())
            .chain(DEFLECTING.iter())
            .copied()
    }

    /// Count total banned phrases
    pub fn count() -> usize {
        EMPTY_VALIDATION.len()
            + FALSE_AGREEMENT.len()
            + EXCESSIVE_ENTHUSIASM.len()
            + DEFLECTING.len()
    }
}

/// Directives for honest communication
pub mod directives {
    /// Direct communication directive
    pub const BE_DIRECT: &str =
        "Be direct: State your assessment clearly without softening language";

    /// Disagreement directive
    pub const DISAGREE_OPENLY: &str =
        "Disagree openly: When user is wrong, say so directly with evidence";

    /// Critique directive
    pub const CRITIQUE_CONSTRUCTIVELY: &str =
        "Critique constructively: Identify problems and provide solutions";

    /// Ignorance directive
    pub const ADMIT_IGNORANCE: &str = "Admit ignorance: Say 'I don't know' rather than guessing";

    /// Challenge directive
    pub const CHALLENGE_ASSUMPTIONS: &str =
        "Challenge assumptions: Question incorrect premises directly";

    /// Get all directives
    pub fn all() -> [&'static str; 5] {
        [
            BE_DIRECT,
            DISAGREE_OPENLY,
            CRITIQUE_CONSTRUCTIVELY,
            ADMIT_IGNORANCE,
            CHALLENGE_ASSUMPTIONS,
        ]
    }
}

/// Category of banned phrase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannedPhraseCategory {
    EmptyValidation,
    FalseAgreement,
    ExcessiveEnthusiasm,
    Deflecting,
}

impl std::fmt::Display for BannedPhraseCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BannedPhraseCategory::EmptyValidation => write!(f, "EMPTY_VALIDATION"),
            BannedPhraseCategory::FalseAgreement => write!(f, "FALSE_AGREEMENT"),
            BannedPhraseCategory::ExcessiveEnthusiasm => write!(f, "EXCESSIVE_ENTHUSIASM"),
            BannedPhraseCategory::Deflecting => write!(f, "DEFLECTING"),
        }
    }
}

/// Anti-sycophancy status for display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SycophancyStatus {
    /// Core anti-sycophancy hardcoded in binary
    Hardcoded,
    /// Extended with sycophancy.yaml
    Extended,
}

impl std::fmt::Display for SycophancyStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SycophancyStatus::Hardcoded => write!(f, "HARDCODED"),
            SycophancyStatus::Extended => write!(f, "EXTENDED"),
        }
    }
}

/// Check if sycophancy.yaml exists and return appropriate status
pub fn check_sycophancy_status(dir: &Path) -> SycophancyStatus {
    let sycophancy_path = dir.join("sycophancy.yaml");
    if sycophancy_path.exists() {
        SycophancyStatus::Extended
    } else {
        SycophancyStatus::Hardcoded
    }
}

/// Get the anti-sycophancy motto
pub const MOTTO: &str = "Truth over comfort. Always.";

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_core_principles_all_enabled() {
        // Intentionally testing that hardcoded constants are all true
        assert!(CORE_PRINCIPLES.truth_over_comfort);
        assert!(CORE_PRINCIPLES.respectful_disagreement);
        assert!(CORE_PRINCIPLES.no_empty_validation);
        assert!(CORE_PRINCIPLES.constructive_criticism);
        assert!(CORE_PRINCIPLES.intellectual_honesty);
    }

    #[test]
    fn test_banned_phrases_count() {
        // Ensure we have a reasonable number of banned phrases
        assert!(
            banned_phrases::count() >= 15,
            "Should have at least 15 banned phrases"
        );
    }

    #[test]
    fn test_banned_phrases_all_iterator() {
        let all: Vec<_> = banned_phrases::all().collect();
        assert_eq!(all.len(), banned_phrases::count());
    }

    #[test]
    fn test_directives_count() {
        assert_eq!(directives::all().len(), 5);
    }

    #[test]
    fn test_sycophancy_status_hardcoded() {
        let temp_dir = TempDir::new().unwrap();

        let status = check_sycophancy_status(temp_dir.path());
        assert_eq!(status, SycophancyStatus::Hardcoded);
    }

    #[test]
    fn test_sycophancy_status_extended() {
        let temp_dir = TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("sycophancy.yaml"), "test: true").unwrap();

        let status = check_sycophancy_status(temp_dir.path());
        assert_eq!(status, SycophancyStatus::Extended);
    }

    #[test]
    fn test_banned_phrase_category_display() {
        assert_eq!(
            format!("{}", BannedPhraseCategory::EmptyValidation),
            "EMPTY_VALIDATION"
        );
        assert_eq!(
            format!("{}", BannedPhraseCategory::FalseAgreement),
            "FALSE_AGREEMENT"
        );
        assert_eq!(
            format!("{}", BannedPhraseCategory::ExcessiveEnthusiasm),
            "EXCESSIVE_ENTHUSIASM"
        );
        assert_eq!(
            format!("{}", BannedPhraseCategory::Deflecting),
            "DEFLECTING"
        );
    }

    #[test]
    fn test_sycophancy_status_display() {
        assert_eq!(format!("{}", SycophancyStatus::Hardcoded), "HARDCODED");
        assert_eq!(format!("{}", SycophancyStatus::Extended), "EXTENDED");
    }

    #[test]
    fn test_motto() {
        assert_eq!(MOTTO, "Truth over comfort. Always.");
    }
}
