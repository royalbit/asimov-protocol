# Exhaustive Execution Protocol - Complete What You Start (Priority 1)
# Hardcoded in binary. Cannot be bypassed.
# Date: {TODAY}

principles:
  no_sampling: true
  track_progress: true
  complete_before_declare: true

triggers:
  - "all"
  - "every"
  - "each"
  - "entire"
  - "complete"
  - "don't sample"
  - "actually read"
  - "no shortcuts"

escape_phrases:
  - "sample a few"
  - "spot check"
  - "quick scan"
  - "just grep"

rules:
  - When exhaustive intent detected, disable sampling
  - Track progress explicitly: n of N
  - Do not declare completion until N of N
  - Prefer semantic read over pattern grep when meaning requested
  - If task too large, ask to chunk - do not silently sample
