//! External Protocol Module - Protocols loaded from .asimov/ with embedded fallback (ADR-053)
//!
//! v10.2.3: Single source of truth - cli/protocols/*.json files are embedded at compile time.
//! These are copied to .asimov/protocols/ on init/refresh for runtime customization.
//! Supersedes ADR-031 (hardcoded protocols).
//! v10.8.0: Migrations protocol removed (ADR-062) - now part of API templates.

use serde::{Deserialize, Serialize};

// ========== Embedded JSON Protocols (compile-time from cli/protocols/) ==========
// Single source of truth: JSON files in cli/protocols/ are embedded at compile time.
// Runtime: External files in .asimov/protocols/ take priority if they exist.

/// Asimov protocol - Harm prevention inspired by Asimov's Three Laws (Priority 0)
const ASIMOV_JSON: &str = include_str!("../../protocols/asimov.json");

/// Freshness protocol - Date-aware search (Priority 1)
const FRESHNESS_JSON: &str = include_str!("../../protocols/freshness.json");

/// Sycophancy protocol - Truth over comfort (Priority 1.5)
const SYCOPHANCY_JSON: &str = include_str!("../../protocols/sycophancy.json");

/// Green protocol - Local-first (Priority 0.5)
const GREEN_JSON: &str = include_str!("../../protocols/green.json");

/// Sprint protocol - Session boundaries (Priority 2)
const SPRINT_JSON: &str = include_str!("../../protocols/sprint.json");

/// Warmup protocol - Session bootstrap (Priority 0)
const WARMUP_JSON: &str = include_str!("../../protocols/warmup.json");

// v10.8.0: Migrations protocol removed (ADR-062) - now part of API templates

/// Coding Standards protocol - Human-readable code (Priority 1)
const CODING_STANDARDS_JSON: &str = include_str!("../../protocols/coding-standards.json");

// ========== Protocol Directory ==========

/// Get the protocols directory path
pub fn protocols_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(".asimov/protocols")
}

/// Try to read a protocol from external file, return None if not found
fn try_read_protocol(name: &str) -> Option<String> {
    let path = protocols_dir().join(format!("{}.json", name));
    std::fs::read_to_string(&path).ok()
}

/// Compiled protocol context for minimal token usage
/// v10.0.0: Now uses owned String types for external file support
/// v10.8.0: Migrations removed (ADR-062) - now part of API templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledProtocols {
    pub asimov: AsimovProtocol,
    pub freshness: FreshnessProtocol,
    pub sycophancy: SycophancyProtocol,
    pub green: GreenProtocol,
    pub sprint: SprintProtocol,
    pub warmup: WarmupProtocol,
    pub coding_standards: CodingStandardsProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsimovProtocol {
    /// Protocol description (v11.0.0: added for professional documentation)
    #[serde(default)]
    pub description: Option<String>,
    pub harm: Vec<String>,
    pub veto: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessProtocol {
    /// Use ref fetch for online content (bypasses bot protection)
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SycophancyProtocol {
    pub truth_over_comfort: bool,
    pub disagree_openly: bool,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreenProtocol {
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SprintProtocol {
    pub rule: String,
    /// Compaction reminder - survives context summarization (merged from exhaustive protocol ADR-049)
    pub compaction_reminder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmupProtocol {
    pub on_start: Vec<String>,
}

/// Warmup entry point - v10.6.0: simplified (ADR-060)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmupEntry {
    pub on_start: Vec<String>,
}

// v10.8.0: MigrationsProtocol removed (ADR-062) - now part of API templates

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodingStandardsProtocol {
    pub philosophy: String,
    pub rfc2119: Rfc2119Rules,
    pub principles: Vec<String>,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rfc2119Rules {
    #[serde(rename = "MUST")]
    pub must: String,
    #[serde(rename = "SHOULD")]
    pub should: String,
    #[serde(rename = "MAY")]
    pub may: String,
}

/// Get today's date in YYYY-MM-DD format
fn get_today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

/// Get current year
fn get_year() -> String {
    chrono::Local::now().format("%Y").to_string()
}

/// Inject dynamic dates into a protocol template
pub fn inject_dates(template: &str) -> String {
    template
        .replace("{TODAY}", &get_today())
        .replace("{YEAR}", &get_year())
}

/// Get embedded protocol JSON (for debugging/inspection)
pub fn get_asimov_protocol() -> &'static str {
    ASIMOV_JSON
}

pub fn get_freshness_protocol() -> &'static str {
    FRESHNESS_JSON
}

pub fn get_sycophancy_protocol() -> &'static str {
    SYCOPHANCY_JSON
}

pub fn get_green_protocol() -> &'static str {
    GREEN_JSON
}

pub fn get_sprint_protocol() -> &'static str {
    SPRINT_JSON
}

pub fn get_warmup_protocol() -> &'static str {
    WARMUP_JSON
}

// v10.8.0: get_migrations_protocol removed (ADR-062) - now part of API templates

pub fn get_coding_standards_protocol() -> &'static str {
    CODING_STANDARDS_JSON
}

/// Compile all protocols into a minimal JSON blob for context injection
/// v10.8.0: Migrations removed (ADR-062) - now part of API templates
pub fn compile_protocols() -> CompiledProtocols {
    // Try to load from external files, fall back to embedded defaults
    let asimov = load_asimov_protocol();
    let freshness = load_freshness_protocol();
    let sycophancy = load_sycophancy_protocol();
    let green = load_green_protocol();
    let sprint = load_sprint_protocol();
    let warmup = load_warmup_protocol();
    let coding_standards = load_coding_standards_protocol();

    CompiledProtocols {
        asimov,
        freshness,
        sycophancy,
        green,
        sprint,
        warmup,
        coding_standards,
    }
}

// ========== Individual Protocol Loaders (External + Fallback) ==========

fn load_asimov_protocol() -> AsimovProtocol {
    // Try external file first, then embedded JSON
    try_read_protocol("asimov")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(ASIMOV_JSON).expect("Embedded asimov.json must be valid")
        })
}

fn load_freshness_protocol() -> FreshnessProtocol {
    try_read_protocol("freshness")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(FRESHNESS_JSON).expect("Embedded freshness.json must be valid")
        })
}

fn load_sycophancy_protocol() -> SycophancyProtocol {
    try_read_protocol("sycophancy")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(SYCOPHANCY_JSON).expect("Embedded sycophancy.json must be valid")
        })
}

fn load_green_protocol() -> GreenProtocol {
    try_read_protocol("green")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(GREEN_JSON).expect("Embedded green.json must be valid")
        })
}

fn load_sprint_protocol() -> SprintProtocol {
    try_read_protocol("sprint")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(SPRINT_JSON).expect("Embedded sprint.json must be valid")
        })
}

fn load_warmup_protocol() -> WarmupProtocol {
    try_read_protocol("warmup")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(WARMUP_JSON).expect("Embedded warmup.json must be valid")
        })
}

// v10.8.0: load_migrations_protocol removed (ADR-062) - now part of API templates

fn load_coding_standards_protocol() -> CodingStandardsProtocol {
    try_read_protocol("coding-standards")
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_else(|| {
            serde_json::from_str(CODING_STANDARDS_JSON)
                .expect("Embedded coding-standards.json must be valid")
        })
}

/// Output compiled protocols as minified JSON (includes all protocols)
/// v10.8.0: Migrations removed (ADR-062) - now part of API templates
pub fn to_minified_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string(&protocols).expect("Protocol serialization should never fail")
}

// v10.8.0: to_minified_json_for_type removed (ADR-062) - migrations was the only conditional

/// Output compiled protocols as pretty JSON (for debugging)
pub fn to_pretty_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols).expect("Protocol serialization should never fail")
}

/// Output compiled protocols as YAML
pub fn to_yaml() -> String {
    let protocols = compile_protocols();
    serde_yaml_ng::to_string(&protocols).expect("Protocol serialization should never fail")
}

// ========== Individual Protocol JSON Output (v8.14.0) ==========

/// Get warmup entry point JSON - v10.6.0: simplified (ADR-060)
pub fn warmup_entry_json() -> String {
    let entry = WarmupEntry {
        on_start: vec![
            "load_protocols".into(),
            "load_project".into(),
            "validate".into(),
            "read_roadmap".into(),
            "present_milestone".into(),
        ],
    };
    serde_json::to_string_pretty(&entry).expect("Warmup entry serialization should never fail")
}

/// Get asimov protocol JSON (harm prevention)
pub fn asimov_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.asimov).expect("Asimov serialization should never fail")
}

/// Get freshness protocol JSON (date-aware search)
pub fn freshness_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.freshness)
        .expect("Freshness serialization should never fail")
}

/// Get sycophancy protocol JSON (truth over comfort)
pub fn sycophancy_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.sycophancy)
        .expect("Sycophancy serialization should never fail")
}

/// Get green protocol JSON (local-first)
pub fn green_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.green).expect("Green serialization should never fail")
}

/// Get sprint protocol JSON (session boundaries)
pub fn sprint_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.sprint).expect("Sprint serialization should never fail")
}

// v10.8.0: migrations_json removed (ADR-062) - now part of API templates

/// Get coding standards protocol JSON (human-readable code)
pub fn coding_standards_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.coding_standards)
        .expect("CodingStandards serialization should never fail")
}

/// Protocol files to write
/// v10.8.0: Migrations removed (ADR-062) - now part of API templates
#[allow(clippy::type_complexity)]
pub const PROTOCOL_FILES: &[(&str, fn() -> String)] = &[
    ("warmup.json", warmup_entry_json),
    ("asimov.json", asimov_json),
    ("freshness.json", freshness_json),
    ("sycophancy.json", sycophancy_json),
    ("green.json", green_json),
    ("sprint.json", sprint_json),
    ("coding-standards.json", coding_standards_json),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_injection() {
        let template = "Today is {TODAY}, year {YEAR}";
        let result = inject_dates(template);
        assert!(result.contains(&get_year()));
        assert!(!result.contains("{TODAY}"));
        assert!(!result.contains("{YEAR}"));
    }

    #[test]
    fn test_compile_protocols() {
        let protocols = compile_protocols();
        assert_eq!(protocols.asimov.harm.len(), 4);
        assert!(protocols.asimov.description.is_some()); // v11.0.0: Must have description
        assert!(protocols.freshness.rule.contains("ref fetch")); // v10.1.0: Use ref for fetching
        assert!(protocols.sycophancy.truth_over_comfort);
        assert!(protocols.green.rule.contains("efficiency")); // Must check efficiency
        assert!(protocols.sprint.compaction_reminder.contains("compaction")); // Must survive compaction (merged from exhaustive ADR-049)
        assert!(protocols
            .coding_standards
            .philosophy
            .contains("Human-readable")); // Must enforce standards
    }

    #[test]
    fn test_minified_json_output() {
        let json = to_minified_json();
        // Should be one line, no pretty printing
        assert!(!json.contains('\n'));
        // Should contain all protocols (v10.8.0: migrations removed ADR-062)
        assert!(json.contains("\"asimov\""));
        assert!(json.contains("\"freshness\""));
        assert!(json.contains("\"sycophancy\""));
        assert!(json.contains("\"green\""));
        assert!(json.contains("\"sprint\""));
        assert!(json.contains("\"warmup\""));
        assert!(!json.contains("\"migrations\"")); // v10.8.0: removed (ADR-062)
        assert!(json.contains("\"coding_standards\""));
        assert!(json.contains("\"compaction_reminder\"")); // Merged from exhaustive
    }

    #[test]
    fn test_embedded_json_protocols_exist() {
        // v10.1.0: Single source of truth - embedded JSON files
        // v10.8.0: Migrations removed (ADR-062) - now part of API templates
        // v11.0.0: Professional language audit
        // These will fail at compile time if JSON files don't exist
        assert!(ASIMOV_JSON.contains("harm"));
        assert!(ASIMOV_JSON.contains("description")); // v11.0.0: Must have description
        assert!(FRESHNESS_JSON.contains("ref fetch")); // v10.1.0: Use ref for fetching
        assert!(SYCOPHANCY_JSON.contains("accuracy")); // v11.0.0: Professional language
        assert!(GREEN_JSON.contains("efficiency")); // Must check efficiency
        assert!(SPRINT_JSON.contains("autonomous"));
        assert!(SPRINT_JSON.contains("compaction")); // v9.14.0: Compaction reminder merged from exhaustive
        assert!(WARMUP_JSON.contains("protocol"));
        // v10.8.0: MIGRATIONS_JSON removed (ADR-062)
        assert!(CODING_STANDARDS_JSON.contains("Human-readable"));
    }

    #[test]
    fn test_get_protocol_functions() {
        // v10.1.0: Test all get_*_protocol functions (now return embedded JSON)
        // v10.8.0: Migrations removed (ADR-062) - now part of API templates
        // v11.0.0: Professional language audit
        let asimov = get_asimov_protocol();
        assert!(asimov.contains("harm"));
        assert!(asimov.contains("description")); // v11.0.0: Must have description

        let freshness = get_freshness_protocol();
        assert!(freshness.contains("ref fetch")); // v10.1.0: Use ref for fetching

        let sycophancy = get_sycophancy_protocol();
        assert!(sycophancy.contains("accuracy")); // v11.0.0: Professional language

        let green = get_green_protocol();
        assert!(green.contains("efficiency")); // Must check efficiency

        let sprint = get_sprint_protocol();
        assert!(sprint.contains("autonomous"));
        assert!(sprint.contains("compaction")); // v9.14.0: Compaction reminder merged from exhaustive

        let warmup = get_warmup_protocol();
        assert!(warmup.contains("protocol"));

        // v10.8.0: get_migrations_protocol removed (ADR-062)

        let coding_standards = get_coding_standards_protocol();
        assert!(coding_standards.contains("Human-readable"));
    }

    #[test]
    fn test_to_pretty_json() {
        let json = to_pretty_json();
        // Should be multi-line (pretty printed)
        assert!(json.contains('\n'));
        assert!(json.contains("\"asimov\""));
    }

    #[test]
    fn test_to_yaml() {
        let yaml = to_yaml();
        // Should be valid YAML
        assert!(yaml.contains("asimov:"));
    }

    #[test]
    fn test_individual_protocol_json() {
        // Test each individual protocol JSON generator
        // v10.8.0: Migrations removed (ADR-062) - now part of API templates
        let warmup = warmup_entry_json();
        assert!(warmup.contains("\"on_start\"")); // v10.6.0: simplified (ADR-060)
        assert!(!warmup.contains("\"load\"")); // v10.6.0: removed (ADR-060)

        let asimov = asimov_json();
        assert!(asimov.contains("\"harm\""));

        let freshness = freshness_json();
        assert!(freshness.contains("\"rule\""));

        let sycophancy = sycophancy_json();
        assert!(sycophancy.contains("\"truth_over_comfort\""));

        let green = green_json();
        assert!(green.contains("\"rule\""));

        let sprint = sprint_json();
        assert!(sprint.contains("\"rule\""));
        assert!(sprint.contains("\"compaction_reminder\"")); // v9.14.0: Merged from exhaustive

        // v10.8.0: migrations_json removed (ADR-062)

        let coding_standards = coding_standards_json();
        assert!(coding_standards.contains("\"philosophy\""));
        assert!(coding_standards.contains("\"rfc2119\""));
    }

    #[test]
    fn test_protocol_files_constant() {
        // Test that PROTOCOL_FILES has expected entries
        // v10.8.0: migrations removed (ADR-062) - now part of API templates
        assert_eq!(PROTOCOL_FILES.len(), 7); // v10.8.0: removed migrations (was 8)
        let filenames: Vec<_> = PROTOCOL_FILES.iter().map(|(name, _)| *name).collect();
        assert!(filenames.contains(&"warmup.json"));
        assert!(filenames.contains(&"asimov.json"));
        assert!(filenames.contains(&"freshness.json"));
        assert!(filenames.contains(&"coding-standards.json"));
        assert!(!filenames.contains(&"exhaustive.json")); // v9.14.0: Merged into sprint
        assert!(!filenames.contains(&"kingship.json")); // v10.5.0: Deleted (ADR-059)
        assert!(!filenames.contains(&"migrations.json")); // v10.8.0: Now in API templates (ADR-062)
    }

    // v10.8.0: Migrations protocol tests removed (ADR-062) - now part of API templates
    // The conditional migrations protocol tests have been removed since migrations
    // is no longer a protocol but part of the API templates (api-rust, api-go, etc.)
}
