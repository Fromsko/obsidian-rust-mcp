# Changelog

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
