//! Hexagonal architecture library for AgilePlus
//!
//! This library provides hexagonal architecture patterns including:
//! - Domain entities (Spec, WorkPackage, Task)
//! - Port traits (InputPort, OutputPort)
//! - Adapters (driven/driving)
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     Driving Adapters                        │
//! │   (CLI, gRPC, HTTP handlers calling InputPorts)           │
//! └─────────────────────┬─────────────────────────────────────┘
//!                       │
//! ┌─────────────────────▼─────────────────────────────────────┐
//! │                    Input Ports                              │
//! │   (Use cases / Application Services)                       │
//! └─────────────────────┬─────────────────────────────────────┘
//!                       │
//! ┌─────────────────────▼─────────────────────────────────────┐
//! │                   Domain Core                               │
//! │   (Entities, Value Objects, Domain Services)               │
//! └─────────────────────┬─────────────────────────────────────┘
//!                       │
//! ┌─────────────────────▼─────────────────────────────────────┐
//! │                   Output Ports                             │
//! │   (Repository interfaces, External service interfaces)     │
//! └─────────────────────┬─────────────────────────────────────┘
//!                       │
//! ┌─────────────────────▼─────────────────────────────────────┐
//! │                    Driven Adapters                          │
//! │   (SQLite, HTTP clients, File system)                      │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod domain;
pub mod ports;
pub mod adapters;
pub mod use_cases;

pub use domain::{Entity, EntityId, Spec, WorkPackage, Task, SpecStatus, TaskStatus};
pub use ports::{InputPort, OutputPort, SpecRepository, TaskRepository};
pub use adapters::{InMemorySpecRepository, InMemoryTaskRepository};
pub use use_cases::{CreateSpecUseCase, UpdateSpecUseCase, ListSpecsUseCase};
