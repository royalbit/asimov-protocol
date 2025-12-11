# Vendor Implementation Guide

AI Tool Compatibility Assessment

## Executive Summary

RoyalBit Asimov works with Claude Code. Due to architectural requirements, it is unlikely to work with other AI tools.

This document provides technical explanation of compatibility requirements and limitations.

## Core Requirements

### What RoyalBit Asimov Requires

```mermaid
flowchart TB
    subgraph reqs["RoyalBit Asimov Requirements"]
        R1["**1. Persistent conversation context that compacts**<br/>The PROBLEM we're solving"]
        R2["**2. Terminal/shell visibility**<br/>How hook output reaches the AI"]
        R3["**3. File system read access mid-session**<br/>How the AI re-reads warmup.json"]
        R4["**4. Auto-loaded config file (CLAUDE.md)**<br/>Bootstrap instruction that survives compaction"]
        ALL["**ALL FOUR required. Missing any one breaks the chain.**"]
    end
    R1 --> R2 --> R3 --> R4 --> ALL
```

### Why Other AI Tools Can't Do This

| AI Tool | Persistent Context | Terminal Visibility | File Re-read | Auto-config | Verdict |
|---------|-------------------|---------------------|--------------|-------------|---------|
| **Claude Code** | ✓ | ✓ | ✓ | ✓ | **Works** |
| **ChatGPT** | ✗ (resets) | ✗ | ✗ | ✗ | **Never** |
| **GitHub Copilot** | ✗ (no conversation) | ✗ | ✗ | ✗ | **Never** |
| **Cursor** | Partial | Partial | Limited | ✓ | **Unlikely** |
| **Gemini** | ✗ (resets) | ✗ | ✗ | ✗ | **Never** |
| **Cody** | ✗ | ✗ | Limited | ✗ | **Never** |

**"Never":** These tools would require fundamental architectural changes.

### The Architecture Problem

**ChatGPT, Gemini:**
- No persistent filesystem access
- Context resets or truncates (no compaction)
- No hook execution or terminal output visibility
- Cloud-based, sandboxed

**GitHub Copilot:**
- Autocomplete, not conversation
- No context, session state, or memory
- Different use case

**Cursor:**
- Has `.cursorrules` (auto-config) ✓
- Has some file access ✓
- Terminal output flow into AI context? Unclear
- Re-read files after compaction? Unlikely

### The Hook Refresh Mechanism (ADR-006)

This is the v2.1.0 innovation that makes RoyalBit Asimov resilient:

```mermaid
flowchart TB
    A["Git commit triggers"] --> B["Pre-commit hook runs"]
    B --> C["asimov refresh outputs banner"]
    C --> D["Terminal shows RoyalBit Asimov reminder"]
    D --> E["Claude Code SEES terminal output"]
    E --> F["Fresh context injection<br/>(not compacted!)"]
    F --> G["AI knows to re-read warmup.json"]
```

**Why this can't work for other AIs:**

1. No terminal visibility - ChatGPT/Gemini can't see local terminal output
2. No local execution - cloud-sandboxed, can't run hooks
3. No file re-read - can't access filesystem mid-session

The hook mechanism requires Claude Code's unique architecture: local CLI with terminal access, filesystem access, and persistent conversational context.

## What Other AIs CAN Use

### The Protocol Files (Universal)

Anyone can use these files - just paste them:

| File | How to Use | Limitation |
|------|------------|------------|
| `warmup.json` | Paste at session start | Must re-paste after context loss |
| `sprint.json` | Paste when asking about work | Manual sync |
| `roadmap.json` | Paste when planning | Manual sync |

**This works but is manual.** When context resets, you lose everything and start over.

### The Validation CLI (Universal)

```bash
# Download from GitHub Releases
curl -L https://github.com/royalbit/asimov/releases/latest/download/asimov-x86_64-unknown-linux-gnu.tar.gz | tar xz
sudo mv asimov /usr/local/bin/

asimov validate          # Works anywhere
asimov lint-docs         # Works anywhere
asimov init              # Works anywhere
```

The CLI is just a Rust binary. It doesn't need AI integration.

## Vendor Compatibility Analysis

### Q: Will other vendors implement these features?

Unlikely:

1. **No business case**: Different products, different goals (chat vs autonomous coding)
2. **Architecture rebuild**: Would require rebuilding from scratch, not a feature request
3. **Security conflicts**: Filesystem access creates liability for sandboxed tools
4. **Market positioning**: Each tool solves different problems

### Q: What if I prefer ChatGPT/Copilot?

Each tool has distinct strengths:
- **ChatGPT**: Brainstorming, research, explanations
- **Copilot**: Autocomplete, inline suggestions
- **Claude Code**: Autonomous extended coding sessions

### Q: Is this vendor lock-in?

Yes, for the full system. Protocol files are vendor-neutral. Self-healing mechanism requires Claude Code's architecture.

If another vendor builds a tool with local CLI execution, terminal visibility, persistent context with compaction, filesystem access, and auto-loaded config files, RoyalBit Asimov would work there too.

### Q: Should I wait for other tools to add support?

Not recommended. Claude Code is available now. The protocol uses standard JSON files, making migration straightforward if alternatives emerge.

## Compatibility Matrix

| Feature | Claude Code | Other AI Tools |
|---------|-------------|----------------|
| Read warmup.json | Auto + re-read | Manual paste |
| Self-healing | ✓ Full | ✗ None |
| Hook refresh | ✓ Works | ✗ Can't |
| Checkpoints | ✓ Auto-written | ✗ Manual |
| Extended sessions | ✓ Yes | ✗ No |
| Quality gates | ✓ Enforced | ✗ Trust-based |
| Context recovery | ✓ Automatic | ✗ Start over |

## Summary

RoyalBit Asimov has two layers:

1. **Protocol Files** (warmup.json, sprint.json, roadmap.json)
   - Universal compatibility via manual paste
   - Useful but requires manual management

2. **RoyalBit Asimov System** (Self-healing, hooks, autonomous sessions)
   - Claude Code exclusive
   - Requires specific architectural capabilities

For full functionality, Claude Code is required. Protocol files remain usable with other AI tools via manual management.

---

## For Vendors

If you're a vendor interested in implementing RoyalBit Asimov compatibility, here are the technical requirements:

### Minimum Requirements

1. **Local CLI that runs in user's terminal**
   - Not a web interface
   - Not a cloud API
   - Actual local process with shell access

2. **Persistent conversational context**
   - That compacts/summarizes (not resets)
   - With detectable compaction events

3. **Terminal output flows into context**
   - AI sees stdout/stderr from commands
   - Including hook output

4. **Filesystem access mid-session**
   - Read files on demand
   - Not just at session start

5. **Auto-loaded config file**
   - Read before first user message
   - Re-read after compaction

### Implementation Notes

- The hook refresh works because hook output is **new input** that arrives after compaction
- "Re-read warmup.json" must be short enough to survive summarization
- Checkpoint files should be in `.gitignore` (session state, not code)

### Testing

If you implement this, test with:

```bash
# 1. Start session, read warmup.json
# 2. Work until context compacts (~15min heavy usage)
# 3. Make a commit (triggers hook)
# 4. Verify AI sees protocol refresh banner
# 5. Verify AI re-reads warmup.json
# 6. Verify rules are restored
```

If all six steps work, you have RoyalBit Asimov compatibility.

---

*Last updated: 2025-12-09*

*This document provides a technical assessment of vendor compatibility based on current architectural capabilities and design constraints. RoyalBit Asimov prioritizes accurate technical documentation over aspirational compatibility claims.*

---
