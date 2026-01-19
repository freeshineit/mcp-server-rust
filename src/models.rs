//! # Data Models
//!
//! This module defines all data structures used by the MCP Server.
//! It includes tools, resources, and JSON-RPC message definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a callable tool with schema and description
///
/// A tool is a function that can be invoked via JSON-RPC protocol.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    /// Tool name/identifier
    pub name: String,
    /// Human-readable description of the tool
    pub description: String,
    /// Input parameter schema for validation
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
}

/// Defines the input schema for a tool using JSON Schema
///
/// Specifies what parameters a tool accepts and their types.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolInputSchema {
    /// Schema type, typically "object"
    #[serde(rename = "type")]
    pub type_: String,
    /// Map of parameter names to their property definitions
    pub properties: HashMap<String, Property>,
    /// List of required parameter names
    pub required: Vec<String>,
}

/// Describes a single property/parameter
///
/// Used within ToolInputSchema to define individual parameters.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    /// Data type of the property (e.g., "string", "number")
    #[serde(rename = "type")]
    pub type_: String,
    /// Description of what this property represents
    pub description: String,
}

/// Represents content in response to a tool call
///
/// Contains the actual result data from a tool execution.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Content {
    /// Content type (typically "text")
    #[serde(rename = "type")]
    pub type_: String,
    /// The actual content/result text
    pub text: String,
}

/// Represents a resource that can be read
///
/// Resources are files or data sources accessible via the server.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Resource {
    /// Unique resource identifier (e.g., "file:///path/to/file")
    pub uri: String,
    /// MIME type of the resource content
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

/// A JSON-RPC 2.0 protocol message
///
/// Represents incoming RPC requests from clients.
#[derive(Debug, Serialize, Deserialize)]
pub struct McpMessage {
    /// JSON-RPC protocol version (should be "2.0")
    pub jsonrpc: String,
    /// The method/procedure name to call
    pub method: String,
    /// Parameters for the method call
    #[serde(default)]
    pub params: serde_json::Value,
    /// Request identifier for matching responses
    pub id: Option<u64>,
}

/// Request to call a tool
///
/// Sent as parameters to a `tools/call` RPC method.
#[derive(Debug, Serialize, Deserialize)]
pub struct CallToolRequest {
    /// Name of the tool to invoke
    pub name: String,
    /// Arguments to pass to the tool
    pub arguments: serde_json::Value,
}

/// Result returned from a tool call
///
/// Contains the output of tool execution.
#[derive(Debug, Serialize, Deserialize)]
pub struct CallToolResult {
    /// Array of content items returned by the tool
    pub content: Vec<Content>,
}

/// Response for `tools/list` RPC method
///
/// Lists all available tools on the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListToolsResult {
    /// Vector of available tools
    pub tools: Vec<Tool>,
}

/// Request to read a resource
///
/// Sent as parameters to a `resources/read` RPC method.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResourceRequest {
    /// URI of the resource to read
    pub uri: String,
}

/// Response for `resources/list` RPC method
///
/// Lists all available resources on the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListResourcesResult {
    /// Vector of available resources
    pub resources: Vec<Resource>,
}
