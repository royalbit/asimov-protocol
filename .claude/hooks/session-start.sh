#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# ROYALBIT ASIMOV - SessionStart Hook (v8.16.0)
# ═══════════════════════════════════════════════════════════════════════════════
#
# Triggers: startup, resume, clear
# Purpose: Auto-initialize RoyalBit Asimov on every session start
#
# When exit code is 0, stdout is injected into Claude's context.
#
# v8.16.0: Run asimov warmup --verbose for full session start output
# v8.1.0: project.yaml added for project context (ADR-032)
# v8.0.0: Protocols are hardcoded in binary
#
# Protocol: https://github.com/royalbit/asimov
# ═══════════════════════════════════════════════════════════════════════════════

set -e

# Check if asimov is available
if ! command -v asimov &> /dev/null; then
    cat << 'EOF'
🔥 ROYALBIT ASIMOV ACTIVE (v8.16.0)

══════════════════════════════════════════════════════════════════════════════
SESSION START - Autonomous Development Protocol Initialized
══════════════════════════════════════════════════════════════════════════════

⚠ asimov not found in PATH

Install from: https://github.com/royalbit/asimov

Or run `cargo install --path cli` from the repo root.

══════════════════════════════════════════════════════════════════════════════
EOF
    exit 0
fi

# Run warmup with full verbose output for session start
asimov warmup --verbose

exit 0
