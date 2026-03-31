//! Base entity traits and identifiers

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Unique identifier for any domain entity
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(String);

impl EntityId {
    /// Create a new random entity ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create from an existing string
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    /// Get the underlying string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Base trait for all domain entities
pub trait Entity: Send + Sync {
    /// Get the entity's unique identifier
    fn id(&self) -> &EntityId;

    /// Get the entity type name
    fn entity_type(&self) -> &'static str;
}
