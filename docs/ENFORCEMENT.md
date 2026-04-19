# MCP Enforcement Guide

**Optional setup** to prevent fragile built-in file tools and guarantee all operations through the MCP server.

## Quick Setup

Edit `~/.claude/settings.json`:

```json
{
  "permissions": {
    "deny": ["Read", "Write", "Edit", "Glob", "Grep"]
  },
  "mcpServers": {
    "file-ops": {
      "command": "/path/to/file_ops_rs"
    }
  }
}
```

## Benefits

✅ Atomic writes (temp + rename)
✅ Auditable operations (request IDs)
✅ Content hashing (change detection)
✅ Proper error codes (JSON-RPC)
✅ 3.15x faster (Rust version)

## Tool Mapping

| Blocked | Use Instead |
|---------|-------------|
| Read | mcp__file-ops__file_read |
| Write | mcp__file-ops__file_create |
| Edit | mcp__file-ops__file_edit |
| Glob | mcp__file-ops__file_search |
| Grep | mcp__file-ops__file_search |

## Reverting

Remove the `deny` list:

```json
{
  "permissions": {
    "defaultMode": "bypassPermissions"
  }
}
```

---

For complete guide, see parent README section: "Optional: Enforce MCP-Only Mode"
