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

    tracing::info!(
        "Obsidian MCP v{} starting, vault: {}",
        env!("CARGO_PKG_VERSION"),
        obsidian_mcp::config::get_vault_root()
    );

    let service = ObsidianMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
