//! Update command implementation

use crate::{check_for_update, perform_update};

#[derive(Debug, Clone)]
pub enum UpdateResult {
    AlreadyLatest {
        current: String,
        latest: String,
    },
    UpdateAvailable {
        current: String,
        latest: String,
    },
    Updated {
        from: String,
        to: String,
    },
    UpdateFailed {
        current: String,
        latest: String,
        error: String,
        download_url: String,
    },
    NoBinaryAvailable {
        current: String,
        latest: String,
    },
    CheckFailed {
        error: String,
    },
}

pub fn run_update(check_only: bool) -> UpdateResult {
    match check_for_update() {
        Ok(info) => {
            if info.update_available {
                if check_only {
                    return UpdateResult::UpdateAvailable {
                        current: info.current,
                        latest: info.latest,
                    };
                }
                if let Some(url) = info.download_url {
                    match perform_update(&url, info.checksums_url.as_deref()) {
                        Ok(()) => UpdateResult::Updated {
                            from: info.current,
                            to: info.latest,
                        },
                        Err(e) => UpdateResult::UpdateFailed {
                            current: info.current,
                            latest: info.latest,
                            error: e,
                            download_url: url,
                        },
                    }
                } else {
                    UpdateResult::NoBinaryAvailable {
                        current: info.current,
                        latest: info.latest,
                    }
                }
            } else {
                UpdateResult::AlreadyLatest {
                    current: info.current,
                    latest: info.latest,
                }
            }
        }
        Err(e) => UpdateResult::CheckFailed {
            error: e.to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_result_variants() {
        let _ = UpdateResult::AlreadyLatest {
            current: "1.0".to_string(),
            latest: "1.0".to_string(),
        };
        let _ = UpdateResult::UpdateAvailable {
            current: "1.0".to_string(),
            latest: "2.0".to_string(),
        };
        let _ = UpdateResult::Updated {
            from: "1.0".to_string(),
            to: "2.0".to_string(),
        };
        let _ = UpdateResult::UpdateFailed {
            current: "1.0".to_string(),
            latest: "2.0".to_string(),
            error: "err".to_string(),
            download_url: "url".to_string(),
        };
        let _ = UpdateResult::NoBinaryAvailable {
            current: "1.0".to_string(),
            latest: "2.0".to_string(),
        };
        let _ = UpdateResult::CheckFailed {
            error: "err".to_string(),
        };
    }

    #[test]
    fn test_run_update_check_only() {
        // This tests the check_only path - won't actually update
        // Note: This will make a network call to check for updates
        let result = run_update(true);
        // Result depends on network state and version comparison
        match result {
            UpdateResult::AlreadyLatest { .. } => {}
            UpdateResult::UpdateAvailable { .. } => {}
            UpdateResult::CheckFailed { .. } => {}
            _ => panic!("Unexpected result for check_only=true"),
        }
    }

    #[test]
    fn test_run_update_network() {
        // This test exercises the network code path
        // May succeed or fail depending on network availability
        let result = run_update(true); // check_only mode
                                       // Just verify it returns a valid variant
        match result {
            UpdateResult::AlreadyLatest { .. } => (),
            UpdateResult::UpdateAvailable { .. } => (),
            UpdateResult::CheckFailed { .. } => (),
            _ => panic!("Unexpected result in check mode"),
        }
    }

    #[test]
    fn test_run_update_actual_check() {
        // Run the actual update check - exercises network code
        // This will hit either AlreadyLatest, UpdateAvailable, or CheckFailed
        let result = run_update(true);

        // Verify we got one of the expected check-only results
        let is_valid = matches!(
            &result,
            UpdateResult::AlreadyLatest { .. }
                | UpdateResult::UpdateAvailable { .. }
                | UpdateResult::CheckFailed { .. }
        );
        assert!(
            is_valid,
            "Got unexpected result: {:?}",
            match result {
                UpdateResult::Updated { .. } => "Updated",
                UpdateResult::UpdateFailed { .. } => "UpdateFailed",
                UpdateResult::NoBinaryAvailable { .. } => "NoBinaryAvailable",
                _ => "other",
            }
        );
    }
}
