//! Integration layer bridging plugin-registry with external agileplus-plugin-core.
//!
//! This crate provides adapters that wrap external plugins (VcsPlugin, StoragePlugin)
//! into our internal Plugin trait for unified management.
//!
//! ## Architecture
//!
//! ```text
//! +---------------------------+     +---------------------------+
//! |   External Repos          |     |   Internal Registry       |
//! |   (plugin-core, -git,     | --> |   (plugin-registry)       |
//! |    -sqlite)               |     |                           |
//! +---------------------------+     +---------------------------+
//!            |                               |
//!            v                               v
//! +---------------------------+     +---------------------------+
//! |   Domain Traits           |     |   Plugin Trait            |
//! |   (VcsPlugin,             | --> |   (name, version,         |
//! |    StoragePlugin)         |     |    initialize, shutdown) |
//! +---------------------------+     +---------------------------+
//!            ^                               ^
//!            |                               |
//! +---------------------------+     +---------------------------+
//! |   AdapterWrappers         | --> |   UnifiedRegistry        |
//! |   (VcsAdapterWrapper,     |     |   (PluginRegistry)        |
//! |    StorageAdapterWrapper) |     +---------------------------+
//! +---------------------------+
//! ```

mod vcs_adapter;
mod storage_adapter;
mod unified_registry;

pub use vcs_adapter::VcsAdapterWrapper;
pub use storage_adapter::StorageAdapterWrapper;
pub use unified_registry::UnifiedPluginRegistry;

pub use plugin_registry::{
    error::PluginError,
    plugin_trait::{Plugin, PluginConfig, PluginMetadata},
    registry::PluginRegistry,
};
