//! # Tools Module
//!
//! Provides tool management and built-in tool implementations.
//! 
//! Tools are functions that can be invoked via JSON-RPC protocol.
//! New tools should be added to the `builtin_tools` module and
//! registered in the `ToolRegistry`.

pub mod tool_handler;
pub mod builtin_tools;

pub use tool_handler::ToolRegistry;


