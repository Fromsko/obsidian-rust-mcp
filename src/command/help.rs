use crate::command::registry::{commands_matching_topic, CommandMeta, CommandTier, COMMANDS};
use crate::types::HelpParams;

pub fn render_help(params: &HelpParams) -> String {
    if let Some(ref topic) = params.topic {
        let matches = commands_matching_topic(topic);
        if matches.is_empty() {
            let mut out = format!("未找到命令或前缀: `{topic}`\n");
            out.push_str("已注册: ");
            out.push_str(
                &COMMANDS
                    .iter()
                    .map(|c| c.name)
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            return out;
        }
        if params.detail {
            return matches
                .iter()
                .map(|c| render_command_detail(c))
                .collect::<Vec<_>>()
                .join("\n\n---\n\n");
        }
        return render_short_list(matches);
    }

    if params.detail {
        return COMMANDS
            .iter()
            .map(|c| render_command_detail(c))
            .collect::<Vec<_>>()
            .join("\n\n---\n\n");
    }

    render_default_catalog()
}

fn render_default_catalog() -> String {
    let mut out = String::from("Obsidian MCP — 常用命令\n\n");
    for c in COMMANDS.iter().filter(|c| c.tier == CommandTier::Primary) {
        out.push_str(&format!("  {:<18} {}\n", c.name, c.summary));
    }
    out.push_str("\n高级:\n");
    for c in COMMANDS.iter().filter(|c| c.tier == CommandTier::Advanced) {
        out.push_str(&format!("  {:<18} {}\n", c.name, c.summary));
    }
    out.push_str("\n调用: executeCommand { \"command\": \"obsidian.<name>\", \"args\": { ... } }\n");
    out.push_str("更多: help { \"topic\": \"obsidian.write\", \"detail\": true }\n");
    out.push('\n');
    out
}

fn render_short_list(commands: Vec<&CommandMeta>) -> String {
    let mut out = String::from("匹配的命令:\n\n");
    for c in commands {
        out.push_str(&format!("  {:<18} {}\n", c.name, c.summary));
    }
    out.push_str("\nhelp detail=true 查看参数与示例。\n");
    out
}

fn render_command_detail(c: &CommandMeta) -> String {
    format!(
        "## {}\n\n{}\n\n**参数**\n{}\n\n**示例 args**\n```json\n{}\n```\n",
        c.name, c.summary, c.detail, c.example_args
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_help_lists_primary() {
        let text = render_help(&HelpParams::default());
        assert!(text.contains("obsidian.guide"));
        assert!(text.contains("obsidian.search"));
        assert!(text.contains("obsidian.write"));
        assert!(!text.contains("obsidian.delete") || text.contains("detail"));
    }

    #[test]
    fn topic_detail_shows_delete() {
        let text = render_help(&HelpParams {
            topic: Some("obsidian.delete".into()),
            detail: true,
        });
        assert!(text.contains("obsidian.delete"));
        assert!(text.contains("path"));
    }

    #[test]
    fn prefix_topic_lists_obsidian_commands() {
        let text = render_help(&HelpParams {
            topic: Some("obsidian.".into()),
            detail: false,
        });
        assert!(text.contains("obsidian.guide"));
        assert!(text.contains("obsidian.index"));
    }
}
