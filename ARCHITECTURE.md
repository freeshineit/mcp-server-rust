# MCP Server Rust - 项目架构文档

## 项目概述

MCP Server Rust 是一个基于 Rust 的 Model Context Protocol (MCP) 服务器实现，支持工具注册、资源管理和 JSON-RPC 2.0 协议通信。

## 架构设计

### 分层架构

```
┌─────────────────────────────────────────────────────────────┐
│                     应用层 (main.rs)                        │
│              (CLI 命令处理、参数解析)                       │
└────────────────┬────────────────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────────────────┐
│                   核心层 (server.rs)                        │
│         (MCP Server 实现、连接管理、消息分发)              │
└────────────────┬────────────────────────────────────────────┘
                 │
        ┌────────┴─────────┐
        │                  │
┌───────▼──────┐   ┌──────▼─────────┐
│  工具模块    │   │   资源模块     │
│ (tools/)     │   │ (resources/)   │
└──────────────┘   └────────────────┘
        │                  │
┌───────▼──────────────────▼──────────┐
│        数据模型层 (models.rs)        │
│  (Tool、Resource、Message 结构体)  │
└────────────────────────────────────┘
```

## 核心模块详解

### 1. 主入口模块 (`main.rs`)

**职责**：
- CLI 参数解析和命令分发
- 创建 McpServer 实例
- 处理启动、列举工具/资源等命令

**命令**：
```bash
# 启动服务器
cargo run -- start [--address <ADDR>]

# 列出所有工具
cargo run -- list-tools

# 列出所有资源
cargo run -- list-resources
```

---

### 2. 服务器核心模块 (`server.rs`)

**职责**：
- TCP 服务器实现，监听客户端连接
- JSON-RPC 2.0 消息解析和处理
- 工具和资源的请求分发
- 异步连接处理

**关键结构体**：
```rust
pub struct McpServer {
    pub tool_registry: ToolRegistry,      // 工具注册表
    pub resource_registry: ResourceRegistry, // 资源注册表
}
```

**支持的 RPC 方法**：
- `tools/list` - 列出所有可用工具
- `tools/call` - 调用指定工具
- `resources/list` - 列出所有资源
- `resources/read` - 读取指定资源

---

### 3. 工具模块 (`src/tools/`)

#### 3.1 工具处理器 (`tool_handler.rs`)

**职责**：
- 管理所有已注册的工具
- 工具的注册和查询
- 工具列表的获取

**关键结构体**：
```rust
pub enum ToolImpl {
    SearchFiles(SearchFilesTool),
    Weather(WeatherTool),
}

pub struct ToolRegistry {
    tools: HashMap<String, ToolImpl>,
}
```

#### 3.2 内置工具 (`builtin_tools.rs`)

实现两个内置工具：

**SearchFilesTool**
- 名称：`search_files`
- 功能：在文件系统中搜索文件
- 参数：
  - `pattern` (必需)：搜索模式
  - `directory` (可选)：搜索目录，默认 "."

**WeatherTool**
- 名称：`get_weather`
- 功能：获取天气信息
- 参数：
  - `city` (必需)：城市名称

**扩展工具**：
1. 在 `builtin_tools.rs` 中定义新工具结构体
2. 实现 `schema()` 和 `execute()` 方法
3. 在 `tool_handler.rs` 的 `ToolImpl` 枚举中添加新变体
4. 在 `ToolRegistry::new()` 中注册新工具

---

### 4. 资源模块 (`src/resources/`)

**职责**：
- 管理和提供文件/数据资源访问
- 资源的列表和读取操作

**关键结构体**：
```rust
pub struct ResourceRegistry {
    resources: HashMap<String, Resource>,
}
```

**支持的资源**：
- `file:///etc/hosts` - 系统 hosts 文件

**扩展资源**：
1. 在 `ResourceRegistry::new()` 中添加新资源
2. 在 `read_resource()` 方法中实现读取逻辑

---

### 5. 数据模型 (`models.rs`)

**核心数据结构**：

```rust
// 工具定义
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: ToolInputSchema,
}

pub struct ToolInputSchema {
    pub type_: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

// RPC 消息
pub struct McpMessage {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: Option<u64>,
}

// 工具调用
pub struct CallToolRequest {
    pub name: String,
    pub arguments: serde_json::Value,
}

// 资源定义
pub struct Resource {
    pub uri: String,
    pub mime_type: String,
}
```

---

## 通信协议

### JSON-RPC 2.0 格式

**工具列表请求**：
```json
{
  "jsonrpc": "2.0",
  "method": "tools/list",
  "id": 1
}
```

**工具调用请求**：
```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "search_files",
    "arguments": {
      "pattern": "*.txt",
      "directory": "/tmp"
    }
  },
  "id": 2
}
```

**响应格式**：
```json
{
  "jsonrpc": "2.0",
  "result": { ... },
  "id": 1
}
```

---

## 性能特性

### 并发处理
- 使用 `tokio` 异步运行时
- 每个客户端连接在独立的异步任务中处理
- 支持高并发客户端

### 内存效率
- 使用 HashMap 进行 O(1) 工具/资源查找
- 流式行处理客户端请求
- 零复制消息转发

---

## 依赖关系

```
mcp-server-rust
├── tokio (async runtime)
├── serde (serialization)
│   └── serde_json
├── clap (CLI parsing)
├── anyhow (error handling)
├── thiserror (error types)
└── tracing (logging)
```

---

## 扩展指南

### 添加新工具

```rust
// 1. 在 builtin_tools.rs 中定义工具
#[derive(Clone, Copy)]
pub struct MyTool;

impl MyTool {
    pub fn schema(&self) -> ToolInputSchema { ... }
    pub async fn execute(&self, arguments: Value) -> Result<CallToolResult> { ... }
}

// 2. 在 tool_handler.rs 的 ToolImpl 中添加变体
pub enum ToolImpl {
    MyTool(MyTool),
    // ... 其他工具
}

// 3. 在 ToolRegistry::new() 中注册
tools.insert("my_tool".to_string(), ToolImpl::MyTool(MyTool));
```

### 添加新资源

```rust
// 在 resource_handler.rs 的 ResourceRegistry::new() 中
resources.insert(
    "file:///my/resource".to_string(),
    Resource {
        uri: "file:///my/resource".to_string(),
        mime_type: "text/plain".to_string(),
    },
);

// 在 read_resource() 中处理读取
"file:///my/resource" => {
    let text = "resource content".to_string();
    Ok(vec![Content { type_: "text".to_string(), text }])
}
```

---

## 错误处理

- 使用 `anyhow::Result<T>` 进行错误传播
- JSON-RPC 错误码约定：
  - `-32601`：方法未找到
  - `-32602`：参数错误或资源未找到
  - `-32603`：内部错误

---

## 部署

### Docker 部署

```bash
docker-compose -f docker-compose.yml up -d
```

### 本地运行

```bash
# 调试版本
cargo run -- start --address 0.0.0.0:8080

# 发布版本
cargo build --release
./target/release/mcp-server-rust start --address 0.0.0.0:8080
```

---

## 项目质量

### 编译检查
- ✅ 无编译错误
- ⚠️ 未使用的方法需要清理（可选）

### 测试客户端
```bash
python3 test_client.py
```

---

## 文件结构汇总

```
mcp-server-rust/
├── Cargo.toml                 # 项目配置和依赖
├── src/
│   ├── main.rs              # CLI 入口
│   ├── server.rs            # 服务器核心逻辑
│   ├── models.rs            # 数据结构定义
│   ├── tools/
│   │   ├── mod.rs           # 工具模块导出
│   │   ├── tool_handler.rs  # 工具注册管理
│   │   └── builtin_tools.rs # 内置工具实现
│   └── resources/
│       ├── mod.rs           # 资源模块导出
│       └── resource_handler.rs # 资源管理
├── docker-compose.yml       # Docker 部署配置
├── Dockerfile               # Docker 镜像定义
└── test_client.py          # Python 测试客户端
```

---

## 总结

该架构采用模块化设计，清晰的分层结构使得：
1. **易于扩展**：添加工具和资源只需实现简单的接口
2. **高效并发**：基于 tokio 异步运行时的高性能
3. **类型安全**：使用枚举而非 trait 对象，避免运行时开销
4. **易于维护**：清晰的职责划分和模块边界

