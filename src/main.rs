mod config;
mod file_tree;
mod frontmatter;
mod index;
mod server;
mod types;
mod validation;

use anyhow::Result;
use rmcp::transport::io::stdio;
use rmcp::ServiceExt;
use server::ObsidianMcp;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!(
        "Obsidian MCP Server starting, vault: {}",
        config::get_vault_root()
    );
    let service = ObsidianMcp::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
