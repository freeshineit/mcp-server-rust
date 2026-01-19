//! # MCP Server Rust
//!
//! A Model Context Protocol (MCP) server implementation in Rust.
//!
//! This library provides a complete implementation of the Model Context Protocol,
//! allowing applications to expose tools and resources through a standardized interface.
//!
//! ## Features
//!
//! - **Tool Support**: Register and execute tools with JSON-RPC 2.0 interface
//! - **Resource Management**: Expose resources with standardized access patterns
//! - **Async Runtime**: Built on tokio for high-performance async operations
//! - **Type-Safe**: Leverages Rust's type system for safety and correctness
//!
//! ## Module Structure
//!
//! - [`models`]: Core data structures for MCP protocol
//! - [`server`]: TCP server implementation and message routing
//! - [`tools`]: Tool registry and implementations
//! - [`resources`]: Resource management and access

pub mod models;
pub mod server;
pub mod tools;
pub mod resources;

// Re-export commonly used types
pub use models::*;
pub use server::McpServer;
pub use tools::ToolRegistry;
pub use resources::ResourceRegistry;
