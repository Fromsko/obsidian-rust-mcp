use std::path::{Path, PathBuf};

use async_trait::async_trait;

use super::backend::{VaultBackend, VaultError};
use crate::config::VaultBackendKind;

/// Local filesystem vault.
#[derive(Clone)]
pub struct LocalVault {
    root: PathBuf,
}

impl LocalVault {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

#[async_trait]
impl VaultBackend for LocalVault {
    fn kind(&self) -> VaultBackendKind {
        VaultBackendKind::Local
    }

    fn root(&self) -> &Path {
        &self.root
    }

    async fn read_text(&self, rel: &str) -> Result<String, VaultError> {
        tokio::fs::read_to_string(self.join(rel))
            .await
            .map_err(|e| VaultError::Read(format!("{rel}: {e}")))
    }

    async fn write_text(&self, rel: &str, content: &str) -> Result<(), VaultError> {
        tokio::fs::write(self.join(rel), content)
            .await
            .map_err(|e| VaultError::Write(format!("{rel}: {e}")))
    }

    async fn delete_file(&self, rel: &str) -> Result<(), VaultError> {
        tokio::fs::remove_file(self.join(rel))
            .await
            .map_err(|e| VaultError::Delete(format!("{rel}: {e}")))
    }

    async fn ensure_dir(&self, rel_dir: &str) -> Result<(), VaultError> {
        tokio::fs::create_dir_all(self.join(rel_dir))
            .await
            .map_err(|e| VaultError::Mkdir(format!("{rel_dir}: {e}")))
    }
}
