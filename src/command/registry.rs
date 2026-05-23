//! Static command catalog for help rendering and dispatch validation.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandTier {
    /// Shown in default `help` (no topic).
    Primary,
    /// Shown as advanced line in short help.
    Advanced,
    /// Only in `help detail` or `help topic=... detail=true`.
    DetailOnly,
}

#[derive(Debug, Clone, Copy)]
pub struct CommandMeta {
    pub name: &'static str,
    pub summary: &'static str,
    pub tier: CommandTier,
    pub detail: &'static str,
    pub example_args: &'static str,
}

pub const COMMANDS: &[CommandMeta] = &[
    CommandMeta {
        name: "obsidian.guide",
        summary: "知识库写入规范（首次 write 前建议执行）",
        tier: CommandTier::Primary,
        detail: "无参数。返回目录结构、命名、Frontmatter、Callout、Wikilinks 等完整规范。",
        example_args: "{}",
    },
    CommandMeta {
        name: "obsidian.search",
        summary: "按标签 / 关键词 / 精确文件名查找笔记",
        tier: CommandTier::Primary,
        detail: "参数: tags (string[], AND), exact_name?, keyword?, include_index? (bool). 至少一个过滤条件。include_index=true 时先输出全库树与标签统计。",
        example_args: r#"{ "tags": ["docker"], "keyword": "nginx" }"#,
    },
    CommandMeta {
        name: "obsidian.write",
        summary: "创建或追加笔记（默认 append=true）",
        tier: CommandTier::Primary,
        detail: "参数: directory, filename, tags[], aliases[], status (active|archived|draft), content, append? (default true).",
        example_args: r##"{ "directory": "tech", "filename": "docker-guide", "tags": ["docker"], "aliases": [], "status": "active", "content": "markdown body", "append": true }"##,
    },
    CommandMeta {
        name: "obsidian.read",
        summary: "按相对路径读取笔记全文",
        tier: CommandTier::Advanced,
        detail: "参数: path (必须以 .md 结尾，如 tech/note.md).",
        example_args: r#"{ "path": "tech/docker-guide.md" }"#,
    },
    CommandMeta {
        name: "obsidian.index",
        summary: "文件树 + 标签统计（全库概览）",
        tier: CommandTier::Advanced,
        detail: "无参数。扫描 vault 输出树状结构与标签表。",
        example_args: "{}",
    },
    CommandMeta {
        name: "obsidian.delete",
        summary: "删除笔记（不可恢复）",
        tier: CommandTier::DetailOnly,
        detail: "参数: path。谨慎使用。",
        example_args: r#"{ "path": "tech/old-note.md" }"#,
    },
    CommandMeta {
        name: "obsidian.semantic_search",
        summary: "语义加权搜索（标题/标签/别名/正文）",
        tier: CommandTier::DetailOnly,
        detail: "参数: query (string), limit? (1–50, default 10). 本地加权检索，无需外部模型。",
        example_args: r#"{ "query": "docker nginx 反向代理", "limit": 5 }"#,
    },
];

pub fn find_command(name: &str) -> Option<&'static CommandMeta> {
    COMMANDS.iter().find(|c| c.name == name)
}

pub fn commands_matching_topic(topic: &str) -> Vec<&'static CommandMeta> {
    let topic = topic.trim();
    if topic.is_empty() {
        return COMMANDS.iter().collect();
    }
    COMMANDS
        .iter()
        .filter(|c| c.name == topic || c.name.starts_with(topic))
        .collect()
}

pub fn suggest_command(partial: &str) -> Option<&'static str> {
    let partial = partial.trim().to_lowercase();
    if partial.is_empty() {
        return None;
    }
    let mut best: Option<&str> = None;
    for c in COMMANDS {
        if c.name.contains(&partial) || partial.contains(c.name.trim_end_matches('*')) {
            best = Some(c.name);
        }
    }
    best
}
