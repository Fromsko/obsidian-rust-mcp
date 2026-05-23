<!--
```yaml
project: Obsidian Rust MCP
description: High-performance MCP server for Obsidian knowledge base management
language: Rust
version: 0.3.0
author: Fromsko
email: fromsko@example.com
license: MIT
keywords:
  - MCP
  - Obsidian
  - Rust
  - Knowledge Management
  - Model Context Protocol
  - Note Management
  - File Tree Indexing
  - Smart Search
  - Tag System
  - High Performance
repository: https://github.com/fromsko/obsidian-rust-mcp
documentation: https://github.com/fromsko/obsidian-rust-mcp/blob/main/README.md
```
-->


# Obsidian Rust MCP

[中文文档](./README_CN.md)

A high-performance MCP (Model Context Protocol) server for Obsidian knowledge base management, built with Rust.

## Features

- 📂 **File Tree Indexing** - Get complete vault structure and tag overview
- 🔍 **Smart Search** - Query notes by tags, exact name, or fuzzy keyword matching
- 📝 **Note Management** - Read, write, and delete notes with automatic frontmatter generation
- 🔄 **Append or Overwrite** - Choose between append mode (default) or overwrite mode for existing files
- 📁 **Subdirectory Support** - Organize notes in nested directories (e.g., `projects/easytier`, `journal/2026-03`)
- 🛡️ **Input Validation** - Directory whitelist, filename sanitization, path traversal protection
- 🏷️ **Tag System** - Organize notes with tags and aliases
- ⚡ **High Performance** - Built with Rust for speed and reliability

## Installation

```bash
cargo build --release
```

## Migration (v0.3 breaking change)

The MCP surface exposes **only** `help` and `executeCommand`. Legacy flat tools are removed:

| Legacy tool | v0.3 replacement |
|-------------|------------------|
| `write_note_tips` | `executeCommand` → `obsidian.guide` |
| `query_note` | `executeCommand` → `obsidian.search` |
| `write_note` | `executeCommand` → `obsidian.write` |
| `read_note` | `executeCommand` → `obsidian.read` |
| `note_index_tree` | `executeCommand` → `obsidian.index` |
| `delete_note` | `executeCommand` → `obsidian.delete` |

## Usage (CLI model — v0.3)

The MCP server exposes **only two tools** to save client tokens:

| Tool | Purpose |
|------|---------|
| `help` | Command manual (short catalog or detailed usage) |
| `executeCommand` | Run a registered `obsidian.*` command |

Typical flow: **`help` → `obsidian.guide` → `obsidian.search` → `obsidian.write`**

### `help`

```json
{}
```

```json
{ "topic": "obsidian.write", "detail": true }
```

### `executeCommand`

```json
{
  "command": "obsidian.search",
  "args": { "tags": ["docker"], "keyword": "nginx" }
}
```

```json
{
  "command": "obsidian.write",
  "args": {
    "directory": "tech",
    "filename": "nginx-guide",
    "tags": ["nginx"],
    "aliases": ["Nginx Guide"],
    "status": "active",
    "content": "markdown body",
    "append": true
  }
}
```

### Registered commands (`obsidian.*`)

| Command | Description |
|---------|-------------|
| `obsidian.guide` | Vault writing guidelines (call before first write) |
| `obsidian.search` | Search by tags / keyword / exact name (`include_index` optional) |
| `obsidian.write` | Create or append note (`append` default `true`) |
| `obsidian.read` | Read note by path (advanced) |
| `obsidian.index` | Full file tree + tag stats (advanced) |
| `obsidian.delete` | Delete note (see `help` with `detail`) |

See [arch.md](./arch.md) for architecture and [todo.md](./todo.md) for roadmap.

## Configuration

### Option 1: Environment Variable (Recommended)
Set the `OBSIDIAN_VAULT_ROOT` environment variable to point to your Obsidian vault:

```bash
# Linux/macOS
export OBSIDIAN_VAULT_ROOT="/path/to/your/vault"

# Windows (cmd)
set OBSIDIAN_VAULT_ROOT=D:\notes\Fromsko

# Windows (PowerShell)
$env:OBSIDIAN_VAULT_ROOT="D:\notes\Fromsko"
```

### Option 2: Hardcoded Path
Edit the `VAULT_ROOT` constant in `src/config.rs`:

```rust
pub const VAULT_ROOT: &str = r"D:\notes\Fromsko";
```

### Option 3: MCP Client Configuration (Recommended for MCP clients)
Configure the vault path directly in your MCP client configuration:

```json
{
  "fromsko-note": {
    "command": "/path/to/obsidian-mcp",
    "disabled": false,
    "env": {
      "OBSIDIAN_VAULT_ROOT": "/path/to/your/vault"
    }
  }
}
```

Replace `/path/to/obsidian-mcp` with the actual path to your compiled binary, and `/path/to/your/vault` with your Obsidian vault path.

**Note**: This is the recommended approach when using this MCP server with clients like Claude Desktop, Cursor, or other MCP-compatible tools.

## Valid Directories

Notes can be organized in the following top-level directories (subdirectories supported, max 3 levels):
- `tech` - Technical notes
- `ai` - AI/ML related notes
- `projects` - Project documentation
- `methods` - Methodologies and processes
- `career` - Career development
- `ideas` - Ideas and brainstorming
- `cheatsheet` - Quick reference guides
- `journal` - Daily journals

## Project Structure

```
src/
  main.rs           # Binary entry
  lib.rs
  server.rs         # MCP: help + executeCommand
  command/          # Registry, help renderer, dispatch
  service/          # Vault operations
  store/            # LocalVault + VaultBackend trait
  config.rs
  types.rs
  validation.rs
  frontmatter.rs
  index.rs
  file_tree.rs
tests/
  service_integration.rs
  mcp_stdio.rs
  registry.rs
arch.md
todo.md
```

## Testing

```bash
cargo test   # unit + integration + MCP stdio
```

## Screenshots

### Agent Integration
![Agent Integration](docs/imgs/agents_readme.png)

### Note Example
![Note Example](docs/imgs/random_note.png)

## License

MIT - see [LICENSE](./LICENSE) file for details.
