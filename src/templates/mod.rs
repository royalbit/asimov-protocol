//! Template generators for RoyalBit Asimov files

mod hooks;
mod project;
mod protocols;
mod warmup;

use std::fmt;
use std::path::Path;

// Re-export all public items
pub use hooks::*;
pub use project::*;
pub use protocols::*;
pub use warmup::*;

/// Supported project types for template generation
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProjectType {
    #[default]
    Generic,
    Rust,
    Python,
    Node,
    Go,
    Flutter,
    Docs,
    Migration,
    Arch,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Generic => write!(f, "generic"),
            ProjectType::Rust => write!(f, "rust"),
            ProjectType::Python => write!(f, "python"),
            ProjectType::Node => write!(f, "node"),
            ProjectType::Go => write!(f, "go"),
            ProjectType::Flutter => write!(f, "flutter"),
            ProjectType::Docs => write!(f, "docs"),
            ProjectType::Migration => write!(f, "migration"),
            ProjectType::Arch => write!(f, "arch"),
        }
    }
}

impl std::str::FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "generic" => Ok(ProjectType::Generic),
            "rust" => Ok(ProjectType::Rust),
            "python" | "py" => Ok(ProjectType::Python),
            "node" | "nodejs" | "js" | "javascript" => Ok(ProjectType::Node),
            "go" | "golang" => Ok(ProjectType::Go),
            "flutter" | "dart" => Ok(ProjectType::Flutter),
            "docs" | "documentation" => Ok(ProjectType::Docs),
            "migration" | "migrations" => Ok(ProjectType::Migration),
            "arch" | "architecture" => Ok(ProjectType::Arch),
            _ => Err(format!(
                "Unknown template: '{}'. Use --help to see all 21 available templates",
                s
            )),
        }
    }
}

/// Detect project type from marker files in the given directory (ADR-032)
/// Returns the detected project type or Generic if no markers found
pub fn detect_project_type(dir: &Path) -> ProjectType {
    // Check for marker files in priority order
    // Flutter/Dart before Node (pubspec.yaml is more specific)
    if dir.join("pubspec.yaml").exists() {
        return ProjectType::Flutter;
    }
    if dir.join("Cargo.toml").exists() {
        return ProjectType::Rust;
    }
    if dir.join("go.mod").exists() {
        return ProjectType::Go;
    }
    if dir.join("pyproject.toml").exists() || dir.join("setup.py").exists() {
        return ProjectType::Python;
    }
    if dir.join("package.json").exists() {
        return ProjectType::Node;
    }
    // Check for arch project (ADR-041)
    // c4-models/ OR decisions/ OR (diagrams/ AND ARCHITECTURE*.md)
    if dir.join("c4-models").is_dir() || dir.join("decisions").is_dir() {
        return ProjectType::Arch;
    }
    if dir.join("diagrams").is_dir() && has_architecture_file(dir) {
        return ProjectType::Arch;
    }
    // Check for docs project (only markdown files in certain patterns)
    if dir.join("docs").is_dir() || dir.join("README.md").exists() {
        // Check if there are no code files, only markdown
        let has_code_markers = dir.join("src").is_dir()
            || dir.join("lib").is_dir()
            || dir.join("cmd").is_dir()
            || dir.join("pkg").is_dir();
        if !has_code_markers {
            return ProjectType::Docs;
        }
    }
    ProjectType::Generic
}

/// Check if directory has an ARCHITECTURE*.md file
fn has_architecture_file(dir: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.starts_with("ARCHITECTURE") && name_str.ends_with(".md") {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_type_default() {
        let pt: ProjectType = Default::default();
        assert!(matches!(pt, ProjectType::Generic));
    }

    #[test]
    fn test_project_type_display() {
        assert_eq!(ProjectType::Generic.to_string(), "generic");
        assert_eq!(ProjectType::Rust.to_string(), "rust");
        assert_eq!(ProjectType::Python.to_string(), "python");
        assert_eq!(ProjectType::Node.to_string(), "node");
        assert_eq!(ProjectType::Go.to_string(), "go");
    }

    #[test]
    fn test_project_type_from_str_valid() {
        assert!(matches!(
            "generic".parse::<ProjectType>(),
            Ok(ProjectType::Generic)
        ));
        assert!(matches!(
            "rust".parse::<ProjectType>(),
            Ok(ProjectType::Rust)
        ));
        assert!(matches!(
            "python".parse::<ProjectType>(),
            Ok(ProjectType::Python)
        ));
        assert!(matches!(
            "py".parse::<ProjectType>(),
            Ok(ProjectType::Python)
        ));
        assert!(matches!(
            "node".parse::<ProjectType>(),
            Ok(ProjectType::Node)
        ));
        assert!(matches!(
            "nodejs".parse::<ProjectType>(),
            Ok(ProjectType::Node)
        ));
        assert!(matches!("js".parse::<ProjectType>(), Ok(ProjectType::Node)));
        assert!(matches!("go".parse::<ProjectType>(), Ok(ProjectType::Go)));
        assert!(matches!(
            "golang".parse::<ProjectType>(),
            Ok(ProjectType::Go)
        ));
    }

    #[test]
    fn test_project_type_from_str_invalid() {
        let result = "invalid".parse::<ProjectType>();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unknown template"));
    }

    #[test]
    fn test_project_type_display_all() {
        assert_eq!(format!("{}", ProjectType::Generic), "generic");
        assert_eq!(format!("{}", ProjectType::Rust), "rust");
        assert_eq!(format!("{}", ProjectType::Python), "python");
        assert_eq!(format!("{}", ProjectType::Node), "node");
        assert_eq!(format!("{}", ProjectType::Go), "go");
        assert_eq!(format!("{}", ProjectType::Flutter), "flutter");
        assert_eq!(format!("{}", ProjectType::Docs), "docs");
        assert_eq!(format!("{}", ProjectType::Migration), "migration");
        assert_eq!(format!("{}", ProjectType::Arch), "arch");
    }

    #[test]
    fn test_project_type_migration_from_str() {
        assert!(matches!(
            "migration".parse::<ProjectType>(),
            Ok(ProjectType::Migration)
        ));
        assert!(matches!(
            "migrations".parse::<ProjectType>(),
            Ok(ProjectType::Migration)
        ));
    }

    #[test]
    fn test_project_type_equality() {
        assert_eq!(ProjectType::Migration, ProjectType::Migration);
        assert_ne!(ProjectType::Migration, ProjectType::Rust);
    }

    #[test]
    fn test_detect_project_type_rust() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Rust
        ));
    }

    #[test]
    fn test_detect_project_type_python() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("pyproject.toml"), "[tool.poetry]").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Python
        ));
    }

    #[test]
    fn test_detect_project_type_python_setup() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("setup.py"), "from setuptools").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Python
        ));
    }

    #[test]
    fn test_detect_project_type_node() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("package.json"), "{}").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Node
        ));
    }

    #[test]
    fn test_detect_project_type_go() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("go.mod"), "module test").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Go
        ));
    }

    #[test]
    fn test_detect_project_type_flutter() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("pubspec.yaml"), "name: test").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Flutter
        ));
    }

    #[test]
    fn test_detect_project_type_docs() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(temp_dir.path().join("docs")).unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Docs
        ));
    }

    #[test]
    fn test_detect_project_type_generic() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Generic
        ));
    }

    #[test]
    fn test_detect_project_type_priority() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("pubspec.yaml"), "name: test").unwrap();
        std::fs::write(temp_dir.path().join("package.json"), "{}").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Flutter
        ));
    }

    #[test]
    fn test_detect_project_type_readme_with_src() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("README.md"), "# Test").unwrap();
        std::fs::create_dir(temp_dir.path().join("src")).unwrap();
        assert!(!matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Docs
        ));
    }

    // v9.3.0: Arch project type tests

    #[test]
    fn test_project_type_arch_from_str() {
        assert!(matches!(
            "arch".parse::<ProjectType>(),
            Ok(ProjectType::Arch)
        ));
        assert!(matches!(
            "architecture".parse::<ProjectType>(),
            Ok(ProjectType::Arch)
        ));
    }

    #[test]
    fn test_project_type_arch_equality() {
        assert_eq!(ProjectType::Arch, ProjectType::Arch);
        assert_ne!(ProjectType::Arch, ProjectType::Docs);
    }

    #[test]
    fn test_detect_project_type_arch_c4_models() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(temp_dir.path().join("c4-models")).unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Arch
        ));
    }

    #[test]
    fn test_detect_project_type_arch_decisions() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(temp_dir.path().join("decisions")).unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Arch
        ));
    }

    #[test]
    fn test_detect_project_type_arch_diagrams_with_architecture() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(temp_dir.path().join("diagrams")).unwrap();
        std::fs::write(temp_dir.path().join("ARCHITECTURE_OVERVIEW.md"), "# Arch").unwrap();
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Arch
        ));
    }

    #[test]
    fn test_detect_project_type_arch_priority_over_docs() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::create_dir(temp_dir.path().join("docs")).unwrap();
        std::fs::create_dir(temp_dir.path().join("decisions")).unwrap();
        // Arch markers take priority over docs markers
        assert!(matches!(
            detect_project_type(temp_dir.path()),
            ProjectType::Arch
        ));
    }

    #[test]
    fn test_has_architecture_file_true() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("ARCHITECTURE.md"), "# Test").unwrap();
        assert!(has_architecture_file(temp_dir.path()));
    }

    #[test]
    fn test_has_architecture_file_false() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        std::fs::write(temp_dir.path().join("README.md"), "# Test").unwrap();
        assert!(!has_architecture_file(temp_dir.path()));
    }

    #[test]
    fn test_has_architecture_file_empty_dir() {
        let temp_dir = tempfile::TempDir::new().unwrap();
        assert!(!has_architecture_file(temp_dir.path()));
    }
}
