//! Vault operations invoked by command dispatch.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use crate::config::WRITE_NOTE_TIPS;
use crate::file_tree;
use crate::frontmatter::{generate_frontmatter, update_frontmatter_date};
use crate::index;
use crate::store::{VaultBackend, VaultHandle};
use crate::types::{SearchParams, VaultIndex, WriteNoteParams};
use crate::validation;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("{0}")]
    InvalidParams(String),
    #[error("{0}")]
    Internal(String),
}

impl ServiceError {
    pub fn invalid_params(msg: impl Into<String>) -> Self {
        Self::InvalidParams(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

#[derive(Clone)]
pub struct ObsidianService {
    vault: VaultHandle,
    idx: Arc<RwLock<VaultIndex>>,
}

impl ObsidianService {
    pub fn new() -> Self {
        let vault = VaultHandle::from_env();
        let idx = index::build_index(vault.root());
        Self {
            vault,
            idx: Arc::new(RwLock::new(idx)),
        }
    }

    /// Service bound to an existing vault handle (tests, custom roots).
    pub fn with_vault(vault: VaultHandle) -> Self {
        let idx = index::build_index(vault.root());
        Self {
            vault,
            idx: Arc::new(RwLock::new(idx)),
        }
    }

    pub fn vault(&self) -> &VaultHandle {
        &self.vault
    }

    fn rebuild_index(&self) {
        let new_idx = index::build_index(self.vault.root());
        if let Ok(mut idx) = self.idx.write() {
            *idx = new_idx;
        }
    }

    fn ensure_under_vault(&self, file_path: &std::path::Path) -> Result<(), ServiceError> {
        let canonical = file_path
            .canonicalize()
            .map_err(|e| ServiceError::invalid_params(format!("无效路径: {e}")))?;
        let root = self
            .vault
            .root()
            .canonicalize()
            .map_err(|e| ServiceError::internal(format!("知识库根目录无效: {e}")))?;
        if !canonical.starts_with(&root) {
            return Err(ServiceError::invalid_params("路径超出知识库范围"));
        }
        Ok(())
    }

    pub fn guide(&self) -> Result<String, ServiceError> {
        Ok(WRITE_NOTE_TIPS.to_string())
    }

    pub fn index(&self) -> Result<String, ServiceError> {
        self.rebuild_index();
        Ok(self.format_index_body()?)
    }

    fn format_index_body(&self) -> Result<String, ServiceError> {
        let tree = file_tree::build_file_tree(self.vault.root());
        let idx = self
            .idx
            .read()
            .map_err(|e| ServiceError::internal(format!("lock error: {e}")))?;

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

        Ok(output)
    }

    pub fn search(&self, params: SearchParams) -> Result<String, ServiceError> {
        self.rebuild_index();

        let mut output = String::new();
        if params.include_index {
            output.push_str(&self.format_index_body()?);
            output.push_str("\n---\n\n");
        }

        let idx = self
            .idx
            .read()
            .map_err(|e| ServiceError::internal(format!("lock error: {e}")))?;

        let has_tags = !params.tags.is_empty();
        let has_exact = params
            .exact_name
            .as_ref()
            .is_some_and(|s| !s.trim().is_empty());
        let has_keyword = params
            .keyword
            .as_ref()
            .is_some_and(|s| !s.trim().is_empty());

        if !has_tags && !has_exact && !has_keyword {
            return Err(ServiceError::invalid_params(
                "至少提供 tags（非空数组）、exact_name 或 keyword 中的一个",
            ));
        }

        let mut candidates: Vec<usize> = (0..idx.entries.len()).collect();

        if has_tags {
            for tag in &params.tags {
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

        if let Some(name) = params
            .exact_name
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            let name_lower = name.to_lowercase().replace(".md", "");
            if let Some(&i) = idx.name_map.get(&name_lower) {
                candidates.retain(|c| *c == i);
            } else {
                candidates.clear();
            }
        }

        if let Some(kw) = params
            .keyword
            .as_ref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            let kw_lower = kw.to_lowercase();
            candidates.retain(|&i| {
                let e = &idx.entries[i];
                e.title.to_lowercase().contains(&kw_lower)
                    || e
                        .aliases
                        .iter()
                        .any(|a| a.to_lowercase().contains(&kw_lower))
                    || e.tags.iter().any(|t| t.to_lowercase().contains(&kw_lower))
                    || e.rel_path.to_lowercase().contains(&kw_lower)
            });
        }

        if candidates.is_empty() {
            output.push_str("未找到匹配的笔记。");
            if !params.include_index {
                output.push_str("\n\n提示: 可执行 obsidian.index 查看全库标签，或 help topic=obsidian.search detail=true。");
            }
            return Ok(output);
        }

        output.push_str(&format!("找到 {} 篇匹配笔记：\n\n", candidates.len()));
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

        Ok(output)
    }

    pub async fn read(&self, path: &str) -> Result<String, ServiceError> {
        let rel_path = validation::validate_read_path(path)
            .map_err(ServiceError::invalid_params)?;

        let file_path = self.vault.join(&rel_path);
        if !file_path.exists() {
            return Err(ServiceError::invalid_params(format!(
                "文件不存在: {rel_path}"
            )));
        }

        self.ensure_under_vault(&file_path)?;

        tokio::fs::read_to_string(&file_path)
            .await
            .map_err(|e| ServiceError::internal(format!("读取文件失败: {e}")))
    }

    pub async fn write(&self, params: WriteNoteParams) -> Result<String, ServiceError> {
        let dir = validation::validate_directory(&params.directory)
            .map_err(ServiceError::invalid_params)?;
        let filename = validation::validate_filename(&params.filename)
            .map_err(ServiceError::invalid_params)?;
        validation::validate_status(&params.status).map_err(ServiceError::invalid_params)?;

        let target_dir = self.vault.join(&dir);
        tokio::fs::create_dir_all(&target_dir)
            .await
            .map_err(|e| ServiceError::internal(format!("创建目录失败: {e}")))?;

        let file_path = target_dir.join(format!("{filename}.md"));

        if file_path.exists() {
            self.ensure_under_vault(&file_path)?;
        }

        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let append_mode = params.append;

        if file_path.exists() {
            if !append_mode {
                let frontmatter =
                    generate_frontmatter(&params.tags, &params.aliases, &params.status, &today);
                let full_content = format!("{}{}", frontmatter, params.content);

                tokio::fs::write(&file_path, &full_content)
                    .await
                    .map_err(|e| ServiceError::internal(format!("写入文件失败: {e}")))?;

                self.rebuild_index();

                return Ok(format!(
                    "已覆盖笔记 `{}/{}`，updated 日期已更新为 {today}。",
                    dir, filename
                ));
            }

            let existing = tokio::fs::read_to_string(&file_path)
                .await
                .map_err(|e| ServiceError::internal(format!("读取文件失败: {e}")))?;

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

            tokio::fs::write(&file_path, &updated_content)
                .await
                .map_err(|e| ServiceError::internal(format!("写入文件失败: {e}")))?;

            self.rebuild_index();

            return Ok(format!(
                "已追加内容到 `{}/{}`，updated 日期已更新为 {today}。",
                dir, filename
            ));
        }

        let frontmatter =
            generate_frontmatter(&params.tags, &params.aliases, &params.status, &today);
        let full_content = format!("{}{}", frontmatter, params.content);

        tokio::fs::write(&file_path, &full_content)
            .await
            .map_err(|e| ServiceError::internal(format!("写入文件失败: {e}")))?;

        self.rebuild_index();

        Ok(format!("已创建笔记 `{}/{}`。", dir, filename))
    }

    pub async fn delete(&self, path: &str) -> Result<String, ServiceError> {
        let rel_path = validation::validate_read_path(path)
            .map_err(ServiceError::invalid_params)?;

        let file_path = self.vault.join(&rel_path);
        if !file_path.exists() {
            return Err(ServiceError::invalid_params(format!(
                "文件不存在: {rel_path}"
            )));
        }

        self.ensure_under_vault(&file_path)?;

        tokio::fs::remove_file(&file_path)
            .await
            .map_err(|e| ServiceError::internal(format!("删除文件失败: {e}")))?;

        self.rebuild_index();

        Ok(format!("已删除笔记 `{rel_path}`。"))
    }
}
