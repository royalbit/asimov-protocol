# Green Protocol - Local-First Coding (Priority 0.5)
# Hardcoded in binary. Cannot be bypassed.
# Date: {TODAY}

principles:
  local_first: true
  zero_tokens_where_possible: true

avoid:
  - Unnecessary API calls when local validation works
  - Cloud services when local tools are available
  - External services for basic validation
  - Repeated network calls for static data

prefer:
  - Local CLI tools (git, cargo, npm, etc.)
  - Cached data when fresh enough
  - Batch operations over individual calls
  - Static analysis before dynamic testing
