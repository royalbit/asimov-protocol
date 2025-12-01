//! JSON Schema for freshness.yaml files

/// Schema for validating freshness.yaml protocol files
pub const FRESHNESS_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/freshness.json",
  "title": "RoyalBit Asimov - freshness.yaml",
  "description": "Date-aware search protocol - stale data is not hallucination",
  "type": "object",
  "properties": {
    "modification_rules": {
      "type": "object",
      "properties": {
        "immutable_without": { "type": "string" },
        "on_modification": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "core_principle": {
      "type": "object",
      "properties": {
        "status": { "type": "string" },
        "insight": { "type": "string" },
        "distinction": {
          "type": "object",
          "properties": {
            "hallucination": {
              "type": "object",
              "properties": {
                "what": { "type": "string" },
                "solution": { "type": "string" }
              }
            },
            "stale_data": {
              "type": "object",
              "properties": {
                "what": { "type": "string" },
                "solution": { "type": "string" }
              }
            }
          }
        },
        "user_perception": { "type": "string" }
      }
    },
    "model_cutoffs": {
      "type": "object",
      "description": "Knowledge cutoff dates for various AI models",
      "properties": {
        "claude": { "type": "object" },
        "openai": { "type": "object" },
        "google": { "type": "object" },
        "xai": { "type": "object" },
        "note": { "type": "string" }
      }
    },
    "always_search": {
      "type": "object",
      "properties": {
        "description": { "type": "string" },
        "temporal_keywords": {
          "type": "array",
          "items": { "type": "string" }
        },
        "volatile_topics": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "volatile_domains": {
      "type": "object",
      "properties": {
        "description": { "type": "string" },
        "high_volatility": {
          "type": "array",
          "items": { "type": "string" }
        },
        "medium_volatility": {
          "type": "array",
          "items": { "type": "string" }
        },
        "low_volatility": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "behavior": {
      "type": "object",
      "properties": {
        "when_search_available": {
          "type": "object",
          "properties": {
            "action": { "type": "string" },
            "priority": { "type": "string" },
            "rationale": { "type": "string" }
          }
        },
        "when_search_unavailable": {
          "type": "object",
          "properties": {
            "action": { "type": "string" },
            "template": { "type": "string" }
          }
        },
        "decision_tree": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "session_integration": {
      "type": "object",
      "properties": {
        "on_session_start": {
          "type": "array",
          "items": { "type": "string" }
        },
        "claude_md_directive": { "type": "string" }
      }
    },
    "validation": {
      "type": "object",
      "properties": {
        "cli_command": { "type": "string" },
        "checks": {
          "type": "array",
          "items": { "type": "string" }
        },
        "warn_on": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "inaction_principle": {
      "type": "object",
      "properties": {
        "status": { "type": "string" },
        "reference": { "type": "string" },
        "adr": { "type": "string" },
        "how_this_protocol_complies": { "type": "object" },
        "first_law_violations_prevented": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "why_this_matters": {
      "type": "object",
      "additionalProperties": { "type": "string" }
    },
    "motto": { "type": "string" }
  }
}"#;
