# Obsidian Vault MCP — Command reference (v0.3)

## `obsidian.guide`

- **Args**: `{}` only
- **Returns**: Full vault writing manual (directories, frontmatter, wikilinks, callouts)

## `obsidian.search`

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `tags` | `string[]` | one of* | AND intersection |
| `exact_name` | `string` | one of* | stem without `.md` |
| `keyword` | `string` | one of* | fuzzy on title, aliases, tags, path |
| `include_index` | `bool` | no | default `false`; if `true`, prepends file tree + tag table |

\* At least one filter must be non-empty.

## `obsidian.write`

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `directory` | `string` | yes | top-level: tech, ai, projects, methods, career, ideas, cheatsheet, journal; max 3 levels |
| `filename` | `string` | yes | lowercase ASCII + digits + hyphens, no `.md` |
| `tags` | `string[]` | no | default `[]` |
| `aliases` | `string[]` | no | default `[]` |
| `status` | `string` | yes | `active` \| `archived` \| `draft` |
| `content` | `string` | yes | markdown body; server adds YAML frontmatter |
| `append` | `bool` | no | default `true`; `false` overwrites entire file |

## `obsidian.read`

| Field | Type | Required |
|-------|------|----------|
| `path` | `string` | yes — must end with `.md`, relative to vault root |

## `obsidian.index`

- **Args**: `{}` only
- **Returns**: File tree, tag counts, note totals

## `obsidian.delete`

| Field | Type | Required |
|-------|------|----------|
| `path` | `string` | yes — `.md` relative path |

Irreversible. Not listed in short `help` output.

## Help tiers

| Tier | Commands shown in default `help` |
|------|----------------------------------|
| Primary | guide, search, write |
| Advanced | read, index (mentioned under "高级") |
| Detail-only | delete (via `help` + `detail`) |

## Error hints

- Unknown command → run `help`
- Invalid args → `help` with `topic=<command>` and `detail: true`
- Search no results → try broader keyword or `obsidian.index`
