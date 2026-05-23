use std::path::{Path, PathBuf};

use crate::config::get_vault_root;

/// Local filesystem vault (walkdir + std/tokio fs).
#[derive(Clone)]
pub struct LocalVault {
    root: PathBuf,
}

impl LocalVault {
    pub fn from_env() -> Self {
        Self::new(PathBuf::from(get_vault_root()))
    }

    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn join(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }
}

impl super::VaultBackend for LocalVault {
    fn root(&self) -> &Path {
        &self.root
    }
}
