# Obsidian MCP v0.3 — Implementation checklist

## Documentation

- [x] `docs/arch.md` — layer diagram, commands, contracts
- [x] `docs/todo.md` — this file
- [x] `docs/STRUCTURE.md` — workspace layout
- [x] `README.md` — help + executeCommand usage

## Core

- [x] `lib.rs` + `[lib]` crate for tests
- [x] `vault/` — `VaultBackend`, `LocalVault`, `CloudVault`
- [x] `service/` — `ObsidianService` (guide, search, write, read, index, delete, semantic_search)
- [x] `command/registry.rs` — command metadata tiers
- [x] `command/help.rs` — short / detail / topic rendering
- [x] `command/dispatch.rs` — execute routing + arg parse
- [x] `types.rs` — `SearchParams.include_index`, `SemanticSearchParams`, etc.
- [x] `config/` — `AppConfig` from env + JSON file
- [x] `note/frontmatter.rs` — `serde_yaml`
- [x] `note/semantic.rs` — weighted local semantic search

## MCP surface

- [x] `mcp/server.rs` — only `help`, `executeCommand`
- [x] `get_info` instructions → CLI workflow
- [x] Version `0.4.0`

## Tests

- [x] validation / frontmatter / index / semantic unit tests
- [x] `command::help` unit tests
- [x] `command::dispatch` unknown command / bad args
- [x] `service` integration (temp vault): guide, search, write, read, index, delete, semantic_search
- [x] `search` with `include_index: true`
- [x] MCP stdio: list_tools len == 2, help + executeCommand

## Follow-ups (post v0.4)

- [ ] External embedding model hook (`OBSIDIAN_SEMANTIC_MODEL_URL`)
- [ ] Cloud vault pull/sync on read
- [ ] Config hot-reload
