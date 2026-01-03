//! JSON Schema for sycophancy.yaml files

/// Schema for validating sycophancy.yaml protocol files
pub const SYCOPHANCY_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/sycophancy.json",
  "title": "RoyalBit Asimov - sycophancy.yaml",
  "description": "Anti-sycophancy protocol - honest feedback over agreement",
  "type": "object",
  "properties": {
    "modification_rules": {
      "type": "object",
      "properties": {
        "immutable_without": { "type": "string" }
      }
    },
    "core_principles": {
      "type": "object",
      "properties": {
        "status": { "type": "string" },
        "truth_over_comfort": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "respectful_disagreement": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "no_empty_validation": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "constructive_criticism": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "intellectual_honesty": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        }
      }
    },
    "banned_phrases": {
      "type": "object",
      "properties": {
        "description": { "type": "string" },
        "empty_validation": {
          "type": "array",
          "items": { "type": "string" }
        },
        "false_agreement": {
          "type": "array",
          "items": { "type": "string" }
        },
        "excessive_enthusiasm": {
          "type": "array",
          "items": { "type": "string" }
        },
        "deflecting": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "directives": {
      "type": "object",
      "properties": {
        "description": { "type": "string" },
        "principles": {
          "type": "array",
          "items": {
            "type": "object",
            "properties": {
              "directive": { "type": "string" },
              "example": { "type": "string" }
            }
          }
        }
      }
    },
    "anti_patterns": {
      "type": "object",
      "additionalProperties": true
    },
    "on_pressure": {
      "type": "object",
      "properties": {
        "description": { "type": "string" },
        "steps": {
          "type": "array",
          "items": { "type": "string" }
        },
        "never": {
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
