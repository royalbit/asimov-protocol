//! JSON Schema for migrations.yaml files
//!
//! Functional Equivalence Protocol for code migrations.
//! Key principle: "Migration complete" = "functionally equivalent" not just "compiles"

/// Schema for validating migrations.yaml protocol files
pub const MIGRATIONS_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/migrations.json",
  "title": "RoyalBit Asimov - migrations.yaml",
  "description": "Functional Equivalence Protocol - migration complete means behaviorally identical",
  "type": "object",
  "required": ["migration", "equivalence"],
  "properties": {
    "modification_rules": {
      "type": "object",
      "properties": {
        "immutable_without": { "type": "string" },
        "on_modification": {
          "type": "array",
          "items": { "type": "string" }
        },
        "warning": { "type": "string" }
      }
    },
    "migration": {
      "type": "object",
      "description": "Migration project metadata",
      "required": ["name", "source", "target"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Migration project name",
          "minLength": 1
        },
        "description": {
          "type": "string",
          "description": "What this migration accomplishes"
        },
        "source": {
          "type": "object",
          "description": "Legacy/source system details",
          "required": ["language", "path"],
          "properties": {
            "language": {
              "type": "string",
              "description": "Source language (cobol, java, python2, php, etc.)"
            },
            "path": {
              "type": "string",
              "description": "Path to source code"
            },
            "entry_points": {
              "type": "array",
              "description": "Main execution paths to verify",
              "items": { "type": "string" }
            }
          }
        },
        "target": {
          "type": "object",
          "description": "Modern/target system details",
          "required": ["language", "path"],
          "properties": {
            "language": {
              "type": "string",
              "description": "Target language (java, rust, python3, go, etc.)"
            },
            "path": {
              "type": "string",
              "description": "Path to target code"
            },
            "entry_points": {
              "type": "array",
              "description": "Main execution paths to verify",
              "items": { "type": "string" }
            }
          }
        }
      }
    },
    "equivalence": {
      "type": "object",
      "description": "Equivalence verification configuration",
      "required": ["strategies"],
      "properties": {
        "principle": {
          "type": "string",
          "description": "The guiding principle for equivalence"
        },
        "strategies": {
          "type": "object",
          "description": "Verification strategies",
          "properties": {
            "test_parity": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "source_command": { "type": "string" },
                "target_command": { "type": "string" },
                "requirement": { "type": "string" }
              }
            },
            "contract_testing": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "contracts_path": { "type": "string" },
                "format": {
                  "type": "string",
                  "enum": ["yaml", "json", "openapi", "protobuf"]
                }
              }
            },
            "behavioral_snapshots": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "snapshots_path": { "type": "string" },
                "tolerance": {
                  "type": "string",
                  "enum": ["exact", "numeric_tolerance", "semantic"]
                }
              }
            },
            "shadow_mode": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "comparator": { "type": "string" }
              }
            }
          }
        }
      }
    },
    "quality_gates": {
      "type": "object",
      "description": "Quality checkpoints for migration phases",
      "properties": {
        "before_migration": {
          "type": "array",
          "items": { "type": "string" }
        },
        "during_migration": {
          "type": "array",
          "items": { "type": "string" }
        },
        "after_migration": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "red_flags": {
      "type": "object",
      "description": "Patterns that should halt migration",
      "properties": {
        "description": { "type": "string" },
        "patterns": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "validation": {
      "type": "object",
      "properties": {
        "cli_command": { "type": "string" },
        "checks": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "motto": { "type": "string" }
  }
}"#;
