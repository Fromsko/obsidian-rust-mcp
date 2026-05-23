//! Vault operations invoked by command dispatch.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};

use crate::config::{AppConfig, WRITE_NOTE_TIPS};
use crate::note::{
    build_file_tree, build_index, generate_frontmatter, semantic_search, update_frontmatter_date,
};
use crate::types::{SearchParams, SemanticSearchParams, VaultIndex, WriteNoteParams};
use crate::validation::Validator;
use crate::vault::{VaultError, VaultHandle};

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

impl From<VaultError> for ServiceError {
    fn from(e: VaultError) -> Self {
        ServiceError::internal(e.to_string())
    }
}

#[derive(Clone)]
pub struct ObsidianService {
    config: Arc<AppConfig>,
    validator: Validator,
    vault: VaultHandle,
    idx: Arc<RwLock<VaultIndex>>,
}

impl ObsidianService {
    pub fn new() -> Self {
        let config = AppConfig::load().unwrap_or_else(|e| {
            tracing::warn!("config load failed ({e}), using defaults");
            AppConfig::default()
        });
        Self::from_config(config)
    }

    fn from_config(config: AppConfig) -> Self {
        let validator = Validator::from_config(&config);
        let vault = VaultHandle::open(&config).unwrap_or_else(|e| {
            tracing::warn!("vault open failed ({e}), falling back to local");
            VaultHandle::from_path(config.vault_root.clone())
        });
        let idx = build_index(vault.root());
        Self {
            config: Arc::new(config),
            validator,
            vault,
            idx: Arc::new(RwLock::new(idx)),
        }
    }

    /// Service bound to a local vault path (tests).
    pub fn with_vault(vault: VaultHandle) -> Self {
        let config = AppConfig::for_test(vault.root().to_path_buf(), None);
        let validator = Validator::from_config(&config);
        let idx = build_index(vault.root());
        Self {
            config: Arc::new(config),
            validator,
            vault,
            idx: Arc::new(RwLock::new(idx)),
        }
    }

    pub fn vault(&self) -> &VaultHandle {
        &self.vault
    }

    pub fn config(&self) -> &AppConfig {
        &self.config
    }

    fn rebuild_index(&self) {
        let new_idx = build_index(self.vault.root());
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
        let tree = build_file_tree(self.vault.root());
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

    pub fn semantic_search(&self, params: SemanticSearchParams) -> Result<String, ServiceError> {
        let query = params.query.trim();
        if query.is_empty() {
            return Err(ServiceError::invalid_params("query 不能为空"));
        }

        self.rebuild_index();
        let limit = params.limit.unwrap_or(10).clamp(1, 50);

        let idx = self
            .idx
            .read()
            .map_err(|e| ServiceError::internal(format!("lock error: {e}")))?;

        let hits = semantic_search(&idx, query, limit);
        if hits.is_empty() {
            return Ok(format!("语义搜索「{query}」无匹配结果。"));
        }

        let mut output = format!("语义搜索「{query}」找到 {} 篇笔记：\n\n", hits.len());
        output.push_str("| 分数 | 文件 | 路径 | 标签 | 状态 |\n");
        output.push_str("|------|------|------|------|------|\n");

        for hit in hits {
            let e = &idx.entries[hit.index];
            output.push_str(&format!(
                "| {:.1} | `{}` | `{}` | {} | {} |\n",
                hit.score,
                e.title,
                e.rel_path,
                e.tags.join(", "),
                e.status,
            ));
        }

        Ok(output)
    }

    pub async fn read(&self, path: &str) -> Result<String, ServiceError> {
        let rel_path = self
            .validator
            .validate_read_path(path)
            .map_err(ServiceError::invalid_params)?;

        if !self.vault.exists(&rel_path) {
            return Err(ServiceError::invalid_params(format!(
                "文件不存在: {rel_path}"
            )));
        }

        self.ensure_under_vault(&self.vault.join(&rel_path))?;

        self.vault.read_text(&rel_path).await.map_err(Into::into)
    }

    pub async fn write(&self, params: WriteNoteParams) -> Result<String, ServiceError> {
        let dir = self
            .validator
            .validate_directory(&params.directory)
            .map_err(ServiceError::invalid_params)?;
        let filename = self
            .validator
            .validate_filename(&params.filename)
            .map_err(ServiceError::invalid_params)?;
        self.validator
            .validate_status(&params.status)
            .map_err(ServiceError::invalid_params)?;

        self.vault.ensure_dir(&dir).await?;

        let rel_path = format!("{dir}/{filename}.md");
        let file_path = self.vault.join(&rel_path);

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

                self.vault.write_text(&rel_path, &full_content).await?;

                self.rebuild_index();

                return Ok(format!(
                    "已覆盖笔记 `{}/{}`，updated 日期已更新为 {today}。",
                    dir, filename
                ));
            }

            let existing = self.vault.read_text(&rel_path).await?;

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

            self.vault.write_text(&rel_path, &updated_content).await?;

            self.rebuild_index();

            return Ok(format!(
                "已追加内容到 `{}/{}`，updated 日期已更新为 {today}。",
                dir, filename
            ));
        }

        let frontmatter =
            generate_frontmatter(&params.tags, &params.aliases, &params.status, &today);
        let full_content = format!("{}{}", frontmatter, params.content);

        self.vault.write_text(&rel_path, &full_content).await?;

        self.rebuild_index();

        Ok(format!("已创建笔记 `{}/{}`。", dir, filename))
    }

    pub async fn delete(&self, path: &str) -> Result<String, ServiceError> {
        let rel_path = self
            .validator
            .validate_read_path(path)
            .map_err(ServiceError::invalid_params)?;

        if !self.vault.exists(&rel_path) {
            return Err(ServiceError::invalid_params(format!(
                "文件不存在: {rel_path}"
            )));
        }

        self.ensure_under_vault(&self.vault.join(&rel_path))?;

        self.vault.delete_file(&rel_path).await?;

        self.rebuild_index();

        Ok(format!("已删除笔记 `{rel_path}`。"))
    }
}
