//! # Resource Handler Module
//!
//! Manages resource registration and access.
//! Resources are files or data sources that can be read via the server.

use crate::models::{Content, Resource};
use anyhow::Result;
use std::collections::HashMap;

/// Registry for managing all available resources
///
/// Provides centralized access to resource metadata and content.
pub struct ResourceRegistry {
    /// Map of resource URIs to resource metadata
    resources: HashMap<String, Resource>,
}

impl ResourceRegistry {
    /// Creates a new resource registry with default resources
    ///
    /// # Returns
    ///
    /// A new `ResourceRegistry` with default system resources
    pub fn new() -> Self {
        let mut resources = HashMap::new();
        
        // Initialize with default resources
        resources.insert(
            "file:///etc/hosts".to_string(),
            Resource {
                uri: "file:///etc/hosts".to_string(),
                mime_type: "text/plain".to_string(),
            },
        );

        ResourceRegistry { resources }
    }

    /// Gets resource metadata by URI
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the resource to retrieve
    ///
    /// # Returns
    ///
    /// Option containing resource metadata if found
    #[allow(dead_code)]
    pub fn get_uri(&self, uri: &str) -> Option<&Resource> {
        self.resources.get(uri)
    }

    /// Gets a list of all available resources
    ///
    /// # Returns
    ///
    /// Vector of Resource definitions
    pub fn list_resources(&self) -> Vec<Resource> {
        self.resources.values().cloned().collect()
    }

    /// Reads the content of a resource
    ///
    /// Asynchronously fetches and returns the content of a resource.
    ///
    /// # Arguments
    ///
    /// * `uri` - The URI of the resource to read
    ///
    /// # Returns
    ///
    /// Result containing vector of Content items or error if resource not found
    ///
    /// # Example
    ///
    /// ```ignore
    /// let registry = ResourceRegistry::new();
    /// let content = registry.read_resource("file:///etc/hosts").await?;
    /// ```
    pub async fn read_resource(&self, uri: &str) -> Result<Vec<Content>> {
        match uri {
            "file:///etc/hosts" => {
                let text = "127.0.0.1 localhost\n::1 localhost\n".to_string();
                Ok(vec![Content {
                    type_: "text".to_string(),
                    text,
                }])
            }
            _ => Err(anyhow::anyhow!("Resource not found")),
        }
    }

    /// Gets all resource URIs
    ///
    /// # Returns
    ///
    /// Vector of resource URI strings
    #[allow(dead_code)]
    pub fn get_resource_uris(&self) -> Vec<String> {
        self.resources.keys().cloned().collect()
    }
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for ResourceRegistry {
    /// Creates a clone of the resource registry
    ///
    /// This is used to pass resource registry to spawned async tasks.
    fn clone(&self) -> Self {
        ResourceRegistry::new()
    }
}

