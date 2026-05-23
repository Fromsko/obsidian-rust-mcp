use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;

pub fn default_false() -> bool {
    false
}

pub fn default_true() -> bool {
    true
}

pub fn empty_string_vec() -> Vec<String> {
    Vec::new()
}

#[derive(Debug, Clone)]
pub struct NoteEntry {
    pub rel_path: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub status: String,
    pub title: String,
    /// Body text excerpt for semantic search (after frontmatter).
    pub body_excerpt: String,
}

#[derive(Debug, Default)]
pub struct VaultIndex {
    pub entries: Vec<NoteEntry>,
    pub tag_map: HashMap<String, Vec<usize>>,
    pub name_map: HashMap<String, usize>,
}

/// MCP `help` tool parameters.
#[derive(Debug, Default, Deserialize, JsonSchema)]
#[schemars(title = "HelpParams")]
pub struct HelpParams {
    #[schemars(description = "Command name or prefix filter, e.g. obsidian.write or obsidian.")]
    pub topic: Option<String>,
    #[schemars(description = "When true, show parameters and examples")]
    #[serde(default = "default_false")]
    pub detail: bool,
}

/// MCP `executeCommand` tool parameters.
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(title = "ExecuteCommandParams")]
pub struct ExecuteCommandParams {
    #[schemars(description = "Registered command name, e.g. obsidian.search")]
    pub command: String,
    #[schemars(description = "Command-specific JSON arguments")]
    #[serde(default)]
    pub args: serde_json::Value,
}

/// `obsidian.search` arguments.
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(title = "SearchParams")]
pub struct SearchParams {
    #[serde(default)]
    pub tags: Vec<String>,
    pub exact_name: Option<String>,
    pub keyword: Option<String>,
    #[schemars(description = "If true, prepend full vault index (tree + tags)")]
    #[serde(default = "default_false")]
    pub include_index: bool,
}

/// `obsidian.write` arguments.
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(title = "WriteNoteParams")]
pub struct WriteNoteParams {
    pub directory: String,
    pub filename: String,
    #[serde(default = "empty_string_vec")]
    pub tags: Vec<String>,
    #[serde(default = "empty_string_vec")]
    pub aliases: Vec<String>,
    pub status: String,
    pub content: String,
    #[serde(default = "default_true")]
    pub append: bool,
}

/// `obsidian.read` / `obsidian.delete` path argument.
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(title = "PathParams")]
pub struct PathParams {
    pub path: String,
}

/// `obsidian.semantic_search` arguments (detail-only command).
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(title = "SemanticSearchParams")]
pub struct SemanticSearchParams {
    #[schemars(description = "Natural language or keyword query")]
    pub query: String,
    #[schemars(description = "Max results (1–50, default 10)")]
    pub limit: Option<usize>,
}
