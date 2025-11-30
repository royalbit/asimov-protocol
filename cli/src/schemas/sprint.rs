//! JSON Schema for sprint.yaml

pub const SPRINT_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/sprint.json",
  "title": "RoyalBit Asimov - sprint.yaml",
  "description": "Active work tracking for the RoyalBit Asimov",
  "type": "object",
  "required": ["sprint"],
  "properties": {
    "sprint": {
      "type": "object",
      "description": "Current sprint information",
      "required": ["current"],
      "properties": {
        "current": {
          "type": "string",
          "description": "Current sprint name or task",
          "minLength": 1
        },
        "started": {
          "type": "string",
          "description": "Sprint start date (YYYY-MM-DD)"
        },
        "status": {
          "type": "string",
          "description": "Sprint status",
          "enum": ["planned", "in_progress", "blocked", "done"]
        },
        "tasks": {
          "type": "array",
          "description": "Task list with checkboxes",
          "items": {
            "type": "string"
          }
        },
        "completed": {
          "type": "array",
          "description": "Completed tasks",
          "items": {
            "type": "string"
          }
        },
        "blockers": {
          "type": "array",
          "description": "Current blockers",
          "items": {
            "type": "string"
          }
        },
        "next_up": {
          "type": "array",
          "description": "Next tasks to work on",
          "items": {
            "type": "string"
          }
        },
        "notes": {
          "type": "string",
          "description": "Additional context or notes"
        }
      }
    }
  }
}
"#;
