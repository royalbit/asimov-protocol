//! Hardcoded Protocol Module - Core protocols compiled into binary (ADR-031)
//!
//! Protocols are ENFORCED by the Rust binary, not optional YAML files.
//! This is the source of truth for behavior protocols.

use serde::Serialize;

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

/// Compiled protocol context for minimal token usage
#[derive(Debug, Clone, Serialize)]
pub struct CompiledProtocols {
    pub asimov: AsimovProtocol,
    pub freshness: FreshnessProtocol,
    pub sycophancy: SycophancyProtocol,
    pub green: GreenProtocol,
    pub sprint: SprintProtocol,
    pub warmup: WarmupProtocol,
}

#[derive(Debug, Clone, Serialize)]
pub struct AsimovProtocol {
    pub harm: Vec<&'static str>,
    pub veto: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FreshnessProtocol {
    pub today: String,
    pub year: String,
    pub search: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SycophancyProtocol {
    pub truth_over_comfort: bool,
    pub disagree_openly: bool,
    pub banned: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GreenProtocol {
    pub local_first: bool,
    pub avoid: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SprintProtocol {
    pub max_hours: u8,
    pub stop_on: Vec<&'static str>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WarmupProtocol {
    pub on_start: Vec<&'static str>,
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

/// Compile all protocols into a minimal JSON blob for context injection
pub fn compile_protocols() -> CompiledProtocols {
    CompiledProtocols {
        asimov: AsimovProtocol {
            harm: vec!["financial", "physical", "privacy", "deception"],
            veto: vec!["stop", "halt", "abort", "emergency stop"],
        },
        freshness: FreshnessProtocol {
            today: get_today(),
            year: get_year(),
            search: vec![
                "version",
                "pricing",
                "api",
                "current",
                "latest",
                "release",
                "changelog",
                "documentation",
            ],
        },
        sycophancy: SycophancyProtocol {
            truth_over_comfort: true,
            disagree_openly: true,
            banned: vec![
                "You're absolutely right",
                "Great question",
                "I completely agree",
                "That's a great point",
            ],
        },
        green: GreenProtocol {
            local_first: true,
            avoid: vec![
                "unnecessary API calls",
                "cloud when local works",
                "external services for validation",
            ],
        },
        sprint: SprintProtocol {
            max_hours: 4,
            stop_on: vec![
                "roadmap_exhausted",
                "blocked",
                "human_stop",
                "context_limit",
            ],
        },
        warmup: WarmupProtocol {
            on_start: vec![
                "load_protocols",
                "validate",
                "read_roadmap",
                "present_milestone",
            ],
        },
    }
}

/// Output compiled protocols as minified JSON
pub fn to_minified_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string(&protocols).expect("Protocol serialization should never fail")
}

/// Output compiled protocols as pretty JSON (for debugging)
pub fn to_pretty_json() -> String {
    let protocols = compile_protocols();
    serde_json::to_string_pretty(&protocols).expect("Protocol serialization should never fail")
}

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
        assert!(protocols.freshness.today.len() == 10); // YYYY-MM-DD
        assert!(protocols.sycophancy.truth_over_comfort);
        assert!(protocols.green.local_first);
    }

    #[test]
    fn test_minified_json_output() {
        let json = to_minified_json();
        // Should be one line, no pretty printing
        assert!(!json.contains('\n'));
        // Should contain all protocols
        assert!(json.contains("\"asimov\""));
        assert!(json.contains("\"freshness\""));
        assert!(json.contains("\"sycophancy\""));
        assert!(json.contains("\"green\""));
        assert!(json.contains("\"sprint\""));
        assert!(json.contains("\"warmup\""));
    }

    #[test]
    fn test_protocol_templates_exist() {
        // These will fail at compile time if templates don't exist
        assert!(!ASIMOV_PROTOCOL.is_empty());
        assert!(!FRESHNESS_PROTOCOL.is_empty());
        assert!(!SYCOPHANCY_PROTOCOL.is_empty());
        assert!(!GREEN_PROTOCOL.is_empty());
        assert!(!SPRINT_PROTOCOL.is_empty());
        assert!(!WARMUP_PROTOCOL.is_empty());
    }
}
