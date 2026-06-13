//! agileplus-triage — rule-based triage engine for synced items.
//!
//! # Modules
//! - `engine`: hexagonal classify port — pure `classify(item, rules) -> TriageOutcome`
//! - `classifier`: free-text keyword classifier (legacy / internal use)
//! - `backlog`: in-memory backlog store
//! - `adapter`: high-level orchestration combining classifier + store
//! - `router`: governance file (CLAUDE.md / AGENTS.md) generator
//! - `dedup`: token-Jaccard, fuzzy ratio, simhash, n-gram, hybrid dedup scorer
//! - `claim`: resource claim primitives (repo/branch/worktree) with TTL + heartbeat
//! - `repo_introspect`: git / mangled / no-git repo classification
//!
//! Traceability: FR-AGP-017

pub mod adapter;
pub mod backlog;
pub mod claim;
pub mod classifier;
pub mod dedup;
pub mod engine;
pub mod repo_introspect;
pub mod router;

#[cfg(test)]
mod tests_dedup;

// Re-export the main engine surface so consumers can do:
//   use agileplus_triage::{SyncedItem, TriageRules, TriageOutcome, classify};
pub use engine::{classify, SyncedItem, TriageOutcome, TriageRule, TriageRules};

// Re-export BacklogStoreOps trait (used by adapter consumers)
pub use adapter::BacklogStoreOps;
