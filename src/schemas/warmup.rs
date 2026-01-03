//! JSON Schema for warmup.yaml

pub const WARMUP_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "$id": "https://github.com/royalbit/asimov/schemas/warmup.json",
  "title": "RoyalBit Asimov - warmup.yaml",
  "description": "Session bootstrap file for the RoyalBit Asimov",
  "type": "object",
  "required": ["identity"],
  "properties": {
    "identity": {
      "type": "object",
      "description": "Project identity and metadata",
      "required": ["name"],
      "properties": {
        "name": {
          "type": "string",
          "description": "Project name",
          "minLength": 1
        },
        "tagline": {
          "type": "string",
          "description": "Brief project description"
        },
        "version": {
          "type": "string",
          "description": "Project version"
        },
        "philosophy": {
          "type": "string",
          "description": "Guiding principle"
        },
        "author": {
          "type": "string",
          "description": "Project author"
        },
        "license": {
          "type": "string",
          "description": "Project license"
        },
        "protocol": {
          "type": "string",
          "description": "Link to protocol documentation"
        }
      }
    },
    "mission": {
      "type": "object",
      "description": "Problem/solution framing",
      "properties": {
        "problem": {
          "type": "string",
          "description": "What problem does this solve?"
        },
        "solution": {
          "type": "string",
          "description": "How does it solve it?"
        },
        "principles": {
          "type": "array",
          "description": "Guiding principles",
          "items": {
            "type": "string"
          }
        },
        "value": {
          "type": "array",
          "description": "Value propositions",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "protocol": {
      "type": "object",
      "description": "Protocol file definitions",
      "properties": {
        "files": {
          "type": "object",
          "additionalProperties": {
            "type": "string"
          }
        },
        "activation": {
          "type": "string",
          "description": "How to activate the protocol"
        }
      }
    },
    "files": {
      "type": "object",
      "description": "Key files for navigation",
      "additionalProperties": {
        "type": "array",
        "items": {
          "type": "string"
        }
      }
    },
    "session": {
      "type": "object",
      "description": "Workflow guidance for AI",
      "properties": {
        "start": {
          "type": "array",
          "description": "Steps to run at session start",
          "items": {
            "type": "string"
          }
        },
        "during": {
          "type": "array",
          "description": "Guidelines during session",
          "items": {
            "type": "string"
          }
        },
        "end": {
          "type": "array",
          "description": "Steps to run at session end",
          "items": {
            "type": "string"
          }
        }
      }
    },
    "quality": {
      "type": "object",
      "description": "Quality gates and standards",
      "additionalProperties": {
        "type": "string"
      }
    },
    "style": {
      "type": "object",
      "description": "Code and documentation style",
      "additionalProperties": {
        "type": "array",
        "items": {
          "type": "string"
        }
      }
    },
    "release": {
      "type": "object",
      "description": "Release process",
      "properties": {
        "checklist": {
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    }
  }
}
"#;
