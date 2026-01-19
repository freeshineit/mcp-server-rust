//! # MCP Server - Model Context Protocol Server
//!
//! This is the main entry point for the MCP Server application.
//! It provides a command-line interface for starting the server and managing tools/resources.

use mcp_server_rust::server::McpServer;
use clap::{Parser, Subcommand};

/// Command-line interface configuration
///
/// Defines the available commands and their arguments for the MCP Server.
#[derive(Parser)]
#[command(author, version, about = "MCP Server - Model Context Protocol Server", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Available CLI commands
///
/// - `Start`: Launch the TCP server on the specified address
/// - `ListTools`: Display all registered tools
/// - `ListResources`: Display all available resources
#[derive(Subcommand)]
enum Commands {
    /// 启动 MCP 服务器 (Start the MCP server)
    Start {
        /// 监听地址 (Listening address)
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        address: String,
    },
    /// 列出所有可用的工具 (List all available tools)
    ListTools,
    /// 列出所有资源 (List all resources)
    ListResources,
}

/// Main entry point for the MCP Server application
///
/// Parses command-line arguments and dispatches to the appropriate handler.
/// 
/// # Returns
/// 
/// Returns `anyhow::Result<()>` indicating success or failure
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let server = McpServer::new();

    match cli.command {
        Commands::Start { address } => {
            println!("启动 MCP 服务器...");
            server.start(&address).await?;
        }
        Commands::ListTools => {
            // Display all registered tools in a formatted manner
            let tools = server.tool_registry.list_tools();
            if tools.is_empty() {
                println!("没有可用的工具");
            } else {
                println!("可用工具:");
                for tool in tools {
                    println!("\n工具: {}", tool.name);
                    println!("描述: {}", tool.description);
                    println!("参数:");
                    for (param_name, prop) in &tool.input_schema.properties {
                        let required = if tool.input_schema.required.contains(param_name) {
                            "[必需]"
                        } else {
                            "[可选]"
                        };
                        println!("  - {}: {} {}", param_name, prop.description, required);
                    }
                }
            }
        }
        Commands::ListResources => {
            // Display all available resources in a formatted manner
            let resources = server.resource_registry.list_resources();
            if resources.is_empty() {
                println!("没有可用的资源");
            } else {
                println!("可用资源:");
                for resource in resources {
                    println!("- {} ({})", resource.uri, resource.mime_type);
                }
            }
        }
    }

    Ok(())
}
