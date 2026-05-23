//! Cloud vault: local cache + best-effort HTTP sync.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use reqwest::Client;
use tracing::warn;

use super::backend::{VaultBackend, VaultError};
use super::local::LocalVault;
use crate::config::{CloudConfig, VaultBackendKind};

/// Hybrid backend — authoritative local cache, syncs to remote on write/delete.
pub struct CloudVault {
    local: LocalVault,
    remote: CloudRemote,
}

impl CloudVault {
    pub fn new(local_root: PathBuf, config: CloudConfig) -> Result<Self, String> {
        if config.base_url.trim().is_empty() {
            return Err("OBSIDIAN_CLOUD_URL 不能为空".into());
        }
        Ok(Self {
            local: LocalVault::new(local_root),
            remote: CloudRemote::new(config)?,
        })
    }
}

#[async_trait]
impl VaultBackend for CloudVault {
    fn kind(&self) -> VaultBackendKind {
        VaultBackendKind::Cloud
    }

    fn root(&self) -> &Path {
        self.local.root()
    }

    async fn read_text(&self, rel: &str) -> Result<String, VaultError> {
        self.local.read_text(rel).await
    }

    async fn write_text(&self, rel: &str, content: &str) -> Result<(), VaultError> {
        self.local.write_text(rel, content).await?;
        if let Err(e) = self.remote.put_note(rel, content).await {
            warn!("cloud sync write failed for {rel}: {e}");
        }
        Ok(())
    }

    async fn delete_file(&self, rel: &str) -> Result<(), VaultError> {
        self.local.delete_file(rel).await?;
        if let Err(e) = self.remote.delete_note(rel).await {
            warn!("cloud sync delete failed for {rel}: {e}");
        }
        Ok(())
    }

    async fn ensure_dir(&self, rel_dir: &str) -> Result<(), VaultError> {
        self.local.ensure_dir(rel_dir).await
    }
}

struct CloudRemote {
    base_url: String,
    token: Option<String>,
    client: Client,
}

impl CloudRemote {
    fn new(config: CloudConfig) -> Result<Self, String> {
        let client = Client::builder()
            .build()
            .map_err(|e| format!("HTTP client: {e}"))?;
        let base_url = config.base_url.trim_end_matches('/').to_string();
        Ok(Self {
            base_url,
            token: config.token,
            client,
        })
    }

    fn note_url(&self, rel: &str) -> String {
        format!("{}/v1/notes/{}", self.base_url, url_encode_path(rel))
    }

    fn auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        if let Some(ref token) = self.token {
            req.bearer_auth(token)
        } else {
            req
        }
    }

    async fn put_note(&self, rel: &str, content: &str) -> Result<(), VaultError> {
        let url = self.note_url(rel);
        let req = self
            .client
            .put(&url)
            .header("content-type", "text/markdown; charset=utf-8")
            .body(content.to_string());
        let resp = self
            .auth(req)
            .send()
            .await
            .map_err(|e| VaultError::Remote(e.to_string()))?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Err(VaultError::Remote(format!(
                "PUT {} → {}",
                url,
                resp.status()
            )))
        }
    }

    async fn delete_note(&self, rel: &str) -> Result<(), VaultError> {
        let url = self.note_url(rel);
        let req = self.client.delete(&url);
        let resp = self
            .auth(req)
            .send()
            .await
            .map_err(|e| VaultError::Remote(e.to_string()))?;
        if resp.status().is_success() || resp.status().as_u16() == 404 {
            Ok(())
        } else {
            Err(VaultError::Remote(format!(
                "DELETE {} → {}",
                url,
                resp.status()
            )))
        }
    }
}

fn url_encode_path(path: &str) -> String {
    path.split('/')
        .map(|seg| urlencoding::encode(seg))
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_encode_segments() {
        assert_eq!(url_encode_path("tech/note.md"), "tech/note.md");
    }

    #[test]
    fn cloud_requires_url() {
        let err = CloudVault::new(
            PathBuf::from("/tmp"),
            CloudConfig {
                base_url: "  ".into(),
                token: None,
            },
        );
        assert!(err.is_err());
    }
}
