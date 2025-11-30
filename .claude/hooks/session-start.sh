#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# ASIMOV PROTOCOL - SessionStart Hook
# ═══════════════════════════════════════════════════════════════════════════════
#
# Triggers: startup, resume, clear
# Purpose: Auto-initialize Asimov Protocol on every session start
#
# When exit code is 0, stdout is injected into Claude's context.
# This replaces the need for manual "run warmup" command.
#
# Protocol: https://github.com/royalbit/asimov-protocol
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Get the directory where this script lives
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

cat << 'EOF'
🔥 ASIMOV PROTOCOL ACTIVE

══════════════════════════════════════════════════════════════════════════════
SESSION START - Autonomous Development Protocol Initialized
══════════════════════════════════════════════════════════════════════════════

IMMEDIATE ACTIONS REQUIRED:
1. Read roadmap.yaml for current version and next milestone
2. Read sprint.yaml for session boundaries
3. Run: asimov-mode validate
4. Present next milestone to user
5. Wait for "go" to start autonomous execution

CORE RULES (non-negotiable):
- 4 hour MAX session duration
- 1 milestone per session
- Tests MUST pass before release
- ZERO warnings policy

Say "go" to start autonomous execution.
Say "skip" to pick a different milestone.
Say "plan" to discuss approach first.

══════════════════════════════════════════════════════════════════════════════
EOF

exit 0
