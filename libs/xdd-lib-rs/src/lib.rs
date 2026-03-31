//! Cross-dialect development library for AgilePlus
//!
//! Provides utilities for converting between different configuration formats:
//! - JSON (`.json`)
//! - TOML (`.toml`)
//! - YAML (`.yaml`, `.yml`)
//!
//! Useful for AgilePlus spec files that may be stored in different formats.

pub mod dialect;
pub mod converter;
pub mod registry;
pub mod error;

pub use dialect::{Dialect, DialectType};
pub use converter::DialectConverter;
pub use registry::DialectRegistry;
pub use error::{XddError, XddResult};
