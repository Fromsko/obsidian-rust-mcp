//! Path and naming validation against configurable allowlists.

use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct Validator {
    valid_dirs: Vec<String>,
    valid_statuses: Vec<String>,
}

impl Validator {
    pub fn from_config(config: &AppConfig) -> Self {
        Self {
            valid_dirs: config.valid_dirs.clone(),
            valid_statuses: config.valid_statuses.clone(),
        }
    }

    pub fn valid_dirs(&self) -> &[String] {
        &self.valid_dirs
    }

    pub fn validate_directory(&self, dir: &str) -> Result<String, String> {
        let dir = dir.trim().replace('\\', "/");
        let dir = dir.trim_matches('/').to_string();

        if dir.is_empty() {
            return Err("目录不能为空".into());
        }
        if dir.contains("..") {
            return Err("目录路径不能包含 ..".into());
        }

        let segments: Vec<&str> = dir.split('/').collect();
        if !self.valid_dirs.iter().any(|d| d == segments[0]) {
            return Err(format!(
                "无效的顶级目录 '{}'，必须是：{}",
                segments[0],
                self.valid_dirs.join(", ")
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

    pub fn validate_filename(&self, filename: &str) -> Result<String, String> {
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

    pub fn validate_status(&self, status: &str) -> Result<(), String> {
        if !self.valid_statuses.iter().any(|s| s == status) {
            return Err(format!(
                "无效的状态 '{}'，必须是：{}",
                status,
                self.valid_statuses.join(", ")
            ));
        }
        Ok(())
    }

    pub fn validate_read_path(&self, path: &str) -> Result<String, String> {
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
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{AppConfig, DEFAULT_VALID_DIRS, DEFAULT_VALID_STATUSES};

    fn default_validator() -> Validator {
        Validator::from_config(&AppConfig::default())
    }

    #[test]
    fn test_valid_directories() {
        let v = default_validator();
        for &dir in DEFAULT_VALID_DIRS {
            assert!(v.validate_directory(dir).is_ok(), "should accept: {dir}");
        }
    }

    #[test]
    fn test_custom_valid_dirs() {
        let cfg = AppConfig::for_test(
            std::path::PathBuf::from("."),
            Some(vec!["custom".into()]),
        );
        let v = Validator::from_config(&cfg);
        assert!(v.validate_directory("custom/sub").is_ok());
        assert!(v.validate_directory("tech/sub").is_err());
    }

    #[test]
    fn test_invalid_directory() {
        let v = default_validator();
        assert!(v.validate_directory("nonexistent").is_err());
        assert!(v.validate_directory("").is_err());
    }

    #[test]
    fn test_directory_subdir_valid() {
        let v = default_validator();
        assert!(v.validate_directory("tech/docker").is_ok());
        assert!(v.validate_directory("ai/mcp/development").is_ok());
    }

    #[test]
    fn test_directory_deep_nesting() {
        let v = default_validator();
        assert!(v.validate_directory("projects/easytier/docs/api").is_ok());
        assert!(v.validate_directory("ai/mcp/protocol/rmcp/guides").is_ok());
        assert!(v.validate_directory("tech/a/b/c/d/e").is_ok());
    }

    #[test]
    fn test_directory_traversal() {
        let v = default_validator();
        assert!(v.validate_directory("tech/../ai").is_err());
    }

    #[test]
    fn test_valid_filenames() {
        let v = default_validator();
        assert_eq!(
            v.validate_filename("docker-guide"),
            Ok("docker-guide".into())
        );
    }

    #[test]
    fn test_valid_statuses() {
        let v = default_validator();
        for &status in DEFAULT_VALID_STATUSES {
            assert!(v.validate_status(status).is_ok());
        }
    }

    #[test]
    fn test_valid_paths() {
        let v = default_validator();
        assert!(v.validate_read_path("tech/docker-guide.md").is_ok());
    }
}
