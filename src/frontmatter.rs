pub fn parse_frontmatter(content: &str) -> (Vec<String>, Vec<String>, String) {
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

pub fn update_frontmatter_date(fm: &str, today: &str) -> String {
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
    let mut fm = String::from("---\ntags:\n");
    for tag in tags {
        fm.push_str(&format!("  - {tag}\n"));
    }
    fm.push_str("aliases:\n");
    for alias in aliases {
        fm.push_str(&format!("  - {alias}\n"));
    }
    fm.push_str(&format!("created: {today}\n"));
    fm.push_str(&format!("updated: {today}\n"));
    fm.push_str(&format!("status: {status}\n"));
    fm.push_str("---\n\n");
    fm
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
        let fm = "---\ntags:\n  - x\nupdated: 2025-01-01\nstatus: active\n---";
        let updated = update_frontmatter_date(fm, "2026-03-26");
        assert!(updated.contains("updated: 2026-03-26"));
        assert!(!updated.contains("2025-01-01"));
        assert!(updated.contains("  - x\n"));
        assert!(updated.contains("status: active\n"));
    }

    #[test]
    fn test_update_frontmatter_no_updated_field() {
        let fm = "---\ntags:\n  - x\n---";
        let updated = update_frontmatter_date(fm, "2026-03-26");
        assert_eq!(updated, fm);
    }

    #[test]
    fn test_generate_frontmatter_basic() {
        let fm = generate_frontmatter(
            &["docker".into(), "linux".into()],
            &["Docker 指南".into()],
            "active",
            "2026-03-26",
        );
        assert!(fm.starts_with("---\ntags:\n"));
        assert!(fm.contains("  - docker\n"));
        assert!(fm.contains("  - linux\n"));
        assert!(fm.contains("created: 2026-03-26\n"));
        assert!(fm.contains("status: active\n"));
        assert!(fm.ends_with("---\n\n"));
    }

    #[test]
    fn test_generate_frontmatter_empty_tags() {
        let fm = generate_frontmatter(&[], &[], "draft", "2026-01-01");
        assert!(fm.contains("tags:\naliases:\n"));
        assert!(fm.contains("status: draft\n"));
    }
}
