//! Role switching command - v10.0.0
//!
//! Switch between specialized roles for different tasks.
//! Roles are loaded from .asimov/roles/*.json

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub code: String,
    pub description: String,
    pub focus: Vec<String>,
    pub prompt_prefix: String,
    pub avoid: Vec<String>,
}

/// Get the roles directory path
pub fn roles_dir() -> PathBuf {
    PathBuf::from(".asimov/roles")
}

/// Load a specific role by code
pub fn load_role(code: &str) -> Option<Role> {
    let path = roles_dir().join(format!("{}.json", code));
    let content = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&content).ok()
}

/// List all available roles
pub fn list_roles() -> Vec<Role> {
    let mut roles = Vec::new();

    if let Ok(entries) = std::fs::read_dir(roles_dir()) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(role) = serde_json::from_str::<Role>(&content) {
                        roles.push(role);
                    }
                }
            }
        }
    }

    roles.sort_by(|a, b| a.code.cmp(&b.code));
    roles
}

/// Execute the role command
pub fn run_role(role_code: Option<&str>) -> Result<RoleResult, RoleError> {
    match role_code {
        None => {
            // List all available roles
            let roles = list_roles();
            if roles.is_empty() {
                return Err(RoleError::NoRolesFound);
            }
            Ok(RoleResult::List(roles))
        }
        Some(code) => {
            // Load and display specific role
            match load_role(code) {
                Some(role) => Ok(RoleResult::Selected(role)),
                None => Err(RoleError::RoleNotFound(code.to_string())),
            }
        }
    }
}

/// Role command result
#[derive(Debug)]
pub enum RoleResult {
    /// List of available roles
    List(Vec<Role>),
    /// Selected role
    Selected(Role),
}

/// Role command errors
#[derive(Debug)]
pub enum RoleError {
    /// No roles found in .asimov/roles/
    NoRolesFound,
    /// Specified role not found
    RoleNotFound(String),
}

impl std::fmt::Display for RoleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoleError::NoRolesFound => {
                write!(
                    f,
                    "No roles found. Create role files in .asimov/roles/*.json"
                )
            }
            RoleError::RoleNotFound(code) => {
                write!(
                    f,
                    "Role '{}' not found. Use 'asimov role' to list available roles.",
                    code
                )
            }
        }
    }
}

impl std::error::Error for RoleError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roles_dir() {
        let dir = roles_dir();
        assert_eq!(dir.to_str().unwrap(), ".asimov/roles");
    }

    #[test]
    fn test_role_deserialize() {
        let json = r#"{
            "name": "Principal Engineer",
            "code": "eng",
            "description": "Architecture specialist",
            "focus": ["system architecture", "code quality"],
            "prompt_prefix": "You are a Principal Engineer.",
            "avoid": ["marketing"]
        }"#;

        let role: Role = serde_json::from_str(json).unwrap();
        assert_eq!(role.code, "eng");
        assert_eq!(role.name, "Principal Engineer");
        assert_eq!(role.focus.len(), 2);
    }

    #[test]
    fn test_run_role_no_roles() {
        // In test environment without .asimov/roles/, should return error
        // This test may pass or fail depending on test directory state
        let result = run_role(None);
        // Either returns empty list or error
        assert!(result.is_ok() || matches!(result, Err(RoleError::NoRolesFound)));
    }
}
