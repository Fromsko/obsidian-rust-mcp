use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::config::VaultBackendKind;

#[derive(Debug, thiserror::Error)]
pub enum VaultError {
    #[error("读取失败: {0}")]
    Read(String),
    #[error("写入失败: {0}")]
    Write(String),
    #[error("删除失败: {0}")]
    Delete(String),
    #[error("目录创建失败: {0}")]
    Mkdir(String),
    #[error("远程同步失败: {0}")]
    Remote(String),
}

/// Storage abstraction — local filesystem or cloud-backed cache.
#[async_trait]
pub trait VaultBackend: Send + Sync {
    fn kind(&self) -> VaultBackendKind;
    fn root(&self) -> &Path;
    fn join(&self, rel: &str) -> PathBuf {
        self.root().join(rel)
    }
    fn exists(&self, rel: &str) -> bool {
        self.join(rel).exists()
    }

    async fn read_text(&self, rel: &str) -> Result<String, VaultError>;
    async fn write_text(&self, rel: &str, content: &str) -> Result<(), VaultError>;
    async fn delete_file(&self, rel: &str) -> Result<(), VaultError>;
    async fn ensure_dir(&self, rel_dir: &str) -> Result<(), VaultError>;
}
