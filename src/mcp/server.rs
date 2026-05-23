use std::sync::Arc;

use rmcp::{
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    tool, tool_handler, tool_router,
    ErrorData as McpError, ServerHandler,
};

use crate::command::{dispatch, render_help, DispatchError};
use crate::service::{ObsidianService, ServiceError};
use crate::types::{ExecuteCommandParams, HelpParams};

#[derive(Clone)]
pub struct ObsidianMcp {
    service: Arc<ObsidianService>,
    tool_router: ToolRouter<Self>,
}

impl ObsidianMcp {
    pub fn new() -> Self {
        Self {
            service: Arc::new(ObsidianService::new()),
            tool_router: Self::tool_router(),
        }
    }

    #[cfg(test)]
    pub fn with_service(service: ObsidianService) -> Self {
        Self {
            service: Arc::new(service),
            tool_router: Self::tool_router(),
        }
    }

    fn map_dispatch_err(e: DispatchError) -> McpError {
        match &e {
            DispatchError::UnknownCommand(_) | DispatchError::InvalidArgs(_) => {
                McpError::invalid_params(e.to_string(), None)
            }
            DispatchError::Service(se) => Self::map_service_err(se),
        }
    }

    fn map_service_err(e: &ServiceError) -> McpError {
        match e {
            ServiceError::InvalidParams(msg) => McpError::invalid_params(msg.clone(), None),
            ServiceError::Internal(msg) => McpError::internal_error(msg.clone(), None),
        }
    }
}

#[tool_router]
impl ObsidianMcp {
    #[tool(
        name = "help",
        description = "CLI-style manual for Obsidian MCP. Lists obsidian.* commands; use topic + detail for full usage. No vault I/O."
    )]
    async fn help(
        &self,
        Parameters(params): Parameters<HelpParams>,
    ) -> Result<CallToolResult, McpError> {
        let text = render_help(&params);
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(
        name = "executeCommand",
        description = "Run a registered command. Use help first. Example: command=obsidian.search, args={\"tags\":[\"docker\"]}"
    )]
    async fn execute_command(
        &self,
        Parameters(params): Parameters<ExecuteCommandParams>,
    ) -> Result<CallToolResult, McpError> {
        let command = params.command.trim();
        if command.is_empty() {
            return Err(McpError::invalid_params(
                "command 不能为空。先执行 help。",
                None,
            ));
        }

        let text = dispatch(self.service.as_ref(), command, params.args)
            .await
            .map_err(Self::map_dispatch_err)?;

        Ok(CallToolResult::success(vec![Content::text(text)]))
    }
}

#[tool_handler]
impl ServerHandler for ObsidianMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Obsidian vault MCP (CLI model). \
                 1) help — list obsidian.* commands. \
                 2) executeCommand — run e.g. obsidian.guide, obsidian.search, obsidian.write. \
                 Typical flow: guide → search → read? → write."
                    .into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}
