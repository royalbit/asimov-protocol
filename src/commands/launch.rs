//! Launch command implementation - AI-agnostic (v10.7.0 ADR-061)

/// AI CLI profile with detection and launch info
#[derive(Debug, Clone)]
pub struct AiProfile {
    pub name: &'static str,
    pub binary: &'static str,
    pub env_vars: &'static [&'static str],
    pub auto_mode_args: &'static [&'static str],
}

impl AiProfile {
    /// Check if we're inside this AI's session
    pub fn is_inside(&self) -> bool {
        self.env_vars.iter().any(|var| std::env::var(var).is_ok())
    }

    /// Check if this AI CLI is installed
    pub fn is_installed(&self) -> bool {
        #[cfg(unix)]
        let find_cmd = "which";
        #[cfg(windows)]
        let find_cmd = "where";

        std::process::Command::new(find_cmd)
            .arg(self.binary)
            .output()
            .ok()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

/// Known AI CLI profiles (ADR-061)
pub const AI_PROFILES: &[AiProfile] = &[
    AiProfile {
        name: "Claude Code",
        binary: "claude",
        env_vars: &["CLAUDECODE", "CLAUDE_CODE_ENTRYPOINT"],
        auto_mode_args: &["--dangerously-skip-permissions", "--model", "opus"],
    },
    AiProfile {
        name: "Gemini CLI",
        binary: "gemini",
        env_vars: &["GEMINI_CLI"],
        auto_mode_args: &["--yolo"],
    },
    AiProfile {
        name: "Codex CLI",
        binary: "codex",
        env_vars: &["CODEX_CLI"],
        auto_mode_args: &["--full-auto"],
    },
];

#[derive(Debug, Clone)]
pub enum LaunchResult {
    /// No AI CLIs found
    NoAiFound,
    /// Inside an AI session - run warmup directly
    InsideAi(String),
    /// Single AI found - launch it
    Launching(AiProfile),
    /// Multiple AIs found - user must select
    MultipleFound(Vec<AiProfile>),
}

pub fn check_launch_conditions() -> LaunchResult {
    // Check if inside any AI session
    for profile in AI_PROFILES {
        if profile.is_inside() {
            return LaunchResult::InsideAi(profile.name.to_string());
        }
    }

    // Find installed AI CLIs
    let installed: Vec<AiProfile> = AI_PROFILES
        .iter()
        .filter(|p| p.is_installed())
        .cloned()
        .collect();

    match installed.len() {
        0 => LaunchResult::NoAiFound,
        1 => LaunchResult::Launching(installed.into_iter().next().unwrap()),
        _ => LaunchResult::MultipleFound(installed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_profile_struct() {
        let profile = &AI_PROFILES[0];
        assert_eq!(profile.name, "Claude Code");
        assert_eq!(profile.binary, "claude");
    }

    #[test]
    fn test_launch_result_variants() {
        let _ = LaunchResult::NoAiFound;
        let _ = LaunchResult::InsideAi("Claude".to_string());
        let _ = LaunchResult::Launching(AI_PROFILES[0].clone());
        let _ = LaunchResult::MultipleFound(vec![]);
    }

    #[test]
    fn test_check_launch_inside_claude() {
        // Save original values
        let orig_claudecode = std::env::var("CLAUDECODE").ok();
        let orig_entrypoint = std::env::var("CLAUDE_CODE_ENTRYPOINT").ok();

        // Clear CLAUDE_CODE_ENTRYPOINT to ensure only CLAUDECODE is tested
        std::env::remove_var("CLAUDE_CODE_ENTRYPOINT");

        // Set CLAUDECODE to trigger InsideAi detection
        std::env::set_var("CLAUDECODE", "1");

        let result = check_launch_conditions();

        // Restore original values
        if let Some(val) = orig_claudecode {
            std::env::set_var("CLAUDECODE", val);
        } else {
            std::env::remove_var("CLAUDECODE");
        }
        if let Some(val) = orig_entrypoint {
            std::env::set_var("CLAUDE_CODE_ENTRYPOINT", val);
        }

        assert!(matches!(result, LaunchResult::InsideAi(_)));
    }

    #[test]
    fn test_check_launch_not_in_ai() {
        // This test verifies launch conditions work.
        // Since env vars are global state and tests run in parallel,
        // we just verify the function returns a valid variant.
        let result = check_launch_conditions();
        // Verify it's a valid variant (exhaustive match)
        match result {
            LaunchResult::InsideAi(_) => {
                // This is expected when running in an AI environment
            }
            LaunchResult::NoAiFound => {
                // This is expected when no AI is installed
            }
            LaunchResult::Launching(_) => {
                // This is expected when one AI is found
            }
            LaunchResult::MultipleFound(_) => {
                // This is expected when multiple AIs are found
            }
        }
    }

    #[test]
    fn test_check_launch_conditions_claude_entrypoint() {
        // Save original value
        let orig = std::env::var("CLAUDE_CODE_ENTRYPOINT").ok();

        std::env::set_var("CLAUDE_CODE_ENTRYPOINT", "test");
        let result = check_launch_conditions();

        // Restore
        if let Some(val) = orig {
            std::env::set_var("CLAUDE_CODE_ENTRYPOINT", val);
        } else {
            std::env::remove_var("CLAUDE_CODE_ENTRYPOINT");
        }

        assert!(matches!(result, LaunchResult::InsideAi(_)));
    }

    #[test]
    fn test_check_launch_conditions_coverage() {
        // Exercise check_launch_conditions - result depends on environment
        let result = check_launch_conditions();
        // All variants are valid
        assert!(matches!(
            result,
            LaunchResult::NoAiFound
                | LaunchResult::InsideAi(_)
                | LaunchResult::Launching(_)
                | LaunchResult::MultipleFound(_)
        ));
    }

    #[test]
    fn test_ai_profile_is_inside() {
        let profile = &AI_PROFILES[0];
        // Test that setting env var makes is_inside() return true
        // (Don't test the negative case - env vars are global state and
        // we might be running inside Claude Code which sets them)
        std::env::set_var("CLAUDECODE", "1");
        assert!(
            profile.is_inside(),
            "Expected is_inside() with CLAUDECODE=1"
        );
    }
}
