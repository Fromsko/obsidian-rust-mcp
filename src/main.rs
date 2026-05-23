use anyhow::Result;
use obsidian_mcp::ObsidianMcp;
use rmcp::{transport::stdio, ServiceExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let config = obsidian_mcp::config::AppConfig::load().unwrap_or_else(|e| {
        tracing::warn!("config load failed ({e}), using defaults");
        obsidian_mcp::config::AppConfig::default()
    });

    tracing::info!(
        "Obsidian MCP v{} starting, vault: {}, backend: {:?}",
        env!("CARGO_PKG_VERSION"),
        config.vault_root.display(),
        config.backend
    );

    let service = ObsidianMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
