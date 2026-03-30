//! Storage Adapter wrapper bridging external StoragePlugin with internal Plugin trait.

use std::sync::Arc;

use async_trait::async_trait;
use plugin_registry::{
    error::PluginError,
    plugin_trait::{Plugin, PluginConfig, PluginMetadata},
};

/// Wrapper that converts an external StoragePlugin into our internal Plugin trait.
///
/// This allows storage operations to be managed by our unified PluginRegistry.
pub struct StorageAdapterWrapper<T: agileplus_plugin_core::traits::StoragePlugin> {
    inner: T,
    metadata: PluginMetadata,
}

impl<T: agileplus_plugin_core::traits::StoragePlugin> StorageAdapterWrapper<T> {
    /// Create a new storage adapter wrapper.
    pub fn new(inner: T, metadata: PluginMetadata) -> Self {
        Self { inner, metadata }
    }

    /// Get a reference to the inner storage adapter.
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

#[async_trait]
impl<T: agileplus_plugin_core::traits::StoragePlugin + Send + Sync> Plugin
    for StorageAdapterWrapper<T>
{
    fn name(&self) -> &str {
        self.inner.name()
    }

    fn version(&self) -> &str {
        self.inner.version()
    }

    fn metadata(&self) -> Option<PluginMetadata> {
        Some(self.metadata.clone())
    }

    async fn initialize(&self, config: PluginConfig) -> Result<(), PluginError> {
        let external_config = agileplus_plugin_core::traits::PluginConfig {
            name: config.name.unwrap_or_else(|| self.name().to_string()),
            version: config.version.unwrap_or_else(|| self.version().to_string()),
            adapter_config: config.config,
        };

        self.inner
            .initialize(external_config)
            .await
            .map_err(|e| PluginError::Initialization(e.to_string()))
    }

    async fn shutdown(&self) -> Result<(), PluginError> {
        self.inner
            .health_check()
            .await
            .map_err(|e| PluginError::Operation(e.to_string()))
    }
}

/// Convert an external StoragePlugin into a boxed Plugin.
pub fn into_plugin<
    T: agileplus_plugin_core::traits::StoragePlugin + Send + Sync + 'static,
>(
    adapter: T,
    metadata: PluginMetadata,
) -> Arc<dyn Plugin> {
    Arc::new(StorageAdapterWrapper::new(adapter, metadata))
}

#[cfg(test)]
mod tests {
    use super::*;
    use agileplus_plugin_core::traits::{StoragePlugin, PluginConfig};

    struct MockStoragePlugin;

    #[async_trait]
    impl agileplus_plugin_core::traits::AdapterPlugin for MockStoragePlugin {
        fn name(&self) -> &str { "mock-storage" }
        fn version(&self) -> &str { "0.1.0" }
        fn initialize(&self, _config: PluginConfig) -> agileplus_plugin_core::error::PluginResult<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl StoragePlugin for MockStoragePlugin {
        async fn create_feature(&self, _: &serde_json::Value) -> agileplus_plugin_core::error::PluginResult<i64> { Ok(1) }
        async fn get_feature_by_slug(&self, _: &str) -> agileplus_plugin_core::error::PluginResult<Option<_>> { Ok(None) }
        async fn get_feature_by_id(&self, _: i64) -> agileplus_plugin_core::error::PluginResult<Option<_>> { Ok(None) }
        async fn update_feature_state(&self, _: i64, _: &str) -> agileplus_plugin_core::error::PluginResult<()> { Ok(()) }
        async fn list_all_features(&self) -> agileplus_plugin_core::error::PluginResult<Vec<_>> { Ok(vec![]) }
        async fn create_work_package(&self, _: &serde_json::Value) -> agileplus_plugin_core::error::PluginResult<i64> { Ok(1) }
        async fn get_work_package(&self, _: i64) -> agileplus_plugin_core::error::PluginResult<Option<_>> { Ok(None) }
        async fn update_wp_state(&self, _: i64, _: &str) -> agileplus_plugin_core::error::PluginResult<()> { Ok(()) }
        async fn append_audit_entry(&self, _: &serde_json::Value) -> agileplus_plugin_core::error::PluginResult<i64> { Ok(1) }
        async fn get_audit_trail(&self, _: i64) -> agileplus_plugin_core::error::PluginResult<Vec<_>> { Ok(vec![]) }
    }

    #[tokio::test]
    async fn test_storage_wrapper_trait_compliance() {
        let wrapper = StorageAdapterWrapper::new(MockStoragePlugin, PluginMetadata::default());

        assert_eq!(wrapper.name(), "mock-storage");
        assert_eq!(wrapper.version(), "0.1.0");

        let config = PluginConfig::default();
        wrapper.initialize(config).await.unwrap();
    }
}
