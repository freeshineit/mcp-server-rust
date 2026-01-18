// src/main.rs
mod models;
mod server;

use clap::{Parser, Subcommand};
use server::McpServer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 启动 MCP 服务器
    Start {
        /// 监听地址
        #[arg(short, long, default_value = "127.0.0.1:8080")]
        address: String,
    },
    /// 列出所有可用的工具
    ListTools,
    /// 列出所有资源
    ListResources,
}

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
            for (name, tool) in &server.tools {
                println!("工具: {}", name);
                println!("描述: {}", tool.description);
                println!("参数:");
                for (param_name, prop) in &tool.input_schema.properties {
                    println!("  - {}: {}", param_name, prop.description);
                }
                println!();
            }
        }
        Commands::ListResources => {
            println!("可用资源:");
            for resource in &server.resources {
                println!("- {}", resource.uri);
            }
        }
    }

    Ok(())
}
