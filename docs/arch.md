# Obsidian MCP — Architecture (v0.3)

## Goals

- **MCP surface**: only `help` and `executeCommand` (minimal tool schema → save client tokens).
- **CLI mental model**: discover via `help`, run via `executeCommand` with `obsidian.*` commands.
- **Independent server**: not a company-wide gateway; future cloud/DB backends behind `VaultBackend`.
- **Primary user path**: `guide` → `search` → (`read`) → `write`.

## Layers

```
┌─────────────────────────────────────────────────────────┐
│  MCP (rmcp 0.16)                                        │
│  help │ executeCommand                                  │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  command/                                               │
│  • registry.rs — CommandMeta (name, tier, detail text)  │
│  • help.rs     — render short / detail / topic          │
│  • dispatch.rs — route command + validate args (JSON)   │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  service/ObsidianService                                │
│  guide │ search │ write │ read │ index │ delete         │
└───────────────────────────┬─────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────┐
│  store/LocalVault — VaultBackend + filesystem ops       │
│  index │ frontmatter │ validation │ file_tree           │
└─────────────────────────────────────────────────────────┘
```

## Commands (`obsidian.*`)

| Command | Tier | Maps from (v0.2) |
|---------|------|------------------|
| `obsidian.guide` | primary | `write_note_tips` |
| `obsidian.search` | primary | `query_note` (+ optional `include_index`) |
| `obsidian.write` | primary | `write_note` |
| `obsidian.read` | advanced | `read_note` |
| `obsidian.index` | advanced | `note_index_tree` |
| `obsidian.delete` | detail-only | `delete_note` |

## MCP tool contracts

### `help`

```json
{ "topic": "obsidian.write", "detail": false }
```

- No `topic`: short catalog (primary + hint for advanced/detail).
- `topic`: filter by exact name or prefix (`obsidian.`).
- `detail: true`: parameters, examples, related commands.

### `executeCommand`

```json
{
  "command": "obsidian.search",
  "args": { "tags": ["docker"], "keyword": null, "include_index": false }
}
```

Server deserializes `args` per command and returns text `CallToolResult`.

## Storage evolution

- **v0.3**: `LocalVault` — `OBSIDIAN_VAULT_ROOT` + walkdir index.
- **Future**: `CloudVault` implements `VaultBackend` (list/read/write by id or URI); MCP unchanged.

## Testing

- Unit: `command::help`, `command::dispatch` validation, `service` with temp dirs.
- Integration: `tests/service_integration.rs` — full guide/search/write/read without MCP.
- MCP: `tests/mcp_stdio.rs` — child process, `list_tools` == 2, `executeCommand` round-trip.
