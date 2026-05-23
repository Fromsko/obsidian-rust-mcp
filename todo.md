# Obsidian MCP v0.3 — Implementation checklist

## Documentation

- [x] `arch.md` — layer diagram, commands, contracts
- [x] `todo.md` — this file
- [x] `README.md` — help + executeCommand usage

## Core

- [x] `lib.rs` + `[lib]` crate for tests
- [x] `store/` — `VaultBackend`, `LocalVault`
- [x] `service/` — `ObsidianService` (guide, search, write, read, index, delete)
- [x] `command/registry.rs` — command metadata tiers
- [x] `command/help.rs` — short / detail / topic rendering
- [x] `command/dispatch.rs` — execute routing + arg parse
- [x] `types.rs` — `SearchParams.include_index`, `HelpParams`, `ExecuteCommandParams`

## MCP surface

- [x] `server.rs` — only `help`, `executeCommand`
- [x] `get_info` instructions → CLI workflow
- [x] Version `0.3.0`

## Tests

- [x] Existing validation / frontmatter / index unit tests
- [x] `command::help` unit tests
- [x] `command::dispatch` unknown command / bad args
- [x] `service` integration (temp vault): guide, search, write, read, index, delete
- [x] `search` with `include_index: true`
- [x] MCP stdio: list_tools len == 2, help + executeCommand

## Follow-ups (post v0.3)

- [x] `README_CN.md` full sync
- [ ] Configurable `VALID_DIRS` via file/env
- [ ] `serde_yaml` frontmatter
- [ ] Semantic search command (internal only, optional in help)
- [ ] Cloud `VaultBackend`
