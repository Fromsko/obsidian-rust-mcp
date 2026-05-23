mod local;

use std::path::{Path, PathBuf};

pub use local::LocalVault;

/// Storage abstraction for future cloud/DB backends.
pub trait VaultBackend: Send + Sync {
    fn root(&self) -> &Path;
}

/// Shared vault handle used by [`crate::service::ObsidianService`].
#[derive(Clone)]
pub struct VaultHandle {
    inner: LocalVault,
}

impl VaultHandle {
    pub fn from_env() -> Self {
        Self {
            inner: LocalVault::from_env(),
        }
    }

    /// Construct a handle for a specific vault directory (tests, custom roots).
    pub fn from_path(root: PathBuf) -> Self {
        Self {
            inner: LocalVault::new(root),
        }
    }

    pub fn local(&self) -> &LocalVault {
        &self.inner
    }

    pub fn join(&self, rel: &str) -> PathBuf {
        self.inner.join(rel)
    }
}

impl VaultBackend for VaultHandle {
    fn root(&self) -> &Path {
        self.inner.root()
    }
}
