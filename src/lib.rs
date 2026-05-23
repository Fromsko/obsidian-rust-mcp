//! Obsidian MCP library — CLI-style `help` + `executeCommand` over `obsidian.*` commands.

pub mod command;
pub mod config;
pub mod file_tree;
pub mod frontmatter;
pub mod index;
pub mod server;
pub mod service;
pub mod store;
pub mod types;
pub mod validation;

pub use server::ObsidianMcp;
pub use service::ObsidianService;
