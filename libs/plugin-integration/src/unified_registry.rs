//! Unified plugin registry combining VCS and Storage plugins.
//!
//! This module provides a unified registry that can manage both VCS plugins
//! (for git operations) and Storage plugins (for database operations) through
//! a common Plugin interface.

use std::sync::Arc;

use plugin_registry::{
    error::PluginError,
    plugin_trait::{Plugin, PluginConfig, PluginMetadata},
    registry::PluginRegistry,
};

/// Unified plugin registry for managing all plugin types.
///
/// This registry wraps the base PluginRegistry and provides convenience methods
/// for loading VCS and Storage plugins from external repositories.
#[derive(Clone)]
pub struct UnifiedPluginRegistry {
    inner: Arc<PluginRegistry>,
}

impl UnifiedPluginRegistry {
    /// Create a new unified plugin registry.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(PluginRegistry::new()),
        }
    }

    /// Get a reference to the inner registry.
    pub fn inner(&self) -> &PluginRegistry {
        &self.inner
    }

    /// Load a plugin by name with configuration.
    pub async fn load_plugin(
        &self,
        name: &str,
        plugin: Arc<dyn Plugin>,
        config: PluginConfig,
    ) -> Result<(), PluginError> {
        self.inner.load(name, plugin).await?;
        self.inner.initialize(name, config).await
    }

    /// List all loaded plugins.
    pub fn list_plugins(&self) -> Vec<String> {
        self.inner.list()
    }

    /// Get a plugin by name.
    pub fn get_plugin(&self, name: &str) -> Option<Arc<dyn Plugin>> {
        self.inner.get(name)
    }

    /// Shutdown a plugin.
    pub async fn shutdown_plugin(&self, name: &str) -> Result<(), PluginError> {
        self.inner.shutdown(name).await
    }

    /// Unload a plugin.
    pub async fn unload_plugin(&self, name: &str) -> Result<(), PluginError> {
        self.inner.unload(name).await
    }
}

impl Default for UnifiedPluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_unified_registry_basic_operations() {
        let registry = UnifiedPluginRegistry::new();

        // Initially empty
        assert!(registry.list_plugins().is_empty());

        // Create a mock plugin
        struct MockPlugin;
        #[async_trait::async_trait]
        impl Plugin for MockPlugin {
            fn name(&self) -> &str { "mock" }
            fn version(&self) -> &str { "0.1.0" }
            fn metadata(&self) -> Option<PluginMetadata> { None }
            async fn initialize(&self, _: PluginConfig) -> Result<(), PluginError> { Ok(()) }
            async fn shutdown(&self) -> Result<(), PluginError> { Ok(()) }
        }

        let plugin: Arc<dyn Plugin> = Arc::new(MockPlugin);
        let config = PluginConfig::default();

        registry.load_plugin("test", plugin, config).await.unwrap();
        assert_eq!(registry.list_plugins(), vec!["test"]);

        let loaded = registry.get_plugin("test");
        assert!(loaded.is_some());

        registry.shutdown_plugin("test").await.unwrap();
        registry.unload_plugin("test").await.unwrap();
        assert!(registry.list_plugins().is_empty());
    }
}
