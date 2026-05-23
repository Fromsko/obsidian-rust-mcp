//! Command registry, help rendering, and dispatch.

pub mod dispatch;
pub mod help;
pub mod registry;

pub use dispatch::{dispatch, DispatchError};
pub use help::render_help;
pub use registry::{find_command, CommandMeta, COMMANDS};
