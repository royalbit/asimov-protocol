# Warmup Protocol - Session Bootstrap (Priority 0)
# Hardcoded in binary. Cannot be bypassed.
# Date: {TODAY}

on_start:
  - load_protocols: Load all behavior protocols (this is enforced by binary)
  - validate: Validate protocol files exist and are valid
  - read_roadmap: Load project roadmap for context
  - present_milestone: Show current/next milestone to user

ready_phrase: 'Say "go" to start autonomous execution.'

context_injection:
  format: minified_json
  protocols:
    - asimov (Three Laws)
    - freshness (date-aware search)
    - sycophancy (truth over comfort)
    - green (local-first)
    - sprint (session boundaries)
