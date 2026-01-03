//! JSON Schema for project.yaml validation
//!
//! Project context file - contains project-specific configuration.
//! Created by ADR-032 to separate project data from hardcoded behavior protocols.

pub const PROJECT_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/project.json",
  "title": "RoyalBit Asimov - project.yaml",
  "description": "Project context file - project-specific configuration (ADR-032)",
  "type": "object",
  "required": ["identity"],
  "properties": {
    "identity": {
      "type": "object",
      "description": "Project identity",
      "required": ["name", "type"],
      "properties": {
        "name": {
          "type": "string",
          "minLength": 1,
          "description": "Project name"
        },
        "type": {
          "type": "string",
          "enum": ["rust", "python", "node", "go", "flutter", "docs", "generic", "migration"],
          "description": "Project type"
        },
        "version": {
          "type": "string",
          "description": "Project version (semver)"
        },
        "tagline": {
          "type": "string",
          "description": "Brief project description"
        },
        "visibility": {
          "type": "string",
          "description": "Visibility notes (e.g., PRIVATE)"
        }
      }
    },
    "quality": {
      "type": "object",
      "description": "Quality gate commands",
      "properties": {
        "test": {
          "type": "string",
          "description": "Test command"
        },
        "lint": {
          "type": "string",
          "description": "Lint command"
        },
        "format": {
          "type": "string",
          "description": "Format check command"
        },
        "build": {
          "type": "string",
          "description": "Build command"
        },
        "types": {
          "type": "string",
          "description": "Type check command (optional)"
        },
        "vet": {
          "type": "string",
          "description": "Vet command (Go)"
        },
        "fix": {
          "type": "string",
          "description": "Auto-fix command"
        },
        "links": {
          "type": "string",
          "description": "Link check command"
        }
      }
    },
    "files": {
      "type": "object",
      "description": "Project file structure",
      "properties": {
        "source": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Source files/directories"
        },
        "config": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Configuration files"
        },
        "tests": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Test files/directories"
        },
        "docs": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Documentation files"
        },
        "diagrams": {
          "type": "array",
          "items": { "type": "string" },
          "description": "Diagram files"
        }
      }
    },
    "patterns": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Language-specific best practices"
    },
    "release": {
      "type": "object",
      "description": "Release configuration",
      "properties": {
        "profile": {
          "type": "string",
          "description": "Release profile configuration"
        },
        "compression": {
          "type": "string",
          "description": "Binary compression command"
        },
        "static_binary": {
          "type": "string",
          "description": "Static binary build command"
        }
      }
    },
    "environment": {
      "type": "object",
      "description": "Environment configuration",
      "properties": {
        "manager": {
          "type": "string",
          "description": "Package/environment manager"
        },
        "python_version": {
          "type": "string",
          "description": "Python version requirement"
        },
        "node_version": {
          "type": "string",
          "description": "Node.js version requirement"
        },
        "package_manager": {
          "type": "string",
          "description": "Package manager preference"
        }
      }
    },
    "platform": {
      "type": "object",
      "description": "Platform-specific configuration",
      "properties": {
        "cupertino_first": {
          "type": "string",
          "description": "Cupertino-first pattern (Flutter)"
        },
        "material_fallback": {
          "type": "string",
          "description": "Material fallback notes (Flutter)"
        }
      }
    },
    "adr_format": {
      "type": "object",
      "description": "ADR format configuration (docs projects)",
      "properties": {
        "template": {
          "type": "string",
          "description": "ADR template"
        }
      }
    }
  },
  "additionalProperties": true
}"#;
