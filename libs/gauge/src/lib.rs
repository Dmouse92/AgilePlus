//! Performance measurement library for AgilePlus
//!
//! Provides utilities for measuring:
//! - Operation latency
//! - Throughput (requests/second)
//! - Memory usage
//! - Benchmarking

pub mod gauge;
pub mod metrics;
pub mod benchmark;

pub use gauge::{Gauge, ScopedGauge};
pub use metrics::{MetricsCollector, MetricValue, MetricSnapshot};
pub use benchmark::{benchmark, BenchmarkResult};
