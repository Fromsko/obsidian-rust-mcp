use crate::command::registry::{find_command, suggest_command};
use crate::service::{ObsidianService, ServiceError};
use crate::types::{PathParams, SearchParams, SemanticSearchParams, WriteNoteParams};

#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("{0}")]
    UnknownCommand(String),
    #[error("{0}")]
    InvalidArgs(String),
    #[error(transparent)]
    Service(#[from] ServiceError),
}

impl DispatchError {
    fn invalid_args(msg: impl Into<String>) -> Self {
        Self::InvalidArgs(msg.into())
    }
}

pub async fn dispatch(
    service: &ObsidianService,
    command: &str,
    args: serde_json::Value,
) -> Result<String, DispatchError> {
    let command = command.trim();
    if find_command(command).is_none() {
        let mut msg = format!("未知命令: `{command}`");
        if let Some(s) = suggest_command(command) {
            msg.push_str(&format!("\n是否指: `{s}` ?"));
        }
        msg.push_str("\n执行 help 查看可用命令。");
        return Err(DispatchError::UnknownCommand(msg));
    }

    match command {
        "obsidian.guide" => {
            ensure_empty_args(command, &args)?;
            service.guide().map_err(DispatchError::from)
        }
        "obsidian.index" => {
            ensure_empty_args(command, &args)?;
            service.index().map_err(DispatchError::from)
        }
        "obsidian.search" => {
            let p: SearchParams = parse_args(command, args)?;
            service.search(p).map_err(DispatchError::from)
        }
        "obsidian.read" => {
            let p: PathParams = parse_args(command, args)?;
            service.read(&p.path).await.map_err(DispatchError::from)
        }
        "obsidian.write" => {
            let p: WriteNoteParams = parse_args(command, args)?;
            service.write(p).await.map_err(DispatchError::from)
        }
        "obsidian.delete" => {
            let p: PathParams = parse_args(command, args)?;
            service.delete(&p.path).await.map_err(DispatchError::from)
        }
        "obsidian.semantic_search" => {
            let p: SemanticSearchParams = parse_args(command, args)?;
            service.semantic_search(p).map_err(DispatchError::from)
        }
        _ => Err(DispatchError::UnknownCommand(format!("未实现: {command}"))),
    }
}

fn ensure_empty_args(command: &str, args: &serde_json::Value) -> Result<(), DispatchError> {
    let empty = args.is_null() || args.as_object().is_some_and(|m| m.is_empty());
    if empty {
        return Ok(());
    }
    Err(DispatchError::invalid_args(format!(
        "{command} 不接受参数，请传 args: {{}}"
    )))
}

fn parse_args<T: serde::de::DeserializeOwned>(
    command: &str,
    args: serde_json::Value,
) -> Result<T, DispatchError> {
    let args = if args.is_null() {
        serde_json::json!({})
    } else if let Some(s) = args.as_str() {
        // 兼容 Mocode 等客户端：args 可能是 JSON 字符串而非对象
        serde_json::from_str(s).unwrap_or(serde_json::json!({}))
    } else {
        args
    };
    serde_json::from_value(args).map_err(|e| {
        DispatchError::invalid_args(format!(
            "{command} 参数无效: {e}\n执行 help topic={command} detail=true 查看说明。"
        ))
    })
}

impl From<DispatchError> for String {
    fn from(e: DispatchError) -> Self {
        e.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vault::VaultHandle;
    use std::fs;
    use tempfile::tempdir;

    fn test_service() -> ObsidianService {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("tech")).unwrap();
        fs::write(
            dir.path().join("tech/sample.md"),
            "---\ntags:\n  - demo\n---\n# Sample\n",
        )
        .unwrap();
        let vault = VaultHandle::from_path(dir.path().to_path_buf());
        ObsidianService::with_vault(vault)
    }

    #[tokio::test]
    async fn dispatch_guide() {
        let svc = test_service();
        let out = dispatch(&svc, "obsidian.guide", serde_json::json!({}))
            .await
            .unwrap();
        assert!(!out.is_empty());
    }

    #[tokio::test]
    async fn dispatch_unknown() {
        let svc = test_service();
        let err = dispatch(&svc, "obsidian.nope", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, DispatchError::UnknownCommand(_)));
    }

    #[tokio::test]
    async fn dispatch_search_requires_filter() {
        let svc = test_service();
        let err = dispatch(&svc, "obsidian.search", serde_json::json!({}))
            .await
            .unwrap_err();
        assert!(matches!(err, DispatchError::Service(_)));
    }
}
