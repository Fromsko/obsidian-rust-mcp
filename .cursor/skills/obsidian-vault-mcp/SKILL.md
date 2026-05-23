---
name: obsidian-vault-mcp
description: Operates the Obsidian vault MCP server (obsidian-mcp v0.3) using the CLI-style help and executeCommand tools. Use when the user works with Obsidian notes via MCP, mentions obsidian.guide/search/write, obsidian-rust-mcp, or needs to query or write markdown notes in a configured vault.
---

# Obsidian Vault MCP (CLI model)

## Critical rules

1. **Only two MCP tools exist**: `help` and `executeCommand`. Do not call `query_note`, `write_note`, `read_note`, etc. — those names are obsolete.
2. **Discover before complex writes**: run `help` (short), then `obsidian.guide` before the first `obsidian.write` in a session.
3. **Primary path**: `obsidian.guide` → `obsidian.search` → optional `obsidian.read` → `obsidian.write`.
4. **Vault path**: server reads `OBSIDIAN_VAULT_ROOT` (required in MCP client config).

## Standard workflow

```
- [ ] help (no args) — see obsidian.* commands
- [ ] executeCommand obsidian.guide — vault writing rules (if writing)
- [ ] executeCommand obsidian.search — find target note
- [ ] executeCommand obsidian.read — only if full body needed
- [ ] executeCommand obsidian.write — create or append
```

## Tool: `help`

```json
{}
```

Topic + detail:

```json
{ "topic": "obsidian.write", "detail": true }
```

Prefix filter:

```json
{ "topic": "obsidian." }
```

Use `detail: true` for parameter tables and JSON examples. Default catalog hides `obsidian.delete` unless detail is requested.

## Tool: `executeCommand`

Shape (always):

```json
{
  "command": "obsidian.<name>",
  "args": { }
}
```

`args` must be a JSON **object** matching the command (arrays for `tags`, not comma-separated strings).

### Examples

**Search by tag**

```json
{
  "command": "obsidian.search",
  "args": { "tags": ["docker"] }
}
```

**Search with vault overview**

```json
{
  "command": "obsidian.search",
  "args": { "tags": ["rust"], "include_index": true }
}
```

**Write (append default)**

```json
{
  "command": "obsidian.write",
  "args": {
    "directory": "tech",
    "filename": "docker-guide",
    "tags": ["docker"],
    "aliases": [],
    "status": "active",
    "content": "markdown body without frontmatter",
    "append": true
  }
}
```

**Read**

```json
{
  "command": "obsidian.read",
  "args": { "path": "tech/docker-guide.md" }
}
```

## Command quick reference

| Command | When to use |
|---------|-------------|
| `obsidian.guide` | Vault conventions before writing |
| `obsidian.search` | Find notes (tags AND, keyword, exact_name) |
| `obsidian.write` | Create/append/overwrite notes |
| `obsidian.read` | Full file after search |
| `obsidian.index` | Full tree + tag stats (rare) |
| `obsidian.delete` | Irreversible — only if user insists |

## Common mistakes

| Mistake | Fix |
|---------|-----|
| Empty `obsidian.search` | Provide non-empty `tags`, `keyword`, or `exact_name` |
| `tags` as string `"a, b"` | Use `["a", "b"]` |
| Path without `.md` on read/delete | Use `tech/note.md` |
| Skip guide before write | Run `obsidian.guide` first |
| Use flat legacy tool names | Use `executeCommand` only |

## MCP client config snippet

```json
{
  "obsidian-mcp": {
    "command": "/absolute/path/to/obsidian-mcp",
    "env": {
      "OBSIDIAN_VAULT_ROOT": "/absolute/path/to/vault"
    }
  }
}
```

## More detail

- Full parameter schemas and tiers: [reference.md](reference.md)
- Architecture: [../../../arch.md](../../../arch.md)
