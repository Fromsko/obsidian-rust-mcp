use crate::config::{VALID_DIRS, VALID_STATUSES};

fn is_valid_name_segment(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    let first = bytes[0];
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }
    for &b in &bytes[1..] {
        if !b.is_ascii_lowercase() && !b.is_ascii_digit() && b != b'-' {
            return false;
        }
    }
    let last = *bytes.last().unwrap();
    last.is_ascii_lowercase() || last.is_ascii_digit()
}

pub fn validate_directory(dir: &str) -> Result<String, String> {
    let dir = dir.trim().replace('\\', "/");
    let dir = dir.trim_matches('/').to_string();

    if dir.is_empty() {
        return Err("目录不能为空".into());
    }
    if dir.contains("..") {
        return Err("目录路径不能包含 ..".into());
    }

    let segments: Vec<&str> = dir.split('/').collect();
    if segments.len() > 3 {
        return Err("目录深度不能超过 3 层".into());
    }
    if !VALID_DIRS.contains(&segments[0]) {
        return Err(format!(
            "无效的顶级目录 '{}'，必须是：{}",
            segments[0],
            VALID_DIRS.join(", ")
        ));
    }

    for &seg in &segments {
        if seg.is_empty() {
            return Err("目录路径不能包含空段（连续 /）".into());
        }
        if seg == "." {
            return Err("目录路径不能包含 .".into());
        }
        if !is_valid_name_segment(seg) {
            return Err(format!(
                "目录段 '{}' 不符合命名规范（仅允许英文小写+数字+短横线）",
                seg
            ));
        }
    }

    Ok(dir)
}

pub fn validate_filename(filename: &str) -> Result<String, String> {
    let trimmed = filename.trim();
    let name = trimmed.strip_suffix(".md").unwrap_or(trimmed);

    if name.is_empty() {
        return Err("文件名不能为空".into());
    }
    if !is_valid_name_segment(name) {
        return Err("文件名必须是英文小写+数字+短横线".into());
    }

    Ok(name.to_string())
}

pub fn validate_status(status: &str) -> Result<(), String> {
    if !VALID_STATUSES.contains(&status) {
        return Err(format!(
            "无效的状态 '{}'，必须是：{}",
            status,
            VALID_STATUSES.join(", ")
        ));
    }
    Ok(())
}

pub fn validate_read_path(path: &str) -> Result<String, String> {
    let path = path.trim().trim_start_matches('/').to_string();

    if path.is_empty() {
        return Err("路径不能为空".into());
    }
    if path.contains("..") {
        return Err("路径不能包含 ..".into());
    }
    if !path.ends_with(".md") {
        return Err("路径必须以 .md 结尾".into());
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── validate_directory ──────────────────────────────────────

    #[test]
    fn test_valid_directories() {
        for &dir in VALID_DIRS {
            let result = validate_directory(dir);
            assert!(result.is_ok(), "should accept: {dir}, got err: {result:?}");
        }
    }

    #[test]
    fn test_invalid_directory() {
        assert!(validate_directory("nonexistent").is_err());
        assert!(validate_directory("").is_err());
        assert!(validate_directory("  ").is_err());
    }

    #[test]
    fn test_directory_whitespace() {
        assert!(validate_directory("  tech  ").is_ok());
    }

    #[test]
    fn test_directory_trailing_slash() {
        assert!(validate_directory("/tech/").is_ok());
        assert!(validate_directory("tech/").is_ok());
        assert!(validate_directory("/tech").is_ok());
    }

    #[test]
    fn test_directory_subdir_valid() {
        assert!(validate_directory("tech/docker").is_ok());
        assert!(validate_directory("ai/mcp/development").is_ok());
        assert!(validate_directory("projects/easytier").is_ok());
        assert!(validate_directory("journal/2026-03").is_ok());
    }

    #[test]
    fn test_directory_subdir_invalid_top() {
        assert!(validate_directory("foo/bar").is_err());
        assert!(validate_directory("invalid/sub").is_err());
    }

    #[test]
    fn test_directory_traversal() {
        assert!(validate_directory("tech/../ai").is_err());
        assert!(validate_directory("../etc").is_err());
        assert!(validate_directory("tech/../../windows").is_err());
    }

    #[test]
    fn test_directory_dot_segment() {
        assert!(validate_directory("tech/./nginx").is_err());
        assert!(validate_directory("./tech").is_err());
    }

    #[test]
    fn test_directory_empty_segment() {
        assert!(validate_directory("tech//nginx").is_err());
        assert!(validate_directory("//tech").is_ok()); // leading slashes trimmed to "tech"
        assert!(validate_directory("tech/").is_ok()); // trailing slash trimmed
    }

    #[test]
    fn test_directory_depth_limit() {
        assert!(validate_directory("a/b/c/d").is_err()); // 4 segments, invalid top anyway
        assert!(validate_directory("tech/a/b/c").is_err()); // 4 segments > 3
        assert!(validate_directory("tech/a/b").is_ok()); // 3 segments = ok
    }

    #[test]
    fn test_directory_special_chars() {
        assert!(validate_directory("tech/Docker Guide").is_err());
        assert!(validate_directory("tech/docker!guide").is_err());
        assert!(validate_directory("tech/docker@nginx").is_err());
        assert!(validate_directory("tech/中文").is_err());
    }

    #[test]
    fn test_directory_backslash() {
        let result = validate_directory("tech\\sub");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "tech/sub");
    }

    #[test]
    fn test_directory_validated_output() {
        assert_eq!(validate_directory("  tech  "), Ok("tech".into()));
        assert_eq!(
            validate_directory("/tech/docker/"),
            Ok("tech/docker".into())
        );
    }

    // ── validate_filename ──────────────────────────────────────

    #[test]
    fn test_valid_filenames() {
        assert_eq!(validate_filename("docker-guide"), Ok("docker-guide".into()));
        assert_eq!(
            validate_filename("rust-mcp-development"),
            Ok("rust-mcp-development".into())
        );
        assert_eq!(validate_filename("a"), Ok("a".into()));
        assert_eq!(validate_filename("abc123"), Ok("abc123".into()));
    }

    #[test]
    fn test_filename_empty() {
        assert!(validate_filename("").is_err());
        assert!(validate_filename("   ").is_err());
    }

    #[test]
    fn test_filename_uppercase() {
        assert!(validate_filename("Docker-Guide").is_err());
        assert!(validate_filename("docker-Guide").is_err());
    }

    #[test]
    fn test_filename_spaces() {
        assert!(validate_filename("docker guide").is_err());
    }

    #[test]
    fn test_filename_non_ascii() {
        assert!(validate_filename("docker指南").is_err());
    }

    #[test]
    fn test_filename_strip_md() {
        assert_eq!(
            validate_filename("docker-guide.md"),
            Ok("docker-guide".into())
        );
    }

    #[test]
    fn test_filename_regex_whitelist() {
        assert!(validate_filename("docker!guide").is_err());
        assert!(validate_filename("docker@guide").is_err());
        assert!(validate_filename("docker.guide").is_err());
        assert!(validate_filename("docker_guide").is_err());
    }

    #[test]
    fn test_filename_strip_suffix_not_char_set() {
        // Bug fix: "dom" should stay "dom", not become "do"
        assert_eq!(validate_filename("dom"), Ok("dom".into()));
        // "cmd" should stay "cmd"
        assert_eq!(validate_filename("cmd"), Ok("cmd".into()));
        // "summary.md" should strip to "summary"
        assert_eq!(validate_filename("summary.md"), Ok("summary".into()));
    }

    #[test]
    fn test_filename_leading_trailing_hyphen() {
        assert!(validate_filename("-docker").is_err());
        assert!(validate_filename("docker-").is_err());
    }

    // ── validate_status ────────────────────────────────────────

    #[test]
    fn test_valid_statuses() {
        for &status in VALID_STATUSES {
            assert!(validate_status(status).is_ok(), "should accept: {status}");
        }
    }

    #[test]
    fn test_invalid_status() {
        assert!(validate_status("pending").is_err());
        assert!(validate_status("").is_err());
        assert!(validate_status("Active").is_err());
        assert!(validate_status("ACTIVE").is_err());
    }

    // ── validate_read_path ─────────────────────────────────────

    #[test]
    fn test_valid_paths() {
        assert!(validate_read_path("tech/docker-guide.md").is_ok());
        assert!(validate_read_path("ai/mcp/development.md").is_ok());
    }

    #[test]
    fn test_path_leading_slash() {
        assert!(validate_read_path("/tech/docker-guide.md").is_ok());
    }

    #[test]
    fn test_path_traversal() {
        assert!(validate_read_path("../secrets.md").is_err());
        assert!(validate_read_path("tech/../../secrets.md").is_err());
    }

    #[test]
    fn test_path_empty() {
        assert!(validate_read_path("").is_err());
        assert!(validate_read_path("  ").is_err());
    }

    #[test]
    fn test_read_path_output() {
        assert_eq!(
            validate_read_path("/tech/docker.md"),
            Ok("tech/docker.md".into())
        );
    }
}
