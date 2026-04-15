use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use rmcp::handler::server::tool::Parameters;
use rmcp::model::ErrorData as McpError;
use rmcp::model::*;
use rmcp::{tool, tool_box, ServerHandler};

use crate::config::{get_vault_root, WRITE_NOTE_TIPS};
use crate::file_tree;
use crate::frontmatter::{generate_frontmatter, update_frontmatter_date};
use crate::index;
use crate::types::*;
use crate::validation;

#[derive(Clone)]
pub struct ObsidianMcp {
    idx: Arc<RwLock<VaultIndex>>,
    vault_root: PathBuf,
}

impl ObsidianMcp {
    pub fn new() -> Self {
        let vault_root = PathBuf::from(get_vault_root());
        let idx = index::build_index(&vault_root);
        Self {
            idx: Arc::new(RwLock::new(idx)),
            vault_root,
        }
    }

    fn rebuild_index(&self) {
        let new_idx = index::build_index(&self.vault_root);
        if let Ok(mut idx) = self.idx.write() {
            *idx = new_idx;
        }
    }
}

#[tool]
impl ObsidianMcp {
    #[tool(
        name = "note_index_tree",
        description = "获取 Obsidian 知识库的完整文件树索引和所有已有标签。用于了解笔记库的整体结构和内容分布。无需参数。"
    )]
    async fn note_index_tree(&self) -> Result<CallToolResult, McpError> {
        self.rebuild_index();
        let tree = file_tree::build_file_tree(&self.vault_root);

        let idx = self
            .idx
            .read()
            .map_err(|e| McpError::internal_error(format!("lock error: {e}"), None))?;

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

        output.push_str(&format!("## 🏷️ 所有标签（共 {} 个）\n\n", all_tags.len()));
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

        let idx = self
            .idx
            .read()
            .map_err(|e| McpError::internal_error(format!("lock error: {e}"), None))?;

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
                    || e.aliases
                        .iter()
                        .any(|a| a.to_lowercase().contains(&kw_lower))
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
        let rel_path = validation::validate_read_path(&params.path)
            .map_err(|e| McpError::invalid_params(e, None))?;

        let file_path = self.vault_root.join(&rel_path);
        if !file_path.exists() {
            return Err(McpError::invalid_params(
                format!("文件不存在: {rel_path}"),
                None,
            ));
        }

        let content = std::fs::read_to_string(&file_path)
            .map_err(|e| McpError::internal_error(format!("读取文件失败: {e}"), None))?;

        Ok(CallToolResult::success(vec![Content::text(content)]))
    }

    #[tool(
        name = "write_note",
        description = "向 Obsidian 知识库写入笔记。自动生成 Frontmatter 头部。如果文件已存在：\n- append=true（默认）：追加内容并更新 updated 日期\n- append=false：覆盖整个文件\n\n⚠️ 如果你还没有调用过 write_note_tips 了解本知识库的操作规范，请先调用它。\n\n所有 6 个参数必填，append 默认为 true。调用示例：{\"directory\": \"tech\", \"filename\": \"nginx-guide\", \"tags\": [\"nginx\"], \"aliases\": [\"Nginx 指南\"], \"status\": \"active\", \"content\": \"> [!abstract] 概述\\n> 内容\\n\\n## 相关笔记\\n\\n- [[docker-guide]]\"}\n\n✅ 支持子目录：directory 可传 \"projects/easytier\"、\"journal/2026-03\" 等路径。\n\n✅ 覆盖模式示例：{\"directory\": \"tech\", \"filename\": \"docker-guide\", \"tags\": [\"docker\"], \"aliases\": [], \"status\": \"active\", \"content\": \"...\", \"append\": false}"
    )]
    async fn write_note(
        &self,
        #[tool(aggr)] Parameters(params): Parameters<WriteNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        let dir = validation::validate_directory(&params.directory)
            .map_err(|e| McpError::invalid_params(e, None))?;
        let filename = validation::validate_filename(&params.filename)
            .map_err(|e| McpError::invalid_params(e, None))?;
        validation::validate_status(&params.status)
            .map_err(|e| McpError::invalid_params(e, None))?;

        let target_dir = self.vault_root.join(&dir);
        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir)
                .map_err(|e| McpError::internal_error(format!("创建目录失败: {e}"), None))?;
        }

        let file_path = target_dir.join(format!("{filename}.md"));

        if let Ok(canonical) = file_path.canonicalize() {
            if let Ok(root) = self.vault_root.canonicalize() {
                if !canonical.starts_with(&root) {
                    return Err(McpError::invalid_params("路径超出知识库范围", None));
                }
            }
        }

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let append_mode = params.append;

        if file_path.exists() {
            if !append_mode {
                // 覆盖模式：重新生成 frontmatter
                let frontmatter =
                    generate_frontmatter(&params.tags, &params.aliases, &params.status, &today);
                let full_content = format!("{}{}", frontmatter, params.content);

                std::fs::write(&file_path, &full_content)
                    .map_err(|e| McpError::internal_error(format!("写入文件失败: {e}"), None))?;

                self.rebuild_index();

                return Ok(CallToolResult::success(vec![Content::text(format!(
                    "已覆盖笔记 `{}/{}`，updated 日期已更新为 {today}。",
                    dir, filename
                ))]));
            }

            // 追加模式（默认）
            let existing = std::fs::read_to_string(&file_path)
                .map_err(|e| McpError::internal_error(format!("读取文件失败: {e}"), None))?;

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

            std::fs::write(&file_path, &updated_content)
                .map_err(|e| McpError::internal_error(format!("写入文件失败: {e}"), None))?;

            self.rebuild_index();

            return Ok(CallToolResult::success(vec![Content::text(format!(
                "已追加内容到 `{}/{}`，updated 日期已更新为 {today}。",
                dir, filename
            ))]));
        }

        let frontmatter =
            generate_frontmatter(&params.tags, &params.aliases, &params.status, &today);
        let full_content = format!("{frontmatter}{}", params.content);

        std::fs::write(&file_path, &full_content)
            .map_err(|e| McpError::internal_error(format!("写入文件失败: {e}"), None))?;

        self.rebuild_index();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "已创建笔记 `{}/{}`。",
            dir, filename
        ))]))
    }

    #[tool(
        name = "delete_note",
        description = "删除 Obsidian 知识库中的笔记。谨慎使用，此操作不可恢复！\n\n调用示例：{\"path\": \"tech/docker-guide.md\"}"
    )]
    async fn delete_note(
        &self,
        #[tool(aggr)] Parameters(params): Parameters<DeleteNoteParams>,
    ) -> Result<CallToolResult, McpError> {
        let rel_path = validation::validate_read_path(&params.path)
            .map_err(|e| McpError::invalid_params(e, None))?;

        let file_path = self.vault_root.join(&rel_path);
        if !file_path.exists() {
            return Err(McpError::invalid_params(
                format!("文件不存在: {rel_path}"),
                None,
            ));
        }

        // 安全检查：确保文件在 vault_root 内
        if let Ok(canonical) = file_path.canonicalize() {
            if let Ok(root) = self.vault_root.canonicalize() {
                if !canonical.starts_with(&root) {
                    return Err(McpError::invalid_params("路径超出知识库范围", None));
                }
            }
        }

        std::fs::remove_file(&file_path)
            .map_err(|e| McpError::internal_error(format!("删除文件失败: {e}"), None))?;

        self.rebuild_index();

        Ok(CallToolResult::success(vec![Content::text(format!(
            "已删除笔记 `{rel_path}`。"
        ))]))
    }
}

impl ObsidianMcp {
    tool_box!(ObsidianMcp {
        note_index_tree,
        write_note_tips,
        query_note,
        read_note,
        write_note,
        delete_note
    });
}

impl ServerHandler for ObsidianMcp {
    tool_box!(@derive);

    fn get_info(&self) -> InitializeResult {
        InitializeResult {
            protocol_version: ProtocolVersion::LATEST,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "obsidian-mcp".into(),
                version: "0.1.0".into(),
            },
            instructions: Some(
                "Obsidian 知识库 MCP 服务。提供笔记索引、查询、写入功能。首次使用请先调用 write_note_tips 查阅操作规范。"
                    .to_string(),
            ),
        }
    }
}
