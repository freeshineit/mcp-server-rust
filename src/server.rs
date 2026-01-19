//! # MCP Server Core
//!
//! Implements the TCP server and JSON-RPC 2.0 protocol handling.
//! Manages client connections and dispatches requests to tools and resources.

use crate::models::*;
use crate::tools::ToolRegistry;
use crate::resources::ResourceRegistry;
use anyhow::Result;
use serde_json::Value;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

/// The main MCP Server
///
/// Manages tool and resource registries and handles client connections.
pub struct McpServer {
    /// Registry of all available tools
    pub tool_registry: ToolRegistry,
    /// Registry of all available resources
    pub resource_registry: ResourceRegistry,
}

impl McpServer {
    /// Creates a new MCP Server instance
    ///
    /// Initializes the server with built-in tools and resources.
    ///
    /// # Returns
    ///
    /// A new `McpServer` instance with default tools and resources
    pub fn new() -> Self {
        let tool_registry = ToolRegistry::new();
        let resource_registry = ResourceRegistry::new();

        McpServer {
            tool_registry,
            resource_registry,
        }
    }

    /// Starts the TCP server and listens for client connections
    ///
    /// Binds to the specified address and accepts connections in a loop.
    /// Each connection is handled in a separate async task.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address to bind to (e.g., "127.0.0.1:8080")
    ///
    /// # Returns
    ///
    /// Returns `Result<()>` with error if binding fails
    ///
    /// # Example
    ///
    /// ```ignore
    /// let server = McpServer::new();
    /// server.start("127.0.0.1:8080").await?;
    /// ```
    pub async fn start(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("MCP Server 监听在 {}", addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let tool_registry = self.clone_tool_registry();
            let resource_registry = self.clone_resource_registry();
            
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(socket, tool_registry, resource_registry).await {
                    eprintln!("处理连接失败: {}", e);
                }
            });
        }
    }

    /// Handles a single client connection
    ///
    /// Reads JSON-RPC messages line by line and processes them.
    ///
    /// # Arguments
    ///
    /// * `socket` - The TCP socket for communication
    /// * `tool_registry` - Registry of available tools
    /// * `resource_registry` - Registry of available resources
    async fn handle_connection(
        mut socket: TcpStream,
        tool_registry: ToolRegistry,
        resource_registry: ResourceRegistry,
    ) -> Result<()> {
        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut buffer = String::new();

        while reader.read_line(&mut buffer).await? > 0 {
            let trimmed = buffer.trim();
            if !trimmed.is_empty() {
                match Self::handle_message(trimmed, &tool_registry, &resource_registry).await {
                    Ok(response) => {
                        writer.write_all(response.as_bytes()).await?;
                    }
                    Err(e) => {
                        eprintln!("处理消息失败: {}", e);
                    }
                }
            }
            buffer.clear();
        }

        Ok(())
    }

    /// Processes a single JSON-RPC message
    ///
    /// Parses the message and dispatches to appropriate handler based on method.
    ///
    /// # Arguments
    ///
    /// * `message` - The JSON-RPC message string
    /// * `tool_registry` - Registry of available tools
    /// * `resource_registry` - Registry of available resources
    async fn handle_message(
        message: &str,
        tool_registry: &ToolRegistry,
        resource_registry: &ResourceRegistry,
    ) -> Result<String> {
        let mcp_msg: McpMessage = serde_json::from_str(message)?;

        let response = match mcp_msg.method.as_str() {
            "tools/list" => Self::handle_list_tools(tool_registry, mcp_msg.id).await?,
            "tools/call" => Self::handle_call_tool(tool_registry, mcp_msg.params, mcp_msg.id).await?,
            "resources/list" => Self::handle_list_resources(resource_registry, mcp_msg.id).await?,
            "resources/read" => Self::handle_read_resource(resource_registry, mcp_msg.params, mcp_msg.id).await?,
            _ => {
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32601,
                        "message": "方法未找到"
                    },
                    "id": mcp_msg.id
                })
                .to_string()
            }
        };

        Ok(response)
    }

    /// Handles `tools/list` RPC method
    ///
    /// Returns a JSON-RPC response containing all available tools.
    ///
    /// # Arguments
    ///
    /// * `tool_registry` - Registry of available tools
    /// * `id` - JSON-RPC request ID
    async fn handle_list_tools(
        tool_registry: &ToolRegistry,
        id: Option<u64>,
    ) -> Result<String> {
        let tools = tool_registry.list_tools();
        let result = ListToolsResult { tools };

        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        })
        .to_string())
    }

    /// Handles `tools/call` RPC method
    ///
    /// Invokes a tool with the provided arguments and returns the result.
    ///
    /// # Arguments
    ///
    /// * `tool_registry` - Registry of available tools
    /// * `params` - RPC parameters containing tool name and arguments
    /// * `id` - JSON-RPC request ID
    async fn handle_call_tool(
        tool_registry: &ToolRegistry,
        params: Value,
        id: Option<u64>,
    ) -> Result<String> {
        let request: CallToolRequest = serde_json::from_value(params)?;

        match tool_registry.get(&request.name) {
            Some(tool) => {
                match tool.execute(request.arguments).await {
                    Ok(result) => {
                        Ok(serde_json::json!({
                            "jsonrpc": "2.0",
                            "result": result,
                            "id": id
                        })
                        .to_string())
                    }
                    Err(e) => {
                        Ok(serde_json::json!({
                            "jsonrpc": "2.0",
                            "error": {
                                "code": -32602,
                                "message": format!("工具执行失败: {}", e)
                            },
                            "id": id
                        })
                        .to_string())
                    }
                }
            }
            None => {
                Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32601,
                        "message": "工具未找到"
                    },
                    "id": id
                })
                .to_string())
            }
        }
    }

    /// Handles `resources/list` RPC method
    ///
    /// Returns a JSON-RPC response containing all available resources.
    ///
    /// # Arguments
    ///
    /// * `resource_registry` - Registry of available resources
    /// * `id` - JSON-RPC request ID
    async fn handle_list_resources(
        resource_registry: &ResourceRegistry,
        id: Option<u64>,
    ) -> Result<String> {
        let resources = resource_registry.list_resources();
        let result = ListResourcesResult { resources };

        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        })
        .to_string())
    }

    /// Handles `resources/read` RPC method
    ///
    /// Reads a resource and returns its content.
    ///
    /// # Arguments
    ///
    /// * `resource_registry` - Registry of available resources
    /// * `params` - RPC parameters containing resource URI
    /// * `id` - JSON-RPC request ID
    async fn handle_read_resource(
        resource_registry: &ResourceRegistry,
        params: Value,
        id: Option<u64>,
    ) -> Result<String> {
        let request: ReadResourceRequest = serde_json::from_value(params)?;

        match resource_registry.read_resource(&request.uri).await {
            Ok(contents) => {
                Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "result": {
                        "contents": contents
                    },
                    "id": id
                })
                .to_string())
            }
            Err(_) => {
                Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32602,
                        "message": "资源未找到"
                    },
                    "id": id
                })
                .to_string())
            }
        }
    }

    /// Creates a clone of the tool registry
    ///
    /// Used for passing to spawned async tasks.
    fn clone_tool_registry(&self) -> ToolRegistry {
        ToolRegistry::new()
    }

    /// Creates a clone of the resource registry
    ///
    /// Used for passing to spawned async tasks.
    fn clone_resource_registry(&self) -> ResourceRegistry {
        ResourceRegistry::new()
    }
}


