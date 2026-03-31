//! Hexkit - Hexagonal Architecture Helpers for AgilePlus
//!
//! This library provides utilities to reduce boilerplate when working with
//! hexagonal architecture patterns in AgilePlus projects.

pub mod builders;
pub mod defaults;
pub mod error;
pub mod registry;
pub mod helpers;

pub use builders::{PortBuilder, ContextBuilder};
pub use defaults::{DefaultPort, NoOpPort};
pub use registry::{PortRegistry, PortKey};
pub use helpers::{with_timeout, retry_on_failure};
pub use crate::error::{HexkitError, HexkitResult};
