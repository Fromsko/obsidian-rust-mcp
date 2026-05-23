mod backend;
mod cloud;
mod local;

pub use backend::{VaultBackend, VaultError};
pub use cloud::CloudVault;
pub use local::LocalVault;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::config::{AppConfig, VaultBackendKind};

/// Shared vault handle used by [`crate::service::ObsidianService`].
#[derive(Clone)]
pub struct VaultHandle {
    inner: Arc<dyn VaultBackend>,
}

impl VaultHandle {
    pub fn open(config: &AppConfig) -> Result<Self, String> {
        let backend: Arc<dyn VaultBackend> = match config.backend {
            VaultBackendKind::Local => Arc::new(LocalVault::new(config.vault_root.clone())),
            VaultBackendKind::Cloud => {
                let cloud = config
                    .cloud
                    .clone()
                    .ok_or("cloud backend 需要 OBSIDIAN_CLOUD_URL 或配置文件 cloud.url")?;
                Arc::new(CloudVault::new(config.vault_root.clone(), cloud)?)
            }
        };
        Ok(Self { inner: backend })
    }

    /// Local vault at path (tests).
    pub fn from_path(root: PathBuf) -> Self {
        Self {
            inner: Arc::new(LocalVault::new(root)),
        }
    }

    pub fn backend(&self) -> &dyn VaultBackend {
        self.inner.as_ref()
    }

    pub fn root(&self) -> &Path {
        self.inner.root()
    }

    pub fn join(&self, rel: &str) -> PathBuf {
        self.inner.join(rel)
    }

    pub fn exists(&self, rel: &str) -> bool {
        self.inner.exists(rel)
    }

    pub async fn read_text(&self, rel: &str) -> Result<String, VaultError> {
        self.inner.read_text(rel).await
    }

    pub async fn write_text(&self, rel: &str, content: &str) -> Result<(), VaultError> {
        self.inner.write_text(rel, content).await
    }

    pub async fn delete_file(&self, rel: &str) -> Result<(), VaultError> {
        self.inner.delete_file(rel).await
    }

    pub async fn ensure_dir(&self, rel_dir: &str) -> Result<(), VaultError> {
        self.inner.ensure_dir(rel_dir).await
    }
}
