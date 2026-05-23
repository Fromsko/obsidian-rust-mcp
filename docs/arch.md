# Obsidian MCP — Architecture (v0.4)

## Goals

- **MCP surface**: only `help` and `executeCommand` (minimal tool schema → save client tokens).
- **CLI mental model**: discover via `help`, run via `executeCommand` with `obsidian.*` commands.
- **Independent server**: not a company-wide gateway; cloud backends behind `vault::VaultBackend`.
- **Primary user path**: `guide` → `search` → (`read`) → `write`.

## Layers

```
┌─────────────────────────────────────────────────────────┐
│  mcp/ (rmcp 0.16)                                       │
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
│  guide │ search │ semantic_search │ write │ read │ …    │
└───────────────┬─────────────────────────┬───────────────┘
                │                         │
┌───────────────▼────────────┐  ┌─────────▼───────────────┐
│  vault/                    │  │  note/                  │
│  LocalVault │ CloudVault   │  │  frontmatter │ index    │
│  (VaultBackend trait)      │  │  semantic │ file_tree   │
└───────────────┬────────────┘  └─────────┬───────────────┘
                │                         │
┌───────────────▼─────────────────────────▼───────────────┐
│  config/AppConfig + validation/Validator                │
└─────────────────────────────────────────────────────────┘
```

## Commands (`obsidian.*`)

| Command | Tier | Notes |
|---------|------|-------|
| `obsidian.guide` | primary | Write conventions |
| `obsidian.search` | primary | tags / keyword / exact_name |
| `obsidian.write` | primary | create / append |
| `obsidian.read` | advanced | full note body |
| `obsidian.index` | advanced | file tree + tag stats |
| `obsidian.delete` | detail-only | irreversible |
| `obsidian.semantic_search` | detail-only | weighted local search |

## Configuration

See `docs/STRUCTURE.md` and `docs/obsidian-mcp.example.json`.

Priority: **env > JSON file > defaults**.

Key env vars: `OBSIDIAN_VAULT_ROOT`, `OBSIDIAN_VALID_DIRS`, `OBSIDIAN_VAULT_BACKEND`, `OBSIDIAN_CLOUD_URL`.

## Storage

- **Local** (default): filesystem under `OBSIDIAN_VAULT_ROOT`.
- **Cloud**: `CloudVault` writes locally first, then syncs via `PUT/DELETE {base}/v1/notes/{path}`.

## Testing

- Unit: `config`, `validation`, `note/*`, `command/*`, `vault/cloud`.
- Integration: `tests/service_integration.rs` — full command flows on temp vault.
- MCP: `tests/mcp_stdio.rs` — child process, `list_tools` == 2.

See also: [STRUCTURE.md](./STRUCTURE.md)
