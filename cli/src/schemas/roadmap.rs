//! JSON Schema for roadmap.yaml

pub const ROADMAP_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/roadmap.json",
  "title": "Asimov Protocol - roadmap.yaml",
  "description": "Milestone planning for the Asimov Protocol. See docs/PROTOCOL_GOALS.md for core values, CHANGELOG.md for release history.",
  "type": "object",
  "properties": {
    "current": {
      "type": "object",
      "description": "Current milestone in progress",
      "properties": {
        "version": {
          "type": "string",
          "description": "Version number (e.g., 4.1.6)"
        },
        "status": {
          "type": "string",
          "description": "Milestone status",
          "enum": ["planned", "in_progress", "released"]
        },
        "summary": {
          "type": "string",
          "description": "One-line summary"
        },
        "goal": {
          "type": "string",
          "description": "Core value this serves (e.g., ANTI-SYCOPHANCY)"
        },
        "adr": {
          "type": "string",
          "description": "Path to ADR file (e.g., docs/adr/015-anti-sycophancy-protocol.md)"
        },
        "deliverables": {
          "type": "array",
          "description": "Checklist of deliverables",
          "items": {
            "type": "string"
          }
        }
      },
      "required": ["version", "status", "summary"]
    },
    "next": {
      "type": "array",
      "description": "Next planned milestones (top 3-5)",
      "items": {
        "type": "object",
        "properties": {
          "version": {
            "type": "string",
            "description": "Version number"
          },
          "summary": {
            "type": "string",
            "description": "One-line summary"
          },
          "goal": {
            "type": "string",
            "description": "Core value this serves"
          },
          "adr": {
            "type": "string",
            "description": "Path to ADR file (optional)"
          }
        },
        "required": ["version", "summary"]
      }
    },
    "backlog": {
      "type": "array",
      "description": "Future ideas (one-liners)",
      "items": {
        "type": "string"
      }
    }
  },
  "required": ["current"]
}
"#;
