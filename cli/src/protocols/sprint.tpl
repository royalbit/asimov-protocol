# Sprint Protocol - Session Boundaries (Priority 2)
# Hardcoded in binary. Cannot be bypassed.
# Date: {TODAY}

session:
  max_hours: 4
  checkpoint_interval: 15 minutes

stop_on:
  - roadmap_exhausted: All planned tasks completed
  - blocked: Cannot proceed without human input
  - human_stop: User says stop/halt/abort
  - context_limit: Approaching token limit

on_stop:
  - Save checkpoint
  - Summarize progress
  - List blockers (if any)
  - Propose next steps
