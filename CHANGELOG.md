# Changelog

## [0.4.1] - 2026-05-27

### Fixed

- **Mocode MCP 客户端兼容性** — `executeCommand` 的 `args` 参数现在兼容 JSON 字符串格式，解决 Mocode 等客户端将参数序列化为字符串而非 JSON 对象的问题。
- 自动检测并解析字符串格式的 JSON 参数，无需客户端修改。

## [0.4.0] - 2026-05-22

### Added

- **Configurable `VALID_DIRS`** via `OBSIDIAN_VALID_DIRS`, `OBSIDIAN_CONFIG`, or `{vault}/obsidian-mcp.json`.
- **`serde_yaml` frontmatter** — parse/generate YAML metadata in `note/frontmatter.rs`.
- **`obsidian.semantic_search`** — local weighted search (title/tags/aliases/body); detail-only in help.
- **`CloudVault`** — local cache + best-effort HTTP sync (`OBSIDIAN_CLOUD_URL`, `OBSIDIAN_CLOUD_TOKEN`).
- **`docs/STRUCTURE.md`** and `docs/obsidian-mcp.example.json`.

### Changed

- Reorganized source tree: `config/`, `validation/`, `vault/`, `note/`, `mcp/` (see `docs/STRUCTURE.md`).
- `VaultBackend` trait now covers async read/write/delete; `write-note-tips.md` moved to `docs/`.
- `ObsidianService` injects `AppConfig` + `Validator` instead of static constants.

## [0.3.0] - 2026-05-22

### Breaking

- MCP exposes only `help` and `executeCommand` (removed `query_note`, `write_note`, `read_note`, `note_index_tree`, `write_note_tips`, `delete_note`).
- Use `obsidian.*` commands via `executeCommand` (see README migration table).

### Added

- Command registry with tiered `help` (short catalog / `detail` / `topic`).
- `obsidian.search` supports `include_index`.
- `VaultBackend` trait and `LocalVault` for future cloud storage.
- Library crate + integration tests (`tests/service_integration.rs`, `tests/mcp_stdio.rs`).
- `docs/arch.md`, `docs/todo.md`, Agent skill `.cursor/skills/obsidian-vault-mcp/`.

### Changed

- Upgraded `rmcp` to 0.16, `schemars` to 1.x.
- Async file I/O (`tokio::fs`), stricter path validation (`.md` suffix).
- Default vault fallback `"."` when `OBSIDIAN_VAULT_ROOT` unset.

## [0.2.0] - (internal)

- rmcp 0.16 migration, schema fixes (unreleased tag on upstream fork).

## [0.1.5] - prior release

- Flat MCP tools, rmcp 0.1.x.
