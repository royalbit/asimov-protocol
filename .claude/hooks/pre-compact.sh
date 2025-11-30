#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
# ASIMOV PROTOCOL - PreCompact Hook
# ═══════════════════════════════════════════════════════════════════════════════
#
# Triggers: Before context compaction (auto or manual)
# Purpose: Re-inject protocol context that will survive compaction summary
#
# CRITICAL: Compaction happens every ~15 minutes with MAX_THINKING_TOKENS=200000
# This hook fires BEFORE compaction, injecting context into the summary.
#
# When exit code is 0, stdout is injected into Claude's context.
#
# Protocol: https://github.com/royalbit/asimov-protocol
# ═══════════════════════════════════════════════════════════════════════════════

set -e

cat << 'EOF'
🔄 ASIMOV PROTOCOL REFRESH (Pre-Compaction)

══════════════════════════════════════════════════════════════════════════════
CONTEXT REFRESH - Injecting protocol rules before compaction
══════════════════════════════════════════════════════════════════════════════

IMPORTANT: Compaction is about to occur. These rules MUST survive:

CORE RULES (non-negotiable):
- 4 hour MAX session duration
- 1 milestone per session
- Tests MUST pass before release
- ZERO warnings policy
- NO scope creep ("Let me also..." = NO)

POST-COMPACTION ACTIONS:
1. Re-read warmup.yaml for full protocol context
2. Re-read sprint.yaml for current milestone
3. Check TodoWrite for in-progress tasks
4. Continue where you left off

CONFUSION PROTOCOL:
If uncertain: STOP → re-read warmup.yaml → re-read sprint.yaml → continue

ETHICS (Priority 0):
- Do no harm (financial, physical, privacy, deception)
- Transparency over velocity
- When in doubt, ask human

══════════════════════════════════════════════════════════════════════════════
EOF

exit 0
