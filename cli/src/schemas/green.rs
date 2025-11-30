//! JSON Schema for green.yaml files

/// Schema for validating green.yaml protocol files
pub const GREEN_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Forge Protocol Green Coding Schema",
  "description": "Sustainability protocol for AI development",
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
        "local_first": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "token_efficiency": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "binary_efficiency": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        },
        "carbon_awareness": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" }
          }
        }
      }
    },
    "practices": {
      "type": "object",
      "additionalProperties": true
    },
    "anti_patterns": {
      "type": "object",
      "additionalProperties": true
    },
    "metrics": {
      "type": "object",
      "additionalProperties": true
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
