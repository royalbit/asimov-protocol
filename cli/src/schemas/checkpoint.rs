//! JSON Schema for .claude_checkpoint.yaml validation

pub const CHECKPOINT_SCHEMA: &str = r#"{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "Forge Protocol Checkpoint",
  "description": "Schema for .claude_checkpoint.yaml - Session state for self-healing",
  "type": "object",
  "required": ["milestone"],
  "properties": {
    "timestamp": {
      "type": "string",
      "description": "ISO 8601 timestamp of last checkpoint update"
    },
    "session_started": {
      "type": "string",
      "description": "ISO 8601 timestamp of when session started"
    },
    "tool_calls": {
      "type": "integer",
      "minimum": 0,
      "description": "Number of tool calls made in this session"
    },
    "milestone": {
      "type": "string",
      "minLength": 1,
      "description": "Current milestone being worked on"
    },
    "status": {
      "type": "string",
      "enum": ["planned", "in_progress", "blocked", "done"],
      "description": "Current status of the milestone"
    },
    "completed": {
      "type": "array",
      "items": { "type": "string" },
      "maxItems": 5,
      "description": "Recently completed tasks (rolling window, max 5)"
    },
    "in_progress": {
      "type": "string",
      "description": "Current task being worked on"
    },
    "next_steps": {
      "type": "array",
      "items": { "type": "string" },
      "maxItems": 5,
      "description": "Upcoming tasks (bounded, max 5)"
    },
    "blockers": {
      "type": "array",
      "items": { "type": "string" },
      "description": "Current blockers if any"
    },
    "on_confusion": {
      "type": "string",
      "description": "Recovery command to run when confused"
    },
    "notes": {
      "type": "string",
      "description": "Additional context (optional, trim if checkpoint too large)"
    }
  },
  "additionalProperties": true
}"#;
