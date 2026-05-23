//! Obsidian MCP library — CLI-style `help` + `executeCommand` over `obsidian.*` commands.

pub mod command;
pub mod config;
pub mod mcp;
pub mod note;
pub mod service;
pub mod types;
pub mod validation;
pub mod vault;

pub use mcp::ObsidianMcp;
pub use service::ObsidianService;
