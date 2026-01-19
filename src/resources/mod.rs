//! # Resources Module
//!
//! Provides resource management and access.
//!
//! Resources are files or data sources that can be read via the server.
//! New resources should be added to the `ResourceRegistry`.

pub mod resource_handler;

pub use resource_handler::ResourceRegistry;

