//! JSON Schema for sprint.yaml

pub const SPRINT_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/sprint.json",
  "title": "RoyalBit Asimov - sprint.yaml",
  "description": "Sprint Autonomy Protocol - WHEN to stop (bounded sessions)",
  "type": "object",
  "required": ["rules"],
  "properties": {
    "rules": {
      "type": "object",
      "description": "Core session rules",
      "required": ["must_ship"],
      "properties": {
        "max_milestones": {
          "type": ["integer", "string"],
          "description": "Maximum milestones per session (or 'unlimited')"
        },
        "must_ship": {
          "type": "boolean",
          "description": "Require shipping before session end"
        },
        "mantra": {
          "type": "string",
          "description": "Session mantra"
        }
      }
    },
    "phases": {
      "type": "object",
      "description": "Session phases",
      "properties": {
        "1_warmup": {
          "type": "object",
          "properties": {
            "duration": { "type": "string" },
            "actions": { "type": "array", "items": { "type": "string" } }
          }
        },
        "2_execute": {
          "type": "object",
          "properties": {
            "duration": { "type": "string" },
            "loop": { "type": "array", "items": { "type": "string" } },
            "stop_when": { "type": "array", "items": { "type": "string" } }
          }
        },
        "3_end": {
          "type": "object",
          "properties": {
            "checklist": { "type": "array", "items": { "type": "string" } }
          }
        }
      }
    },
    "anti_patterns": {
      "type": "object",
      "description": "Anti-patterns to avoid",
      "additionalProperties": { "type": "string" }
    },
    "authority": {
      "type": "object",
      "description": "AI authority boundaries",
      "properties": {
        "principle": { "type": "string" },
        "can_release_when": { "type": "array", "items": { "type": "string" } },
        "stop_when": { "type": "array", "items": { "type": "string" } },
        "never_stop_for": { "type": "array", "items": { "type": "string" } },
        "ask_human_only": { "type": "array", "items": { "type": "string" } }
      }
    }
  }
}
"#;
