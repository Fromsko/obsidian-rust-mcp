use serde::{Deserialize, Serialize};

fn default_status() -> String {
    "active".to_string()
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FrontmatterYaml {
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default = "default_status")]
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

pub fn split_frontmatter(content: &str) -> Option<(&str, &str)> {
    let trimmed = content.trim_start_matches('\u{feff}');
    if !trimmed.starts_with("---") {
        return None;
    }
    let after_first = &trimmed[3..];
    let end = after_first.find("\n---")?;
    let fm_block = &after_first[..end];
    let body_start = end + 4; // \n---
    let body = &after_first[body_start..];
    Some((fm_block, body))
}

pub fn parse_frontmatter(content: &str) -> (Vec<String>, Vec<String>, String) {
    let Some((fm_block, _)) = split_frontmatter(content) else {
        return (Vec::new(), Vec::new(), default_status());
    };

    match serde_yaml::from_str::<FrontmatterYaml>(fm_block) {
        Ok(fm) => (fm.tags, fm.aliases, fm.status),
        Err(_) => (Vec::new(), Vec::new(), default_status()),
    }
}

pub fn update_frontmatter_date(fm: &str, today: &str) -> String {
    if let Ok(mut parsed) = serde_yaml::from_str::<FrontmatterYaml>(fm) {
        parsed.updated = Some(today.to_string());
        return serde_yaml::to_string(&parsed).unwrap_or_else(|_| fm.to_string());
    }

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

pub fn generate_frontmatter(
    tags: &[String],
    aliases: &[String],
    status: &str,
    today: &str,
) -> String {
    let fm = FrontmatterYaml {
        tags: tags.to_vec(),
        aliases: aliases.to_vec(),
        status: status.to_string(),
        created: Some(today.to_string()),
        updated: Some(today.to_string()),
    };
    let yaml = serde_yaml::to_string(&fm).unwrap_or_default();
    format!("---\n{yaml}---\n\n")
}

pub fn body_excerpt(content: &str, max_len: usize) -> String {
    let body = split_frontmatter(content)
        .map(|(_, b)| b)
        .unwrap_or(content);
    let normalized: String = body.chars().filter(|c| !c.is_control()).collect();
    if normalized.len() <= max_len {
        normalized
    } else {
        normalized.chars().take(max_len).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter_with_tags() {
        let content = "---\ntags:\n  - docker\n  - linux\naliases:\n  - Docker 指南\nstatus: active\n---\n# Title\n";
        let (tags, aliases, status) = parse_frontmatter(content);
        assert_eq!(tags, vec!["docker", "linux"]);
        assert_eq!(aliases, vec!["Docker 指南"]);
        assert_eq!(status, "active");
    }

    #[test]
    fn test_parse_frontmatter_no_frontmatter() {
        let content = "# Just a heading\nSome text";
        let (tags, aliases, status) = parse_frontmatter(content);
        assert!(tags.is_empty());
        assert!(aliases.is_empty());
        assert_eq!(status, "active");
    }

    #[test]
    fn test_parse_frontmatter_with_bom() {
        let content = "\u{feff}---\ntags:\n  - bom-test\n---\nContent";
        let (tags, _, _) = parse_frontmatter(content);
        assert_eq!(tags, vec!["bom-test"]);
    }

    #[test]
    fn test_parse_frontmatter_default_status() {
        let content = "---\ntags:\n  - x\n---\n";
        let (_, _, status) = parse_frontmatter(content);
        assert_eq!(status, "active");
    }

    #[test]
    fn test_update_frontmatter_date() {
        let fm = "tags:\n  - x\nupdated: 2025-01-01\nstatus: active\n";
        let updated = update_frontmatter_date(fm, "2026-03-26");
        assert!(updated.contains("updated: 2026-03-26"));
        assert!(!updated.contains("2025-01-01"));
    }

    #[test]
    fn test_generate_frontmatter_basic() {
        let fm = generate_frontmatter(
            &["docker".into(), "linux".into()],
            &["Docker 指南".into()],
            "active",
            "2026-03-26",
        );
        assert!(fm.starts_with("---\n"));
        assert!(fm.contains("docker"));
        assert!(fm.contains("linux"));
        assert!(fm.contains("2026-03-26"));
        assert!(fm.contains("active"));
        assert!(fm.ends_with("---\n\n"));
    }

    #[test]
    fn test_body_excerpt() {
        let content = "---\ntags: []\n---\n# Hello\n\nLong body text.";
        let excerpt = body_excerpt(content, 20);
        assert!(excerpt.contains("Hello"));
    }
}
