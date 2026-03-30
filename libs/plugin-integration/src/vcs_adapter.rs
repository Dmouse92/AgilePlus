//! VCS Adapter wrapper bridging external VcsPlugin with internal Plugin trait.

use std::sync::Arc;

use async_trait::async_trait;
use plugin_registry::{
    error::PluginError,
    plugin_trait::{Plugin, PluginConfig, PluginMetadata},
};

/// Wrapper that converts an external VcsPlugin into our internal Plugin trait.
///
/// This allows git operations to be managed by our unified PluginRegistry.
pub struct VcsAdapterWrapper<T: agileplus_plugin_core::traits::VcsPlugin> {
    inner: T,
    metadata: PluginMetadata,
}

impl<T: agileplus_plugin_core::traits::VcsPlugin> VcsAdapterWrapper<T> {
    /// Create a new VCS adapter wrapper.
    pub fn new(inner: T, metadata: PluginMetadata) -> Self {
        Self { inner, metadata }
    }

    /// Get a reference to the inner VCS adapter.
    pub fn inner(&self) -> &T {
        &self.inner
    }
}

#[async_trait]
impl<T: agileplus_plugin_core::traits::VcsPlugin + Send + Sync> Plugin
    for VcsAdapterWrapper<T>
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
        // Convert our config to external format
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
        // VcsPlugin doesn't have a shutdown method, but we can do health check
        self.inner
            .health_check()
            .await
            .map_err(|e| PluginError::Operation(e.to_string()))
    }
}

/// Convert an external VcsPlugin into a boxed Plugin.
pub fn into_plugin<T: agileplus_plugin_core::traits::VcsPlugin + Send + Sync + 'static>(
    adapter: T,
    metadata: PluginMetadata,
) -> Arc<dyn Plugin> {
    Arc::new(VcsAdapterWrapper::new(adapter, metadata))
}

#[cfg(test)]
mod tests {
    use super::*;
    use agileplus_plugin_core::traits::{VcsPlugin, PluginConfig};

    struct MockVcsPlugin;

    #[async_trait]
    impl agileplus_plugin_core::traits::AdapterPlugin for MockVcsPlugin {
        fn name(&self) -> &str { "mock-vcs" }
        fn version(&self) -> &str { "0.1.0" }
        fn initialize(&self, _config: PluginConfig) -> agileplus_plugin_core::error::PluginResult<()> {
            Ok(())
        }
    }

    #[async_trait]
    impl VcsPlugin for MockVcsPlugin {
        async fn create_worktree(&self, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<std::path::PathBuf> {
            Ok(std::path::PathBuf::new())
        }
        async fn list_worktrees(&self) -> agileplus_plugin_core::error::PluginResult<Vec<_>> { Ok(vec![]) }
        async fn cleanup_worktree(&self, _: &std::path::Path) -> agileplus_plugin_core::error::PluginResult<()> { Ok(()) }
        async fn create_branch(&self, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<()> { Ok(()) }
        async fn checkout_branch(&self, _: &str) -> agileplus_plugin_core::error::PluginResult<()> { Ok(()) }
        async fn merge_to_target(&self, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<_> { Ok(Default::default()) }
        async fn detect_conflicts(&self, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<Vec<_>> { Ok(vec![]) }
        async fn read_artifact(&self, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<String> { Ok(String::new()) }
        async fn write_artifact(&self, _: &str, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<()> { Ok(()) }
        async fn artifact_exists(&self, _: &str, _: &str) -> agileplus_plugin_core::error::PluginResult<bool> { Ok(false) }
        async fn scan_feature_artifacts(&self, _: &str) -> agileplus_plugin_core::error::PluginResult<_> { Ok(Default::default()) }
    }

    #[tokio::test]
    async fn test_vcs_wrapper_trait_compliance() {
        let wrapper = VcsAdapterWrapper::new(MockVcsPlugin, PluginMetadata::default());

        assert_eq!(wrapper.name(), "mock-vcs");
        assert_eq!(wrapper.version(), "0.1.0");

        let config = PluginConfig::default();
        wrapper.initialize(config).await.unwrap();
    }
}
