//! # Tool Handler Module
//!
//! Manages tool registration and execution using an enum-based approach
//! for type safety and zero-cost abstractions.

use crate::models::{Tool, ToolInputSchema, CallToolResult};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use super::builtin_tools::{SearchFilesTool, WeatherTool};

/// Enumeration of all available tool implementations
///
/// This enum provides type-safe tool dispatch without dynamic allocation.
/// Each variant holds a concrete tool implementation.
pub enum ToolImpl {
    /// File search tool for finding files in the filesystem
    SearchFiles(SearchFilesTool),
    /// Weather query tool
    Weather(WeatherTool),
}

impl ToolImpl {
    /// Gets the name of this tool
    pub fn name(&self) -> &str {
        match self {
            ToolImpl::SearchFiles(_) => "search_files",
            ToolImpl::Weather(_) => "get_weather",
        }
    }

    /// Gets the human-readable description of this tool
    pub fn description(&self) -> &str {
        match self {
            ToolImpl::SearchFiles(_) => "在文件系统中搜索文件",
            ToolImpl::Weather(_) => "获取天气信息",
        }
    }

    /// Gets the input schema for this tool
    ///
    /// Describes what parameters the tool accepts.
    pub fn schema(&self) -> ToolInputSchema {
        match self {
            ToolImpl::SearchFiles(tool) => tool.schema(),
            ToolImpl::Weather(tool) => tool.schema(),
        }
    }

    /// Executes this tool with the given arguments
    ///
    /// # Arguments
    ///
    /// * `arguments` - JSON value containing tool arguments
    ///
    /// # Returns
    ///
    /// Result containing the tool's output or an error
    pub async fn execute(&self, arguments: Value) -> Result<CallToolResult> {
        match self {
            ToolImpl::SearchFiles(tool) => tool.execute(arguments).await,
            ToolImpl::Weather(tool) => tool.execute(arguments).await,
        }
    }
}

/// Registry for managing all available tools
///
/// Provides centralized access to tools and their metadata.
pub struct ToolRegistry {
    /// Map of tool names to tool implementations
    tools: HashMap<String, ToolImpl>,
}

impl ToolRegistry {
    /// Creates a new tool registry with all built-in tools
    ///
    /// # Returns
    ///
    /// A new `ToolRegistry` with default tools registered
    pub fn new() -> Self {
        let mut tools = HashMap::new();
        tools.insert("search_files".to_string(), ToolImpl::SearchFiles(SearchFilesTool));
        tools.insert("get_weather".to_string(), ToolImpl::Weather(WeatherTool));

        ToolRegistry { tools }
    }

    /// Gets a tool by name
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the tool to retrieve
    ///
    /// # Returns
    ///
    /// Option containing a reference to the tool if found
    pub fn get(&self, name: &str) -> Option<&ToolImpl> {
        self.tools.get(name)
    }

    /// Gets a list of all available tools
    ///
    /// Converts internal tool implementations to public Tool structures.
    ///
    /// # Returns
    ///
    /// Vector of Tool definitions
    pub fn list_tools(&self) -> Vec<Tool> {
        self.tools
            .values()
            .map(|tool| Tool {
                name: tool.name().to_string(),
                description: tool.description().to_string(),
                input_schema: tool.schema(),
            })
            .collect()
    }

    /// Gets all tool names
    ///
    /// # Returns
    ///
    /// Vector of tool name strings
    #[allow(dead_code)]
    pub fn get_tool_names(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ToolRegistry {
    /// Creates a clone of the tool registry
    ///
    /// This is used to pass tool registry to spawned async tasks.
    fn clone(&self) -> Self {
        ToolRegistry::new()
    }
}

