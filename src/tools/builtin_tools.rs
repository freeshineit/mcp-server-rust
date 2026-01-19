//! # Built-in Tools
//!
//! Contains implementations of default tools provided by the MCP Server.
//! New tools should be added here and registered in ToolRegistry.

use crate::models::{CallToolResult, Content, Property, ToolInputSchema};
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;

/// File search tool implementation
///
/// Searches for files matching a pattern in a specified directory.
#[derive(Clone, Copy)]
pub struct SearchFilesTool;

impl SearchFilesTool {
    /// Gets the input schema for file search parameters
    ///
    /// # Returns
    ///
    /// ToolInputSchema defining 'pattern' and 'directory' parameters
    pub fn schema(&self) -> ToolInputSchema {
        let mut properties = HashMap::new();
        properties.insert(
            "pattern".to_string(),
            Property {
                type_: "string".to_string(),
                description: "搜索模式（支持通配符）".to_string(),
            },
        );
        properties.insert(
            "directory".to_string(),
            Property {
                type_: "string".to_string(),
                description: "搜索目录".to_string(),
            },
        );

        ToolInputSchema {
            type_: "object".to_string(),
            properties,
            required: vec!["pattern".to_string()],
        }
    }

    /// Executes the file search tool
    ///
    /// # Arguments
    ///
    /// * `arguments` - JSON value containing:
    ///   - `pattern` (required): Search pattern
    ///   - `directory` (optional): Directory to search in (defaults to ".")
    ///
    /// # Returns
    ///
    /// Result containing search results or error if pattern is missing
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tool = SearchFilesTool;
    /// let args = serde_json::json!({
    ///     "pattern": "*.txt",
    ///     "directory": "/tmp"
    /// });
    /// let result = tool.execute(args).await?;
    /// ```
    pub async fn execute(&self, arguments: Value) -> Result<CallToolResult> {
        let pattern = arguments["pattern"]
            .as_str()
            .context("缺少 pattern 参数")?;
        let directory = arguments["directory"]
            .as_str()
            .unwrap_or(".");

        // Mock implementation - in real scenario, use globbing or similar
        let text = format!(
            "在目录 {} 中搜索模式 '{}'\n找到以下文件:\n1. /path/to/file1.txt\n2. /path/to/file2.log",
            directory, pattern
        );

        Ok(CallToolResult {
            content: vec![Content {
                type_: "text".to_string(),
                text,
            }],
        })
    }
}

/// Weather query tool implementation
///
/// Retrieves weather information for a specified city.
#[derive(Clone, Copy)]
pub struct WeatherTool;

impl WeatherTool {
    /// Gets the input schema for weather query parameters
    ///
    /// # Returns
    ///
    /// ToolInputSchema defining the 'city' parameter
    pub fn schema(&self) -> ToolInputSchema {
        let mut properties = HashMap::new();
        properties.insert(
            "city".to_string(),
            Property {
                type_: "string".to_string(),
                description: "城市名称".to_string(),
            },
        );

        ToolInputSchema {
            type_: "object".to_string(),
            properties,
            required: vec!["city".to_string()],
        }
    }

    /// Executes the weather query tool
    ///
    /// # Arguments
    ///
    /// * `arguments` - JSON value containing:
    ///   - `city` (required): The city name to get weather for
    ///
    /// # Returns
    ///
    /// Result containing weather information or error if city is missing
    ///
    /// # Example
    ///
    /// ```ignore
    /// let tool = WeatherTool;
    /// let args = serde_json::json!({ "city": "Beijing" });
    /// let result = tool.execute(args).await?;
    /// ```
    pub async fn execute(&self, arguments: Value) -> Result<CallToolResult> {
        let city = arguments["city"]
            .as_str()
            .context("缺少 city 参数")?;

        // Mock implementation - in real scenario, call weather API
        let text = format!(
            "{} 的天气:\n温度: 22°C\n天气: 晴朗\n湿度: 65%",
            city
        );

        Ok(CallToolResult {
            content: vec![Content {
                type_: "text".to_string(),
                text,
            }],
        })
    }
}

