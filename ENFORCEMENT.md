# MCP-Only Enforcement Mode

**Optional setup:** Force Claude Code to use file-ops MCP server exclusively.

## Why Enforce?

- ✅ Guarantees all file ops use robust JSON-RPC 2.0 protocol
- ✅ All operations logged with request IDs (audit trail)
- ✅ Atomic writes (no partial updates on crash)
- ✅ Content hashing (automatic change detection)
- ✅ No accidental use of fragile built-in tools

## Setup (2 minutes)

Edit `~/.claude/settings.json`:

```json
{
  "permissions": {
    "defaultMode": "bypassPermissions",
    "deny": [
      "Read",
      "Write",
      "Edit",
      "Glob",
      "Grep"
    ]
  },
  "mcpServers": {
    "file-ops": {
      "command": "/path/to/file-ops-rs/target/release/file_ops_rs"
    }
  }
}
```

## What Gets Disabled

| Tool | Instead Use |
|------|-------------|
| `Read` | `mcp__file-ops__file_read` |
| `Write` | `mcp__file-ops__file_create` |
| `Edit` | `mcp__file-ops__file_edit` |
| `Glob` | `mcp__file-ops__file_search` (regex) |
| `Grep` | `mcp__file-ops__file_search` |

## What Stays Enabled

- **Bash** — Shell commands, system access
- **Pencil** — Design system (if configured)
- **MCP Servers** — All other MCP tools
- **Agent** — Parallel task execution

## Verify It Works

Start Claude Code and request a file read:

```
User: "Read src/main.rs"

Claude (will fail):
[ERROR] Cannot use Read tool - denied by permissions
[USING MCP] mcp__file-ops__file_read({"path": "src/main.rs"})

Result: File read via MCP server
```

All operations now go through JSON-RPC with full traceability.

## Rollback

Remove the `deny` array from settings.json to restore built-in tools.

---

**More info:** See [DEPLOYMENT.md](docs/DEPLOYMENT.md)
