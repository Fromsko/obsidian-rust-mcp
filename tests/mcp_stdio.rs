//! MCP stdio integration: only `help` and `executeCommand` are exposed.

use anyhow::{Context, Result};
use rmcp::{
    model::CallToolRequestParams,
    transport::{ConfigureCommandExt, TokioChildProcess},
    ServiceExt,
};
use serde_json::json;
use std::path::PathBuf;
use tempfile::TempDir;

fn binary_name() -> &'static str {
    #[cfg(windows)]
    {
        "obsidian-mcp.exe"
    }
    #[cfg(not(windows))]
    {
        "obsidian-mcp"
    }
}

fn server_binary() -> Result<PathBuf> {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_obsidian-mcp") {
        return Ok(PathBuf::from(p));
    }
    let exe = std::env::current_exe().context("current_exe")?;
    let debug = exe.parent().context("parent")?;
    let candidate = debug.join(binary_name());
    if candidate.exists() {
        return Ok(candidate);
    }
    let fallback = PathBuf::from("target")
        .join("debug")
        .join(binary_name());
    anyhow::ensure!(fallback.exists(), "obsidian-mcp binary not found");
    Ok(fallback)
}

async fn connect_with_vault(
    vault: &TempDir,
) -> Result<rmcp::service::RunningService<rmcp::service::RoleClient, ()>> {
    let bin = server_binary()?;
    let vault_path = vault.path().to_string_lossy().to_string();
    let client = ()
        .serve(TokioChildProcess::new(
            tokio::process::Command::new(&bin).configure(move |cmd| {
                cmd.env("OBSIDIAN_VAULT_ROOT", &vault_path);
            }),
        )?)
        .await
        .context("connect mcp")?;
    Ok(client)
}

#[tokio::test]
async fn lists_exactly_two_tools() -> Result<()> {
    let vault = TempDir::new()?;
    let client = connect_with_vault(&vault).await?;
    let tools = client.list_tools(Default::default()).await?.tools;
    let names: Vec<_> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert_eq!(names.len(), 2, "tools: {names:?}");
    assert!(names.contains(&"help"));
    assert!(names.contains(&"executeCommand"));
    client.cancel().await.ok();
    Ok(())
}

#[tokio::test]
async fn help_and_execute_search() -> Result<()> {
    let vault = TempDir::new()?;
    std::fs::create_dir_all(vault.path().join("tech"))?;
    std::fs::write(
        vault.path().join("tech/note.md"),
        "---\ntags:\n  - pilot\n---\n# Note\n",
    )?;

    let client = connect_with_vault(&vault).await?;

    let help_result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "help".into(),
            arguments: json!({ "detail": false }).as_object().cloned(),
            task: None,
        })
        .await?;
    let help_text = help_result
        .content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.as_str())
        .unwrap_or("");
    assert!(help_text.contains("obsidian.search"));

    let search_result = client
        .call_tool(CallToolRequestParams {
            meta: None,
            name: "executeCommand".into(),
            arguments: json!({
                "command": "obsidian.search",
                "args": { "tags": ["pilot"] }
            })
            .as_object()
            .cloned(),
            task: None,
        })
        .await?;
    let search_text = search_result
        .content
        .first()
        .and_then(|c| c.as_text())
        .map(|t| t.text.as_str())
        .unwrap_or("");
    assert!(search_text.contains("note.md"));

    client.cancel().await.ok();
    Ok(())
}
