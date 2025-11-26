//! JSON Schema for roadmap.yaml

pub const ROADMAP_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/forge-protocol/schemas/roadmap.json",
  "title": "Forge Protocol - roadmap.yaml",
  "description": "Milestone planning for the Forge Protocol",
  "type": "object",
  "properties": {
    "metadata": {
      "type": "object",
      "description": "Roadmap metadata",
      "properties": {
        "current_version": {
          "type": "string",
          "description": "Current project version"
        },
        "last_updated": {
          "type": "string",
          "description": "Last update date (YYYY-MM-DD)"
        },
        "philosophy": {
          "type": "string",
          "description": "Project philosophy"
        }
      }
    },
    "current": {
      "type": "object",
      "description": "Current release information",
      "properties": {
        "version": {
          "type": "string",
          "description": "Version number"
        },
        "status": {
          "type": "string",
          "description": "Release status",
          "enum": ["planned", "in_progress", "released"]
        },
        "date": {
          "type": "string",
          "description": "Release date (YYYY-MM-DD)"
        },
        "summary": {
          "type": "string",
          "description": "Release summary"
        },
        "highlights": {
          "type": "array",
          "description": "Key highlights",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "next": {
      "type": "object",
      "description": "Next milestone",
      "properties": {
        "version": {
          "type": "string",
          "description": "Version number"
        },
        "status": {
          "type": "string",
          "description": "Milestone status",
          "enum": ["planned", "in_progress", "released"]
        },
        "summary": {
          "type": "string",
          "description": "Milestone summary"
        },
        "features": {
          "type": "array",
          "description": "Planned features",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "backlog": {
      "type": "array",
      "description": "Future ideas and features",
      "items": {
        "type": "string"
      }
    }
  }
}
"#;
