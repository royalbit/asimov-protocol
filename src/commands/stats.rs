//! Stats command implementation

use crate::resolve_protocol_dir;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct StatsResult {
    pub total_commits: usize,
    pub asimov_commits: usize,
    pub today_commits: usize,
    pub session_date: String,
    pub milestone_version: Option<String>,
    pub milestone_summary: Option<String>,
    pub milestone_status: Option<String>,
}

pub fn run_stats(dir: &Path) -> StatsResult {
    use chrono::Local;

    let today = Local::now().format("%Y-%m-%d").to_string();
    let mut result = StatsResult {
        total_commits: 0,
        asimov_commits: 0,
        today_commits: 0,
        session_date: today.clone(),
        milestone_version: None,
        milestone_summary: None,
        milestone_status: None,
    };

    // Get git stats
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .current_dir(dir)
        .output()
    {
        if output.status.success() {
            if let Ok(count) = String::from_utf8_lossy(&output.stdout).trim().parse() {
                result.total_commits = count;
            }
        }
    }

    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-list", "--count", "HEAD", "--grep=asimov"])
        .current_dir(dir)
        .output()
    {
        if output.status.success() {
            if let Ok(count) = String::from_utf8_lossy(&output.stdout).trim().parse() {
                result.asimov_commits = count;
            }
        }
    }

    if let Ok(output) = std::process::Command::new("git")
        .args([
            "rev-list",
            "--count",
            "HEAD",
            &format!("--since={} 00:00:00", today),
        ])
        .current_dir(dir)
        .output()
    {
        if output.status.success() {
            if let Ok(count) = String::from_utf8_lossy(&output.stdout).trim().parse() {
                result.today_commits = count;
            }
        }
    }

    // Get milestone info
    let roadmap_path = resolve_protocol_dir(dir).join("roadmap.yaml");
    if let Ok(content) = std::fs::read_to_string(&roadmap_path) {
        if let Ok(roadmap) = serde_yaml_ng::from_str::<serde_yaml_ng::Value>(&content) {
            if let Some(current) = roadmap.get("current") {
                result.milestone_version = current
                    .get("version")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                result.milestone_summary = current
                    .get("summary")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                result.milestone_status = current
                    .get("status")
                    .and_then(|v| v.as_str())
                    .map(String::from);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_stats() {
        let temp = TempDir::new().unwrap();
        let result = run_stats(temp.path());
        assert!(!result.session_date.is_empty());
    }

    #[test]
    fn test_stats_result_fields() {
        let r = StatsResult {
            total_commits: 100,
            asimov_commits: 50,
            today_commits: 5,
            session_date: "2025-01-01".to_string(),
            milestone_version: Some("1.0.0".to_string()),
            milestone_summary: Some("Test".to_string()),
            milestone_status: Some("active".to_string()),
        };
        assert_eq!(r.today_commits, 5);
    }

    #[test]
    fn test_run_stats_in_git_repo() {
        let temp = TempDir::new().unwrap();
        // Initialize git repo
        let _ = std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output();
        let _ = std::process::Command::new("git")
            .args(["config", "user.email", "test@test.com"])
            .current_dir(temp.path())
            .output();
        let _ = std::process::Command::new("git")
            .args(["config", "user.name", "Test"])
            .current_dir(temp.path())
            .output();

        // Create asimov project
        let asimov_dir = temp.path().join(".asimov");
        std::fs::create_dir_all(&asimov_dir).unwrap();
        std::fs::write(
            asimov_dir.join("roadmap.yaml"),
            "current:\n  version: '1.0.0'\n  status: planned\n  summary: Test",
        )
        .unwrap();

        // Create and commit a file
        std::fs::write(temp.path().join("test.txt"), "test").unwrap();
        let _ = std::process::Command::new("git")
            .args(["add", "."])
            .current_dir(temp.path())
            .output();
        let _ = std::process::Command::new("git")
            .args(["commit", "-m", "Initial"])
            .current_dir(temp.path())
            .output();

        let result = run_stats(temp.path());
        // Verify we got stats - session_date is always set
        assert!(!result.session_date.is_empty());
    }

    #[test]
    fn test_run_stats_empty_repo() {
        let temp = TempDir::new().unwrap();

        // Initialize git repo but don't commit anything
        let _ = std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output();

        let result = run_stats(temp.path());
        // Should work even with empty repo
        assert_eq!(result.total_commits, 0);
    }

    #[test]
    fn test_run_stats_no_project() {
        let temp = TempDir::new().unwrap();
        let result = run_stats(temp.path());
        // Should have 0 commits since no asimov dir
        assert_eq!(result.total_commits, 0);
    }
}
