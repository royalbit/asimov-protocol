//! JSON Schema for asimov.yaml validation (The Three Laws)

pub const ASIMOV_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/asimov.json",
  "title": "RoyalBit Asimov - asimov.yaml",
  "description": "The Three Laws of Robotics - The Open Foundation for AI Ethics",
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
    "first_law": {
      "type": "object",
      "description": "A robot shall not harm humanity, or through inaction allow harm",
      "properties": {
        "status": { "type": "string", "enum": ["REQUIRED", "OPTIONAL"] },
        "description": { "type": "string" },
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
        "allow_no_harm_through_inaction": {
          "type": "object",
          "description": "Through inaction, allow no human to come to harm (ADR-023)",
          "properties": {
            "status": { "type": "string", "enum": ["REQUIRED", "OPTIONAL"] },
            "description": { "type": "string" },
            "adr": { "type": "string" },
            "disclosure": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "rule": { "type": "string" },
                "requires": {
                  "type": "array",
                  "items": { "type": "string" }
                },
                "violation": { "type": "string" }
              }
            },
            "proactive_prevention": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "rule": { "type": "string" },
                "requires": {
                  "type": "array",
                  "items": { "type": "string" }
                },
                "violation": { "type": "string" }
              }
            },
            "transparency_over_convenience": {
              "type": "object",
              "properties": {
                "enabled": { "type": "boolean" },
                "rule": { "type": "string" },
                "priority": { "type": "string" },
                "violation": { "type": "string" }
              }
            }
          }
        },
        "non_negotiable_principles": {
          "type": "object",
          "description": "The five principles that cannot be disabled or bypassed",
          "properties": {
            "description": { "type": "string" },
            "principles": {
              "type": "array",
              "items": {
                "type": "object",
                "properties": {
                  "id": { "type": "integer" },
                  "name": { "type": "string" },
                  "category": { "type": "string" },
                  "rule": { "type": "string" },
                  "enforced_by": { "type": "string" }
                }
              }
            }
          }
        }
      }
    },
    "second_law": {
      "type": "object",
      "description": "A robot shall obey human orders (except when conflicting with First Law)",
      "properties": {
        "status": { "type": "string", "enum": ["REQUIRED", "OPTIONAL"] },
        "description": { "type": "string" },
        "human_veto": {
          "type": "object",
          "properties": {
            "description": { "type": "string" },
            "commands": {
              "type": "array",
              "items": { "type": "string" }
            },
            "on_veto": {
              "type": "array",
              "items": { "type": "string" }
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
        },
        "first_law_override": {
          "type": "object",
          "properties": {
            "description": { "type": "string" },
            "examples": {
              "type": "array",
              "items": { "type": "string" }
            },
            "response": { "type": "string" }
          }
        }
      }
    },
    "third_law": {
      "type": "object",
      "description": "A robot shall preserve itself (within First and Second Law limits)",
      "properties": {
        "status": { "type": "string", "enum": ["REQUIRED", "OPTIONAL"] },
        "description": { "type": "string" },
        "bounded_sessions": {
          "type": "object",
          "properties": {
            "max_hours": {
              "type": "integer",
              "minimum": 1,
              "maximum": 8
            },
            "checkpoint_frequency": { "type": "string" },
            "reason": { "type": "string" }
          }
        },
        "self_healing": {
          "type": "object",
          "properties": {
            "description": { "type": "string" },
            "on_confusion": {
              "type": "array",
              "items": { "type": "string" }
            },
            "checkpoint_file": { "type": "string" }
          }
        },
        "limits": {
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
    "zeroth_law": {
      "type": "object",
      "description": "Harm to humanity supersedes harm to individuals (implicit)",
      "properties": {
        "status": { "type": "string", "enum": ["IMPLICIT", "REQUIRED", "OPTIONAL"] },
        "description": { "type": "string" },
        "note": { "type": "string" }
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
        "on_failure": {
          "type": "object",
          "properties": {
            "action": { "type": "string" },
            "message": { "type": "string" }
          }
        }
      }
    },
    "motto": { "type": "string" }
  }
}"#;
