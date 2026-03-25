use schemars::JsonSchema;
use serde::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;

pub fn flexible_string_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
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
            Ok(s.split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect())
        }
    }
}

pub fn flexible_string_vec_opt<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
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
            if v.is_empty() {
                Ok(None)
            } else {
                Ok(Some(v))
            }
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
            let v: Vec<String> = s
                .split(',')
                .map(|t| t.trim().to_string())
                .filter(|t| !t.is_empty())
                .collect();
            Ok(if v.is_empty() { None } else { Some(v) })
        }
    }
}

#[derive(Debug, Clone)]
pub struct NoteEntry {
    pub rel_path: String,
    pub tags: Vec<String>,
    pub aliases: Vec<String>,
    pub status: String,
    pub title: String,
}

#[derive(Debug, Default)]
pub struct VaultIndex {
    pub entries: Vec<NoteEntry>,
    pub tag_map: HashMap<String, Vec<usize>>,
    pub name_map: HashMap<String, usize>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "查询笔记的参数。支持三种模式混合使用：按标签过滤、精确文件名匹配、模糊关键词搜索。至少提供一个参数。"
)]
pub struct QueryNoteParams {
    #[schemars(
        description = "按标签过滤，可传多个标签（取交集），如 [\"docker\", \"linux\"] 或 \"docker, linux\""
    )]
    #[serde(default, deserialize_with = "flexible_string_vec_opt")]
    pub tags: Option<Vec<String>>,

    #[schemars(description = "精确匹配文件名（不含 .md 后缀），如 \"docker-guide\"")]
    pub exact_name: Option<String>,

    #[schemars(description = "模糊搜索关键词，同时匹配文件名、别名和标签")]
    pub keyword: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "写入笔记的参数。所有元标签字段必须显式提供。")]
pub struct WriteNoteParams {
    #[schemars(
        description = "目标目录路径，顶级目录必须是以下之一：tech, ai, projects, methods, career, ideas, cheatsheet, journal。支持子目录，如 \"projects/easytier\"、\"journal/2026-03\"。"
    )]
    pub directory: String,

    #[schemars(
        description = "文件名（不含 .md 后缀），必须是英文小写+数字+短横线，如 docker-guide"
    )]
    pub filename: String,

    #[schemars(description = "标签列表，如 [\"docker\", \"linux\"] 或 \"docker, linux\"")]
    #[serde(deserialize_with = "flexible_string_vec")]
    pub tags: Vec<String>,

    #[schemars(description = "中文别名列表，如 [\"Docker 指南\"] 或 \"Docker 指南, Docker 入门\"")]
    #[serde(deserialize_with = "flexible_string_vec")]
    pub aliases: Vec<String>,

    #[schemars(description = "笔记状态：active | archived | draft")]
    pub status: String,

    #[schemars(
        description = "Markdown 正文内容（不含 frontmatter，由服务自动生成）。内容应遵循 Obsidian 格式规范：使用 Callout、Wikilinks、末尾包含 ## 相关笔记 章节。"
    )]
    pub content: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(
    description = "读取笔记的参数。通过相对路径读取笔记完整内容。路径来自 query_note 或 note_index_tree 的返回结果。"
)]
pub struct ReadNoteParams {
    #[schemars(
        description = "笔记的相对路径（从 query_note 返回的 path 字段获取），如 \"tech/docker-guide.md\" 或 \"ai/mcp-development.md\""
    )]
    pub path: String,
}
