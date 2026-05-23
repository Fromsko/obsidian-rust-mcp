//! End-to-end command flow without MCP transport.

use obsidian_mcp::command::dispatch;
use obsidian_mcp::service::ObsidianService;
use obsidian_mcp::store::VaultHandle;
use serde_json::json;
use std::fs;
use tempfile::TempDir;

fn setup_vault() -> (TempDir, ObsidianService) {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("tech")).unwrap();
    fs::write(
        dir.path().join("tech/existing.md"),
        "---\ntags:\n  - rust\n  - mcp\naliases:\n  - Existing\nstatus: active\n---\n# Existing\n",
    )
    .unwrap();
    let vault = VaultHandle::from_path(dir.path().to_path_buf());
    (dir, ObsidianService::with_vault(vault))
}

#[tokio::test]
async fn guide_returns_content() {
    let (_dir, svc) = setup_vault();
    let out = dispatch(&svc, "obsidian.guide", json!({}))
        .await
        .unwrap();
    assert!(out.len() > 100);
}

#[tokio::test]
async fn search_by_tag() {
    let (_dir, svc) = setup_vault();
    let out = dispatch(
        &svc,
        "obsidian.search",
        json!({ "tags": ["rust"] }),
    )
    .await
    .unwrap();
    assert!(out.contains("existing"));
    assert!(out.contains("tech/existing.md"));
}

#[tokio::test]
async fn search_with_include_index() {
    let (_dir, svc) = setup_vault();
    let out = dispatch(
        &svc,
        "obsidian.search",
        json!({ "tags": ["rust"], "include_index": true }),
    )
    .await
    .unwrap();
    assert!(out.contains("文件树"));
    assert!(out.contains("existing"));
}

#[tokio::test]
async fn write_create_and_read() {
    let (_dir, svc) = setup_vault();
    let write_out = dispatch(
        &svc,
        "obsidian.write",
        json!({
            "directory": "tech",
            "filename": "new-note",
            "tags": ["demo"],
            "aliases": [],
            "status": "active",
            "content": "# New\n\nBody.",
            "append": false
        }),
    )
    .await
    .unwrap();
    assert!(write_out.contains("已创建"));

    let body = dispatch(
        &svc,
        "obsidian.read",
        json!({ "path": "tech/new-note.md" }),
    )
    .await
    .unwrap();
    assert!(body.contains("# New"));
    assert!(body.contains("tags:"));
}

#[tokio::test]
async fn write_append() {
    let (_dir, svc) = setup_vault();
    dispatch(
        &svc,
        "obsidian.write",
        json!({
            "directory": "tech",
            "filename": "append-test",
            "tags": ["t"],
            "aliases": [],
            "status": "draft",
            "content": "Part A",
            "append": false
        }),
    )
    .await
    .unwrap();

    dispatch(
        &svc,
        "obsidian.write",
        json!({
            "directory": "tech",
            "filename": "append-test",
            "tags": ["t"],
            "aliases": [],
            "status": "draft",
            "content": "Part B",
            "append": true
        }),
    )
    .await
    .unwrap();

    let body = dispatch(
        &svc,
        "obsidian.read",
        json!({ "path": "tech/append-test.md" }),
    )
    .await
    .unwrap();
    assert!(body.contains("Part A"));
    assert!(body.contains("Part B"));
}

#[tokio::test]
async fn index_lists_vault() {
    let (_dir, svc) = setup_vault();
    let out = dispatch(&svc, "obsidian.index", json!({}))
        .await
        .unwrap();
    assert!(out.contains("文件树"));
    assert!(out.contains("rust"));
}

#[tokio::test]
async fn delete_removes_file() {
    let (dir, svc) = setup_vault();
    let path = dir.path().join("tech/to-delete.md");
    fs::write(&path, "---\ntags:\n  - x\n---\n").unwrap();

    let out = dispatch(
        &svc,
        "obsidian.delete",
        json!({ "path": "tech/to-delete.md" }),
    )
    .await
    .unwrap();
    assert!(out.contains("已删除"));
    assert!(!path.exists());
}

#[tokio::test]
async fn read_rejects_non_md() {
    let (_dir, svc) = setup_vault();
    let err = dispatch(
        &svc,
        "obsidian.read",
        json!({ "path": "tech/existing" }),
    )
    .await
    .unwrap_err();
    assert!(err.to_string().contains(".md"));
}
