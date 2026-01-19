//! # Integration Tests
//!
//! Tests for the MCP Server functionality including tools, resources, and RPC handling.

#[cfg(test)]
mod tests {
    use mcp_server_rust::models::*;
    use serde_json::json;

    // Tool Registry Tests
    #[test]
    fn test_tool_registry_creation() {
        // Test that tool registry can be created successfully
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tools = registry.list_tools();
        
        assert!(!tools.is_empty(), "Tool registry should have default tools");
        assert_eq!(tools.len(), 2, "Should have exactly 2 built-in tools");
    }

    #[test]
    fn test_tool_registry_get_tools() {
        // Test retrieving specific tools by name
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        
        let search_tool = registry.get("search_files");
        assert!(search_tool.is_some(), "search_files tool should exist");
        
        let weather_tool = registry.get("get_weather");
        assert!(weather_tool.is_some(), "get_weather tool should exist");
    }

    #[test]
    fn test_tool_registry_get_nonexistent() {
        // Test that nonexistent tools return None
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        
        let tool = registry.get("nonexistent_tool");
        assert!(tool.is_none(), "Nonexistent tool should return None");
    }

    #[test]
    fn test_tool_registry_list_tools_metadata() {
        // Test that tool metadata is correctly returned
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tools = registry.list_tools();
        
        // Check search_files tool
        let search_tool = tools.iter().find(|t| t.name == "search_files");
        assert!(search_tool.is_some());
        
        let search_tool = search_tool.unwrap();
        assert!(!search_tool.description.is_empty());
        assert_eq!(search_tool.input_schema.type_, "object");
        assert!(search_tool.input_schema.properties.contains_key("pattern"));
    }

    // Resource Registry Tests
    #[test]
    fn test_resource_registry_creation() {
        // Test that resource registry can be created successfully
        let registry = mcp_server_rust::resources::ResourceRegistry::new();
        let resources = registry.list_resources();
        
        assert!(!resources.is_empty(), "Resource registry should have default resources");
    }

    #[test]
    fn test_resource_registry_list_resources() {
        // Test listing all resources
        let registry = mcp_server_rust::resources::ResourceRegistry::new();
        let resources = registry.list_resources();
        
        let hosts = resources.iter().find(|r| r.uri == "file:///etc/hosts");
        assert!(hosts.is_some(), "file:///etc/hosts resource should exist");
    }

    #[tokio::test]
    async fn test_resource_read_hosts_file() {
        // Test reading the hosts resource
        let registry = mcp_server_rust::resources::ResourceRegistry::new();
        let result = registry.read_resource("file:///etc/hosts").await;
        
        assert!(result.is_ok(), "Reading hosts file should succeed");
        let contents = result.unwrap();
        assert!(!contents.is_empty(), "Hosts file should have content");
        assert_eq!(contents[0].type_, "text");
    }

    #[tokio::test]
    async fn test_resource_read_nonexistent() {
        // Test that reading nonexistent resource returns error
        let registry = mcp_server_rust::resources::ResourceRegistry::new();
        let result = registry.read_resource("file:///nonexistent").await;
        
        assert!(result.is_err(), "Reading nonexistent resource should fail");
    }

    // Data Model Tests
    #[test]
    fn test_tool_model_serialization() {
        // Test that Tool struct can be serialized to JSON
        let tool = Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            input_schema: ToolInputSchema {
                type_: "object".to_string(),
                properties: Default::default(),
                required: vec![],
            },
        };
        
        let json = serde_json::to_string(&tool);
        assert!(json.is_ok());
    }

    #[test]
    fn test_resource_model_serialization() {
        // Test that Resource struct can be serialized to JSON
        let resource = Resource {
            uri: "file:///test".to_string(),
            mime_type: "text/plain".to_string(),
        };
        
        let json = serde_json::to_string(&resource);
        assert!(json.is_ok());
    }

    #[test]
    fn test_mcp_message_deserialization() {
        // Test parsing JSON-RPC messages
        let json_str = r#"{
            "jsonrpc": "2.0",
            "method": "tools/list",
            "id": 1
        }"#;
        
        let message: Result<McpMessage, _> = serde_json::from_str(json_str);
        assert!(message.is_ok());
        
        let msg = message.unwrap();
        assert_eq!(msg.jsonrpc, "2.0");
        assert_eq!(msg.method, "tools/list");
        assert_eq!(msg.id, Some(1));
    }

    #[test]
    fn test_call_tool_request_deserialization() {
        // Test parsing tool call requests
        let json_str = r#"{
            "name": "search_files",
            "arguments": {
                "pattern": "*.txt",
                "directory": "/tmp"
            }
        }"#;
        
        let request: Result<CallToolRequest, _> = serde_json::from_str(json_str);
        assert!(request.is_ok());
        
        let req = request.unwrap();
        assert_eq!(req.name, "search_files");
        assert_eq!(req.arguments["pattern"], "*.txt");
    }

    // Server Tests
    #[tokio::test]
    async fn test_mcpserver_creation() {
        // Test that McpServer can be created
        let server = mcp_server_rust::server::McpServer::new();
        
        // Verify tools are registered
        let tools = server.tool_registry.list_tools();
        assert!(!tools.is_empty());
        
        // Verify resources are registered
        let resources = server.resource_registry.list_resources();
        assert!(!resources.is_empty());
    }

    #[test]
    fn test_tool_impl_names() {
        // Test that tool implementations have correct names
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tools = registry.list_tools();
        
        let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
        assert!(names.contains(&"search_files"));
        assert!(names.contains(&"get_weather"));
    }

    #[tokio::test]
    async fn test_search_files_tool_schema() {
        // Test that search_files tool has correct schema
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tool = registry.get("search_files").unwrap();
        
        // Verify we can call schema
        let schema = tool.schema();
        assert_eq!(schema.type_, "object");
        assert!(schema.properties.contains_key("pattern"));
        assert!(schema.required.contains(&"pattern".to_string()));
    }

    #[tokio::test]
    async fn test_weather_tool_schema() {
        // Test that get_weather tool has correct schema
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tool = registry.get("get_weather").unwrap();
        
        // Verify we can call schema
        let schema = tool.schema();
        assert_eq!(schema.type_, "object");
        assert!(schema.properties.contains_key("city"));
        assert!(schema.required.contains(&"city".to_string()));
    }

    #[tokio::test]
    async fn test_search_files_tool_execution() {
        // Test executing search_files tool
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tool = registry.get("search_files").unwrap();
        
        let args = json!({
            "pattern": "*.txt",
            "directory": "/tmp"
        });
        
        let result = tool.execute(args).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        assert!(!call_result.content.is_empty());
        assert_eq!(call_result.content[0].type_, "text");
    }

    #[tokio::test]
    async fn test_weather_tool_execution() {
        // Test executing get_weather tool
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tool = registry.get("get_weather").unwrap();
        
        let args = json!({
            "city": "Beijing"
        });
        
        let result = tool.execute(args).await;
        assert!(result.is_ok());
        
        let call_result = result.unwrap();
        assert!(!call_result.content.is_empty());
    }

    #[tokio::test]
    async fn test_tool_execution_missing_required_param() {
        // Test that tool execution fails with missing required parameter
        let registry = mcp_server_rust::tools::ToolRegistry::new();
        let tool = registry.get("search_files").unwrap();
        
        let args = json!({
            "directory": "/tmp"
            // Missing required "pattern"
        });
        
        let result = tool.execute(args).await;
        assert!(result.is_err(), "Should fail with missing required parameter");
    }

    #[test]
    fn test_content_model() {
        // Test Content model creation and serialization
        let content = Content {
            type_: "text".to_string(),
            text: "Test content".to_string(),
        };
        
        let json = serde_json::to_value(&content);
        assert!(json.is_ok());
        
        let json = json.unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"], "Test content");
    }

    #[test]
    fn test_tool_input_schema_model() {
        // Test ToolInputSchema model
        let mut properties = Default::default();
        let schema = ToolInputSchema {
            type_: "object".to_string(),
            properties,
            required: vec!["param1".to_string()],
        };
        
        assert_eq!(schema.type_, "object");
        assert!(schema.required.contains(&"param1".to_string()));
    }

    #[test]
    fn test_property_model() {
        // Test Property model
        let prop = Property {
            type_: "string".to_string(),
            description: "A test property".to_string(),
        };
        
        assert_eq!(prop.type_, "string");
        assert_eq!(prop.description, "A test property");
    }

    #[test]
    fn test_registry_clone() {
        // Test that registries can be cloned
        let registry1 = mcp_server_rust::tools::ToolRegistry::new();
        let registry2 = registry1.clone();
        
        let tools1 = registry1.list_tools();
        let tools2 = registry2.list_tools();
        
        assert_eq!(tools1.len(), tools2.len());
    }
}
