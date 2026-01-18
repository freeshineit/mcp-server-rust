// src/server.rs
use crate::models::*;
use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

pub struct McpServer {
    pub tools: HashMap<String, Tool>,
    pub resources: Vec<Resource>,
}

impl McpServer {
    pub fn new() -> Self {
        let mut tools = HashMap::new();

        // 注册示例工具
        tools.insert(
            "search_files".to_string(),
            Tool {
                name: "search_files".to_string(),
                description: "在文件系统中搜索文件".to_string(),
                input_schema: ToolInputSchema {
                    type_: "object".to_string(),
                    properties: {
                        let mut map = HashMap::new();
                        map.insert(
                            "pattern".to_string(),
                            Property {
                                type_: "string".to_string(),
                                description: "搜索模式（支持通配符）".to_string(),
                            },
                        );
                        map.insert(
                            "directory".to_string(),
                            Property {
                                type_: "string".to_string(),
                                description: "搜索目录".to_string(),
                            },
                        );
                        map
                    },
                    required: vec!["pattern".to_string()],
                },
            },
        );

        tools.insert(
            "get_weather".to_string(),
            Tool {
                name: "get_weather".to_string(),
                description: "获取天气信息".to_string(),
                input_schema: ToolInputSchema {
                    type_: "object".to_string(),
                    properties: {
                        let mut map = HashMap::new();
                        map.insert(
                            "city".to_string(),
                            Property {
                                type_: "string".to_string(),
                                description: "城市名称".to_string(),
                            },
                        );
                        map
                    },
                    required: vec!["city".to_string()],
                },
            },
        );

        let resources = vec![
            Resource {
                uri: "file:///etc/hosts".to_string(),
                mime_type: "text/plain".to_string(),
            },
            Resource {
                uri: "file:///var/log/system.log".to_string(),
                mime_type: "text/plain".to_string(),
            },
        ];

        McpServer { tools, resources }
    }

    pub async fn start(&self, addr: &str) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        println!("MCP Server 监听在 http://{}", addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let server_clone = self.clone();
            tokio::spawn(async move {
                if let Err(e) = server_clone.handle_connection(socket).await {
                    eprintln!("处理连接失败: {}", e);
                }
            });
        }
    }

    async fn handle_connection(&self, mut socket: TcpStream) -> Result<()> {
        let (reader, mut writer) = socket.split();
        let mut reader = BufReader::new(reader);
        let mut buffer = String::new();

        // 发送初始化消息
        let init_message = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": {}
                }
            },
            "id": 1
        });

        writer
            .write_all(format!("{}\n", init_message.to_string()).as_bytes())
            .await?;

        while reader.read_line(&mut buffer).await? > 0 {
            let trimmed = buffer.trim();
            if !trimmed.is_empty() {
                match self.handle_message(trimmed).await {
                    Ok(response) => {
                        writer.write_all(response.as_bytes()).await?;
                        writer.write_all(b"\n").await?;
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

    async fn handle_message(&self, message: &str) -> Result<String> {
        let mcp_msg: McpMessage = serde_json::from_str(message)?;

        let response = match mcp_msg.method.as_str() {
            "tools/list" => self.handle_list_tools(mcp_msg.id).await?,
            "tools/call" => self.handle_call_tool(mcp_msg.params, mcp_msg.id).await?,
            "resources/list" => self.handle_list_resources(mcp_msg.id).await?,
            "resources/read" => self.handle_read_resource(mcp_msg.params, mcp_msg.id).await?,
            _ => {
                // 错误响应
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

    async fn handle_list_tools(&self, id: Option<u64>) -> Result<String> {
        let tools = self.tools.values().cloned().collect();
        let result = ListToolsResult { tools };

        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        })
        .to_string())
    }

    async fn handle_call_tool(&self, params: Value, id: Option<u64>) -> Result<String> {
        let request: CallToolRequest = serde_json::from_value(params)?;

        let result = match request.name.as_str() {
            "search_files" => self.handle_search_files(request.arguments).await?,
            "get_weather" => self.handle_get_weather(request.arguments).await?,
            _ => {
                return Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32601,
                        "message": "工具未找到"
                    },
                    "id": id
                })
                .to_string())
            }
        };

        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        })
        .to_string())
    }

    async fn handle_list_resources(&self, id: Option<u64>) -> Result<String> {
        let result = ListResourcesResult {
            resources: self.resources.clone(),
        };

        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": id
        })
        .to_string())
    }

    async fn handle_read_resource(&self, params: Value, id: Option<u64>) -> Result<String> {
        let request: ReadResourceRequest = serde_json::from_value(params)?;

        // 这里实现资源读取逻辑
        let content = match request.uri.as_str() {
            "file:///etc/hosts" => {
                // 读取示例文件内容
                let text = "127.0.0.1 localhost\n::1 localhost\n".to_string();
                vec![Content {
                    type_: "text".to_string(),
                    text,
                }]
            }
            _ => {
                return Ok(serde_json::json!({
                    "jsonrpc": "2.0",
                    "error": {
                        "code": -32602,
                        "message": "资源未找到"
                    },
                    "id": id
                })
                .to_string())
            }
        };

        Ok(serde_json::json!({
            "jsonrpc": "2.0",
            "result": {
                "contents": content
            },
            "id": id
        })
        .to_string())
    }

    async fn handle_search_files(&self, arguments: Value) -> Result<CallToolResult> {
        let pattern = arguments["pattern"]
            .as_str()
            .context("缺少 pattern 参数")?;
        let directory = arguments["directory"]
            .as_str()
            .unwrap_or(".");

        // 模拟文件搜索
        let text = format!("在目录 {} 中搜索模式 '{}'\n找到以下文件:\n1. /path/to/file1.txt\n2. /path/to/file2.log",
            directory, pattern);

        Ok(CallToolResult {
            content: vec![Content {
                type_: "text".to_string(),
                text,
            }],
        })
    }

    async fn handle_get_weather(&self, arguments: Value) -> Result<CallToolResult> {
        let city = arguments["city"]
            .as_str()
            .context("缺少 city 参数")?;

        // 模拟天气查询
        let text = format!("{} 的天气:\n温度: 22°C\n天气: 晴朗\n湿度: 65%", city);

        Ok(CallToolResult {
            content: vec![Content {
                type_: "text".to_string(),
                text,
            }],
        })
    }
}

impl Clone for McpServer {
    fn clone(&self) -> Self {
        McpServer {
            tools: self.tools.clone(),
            resources: self.resources.clone(),
        }
    }
}
