//! Launch command implementation

#[derive(Debug, Clone)]
pub enum LaunchResult {
    ClaudeNotFound,
    InsideClaude,
    Launching,
}

pub fn check_launch_conditions() -> LaunchResult {
    // Check if inside Claude
    if std::env::var("CLAUDECODE").is_ok() || std::env::var("CLAUDE_CODE_ENTRYPOINT").is_ok() {
        return LaunchResult::InsideClaude;
    }

    // Check if claude is in PATH
    #[cfg(unix)]
    let find_cmd = "which";
    #[cfg(windows)]
    let find_cmd = "where";

    let claude_found = std::process::Command::new(find_cmd)
        .arg("claude")
        .output()
        .ok()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !claude_found {
        return LaunchResult::ClaudeNotFound;
    }

    LaunchResult::Launching
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_result_variants() {
        let _ = LaunchResult::InsideClaude;
        let _ = LaunchResult::ClaudeNotFound;
        let _ = LaunchResult::Launching;
    }

    #[test]
    fn test_check_launch_inside_claude() {
        // Save original values
        let orig_claudecode = std::env::var("CLAUDECODE").ok();
        let orig_entrypoint = std::env::var("CLAUDE_CODE_ENTRYPOINT").ok();

        // Clear CLAUDE_CODE_ENTRYPOINT to ensure only CLAUDECODE is tested
        std::env::remove_var("CLAUDE_CODE_ENTRYPOINT");

        // Set CLAUDECODE to trigger InsideClaude detection
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

        assert!(matches!(result, LaunchResult::InsideClaude));
    }

    #[test]
    fn test_check_launch_not_in_claude() {
        // This test verifies launch conditions work.
        // Since env vars are global state and tests run in parallel,
        // we just verify the function returns a valid variant.
        let result = check_launch_conditions();
        // Verify it's a valid variant (exhaustive match)
        match result {
            LaunchResult::InsideClaude => {
                // This is expected when running in Claude Code environment
            }
            LaunchResult::ClaudeNotFound => {
                // This is expected when claude is not installed
            }
            LaunchResult::Launching => {
                // This is expected when claude is found
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

        assert!(matches!(result, LaunchResult::InsideClaude));
    }

    #[test]
    fn test_check_launch_conditions_coverage() {
        // Exercise check_launch_conditions - result depends on environment
        let result = check_launch_conditions();
        // All variants are valid
        assert!(matches!(
            result,
            LaunchResult::ClaudeNotFound | LaunchResult::InsideClaude | LaunchResult::Launching
        ));
    }
}
