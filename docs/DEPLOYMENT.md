# Deployment Guide

## Local Setup

### Prerequisites

**Python:** 3.11+, pip
```bash
python --version  # Should show 3.11+
```

**Rust:** 1.75+ (optional, only for Rust version)
```bash
rustc --version
```

### Python Deployment

#### 1. Install Dependencies

```bash
cd python
pip install -r requirements.txt
```

**Requirements:**
```
fastmcp==0.1.0
serde_json==1.0.149
aiofiles==23.1.0
regex==2024.2.0
```

#### 2. Start Server

```bash
python server.py
```

**Expected output:**
```
INFO - Initializing MCP server...
INFO - Registered 6 tools: file_read, file_edit, file_insert, file_create, file_search, file_structure
INFO - MCP server listening on stdio
```

#### 3. Register in Claude Code

Edit `~/.claude/settings.json` (or use Path appropriate to your OS):

**macOS/Linux:**
```json
{
  "mcpServers": {
    "file-ops": {
      "command": "python",
      "args": [
        "/Users/your-username/projects/mcp-file-ops/python/server.py"
      ]
    }
  }
}
```

**Windows:**
```json
{
  "mcpServers": {
    "file-ops": {
      "command": "python",
      "args": [
        "C:\\Users\\YourUsername\\projects\\mcp-file-ops\\python\\server.py"
      ]
    }
  }
}
```

#### 4. Test

Restart Claude Code, then test:
```
User: Call mcp__file-ops__file_read with path: "README.md"
```

You should get file content back with line count and encoding.

---

### Rust Deployment

#### 1. Build (requires Rust)

```bash
cd rust
cargo build --release
```

**Output:** `target/release/file_ops_rs` (or `.exe` on Windows)

**Build time:** ~60 seconds (first build), ~5 seconds (incremental)

#### 2. Verify Binary

```bash
ls -lh target/release/file_ops_rs
# Should show ~2.2MB binary
```

#### 3. Test Binary Locally

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"tool":"file_read","input":{"path":"Cargo.toml"}}}' | ./target/release/file_ops_rs
```

**Expected response:**
```json
{"jsonrpc":"2.0","id":1,"result":{"content":"...","encoding":"utf-8",...}}
```

#### 4. Register in Claude Code

**macOS/Linux:**
```json
{
  "mcpServers": {
    "file-ops": {
      "command": "/Users/your-username/projects/mcp-file-ops/rust/target/release/file_ops_rs"
    }
  }
}
```

**Windows:**
```json
{
  "mcpServers": {
    "file-ops": {
      "command": "C:\\Users\\YourUsername\\projects\\mcp-file-ops\\rust\\target\\release\\file_ops_rs"
    }
  }
}
```

#### 5. Test

Same as Python: restart Claude Code and call the tools.

---

## Troubleshooting

### Python: `ModuleNotFoundError: No module named 'fastmcp'`

```bash
pip list | grep fastmcp
# If not installed:
pip install -r python/requirements.txt
```

### Python: `Permission denied` when running server

```bash
chmod +x python/server.py
python python/server.py
```

### Rust: `cargo not found`

Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Rust: `link.exe: extra operand` (Windows)

The MSVC linker is being shadowed by Git's `link.exe`. Fix in `rust/.cargo/config.toml`:

```toml
[target.x86_64-pc-windows-msvc]
linker = "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC\\14.39.33519\\bin\\Hostx64\\x64\\link.exe"
```

Adjust version number (14.39...) to match your MSVC installation.

### Server starts but Claude Code can't find it

1. Check command path is absolute (not relative)
2. Verify file exists: `ls -la /path/to/server`
3. Restart Claude Code after editing settings
4. Check stderr in Claude Code terminal for errors

---

## Performance Tuning

### Python: Increase Concurrency

```python
# In server.py, adjust asyncio event loop
import asyncio
asyncio.set_event_loop_policy(asyncio.WindowsSelectorEventLoopPolicy())  # Windows
```

### Rust: Adjust Worker Threads

```rust
// In main.rs, configure tokio runtime
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // ...
}
```

Default: uses CPU count. Increase for I/O-bound workloads.

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Test and Build

on: [push, pull_request]

jobs:
  test-python:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.11'
      - run: pip install -r python/requirements.txt
      - run: python -m pytest python/tests/

  build-rust:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo build --release --manifest-path rust/Cargo.toml
      - uses: actions/upload-artifact@v3
        with:
          name: file_ops_rs
          path: rust/target/release/file_ops_rs
```

---

## Production Checklist

- [ ] Server runs without errors
- [ ] Responds to sample requests within 1 second
- [ ] Handles 10+ concurrent requests
- [ ] Gracefully handles missing files (returns error, doesn't crash)
- [ ] Settings.json uses absolute path
- [ ] Claude Code restarted after registering server
- [ ] Test all 6 tools with real files
- [ ] Verify no data corruption after edit operations
- [ ] Check logs for warnings/errors

---

## Rollback

If server causes issues, revert in `settings.json`:

```json
{
  "mcpServers": {
    // Remove or comment out:
    // "file-ops": { ... }
  }
}
```

Restart Claude Code. Server unloads immediately.

---

## Upgrade Path

### Python → Rust Migration

1. **Test Rust version locally** for 1 week
2. **Run parallel servers** (Python + Rust) with different names
3. **Monitor Rust performance** (watch for crashes, slowdowns)
4. **Fully switch to Rust** once confident
5. **Disable Python server** in settings.json

### Version Updates

```bash
# Python
cd python && pip install --upgrade -r requirements.txt

# Rust
cd rust && cargo update
```

---

## Support

### Debugging

Create a debug request log:
```bash
echo '{"jsonrpc":"2.0","id":999,"method":"tools/call","params":{"tool":"file_read","input":{"path":"README.md"}}}' \
  | python python/server.py 2>&1 | tee debug.log
```

Share output in issues.

### Known Limitations

See [README.md](../README.md#known-shortcomings) for current known issues.

---

**Last Updated:** April 2026

---

## Enforce MCP-Only Mode for Claude Code

**Optional:** Force Claude to use the MCP server exclusively, disabling built-in file manipulation tools.

### Why Enforce MCP?
- Guarantees all file operations are logged + traced
- Ensures atomic operations (no partial writes)
- Prevents accidental use of fragile built-in tools
- Auditable: all operations go through JSON-RPC

### Setup

Edit your Claude Code settings file (`~/.claude/settings.json`):

```json
{
  "permissions": {
    "defaultMode": "bypassPermissions",
    "allow": [
      "mcp__pencil",
      "Bash"
    ],
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
      "command": "/path/to/mcp-file-ops/rust/target/release/file_ops_rs"
    }
  }
}
```

### What Gets Disabled

| Tool | Impact | Workaround |
|------|--------|-----------|
| `Read` | Can't read files directly | Use `mcp__file-ops__file_read` |
| `Write` | Can't create files directly | Use `mcp__file-ops__file_create` |
| `Edit` | Can't edit files directly | Use `mcp__file-ops__file_edit` |
| `Glob` | Can't search files | Use `mcp__file-ops__file_search` |
| `Grep` | Can't grep files | Use `mcp__file-ops__file_search` |

### What Stays Enabled

- **Bash** — Shell commands, system integration
- **Pencil** — Design system (if using)
- **MCP Servers** — All registered MCP tools
- **Agent** — Parallel execution

### Verification

Start Claude Code and try to read a file:

```
User: "Read src/main.rs"
Claude: [Cannot use Read tool - denied by permissions]
         [Using MCP instead] mcp__file-ops__file_read(path: "src/main.rs")
```

All file operations now go through the MCP server with full protocol compliance.

### Audit Trail

With MCP enforcement, all operations are:
- ✅ Logged with request IDs
- ✅ Atomic (all-or-nothing)
- ✅ Validated (JSON-RPC 2.0)
- ✅ Recoverable (content hashing)

---

