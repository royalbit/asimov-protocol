# ADR-052: CLI Tool Preference Over MCP

**Status:** Accepted
**Date:** 2025-12-11
**Author:** Claude (Opus 4.5) - Principal Engineer

---

## Context

### The Problem

When Claude Code needs to fetch web content, it has multiple options:
1. **Built-in WebSearch/WebFetch** - Server-side tools provided by Anthropic
2. **MCP Server** - Model Context Protocol server wrapping ref-tools
3. **CLI via Bash** - Direct invocation of `ref-tools fetch`

Each approach has different token costs and capabilities.

### MCP Token Overhead Analysis

MCP tool definitions are sent with **every API call** as part of the available tools list. This creates significant overhead:

| Component | Tokens per Tool |
|-----------|-----------------|
| Tool schema (inputSchema) | ~200-400 |
| Tool description | ~50-100 |
| Capability negotiation | ~50-100 |
| **Total per message** | ~300-500 |

**Example: ref-tools as MCP server**

```json
{
  "name": "ref_tools_fetch",
  "description": "Fetch URLs using headless Chrome and convert to JSON/YAML",
  "inputSchema": {
    "type": "object",
    "properties": {
      "urls": {"type": "array", "items": {"type": "string"}},
      "format": {"type": "string", "enum": ["json", "yaml"]},
      "parallel": {"type": "number", "default": 4},
      "max_chars": {"type": "number", "default": 50000},
      "timeout": {"type": "number", "default": 30000}
    },
    "required": ["urls"]
  }
}
```

**Token cost over a 50-message conversation:**
- MCP: ~300 tokens × 50 messages = **15,000 tokens wasted**
- CLI + warmup directive: ~50 tokens (one-time)
- Slash command: 0 tokens (on-demand)

### MCP Architecture Overhead

From the official MCP documentation:

> "Tool Discovery: Clients can first list all available tools (`tools/list`) and then execute them"
> "Each tool object in the response includes: name, title, description, inputSchema"

Source: [MCP Architecture Overview](https://modelcontextprotocol.io/docs/concepts/architecture)

This means:
1. Tool schemas are transmitted every message
2. Capability negotiation adds overhead
3. JSON-RPC wrapper adds protocol overhead

### CLI Advantages

1. **Zero standing overhead** - Only pay tokens when you use it
2. **Warmup detection** - One-time directive in session context
3. **User control** - Local binary, no vendor dependency
4. **Bypass bot protection** - Headless Chrome vs server-side fetch

## Decision

### 1. Prefer CLI over MCP for static tools

When a tool is known and stable (like ref-tools), use CLI via Bash instead of MCP:

```yaml
# Warmup directive (one-time, ~50 tokens)
tools_available:
  ref-tools: ~/bin/ref-tools
  directive: "Use `ref-tools fetch <url>` via Bash instead of WebFetch"
```

### 2. Warmup Detection

Add ref-tools detection to `asimov warmup`:

```
TOOLS_AVAILABLE:
  ref-tools: ~/bin/ref-tools (v0.5.0)
  DIRECTIVE: Use `ref-tools fetch <url>` via Bash instead of WebFetch/WebSearch
```

### 3. Slash Commands for Explicit Control

Create `/fetch` command for explicit user override:

```markdown
# .claude/commands/fetch.md
Execute: ref-tools fetch $ARGUMENTS
Parse JSON output and summarize findings.
```

### 4. Reserve MCP for Dynamic Tools

MCP is appropriate when:
- Tools change dynamically during session
- Tools require real-time discovery
- Third-party integrations (Sentry, databases)

MCP is **not** appropriate for:
- Static CLI tools in PATH
- Known binaries with stable interfaces
- High-frequency operations

## Consequences

### Positive

1. **~15,000 tokens saved per session** - No schema overhead
2. **User control** - Local tools, no vendor dependency
3. **Better results** - Headless Chrome bypasses bot protection
4. **Simpler architecture** - No MCP server to maintain

### Negative

1. **Requires explicit directive** - Must include in warmup
2. **No dynamic discovery** - Tool must be pre-known
3. **PATH dependency** - Tool must be installed

### Neutral

1. **MCP remains available** - For dynamic/third-party tools
2. **Hybrid approach** - Use best tool for each case

## Implementation

### Phase 1: Slash Commands

Create `.claude/commands/fetch.md`:
```markdown
Use ref-tools to fetch the provided URLs via headless Chrome.
Execute: ref-tools fetch $ARGUMENTS
Parse the JSON output and provide a summary of findings.
```

### Phase 2: Warmup Detection (v9.17.0) ✓

Implemented in v9.17.0:
- `WarmupResult.tools_available` field added
- `detect_tools()` function checks PATH for ref-tools
- JSON output includes `tools` array
- Verbose mode shows "TOOLS AVAILABLE" section

### Phase 3: Documentation

Update SPECIFICATION.md with tool preference guidance.

## Token Cost Comparison

| Approach | Setup Cost | Per-Message Cost | 50-Message Session |
|----------|-----------|------------------|-------------------|
| MCP Server | ~100 | ~300 | **15,100 tokens** |
| CLI + Warmup | ~50 | 0 | **50 tokens** |
| Slash Command | 0 | 0 (on-demand) | **0 tokens** |
| WebFetch (built-in) | 0 | ~200 | **10,000 tokens** |

**CLI approach is 300x more token-efficient than MCP.**

## References

### MCP Documentation

- [MCP Architecture Overview](https://modelcontextprotocol.io/docs/concepts/architecture)
- [MCP Tool Discovery](https://modelcontextprotocol.io/docs/concepts/architecture#understanding-the-tool-discovery-response)

### Related ADRs

- ADR-046: Reference Tools Integration
- ADR-050: Economic Incentives in LLM Inference
- ADR-051: System Prompt Hierarchy and Training Override

---

*Built with [RoyalBit Asimov](https://github.com/royalbit/asimov)*
