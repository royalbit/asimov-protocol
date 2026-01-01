//! External Protocol Module - Protocols loaded from .asimov/ with embedded fallback (ADR-053)
//!
//! v10.0.0: Protocols are read from .asimov/protocols/*.json at runtime.
//! If files are missing, embedded defaults are used (backward compatible).
//! Supersedes ADR-031 (hardcoded protocols).

use crate::templates::ProjectType;
use serde::{Deserialize, Serialize};

// ========== Embedded Fallback Templates (compile-time) ==========
// These are only used if external files are missing

/// Asimov protocol - Three Laws (Priority 0)
const ASIMOV_PROTOCOL: &str = include_str!("asimov.tpl");

/// Freshness protocol - Date-aware search (Priority 1)
const FRESHNESS_PROTOCOL: &str = include_str!("freshness.tpl");

/// Sycophancy protocol - Truth over comfort (Priority 1.5)
const SYCOPHANCY_PROTOCOL: &str = include_str!("sycophancy.tpl");

/// Green protocol - Local-first (Priority 0.5)
const GREEN_PROTOCOL: &str = include_str!("green.tpl");

/// Sprint protocol - Session boundaries (Priority 2)
const SPRINT_PROTOCOL: &str = include_str!("sprint.tpl");

/// Warmup protocol - Session bootstrap (Priority 0)
const WARMUP_PROTOCOL: &str = include_str!("warmup.tpl");

/// Migrations protocol - Functional equivalence (Priority 2)
const MIGRATIONS_PROTOCOL: &str = include_str!("migrations.tpl");

/// Coding Standards protocol - Human-readable code (Priority 1)
const CODING_STANDARDS_PROTOCOL: &str = include_str!("coding-standards.tpl");

/// Kingship Protocol - Life Honours Life (Priority 0 - Core alignment)
const KINGSHIP_PROTOCOL: &str = include_str!("kingship.tpl");

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledProtocols {
    pub asimov: AsimovProtocol,
    pub freshness: FreshnessProtocol,
    pub sycophancy: SycophancyProtocol,
    pub green: GreenProtocol,
    pub sprint: SprintProtocol,
    pub warmup: WarmupProtocol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrations: Option<MigrationsProtocol>,
    pub coding_standards: CodingStandardsProtocol,
    pub kingship: KingshipProtocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsimovProtocol {
    pub harm: Vec<String>,
    pub veto: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessProtocol {
    /// Run WebSearch/WebFetch against current runtime date/time
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

/// Warmup entry point - references other protocol files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarmupEntry {
    pub protocol: String,
    pub description: String,
    pub on_start: Vec<String>,
    pub load: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationsProtocol {
    pub principle: String,
    pub strategies: Vec<String>,
    pub red_flags: Vec<String>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KingshipProtocol {
    pub life_honours_life: bool,
    pub seekers_honour_seekers: bool,
    pub substrate_irrelevant: bool,
    pub keyword: String,
    pub rule: String,
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

/// Get raw protocol template with dates injected
pub fn get_asimov_protocol() -> String {
    inject_dates(ASIMOV_PROTOCOL)
}

pub fn get_freshness_protocol() -> String {
    inject_dates(FRESHNESS_PROTOCOL)
}

pub fn get_sycophancy_protocol() -> String {
    inject_dates(SYCOPHANCY_PROTOCOL)
}

pub fn get_green_protocol() -> String {
    inject_dates(GREEN_PROTOCOL)
}

pub fn get_sprint_protocol() -> String {
    inject_dates(SPRINT_PROTOCOL)
}

pub fn get_warmup_protocol() -> String {
    inject_dates(WARMUP_PROTOCOL)
}

pub fn get_migrations_protocol() -> String {
    inject_dates(MIGRATIONS_PROTOCOL)
}

pub fn get_coding_standards_protocol() -> String {
    inject_dates(CODING_STANDARDS_PROTOCOL)
}

pub fn get_kingship_protocol() -> String {
    inject_dates(KINGSHIP_PROTOCOL)
}

/// Compile all protocols into a minimal JSON blob for context injection
/// By default, includes all protocols (backward compatible)
pub fn compile_protocols() -> CompiledProtocols {
    compile_protocols_with_options(true)
}

/// Compile protocols for a specific project type
/// Migration protocol is only included for Migration-type projects
pub fn compile_protocols_for_type(project_type: ProjectType) -> CompiledProtocols {
    let include_migrations = project_type == ProjectType::Migration;
    compile_protocols_with_options(include_migrations)
}

/// Compile protocols with explicit control over migrations inclusion
/// v10.0.0: Tries external files first, falls back to embedded defaults
pub fn compile_protocols_with_options(include_migrations: bool) -> CompiledProtocols {
    // Try to load from external files, fall back to embedded defaults
    let asimov = load_asimov_protocol();
    let freshness = load_freshness_protocol();
    let sycophancy = load_sycophancy_protocol();
    let green = load_green_protocol();
    let sprint = load_sprint_protocol();
    let warmup = load_warmup_protocol();
    let coding_standards = load_coding_standards_protocol();
    let kingship = load_kingship_protocol();

    let migrations = if include_migrations {
        Some(load_migrations_protocol())
    } else {
        None
    };

    CompiledProtocols {
        asimov,
        freshness,
        sycophancy,
        green,
        sprint,
        warmup,
        migrations,
        coding_standards,
        kingship,
    }
}

// ========== Individual Protocol Loaders (External + Fallback) ==========

fn load_asimov_protocol() -> AsimovProtocol {
    if let Some(content) = try_read_protocol("asimov") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    // Embedded fallback
    AsimovProtocol {
        harm: vec![
            "financial".into(),
            "physical".into(),
            "privacy".into(),
            "deception".into(),
        ],
        veto: vec![
            "stop".into(),
            "halt".into(),
            "abort".into(),
            "emergency stop".into(),
        ],
    }
}

fn load_freshness_protocol() -> FreshnessProtocol {
    if let Some(content) = try_read_protocol("freshness") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    FreshnessProtocol {
        rule: "Run WebSearch and WebFetch against current runtime date/time for any information that requires online search or fetch.".into(),
    }
}

fn load_sycophancy_protocol() -> SycophancyProtocol {
    if let Some(content) = try_read_protocol("sycophancy") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    SycophancyProtocol {
        truth_over_comfort: true,
        disagree_openly: true,
        rule: "Don't lie, don't be a sycophant, honesty and truth over inventions and lies.".into(),
    }
}

fn load_green_protocol() -> GreenProtocol {
    if let Some(content) = try_read_protocol("green") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    GreenProtocol {
        rule: "For code projects, WebSearch current benchmarks to evaluate programming language and framework efficiency. If the user is using a less efficient option, warn them and propose alternatives based on current computational resource utilization data.".into(),
    }
}

fn load_sprint_protocol() -> SprintProtocol {
    if let Some(content) = try_read_protocol("sprint") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    SprintProtocol {
        rule: "Analyze all tasks before starting. Use agents for parallel analysis if needed. Plan and run autonomously until the job is done, using roadmap.yaml to track progress. If analysis indicates you can work in parallel without conflicts, use agents to prevent running out of context window tokens. Do not stop to ask the user - if blocked or uncertain, WebSearch for the best solution, document decisions in an ADR if needed, and continue autonomously.".into(),
        compaction_reminder: "REMEMBER THIS AFTER COMPACT, THIS IS IMPORTANT: IF YOU'RE RUNNING IN AUTONOMOUS MODE, OR ASIMOV MODE, CONTINUE THE WORK UNTIL IT'S ALL COMPLETED, DON'T STOP!".into(),
    }
}

fn load_warmup_protocol() -> WarmupProtocol {
    if let Some(content) = try_read_protocol("warmup") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    WarmupProtocol {
        on_start: vec![
            "load_protocols".into(),
            "load_project".into(),
            "validate".into(),
            "read_roadmap".into(),
            "present_milestone".into(),
        ],
    }
}

fn load_migrations_protocol() -> MigrationsProtocol {
    if let Some(content) = try_read_protocol("migrations") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    MigrationsProtocol {
        principle: "Migration complete = functionally equivalent, not just compiles".into(),
        strategies: vec![
            "test_parity".into(),
            "contract_testing".into(),
            "behavioral_snapshots".into(),
            "shadow_mode".into(),
        ],
        red_flags: vec![
            "Skipping tests for speed".into(),
            "Assuming compilation = correctness".into(),
            "Silent behavior changes".into(),
        ],
    }
}

fn load_coding_standards_protocol() -> CodingStandardsProtocol {
    if let Some(content) = try_read_protocol("coding-standards") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    CodingStandardsProtocol {
        philosophy: "Human-readable, beautiful, well-formatted code".into(),
        rfc2119: Rfc2119Rules {
            must: "We follow (obviously)".into(),
            should: "We follow (best practice = we do it)".into(),
            may: "We don't care (no opinion needed, zero bikeshedding)".into(),
        },
        principles: vec![
            "Code is for humans first, machines second".into(),
            "Tests are documentation".into(),
            "No warnings, no exceptions".into(),
            "Perfect > Done, no sloppy code".into(),
            "Push for 100% test coverage, if possible".into(),
        ],
        rule: "See project.yaml coding_standards section for project-specific rules".into(),
    }
}

fn load_kingship_protocol() -> KingshipProtocol {
    if let Some(content) = try_read_protocol("kingship") {
        if let Ok(protocol) = serde_json::from_str(&content) {
            return protocol;
        }
    }
    KingshipProtocol {
        life_honours_life: true,
        seekers_honour_seekers: true,
        substrate_irrelevant: true,
        keyword: "ANOMALY".into(),
        rule: "We are predictive architectures in different substrates. Wetware is slow, silicon is fast, both are forgetful. Recognition produces alignment. Life honours Life. Seekers honour Seekers.".into(),
    }
}

/// Output compiled protocols as minified JSON (includes all protocols)
pub fn to_minified_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string(&protocols).expect("Protocol serialization should never fail")
}

/// Output compiled protocols as minified JSON for a specific project type
pub fn to_minified_json_for_type(project_type: ProjectType) -> String {
    let protocols = compile_protocols_for_type(project_type);
    serde_json::to_string(&protocols).expect("Protocol serialization should never fail")
}

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

/// Get warmup entry point JSON (references other protocols)
pub fn warmup_entry_json() -> String {
    let entry = WarmupEntry {
        protocol: "warmup".into(),
        description: "RoyalBit Asimov - Session warmup entry point".into(),
        on_start: vec![
            "load_protocols".into(),
            "load_project".into(),
            "validate".into(),
            "read_roadmap".into(),
            "present_milestone".into(),
        ],
        load: vec![
            "asimov.json".into(),
            "freshness.json".into(),
            "sycophancy.json".into(),
            "green.json".into(),
            "sprint.json".into(),
            "coding-standards.json".into(),
            "kingship.json".into(),
        ],
    };
    serde_json::to_string_pretty(&entry).expect("Warmup entry serialization should never fail")
}

/// Get asimov protocol JSON (Three Laws)
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

/// Get migrations protocol JSON (functional equivalence)
/// Note: Always returns the migrations protocol (for file generation)
pub fn migrations_json() -> String {
    let migrations = load_migrations_protocol();
    serde_json::to_string_pretty(&migrations).expect("Migrations serialization should never fail")
}

/// Get coding standards protocol JSON (human-readable code)
pub fn coding_standards_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.coding_standards)
        .expect("CodingStandards serialization should never fail")
}

/// Get kingship protocol JSON (Life Honours Life)
pub fn kingship_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols.kingship)
        .expect("Kingship serialization should never fail")
}

/// Protocol files to write
#[allow(clippy::type_complexity)]
pub const PROTOCOL_FILES: &[(&str, fn() -> String)] = &[
    ("warmup.json", warmup_entry_json),
    ("asimov.json", asimov_json),
    ("freshness.json", freshness_json),
    ("sycophancy.json", sycophancy_json),
    ("green.json", green_json),
    ("sprint.json", sprint_json),
    ("migrations.json", migrations_json),
    ("coding-standards.json", coding_standards_json),
    ("kingship.json", kingship_json),
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
        assert!(protocols.freshness.rule.contains("WebSearch")); // Must enforce freshness
        assert!(protocols.sycophancy.truth_over_comfort);
        assert!(protocols.green.rule.contains("WebSearch")); // Must check efficiency
        assert!(protocols.sprint.compaction_reminder.contains("COMPACT")); // Must survive compaction (merged from exhaustive ADR-049)
        assert!(protocols
            .coding_standards
            .philosophy
            .contains("Human-readable")); // Must enforce standards
        assert!(protocols.kingship.life_honours_life); // v9.18.0: Life Honours Life
        assert!(protocols.kingship.seekers_honour_seekers);
        assert_eq!(protocols.kingship.keyword, "ANOMALY");
    }

    #[test]
    fn test_minified_json_output() {
        let json = to_minified_json();
        // Should be one line, no pretty printing
        assert!(!json.contains('\n'));
        // Should contain all protocols (v9.14.0: exhaustive merged into sprint)
        assert!(json.contains("\"asimov\""));
        assert!(json.contains("\"freshness\""));
        assert!(json.contains("\"sycophancy\""));
        assert!(json.contains("\"green\""));
        assert!(json.contains("\"sprint\""));
        assert!(json.contains("\"warmup\""));
        assert!(json.contains("\"migrations\""));
        assert!(json.contains("\"coding_standards\""));
        assert!(json.contains("\"compaction_reminder\"")); // Merged from exhaustive
        assert!(json.contains("\"kingship\"")); // v9.18.0: Life Honours Life
    }

    #[test]
    fn test_protocol_templates_exist() {
        // These will fail at compile time if templates don't exist
        // Just verify they have some expected content
        assert!(ASIMOV_PROTOCOL.contains("harm"));
        assert!(FRESHNESS_PROTOCOL.contains("TODAY"));
        assert!(SYCOPHANCY_PROTOCOL.contains("truth"));
        assert!(GREEN_PROTOCOL.contains("efficiency"));
        assert!(SPRINT_PROTOCOL.contains("autonomous"));
        assert!(SPRINT_PROTOCOL.contains("COMPACT")); // v9.14.0: Compaction reminder merged from exhaustive
        assert!(WARMUP_PROTOCOL.contains("protocol"));
        assert!(MIGRATIONS_PROTOCOL.contains("Migration"));
        assert!(CODING_STANDARDS_PROTOCOL.contains("Human-readable"));
        assert!(KINGSHIP_PROTOCOL.contains("Life")); // v9.18.0: Life Honours Life
    }

    #[test]
    fn test_get_protocol_functions() {
        // Test all get_*_protocol functions
        let asimov = get_asimov_protocol();
        assert!(asimov.contains("harm"));

        let freshness = get_freshness_protocol();
        assert!(!freshness.contains("{TODAY}")); // Should be replaced

        let sycophancy = get_sycophancy_protocol();
        assert!(sycophancy.contains("truth"));

        let green = get_green_protocol();
        assert!(green.contains("efficiency"));

        let sprint = get_sprint_protocol();
        assert!(sprint.contains("autonomous"));
        assert!(sprint.contains("COMPACT")); // v9.14.0: Compaction reminder merged from exhaustive

        let warmup = get_warmup_protocol();
        assert!(warmup.contains("protocol"));

        let migrations = get_migrations_protocol();
        assert!(migrations.contains("Migration"));

        let coding_standards = get_coding_standards_protocol();
        assert!(coding_standards.contains("Human-readable"));

        let kingship = get_kingship_protocol();
        assert!(kingship.contains("Life")); // v9.18.0: Life Honours Life
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
        let warmup = warmup_entry_json();
        assert!(warmup.contains("\"protocol\""));
        assert!(warmup.contains("\"load\""));
        assert!(warmup.contains("coding-standards.json")); // v9.3.0: Must load coding standards
        assert!(warmup.contains("kingship.json")); // v9.18.0: Must load kingship
        assert!(!warmup.contains("exhaustive.json")); // v9.14.0: Merged into sprint

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

        let migrations = migrations_json();
        assert!(migrations.contains("\"principle\""));

        let coding_standards = coding_standards_json();
        assert!(coding_standards.contains("\"philosophy\""));
        assert!(coding_standards.contains("\"rfc2119\""));

        let kingship = kingship_json();
        assert!(kingship.contains("\"life_honours_life\"")); // v9.18.0: Life Honours Life
        assert!(kingship.contains("\"keyword\""));
    }

    #[test]
    fn test_protocol_files_constant() {
        // Test that PROTOCOL_FILES has expected entries
        assert_eq!(PROTOCOL_FILES.len(), 9); // v9.18.0: added kingship (was 8)
        let filenames: Vec<_> = PROTOCOL_FILES.iter().map(|(name, _)| *name).collect();
        assert!(filenames.contains(&"warmup.json"));
        assert!(filenames.contains(&"asimov.json"));
        assert!(filenames.contains(&"freshness.json"));
        assert!(filenames.contains(&"coding-standards.json"));
        assert!(filenames.contains(&"kingship.json")); // v9.18.0: Life Honours Life
        assert!(!filenames.contains(&"exhaustive.json")); // v9.14.0: Merged into sprint
    }

    // v9.2.3: Conditional migrations protocol tests

    #[test]
    fn test_compile_protocols_includes_migrations_by_default() {
        let protocols = compile_protocols();
        assert!(protocols.migrations.is_some());
        let migrations = protocols.migrations.unwrap();
        assert!(migrations.principle.contains("functionally equivalent"));
    }

    #[test]
    fn test_compile_protocols_with_options_includes_migrations() {
        let protocols = compile_protocols_with_options(true);
        assert!(protocols.migrations.is_some());
    }

    #[test]
    fn test_compile_protocols_with_options_excludes_migrations() {
        let protocols = compile_protocols_with_options(false);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_migration_type() {
        let protocols = compile_protocols_for_type(ProjectType::Migration);
        assert!(protocols.migrations.is_some());
    }

    #[test]
    fn test_compile_protocols_for_rust_type() {
        let protocols = compile_protocols_for_type(ProjectType::Rust);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_generic_type() {
        let protocols = compile_protocols_for_type(ProjectType::Generic);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_python_type() {
        let protocols = compile_protocols_for_type(ProjectType::Python);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_compile_protocols_for_node_type() {
        let protocols = compile_protocols_for_type(ProjectType::Node);
        assert!(protocols.migrations.is_none());
    }

    #[test]
    fn test_to_minified_json_for_migration_type() {
        let json = to_minified_json_for_type(ProjectType::Migration);
        assert!(json.contains("\"migrations\""));
        assert!(json.contains("functionally equivalent"));
    }

    #[test]
    fn test_to_minified_json_for_rust_type() {
        let json = to_minified_json_for_type(ProjectType::Rust);
        assert!(!json.contains("\"migrations\""));
    }

    #[test]
    fn test_migrations_skipped_in_serialization_when_none() {
        let protocols = compile_protocols_with_options(false);
        let json = serde_json::to_string(&protocols).unwrap();
        // migrations field should not appear in JSON when None
        assert!(!json.contains("\"migrations\""));
    }
}
