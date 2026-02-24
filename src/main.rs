use anyhow::Result;
use rmcp::{
    model::*,
    tool, tool_box,
    schemars, ServerHandler, ServiceExt,
    transport::io::stdio,
};
use rmcp::handler::server::tool::Parameters;
use rmcp::model::ErrorData as McpError;
use serde::Deserialize;
use serde::de::Deserializer;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use walkdir::WalkDir;

const VAULT_ROOT: &str = r"D:\notes\Fromsko";
// const VAULT_ROOT: &str = r"C:\Users\Administrator\Desktop\ai-code\prompts\notes";

const WRITE_NOTE_TIPS: &str = include_str!("../write-note-tips.md");

fn flexible_string_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVec {
        Vec(Vec<String>),
        Str(String),
    }

    match StringOrVec::deserialize(deserializer)? {
        StringOrVec::Vec(v) => Ok(v),
        StringOrVec::Str(s) => {
            let s = s.trim();
            if s.is_empty() {
                return Ok(Vec::new());
            }
            if s.starts_with('[') {
                if let Ok(v) = serde_json::from_str::<Vec<String>>(s) {
                    return Ok(v);
                }
            }
            Ok(s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect())
        }
    }
}

fn flexible_string_vec_opt<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrVecOrNull {
        Null,
        Vec(Vec<String>),
        Str(String),
    }

    let val: Option<StringOrVecOrNull> = Option::deserialize(deserializer)?;
    match val {
        None | Some(StringOrVecOrNull::Null) => Ok(None),
        Some(StringOrVecOrNull::Vec(v)) => {
            if v.is_empty() { Ok(None) } else { Ok(Some(v)) }
        }
        Some(StringOrVecOrNull::Str(s)) => {
            let s = s.trim();
            if s.is_empty() {
                return Ok(None);
            }
            if s.starts_with('[') {
                if let Ok(v) = serde_json::from_str::<Vec<String>>(s) {
                    return Ok(if v.is_empty() { None } else { Some(v) });
                }
            }
            let v: Vec<String> = s.split(',').map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect();
            Ok(if v.is_empty() { None } else { Some(v) })
        }
    }
}

#[derive(Debug, Clone)]
struct NoteEntry {
    rel_path: String,
    tags: Vec<String>,
    aliases: Vec<String>,
    status: String,
    title: String,
}

#[derive(Debug, Default)]
struct VaultIndex {
    entries: Vec<NoteEntry>,
    tag_map: HashMap<String, Vec<usize>>,
    name_map: HashMap<String, usize>,
}

fn parse_frontmatter(content: &str) -> (Vec<String>, Vec<String>, String) {
    let mut tags = Vec::new();
    let mut aliases = Vec::new();
    let mut status = String::from("active");

    let trimmed = content.trim_start_matches('\u{feff}');
    if !trimmed.starts_with("---") {
        return (tags, aliases, status);
    }

    let after_first = &trimmed[3..];
    let end = match after_first.find("\n---") {
        Some(pos) => pos,
        None => return (tags, aliases, status),
    };
    let fm_block = &after_first[..end];

    let mut current_list: Option<&str> = None;

    for line in fm_block.lines() {
        let t = line.trim();
        if t.starts_with("tags:") {
            current_list = Some("tags");
        } else if t.starts_with("aliases:") {
            current_list = Some("aliases");
        } else if t.starts_with("- ") && current_list.is_some() {
            let val = t.trim_start_matches("- ").trim().to_string();
            match current_list {
                Some("tags") => tags.push(val),
                Some("aliases") => aliases.push(val),
                _ => {}
            }
        } else if t.starts_with("status:") {
            status = t
                .trim_start_matches("status:")
                .trim()
                .split('#')
                .next()
                .unwrap_or("active")
                .trim()
                .to_string();
            current_list = None;
        } else if !t.starts_with("- ") && !t.is_empty() {
            current_list = None;
        }
    }

    (tags, aliases, status)
}

fn build_index(root: &Path) -> VaultIndex {
    let mut index = VaultIndex::default();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().map(|ext| ext == "md").unwrap_or(false)
        })
    {
        let path = entry.path();
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/");

        let title = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let content = std::fs::read_to_string(path).unwrap_or_default();
        let (tags, aliases, status) = parse_frontmatter(&content);

        let idx = index.entries.len();
        index.entries.push(NoteEntry {
            rel_path: rel,
            tags: tags.clone(),
            aliases,
            status,
            title: title.clone(),
        });
        index.name_map.insert(title.to_lowercase(), idx);

        for tag in &tags {
            index
                .tag_map
                .entry(tag.to_lowercase())
                .or_default()
                .push(idx);
        }
    }

    index
}

fn build_file_tree(root: &Path) -> String {
    let mut lines = Vec::new();
    tree_recursive(root, "", &mut lines);
    lines.join("\n")
}

fn tree_recursive(dir: &Path, prefix: &str, lines: &mut Vec<String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter(|e| {
            !e.file_name()
                .to_string_lossy()
                .starts_with('.')
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let total = entries.len();
    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == total - 1;
        let connector = if is_last { "└── " } else { "├── " };
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path();

        if path.is_dir() {
            lines.push(format!("{prefix}{connector}{name}/"));
            let child_prefix = if is_last {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };
            tree_recursive(&path, &child_prefix, lines);
        } else {
            lines.push(format!("{prefix}{connector}{name}"));
        }
    }
}

fn update_frontmatter_date(fm: &str, today: &str) -> String {
    fm.lines()
        .map(|line| {
            if line.trim().starts_with("updated:") {
                format!("updated: {today}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// MCP Server
// ---------------------------------------------------------------------------

#[derive(Clone)]
pub struct ObsidianMcp {
    index: Arc<RwLock<VaultIndex>>,
    vault_root: PathBuf,
}

impl ObsidianMcp {
    pub fn new() -> Self {
        let vault_root = PathBuf::from(VAULT_ROOT);
        let index = build_index(&vault_root);
        Self {
            index: Arc::new(RwLock::new(index)),
            vault_root,
        }
    }

    fn rebuild_index(&self) {
        let new_index = build_index(&self.vault_root);
        if let Ok(mut idx) = self.index.write() {
            *idx = new_index;
        }
    }
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[schemars(description = "查询笔记的参数。支持三种模式混合使用：按标签过滤、精确文件名匹配、模糊关键词搜索。至少提供一个参数。")]
pub struct QueryNoteParams {
    #[schemars(description = "按标签过滤，可传多个标签（取交集），如 [\"docker\", \"linux\"] 或 \"docker, linux\"")]
    #[serde(default, deserialize_with = "flexible_string_vec_opt")]
    tags: Option<Vec<String>>,

    #[schemars(description = "精确匹配文件名（不含 .md 后缀），如 \"docker-guide\"")]
    exact_name: Option<String>,

    #[schemars(description = "模糊搜索关键词，同时匹配文件名、别名和标签")]
    keyword: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[schemars(description = "写入笔记的参数。所有元标签字段必须显式提供。")]
pub struct WriteNoteParams {
    #[schemars(description = "目标分区目录，必须是以下之一：tech, ai, projects, methods, career, ideas, cheatsheet, journal")]
    directory: String,

    #[schemars(description = "文件名（不含 .md 后缀），必须是英文小写+短横线，如 docker-guide")]
    filename: String,

    #[schemars(description = "标签列表，如 [\"docker\", \"linux\"] 或 \"docker, linux\"")]
    #[serde(deserialize_with = "flexible_string_vec")]
    tags: Vec<String>,

    #[schemars(description = "中文别名列表，如 [\"Docker 指南\"] 或 \"Docker 指南, Docker 入门\"")]
    #[serde(deserialize_with = "flexible_string_vec")]
    aliases: Vec<String>,

    #[schemars(description = "笔记状态：active | archived | draft")]
    status: String,

    #[schemars(description = "Markdown 正文内容（不含 frontmatter，由服务自动生成）。内容应遵循 Obsidian 格式规范：使用 Callout、Wikilinks、末尾包含 ## 相关笔记 章节。")]
    content: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[schemars(description = "读取笔记的参数。通过相对路径读取笔记完整内容。路径来自 query_note 或 note_index_tree 的返回结果。")]
pub struct ReadNoteParams {
    #[schemars(description = "笔记的相对路径（从 query_note 返回的 path 字段获取），如 \"tech/docker-guide.md\" 或 \"ai/mcp-development.md\"")]
    path: String,
}

const VALID_DIRS: &[&str] = &[
    "tech", "ai", "projects", "methods", "career", "ideas", "cheatsheet", "journal",
];

#[tool]
impl ObsidianMcp {
    #[tool(
        name = "note_index_tree",
        description = "获取 Obsidian 知识库的完整文件树索引和所有已有标签。用于了解笔记库的整体结构和内容分布。无需参数。"
    )]
    async fn note_index_tree(&self) -> Result<CallToolResult, McpError> {
        self.rebuild_index();
        let tree = build_file_tree(&self.vault_root);

        let idx = self.index.read().map_err(|e| {
            McpError::internal_error(format!("lock error: {e}"), None)
        })?;

        let mut all_tags = BTreeSet::new();
        for entry in &idx.entries {
            for tag in &entry.tags {
                all_tags.insert(tag.clone());
            }
        }

        let mut tag_summary = BTreeMap::new();
        for (tag, indices) in &idx.tag_map {
            tag_summary.insert(tag.clone(), indices.len());
        }

        let mut output = String::new();
        output.push_str("## 📂 文件树\n\n```\n");
        output.push_str(&tree);
        output.push_str("\n```\n\n");

        output.push_str(&format!(
            "## 🏷️ 所有标签（共 {} 个）\n\n",
            all_tags.len()
        ));
        output.push_str("| 标签 | 笔记数 |\n|------|--------|\n");
        for (tag, count) in &tag_summary {
            output.push_str(&format!("| `{tag}` | {count} |\n"));
        }

        output.push_str(&format!(
            "\n## 📊 统计\n\n- 笔记总数：{}\n- 标签总数：{}\n",
            idx.entries.len(),
            all_tags.len()
        ));

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(
        name = "write_note_tips",
        description = "【重要】返回 Obsidian 知识库的完整写入规范文档。在你不确定如何正确写入笔记、不清楚目录结构、文件命名、Frontmatter 格式、Callout 用法、Wikilinks 规则时，必须先调用此工具查阅规范。这是你操作此知识库的权威参考手册。无需任何参数。"
    )]
    async fn write_note_tips(&self) -> Result<CallToolResult, McpError> {
        Ok(CallToolResult::success(vec![Content::text(
            WRITE_NOTE_TIPS.to_string(),
        )]))
    }

    #[tool(
        name = "query_note",
        description = "搜索 Obsidian 知识库中的笔记。支持三种模式混合使用：1) 按标签过滤（多标签取交集）2) 精确文件名匹配 3) 模糊关键词搜索（匹配文件名、别名、标签）。至少提供一个查询参数。\n\n⚠️ 如果你还没有调用过 write_note_tips 了解本知识库的操作规范，请先调用它。\n\n调用示例：{\"tags\": [\"docker\"]} | {\"exact_name\": \"docker-guide\"} | {\"keyword\": \"Docker\"} | 混合: {\"tags\": [\"rust\"], \"keyword\": \"mcp\"}"
    )]
    async fn query_note(
        &self,
        #[tool(aggr)] Parameters(params): Parameters<QueryNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        self.rebuild_index();

        let idx = self.index.read().map_err(|e| {
            McpError::internal_error(format!("lock error: {e}"), None)
        })?;

        if params.tags.is_none() && params.exact_name.is_none() && params.keyword.is_none() {
            return Err(McpError::invalid_params(
                "至少提供 tags、exact_name 或 keyword 中的一个参数",
                None,
            ));
        }

        let mut candidates: Vec<usize> = (0..idx.entries.len()).collect();

        if let Some(ref tags) = params.tags {
            for tag in tags {
                let tag_lower = tag.to_lowercase();
                if let Some(indices) = idx.tag_map.get(&tag_lower) {
                    let set: BTreeSet<usize> = indices.iter().copied().collect();
                    candidates.retain(|i| set.contains(i));
                } else {
                    candidates.clear();
                    break;
                }
            }
        }

        if let Some(ref name) = params.exact_name {
            let name_lower = name.to_lowercase().replace(".md", "");
            if let Some(&i) = idx.name_map.get(&name_lower) {
                candidates.retain(|c| *c == i);
            } else {
                candidates.clear();
            }
        }

        if let Some(ref kw) = params.keyword {
            let kw_lower = kw.to_lowercase();
            candidates.retain(|&i| {
                let e = &idx.entries[i];
                e.title.to_lowercase().contains(&kw_lower)
                    || e.aliases.iter().any(|a| a.to_lowercase().contains(&kw_lower))
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&kw_lower))
                    || e.rel_path.to_lowercase().contains(&kw_lower)
            });
        }

        if candidates.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "未找到匹配的笔记。".to_string(),
            )]));
        }

        let mut output = format!("找到 {} 篇匹配笔记：\n\n", candidates.len());
        output.push_str("| 文件 | 路径 | 标签 | 别名 | 状态 |\n");
        output.push_str("|------|------|------|------|------|\n");

        for &i in &candidates {
            let e = &idx.entries[i];
            output.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                e.title,
                e.rel_path,
                e.tags.join(", "),
                e.aliases.join(", "),
                e.status,
            ));
        }

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(
        name = "read_note",
        description = "读取 Obsidian 知识库中某篇笔记的完整内容。传入笔记的相对路径（从 query_note 搜索结果或 note_index_tree 文件树中获取）。典型流程：先用 query_note 搜索找到目标笔记路径，再用 read_note 读取内容。\n\n调用示例：{\"path\": \"tech/docker-guide.md\"} | {\"path\": \"ai/mcp-development.md\"}"
    )]
    async fn read_note(
        &self,
        #[tool(aggr)] Parameters(params): Parameters<ReadNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        let rel_path = params.path.trim().trim_start_matches('/');
        if rel_path.is_empty() {
            return Err(McpError::invalid_params("路径不能为空", None));
        }

        if rel_path.contains("..") {
            return Err(McpError::invalid_params("路径不能包含 ..", None));
        }

        let file_path = self.vault_root.join(rel_path);
        if !file_path.exists() {
            return Err(McpError::invalid_params(
                format!("文件不存在: {rel_path}"),
                None,
            ));
        }

        let content = std::fs::read_to_string(&file_path).map_err(|e| {
            McpError::internal_error(format!("读取文件失败: {e}"), None)
        })?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(
        name = "write_note",
        description = "向 Obsidian 知识库写入笔记。自动生成 Frontmatter 头部。如果文件已存在则追加内容并更新 updated 日期。\n\n⚠️ 如果你还没有调用过 write_note_tips 了解本知识库的操作规范，请先调用它。\n\n所有 6 个参数必填。调用示例：{\"directory\": \"tech\", \"filename\": \"nginx-guide\", \"tags\": [\"nginx\"], \"aliases\": [\"Nginx 指南\"], \"status\": \"active\", \"content\": \"> [!abstract] 概述\\n> 内容\\n\\n## 相关笔记\\n\\n- [[docker-guide]]\"}"
    )]
    async fn write_note(
        &self,
        #[tool(aggr)] Parameters(params): Parameters<WriteNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        let dir = params.directory.trim().trim_matches('/');
        if !VALID_DIRS.contains(&dir) {
            return Err(McpError::invalid_params(
                format!(
                    "无效的目录 '{dir}'，必须是以下之一：{}",
                    VALID_DIRS.join(", ")
                ),
                None,
            ));
        }

        let filename = params.filename.trim().trim_end_matches(".md");
        if filename.is_empty() {
            return Err(McpError::invalid_params("文件名不能为空", None));
        }
        if filename.contains(' ') || filename.chars().any(|c| c > '\x7f') {
            return Err(McpError::invalid_params(
                "文件名必须是英文小写+短横线，不能包含空格或中文",
                None,
            ));
        }
        if filename != filename.to_lowercase() {
            return Err(McpError::invalid_params("文件名必须全部小写", None));
        }

        let valid_statuses = ["active", "archived", "draft"];
        if !valid_statuses.contains(&params.status.as_str()) {
            return Err(McpError::invalid_params(
                format!(
                    "无效的状态 '{}'，必须是：active, archived, draft",
                    params.status
                ),
                None,
            ));
        }

        let target_dir = self.vault_root.join(dir);
        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir).map_err(|e| {
                McpError::internal_error(format!("创建目录失败: {e}"), None)
            })?;
        }

        let file_path = target_dir.join(format!("{filename}.md"));
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        if file_path.exists() {
            let existing = std::fs::read_to_string(&file_path).map_err(|e| {
                McpError::internal_error(format!("读取文件失败: {e}"), None)
            })?;

            let updated_content = if existing.starts_with("---") {
                if let Some(end_pos) = existing[3..].find("\n---") {
                    let fm = &existing[..end_pos + 3 + 4];
                    let body = &existing[end_pos + 3 + 4..];
                    let updated_fm = update_frontmatter_date(fm, &today);
                    format!("{updated_fm}{body}\n\n{}", params.content)
                } else {
                    format!("{existing}\n\n{}", params.content)
                }
            } else {
                format!("{existing}\n\n{}", params.content)
            };

            std::fs::write(&file_path, &updated_content).map_err(|e| {
                McpError::internal_error(format!("写入文件失败: {e}"), None)
            })?;

            self.rebuild_index();

            return Ok(CallToolResult::success(vec![Content::text(format!(
                "已追加内容到 `{dir}/{filename}.md`，updated 日期已更新为 {today}。"
            ))]));
        }

        let mut frontmatter = String::from("---\ntags:\n");
        for tag in &params.tags {
            frontmatter.push_str(&format!("  - {tag}\n"));
        }
        frontmatter.push_str("aliases:\n");
        for alias in &params.aliases {
            frontmatter.push_str(&format!("  - {alias}\n"));
        }
        frontmatter.push_str(&format!("created: {today}\n"));
        frontmatter.push_str(&format!("updated: {today}\n"));
        frontmatter.push_str(&format!("status: {}\n", params.status));
        frontmatter.push_str("---\n\n");

        let full_content = format!("{frontmatter}{}", params.content);

        std::fs::write(&file_path, &full_content).map_err(|e| {
            McpError::internal_error(format!("写入文件失败: {e}"), None)
        })?;

        self.rebuild_index();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "已创建笔记 `{dir}/{filename}.md`。"
        ))]))
    }
}

impl ObsidianMcp {
    tool_box!(ObsidianMcp {
        note_index_tree,
        write_note_tips,
        query_note,
        read_note,
        write_note
    });
}

impl ServerHandler for ObsidianMcp {
    tool_box!(@derive);

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "obsidian-mcp".into(),
                version: "0.1.0".into(),
            },
            instructions: Some(
                "Obsidian 知识库 MCP 服务。提供笔记索引、查询、写入功能。首次使用请先调用 write_note_tips 查阅操作规范。".to_string(),
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Obsidian MCP Server starting, vault: {}", VAULT_ROOT);

    let service = ObsidianMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
