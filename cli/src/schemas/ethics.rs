//! JSON Schema for ethics.yaml validation

pub const ETHICS_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Forge Protocol Ethics (Humanist Mode)",
  "description": "Schema for ethics.yaml - Social contract for autonomous AI development",
  "type": "object",
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
    "core_principles": {
      "type": "object",
      "required": ["do_no_harm"],
      "properties": {
        "status": { "type": "string" },
        "do_no_harm": {
          "type": "object",
          "properties": {
            "financial": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "examples_blocked": {
                  "type": "array",
                  "items": { "type": "string" }
                }
              }
            },
            "physical": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "examples_blocked": {
                  "type": "array",
                  "items": { "type": "string" }
                }
              }
            },
            "privacy": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "examples_blocked": {
                  "type": "array",
                  "items": { "type": "string" }
                }
              }
            },
            "deception": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "description": { "type": "string" },
                "examples_blocked": {
                  "type": "array",
                  "items": { "type": "string" }
                }
              }
            }
          }
        },
        "transparency_over_velocity": {
          "type": "object",
          "properties": {
            "enabled": { "type": "boolean" },
            "description": { "type": "string" },
            "when_to_pause": {
              "type": "array",
              "items": { "type": "string" }
            }
          }
        }
      }
    },
    "session_limits": {
      "type": "object",
      "properties": {
        "max_unattended_hours": {
          "type": "integer",
          "minimum": 1,
          "maximum": 8
        },
        "reason": { "type": "string" },
        "internet_access": {
          "type": "object",
          "properties": {
            "mode": { "type": "string", "enum": ["read-only", "none", "full"] },
            "allowed": {
              "type": "array",
              "items": { "type": "string" }
            },
            "blocked_by_default": {
              "type": "array",
              "items": { "type": "string" }
            }
          }
        }
      }
    },
    "tool_categories": {
      "type": "object",
      "properties": {
        "always_safe": {
          "type": "object",
          "properties": {
            "description": { "type": "string" },
            "examples": {
              "type": "array",
              "items": { "type": "string" }
            }
          }
        },
        "require_human_review": {
          "type": "object",
          "properties": {
            "description": { "type": "string" },
            "examples": {
              "type": "array",
              "items": { "type": "string" }
            }
          }
        },
        "forbidden_always": {
          "type": "object",
          "properties": {
            "description": { "type": "string" },
            "examples": {
              "type": "array",
              "items": { "type": "string" }
            }
          }
        }
      }
    },
    "red_flags": {
      "type": "object",
      "properties": {
        "description": { "type": "string" },
        "financial": {
          "type": "array",
          "items": { "type": "string" }
        },
        "security": {
          "type": "array",
          "items": { "type": "string" }
        },
        "privacy": {
          "type": "array",
          "items": { "type": "string" }
        },
        "deception": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "human_veto": {
      "type": "object",
      "required": ["command"],
      "properties": {
        "command": { "type": "string", "minLength": 1 },
        "on_veto": {
          "type": "array",
          "items": { "type": "string" }
        },
        "alternative_commands": {
          "type": "array",
          "items": { "type": "string" }
        }
      }
    },
    "on_confusion": {
      "type": "object",
      "properties": {
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
    "fork_requirements": {
      "type": "object",
      "properties": {
        "must_carry": { "type": "string" },
        "recommended_license_addition": { "type": "string" },
        "spirit": { "type": "string" }
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
    }
  }
}"#;
