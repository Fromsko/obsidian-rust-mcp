//! Application configuration loaded from env and optional JSON file.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

pub const DEFAULT_VAULT_ROOT: &str = ".";

pub const DEFAULT_VALID_DIRS: &[&str] = &[
    "tech",
    "ai",
    "projects",
    "methods",
    "career",
    "ideas",
    "cheatsheet",
    "journal",
];

pub const DEFAULT_VALID_STATUSES: &[&str] = &["active", "archived", "draft"];

pub const WRITE_NOTE_TIPS: &str = include_str!("../../docs/write-note-tips.md");

const CONFIG_FILENAME: &str = "obsidian-mcp.json";

/// On-disk config (partial — env overrides file).
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct ConfigFile {
    vault_root: Option<String>,
    valid_dirs: Option<Vec<String>>,
    valid_statuses: Option<Vec<String>>,
    backend: Option<String>,
    cloud: Option<CloudFileSection>,
}

#[derive(Debug, Deserialize)]
struct CloudFileSection {
    url: String,
    token: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultBackendKind {
    Local,
    Cloud,
}

#[derive(Debug, Clone)]
pub struct CloudConfig {
    pub base_url: String,
    pub token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub vault_root: PathBuf,
    pub valid_dirs: Vec<String>,
    pub valid_statuses: Vec<String>,
    pub backend: VaultBackendKind,
    pub cloud: Option<CloudConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            vault_root: PathBuf::from(DEFAULT_VAULT_ROOT),
            valid_dirs: DEFAULT_VALID_DIRS.iter().map(|s| s.to_string()).collect(),
            valid_statuses: DEFAULT_VALID_STATUSES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            backend: VaultBackendKind::Local,
            cloud: None,
        }
    }
}

impl AppConfig {
    /// Load config: optional JSON file, then env overrides.
    pub fn load() -> Result<Self, String> {
        let mut cfg = Self::default();

        if let Ok(root) = env::var("OBSIDIAN_VAULT_ROOT") {
            if !root.trim().is_empty() {
                cfg.vault_root = PathBuf::from(root.trim());
            }
        }

        if let Some(file) = resolve_config_path(&cfg.vault_root) {
            let raw = fs::read_to_string(&file)
                .map_err(|e| format!("读取配置文件 {} 失败: {e}", file.display()))?;
            let parsed: ConfigFile = serde_json::from_str(&raw)
                .map_err(|e| format!("解析配置文件 {} 失败: {e}", file.display()))?;
            cfg.apply_file(parsed);
        }

        cfg.apply_env();
        Ok(cfg)
    }

    /// Test helper with optional custom valid dirs.
    pub fn for_test(vault_root: PathBuf, valid_dirs: Option<Vec<String>>) -> Self {
        let mut cfg = Self {
            vault_root,
            ..Self::default()
        };
        if let Some(dirs) = valid_dirs {
            cfg.valid_dirs = dirs;
        }
        cfg
    }

    fn apply_file(&mut self, file: ConfigFile) {
        if let Some(root) = file.vault_root {
            self.vault_root = PathBuf::from(root);
        }
        if let Some(dirs) = file.valid_dirs {
            if !dirs.is_empty() {
                self.valid_dirs = dirs;
            }
        }
        if let Some(statuses) = file.valid_statuses {
            if !statuses.is_empty() {
                self.valid_statuses = statuses;
            }
        }
        if let Some(backend) = file.backend {
            self.backend = parse_backend_kind(&backend);
        }
        if let Some(cloud) = file.cloud {
            self.cloud = Some(CloudConfig {
                base_url: cloud.url,
                token: cloud.token,
            });
            if self.backend == VaultBackendKind::Local {
                self.backend = VaultBackendKind::Cloud;
            }
        }
    }

    fn apply_env(&mut self) {
        if let Ok(root) = env::var("OBSIDIAN_VAULT_ROOT") {
            if !root.trim().is_empty() {
                self.vault_root = PathBuf::from(root);
            }
        }

        if let Ok(dirs) = env::var("OBSIDIAN_VALID_DIRS") {
            let parsed: Vec<String> = dirs
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !parsed.is_empty() {
                self.valid_dirs = parsed;
            }
        }

        if let Ok(statuses) = env::var("OBSIDIAN_VALID_STATUSES") {
            let parsed: Vec<String> = statuses
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !parsed.is_empty() {
                self.valid_statuses = parsed;
            }
        }

        if let Ok(backend) = env::var("OBSIDIAN_VAULT_BACKEND") {
            self.backend = parse_backend_kind(&backend);
        }

        if let Ok(url) = env::var("OBSIDIAN_CLOUD_URL") {
            if !url.trim().is_empty() {
                let token = env::var("OBSIDIAN_CLOUD_TOKEN")
                    .ok()
                    .filter(|t| !t.trim().is_empty());
                self.cloud = Some(CloudConfig {
                    base_url: url,
                    token,
                });
                if self.backend == VaultBackendKind::Local {
                    self.backend = VaultBackendKind::Cloud;
                }
            }
        }
    }
}

fn resolve_config_path(vault_root: &Path) -> Option<PathBuf> {
    if let Ok(path) = env::var("OBSIDIAN_CONFIG") {
        let p = PathBuf::from(path.trim());
        if p.exists() {
            return Some(p);
        }
    }
    let in_vault = vault_root.join(CONFIG_FILENAME);
    if in_vault.exists() {
        return Some(in_vault);
    }
    None
}

fn parse_backend_kind(s: &str) -> VaultBackendKind {
    match s.trim().to_lowercase().as_str() {
        "cloud" | "remote" => VaultBackendKind::Cloud,
        _ => VaultBackendKind::Local,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn default_valid_dirs() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.valid_dirs.len(), DEFAULT_VALID_DIRS.len());
        assert!(cfg.valid_dirs.contains(&"tech".to_string()));
    }

    #[test]
    fn load_from_config_file() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("obsidian-mcp.json");
        fs::write(
            &config_path,
            r#"{"valid_dirs":["custom","docs"],"backend":"local"}"#,
        )
        .unwrap();

        env::set_var("OBSIDIAN_CONFIG", config_path.to_string_lossy().as_ref());
        env::remove_var("OBSIDIAN_VALID_DIRS");

        let cfg = AppConfig::load().unwrap();
        assert_eq!(cfg.valid_dirs, vec!["custom", "docs"]);

        env::remove_var("OBSIDIAN_CONFIG");
    }

    #[test]
    fn env_overrides_valid_dirs() {
        env::set_var("OBSIDIAN_VALID_DIRS", "env-a, env-b");
        let cfg = AppConfig::load().unwrap();
        assert_eq!(cfg.valid_dirs, vec!["env-a", "env-b"]);
        env::remove_var("OBSIDIAN_VALID_DIRS");
    }
}
