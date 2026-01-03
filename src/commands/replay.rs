//! Replay command implementation

use std::path::Path;

#[derive(Debug, Clone)]
pub struct CommitInfo {
    pub hash: String,
    pub date: String,
    pub time: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ReplayResult {
    pub success: bool,
    pub is_git_repo: bool,
    pub range_description: String,
    pub commits: Vec<CommitInfo>,
    pub total_files_changed: usize,
    pub total_insertions: usize,
    pub total_deletions: usize,
    pub error: Option<String>,
}

pub fn run_replay(
    dir: &Path,
    commits: Option<usize>,
    yesterday: bool,
    since: Option<String>,
) -> ReplayResult {
    use chrono::Local;

    let mut result = ReplayResult {
        success: false,
        is_git_repo: false,
        range_description: String::new(),
        commits: Vec::new(),
        total_files_changed: 0,
        total_insertions: 0,
        total_deletions: 0,
        error: None,
    };

    if !dir.join(".git").exists() {
        result.error = Some("Not a git repository".to_string());
        return result;
    }
    result.is_git_repo = true;

    let mut args = vec![
        "log".to_string(),
        "--pretty=format:%H|%ci|%s".to_string(),
        "--date=local".to_string(),
    ];

    result.range_description = if let Some(n) = commits {
        args.push(format!("-{}", n));
        format!("Last {} commits", n)
    } else if yesterday {
        let yesterday_date = Local::now().date_naive() - chrono::Duration::days(1);
        args.push(format!("--since={} 00:00:00", yesterday_date));
        args.push(format!("--until={} 23:59:59", yesterday_date));
        format!("Yesterday ({})", yesterday_date)
    } else if let Some(ref since_arg) = since {
        args.push(format!("--since={}", since_arg));
        format!("Since {}", since_arg)
    } else {
        let today = Local::now().format("%Y-%m-%d").to_string();
        args.push(format!("--since={} 00:00:00", today));
        format!("Today ({})", today)
    };

    let output = std::process::Command::new("git")
        .args(&args)
        .current_dir(dir)
        .output();

    let commits_output = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => {
            result.error = Some("Failed to get git log".to_string());
            return result;
        }
    };

    for line in commits_output.lines() {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() >= 3 {
            let datetime_parts: Vec<&str> = parts[1].split_whitespace().collect();
            result.commits.push(CommitInfo {
                hash: parts[0][..7].to_string(),
                date: datetime_parts.first().unwrap_or(&"").to_string(),
                time: datetime_parts.get(1).unwrap_or(&"").to_string(),
                message: parts[2].to_string(),
            });
        }
    }

    // Get diff stats
    if let Ok(output) = std::process::Command::new("git")
        .args(["diff", "--stat", "HEAD~1..HEAD"])
        .current_dir(dir)
        .output()
    {
        if output.status.success() {
            let stat = String::from_utf8_lossy(&output.stdout);
            for line in stat.lines() {
                if line.contains("changed") {
                    if let Some(files) = line.split_whitespace().next() {
                        result.total_files_changed = files.parse().unwrap_or(0);
                    }
                    if line.contains("insertion") {
                        for part in line.split(',') {
                            if part.contains("insertion") {
                                if let Some(num) = part.split_whitespace().next() {
                                    result.total_insertions = num.parse().unwrap_or(0);
                                }
                            }
                            if part.contains("deletion") {
                                if let Some(num) = part.split_whitespace().next() {
                                    result.total_deletions = num.parse().unwrap_or(0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    result.success = true;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_run_replay_not_git() {
        let temp = TempDir::new().unwrap();
        let result = run_replay(temp.path(), None, false, None);
        assert!(!result.success);
        assert!(!result.is_git_repo);
    }

    #[test]
    fn test_commit_info_struct() {
        let c = CommitInfo {
            hash: "abc1234".to_string(),
            date: "2025-01-01".to_string(),
            time: "12:00".to_string(),
            message: "Test".to_string(),
        };
        assert_eq!(c.hash, "abc1234");
    }

    #[test]
    fn test_replay_result_fields() {
        let r = ReplayResult {
            success: true,
            is_git_repo: true,
            range_description: "today".to_string(),
            commits: vec![],
            total_files_changed: 10,
            total_insertions: 100,
            total_deletions: 50,
            error: None,
        };
        assert!(r.is_git_repo);
    }

    #[test]
    fn test_run_replay_in_git_repo() {
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

        // Test replay with various options
        let result = run_replay(temp.path(), Some(5), false, None);
        assert!(result.is_git_repo);

        let result2 = run_replay(temp.path(), None, true, None);
        assert!(result2.is_git_repo);

        let result3 = run_replay(temp.path(), None, false, Some("1 hour ago".to_string()));
        assert!(result3.is_git_repo);
    }

    #[test]
    fn test_run_replay_yesterday() {
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

        let result = run_replay(temp.path(), None, true, None);
        assert!(result.is_git_repo);
    }

    #[test]
    fn test_run_replay_with_commits_no_author() {
        let temp = TempDir::new().unwrap();
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .unwrap();
        let result = run_replay(temp.path(), None, false, None);
        // No commits
        assert!(result.commits.is_empty());
    }
}
