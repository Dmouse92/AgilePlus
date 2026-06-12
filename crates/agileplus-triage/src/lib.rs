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
//! - `minhash`: Broder MinHash signatures with k-permutation FNV-1a
//! - `bloom`: feature `bloom` — bitvec-backed Bloom filter for membership tests
//! - `embeddings`: `EmbeddingBackend` trait + `LocalMockEmbeddings`; feature `oai` adds
//!   `OaiEmbeddings` (api.openai.com), feature `voyage` adds `VoyageEmbeddings`
//! - `hybrid_pipeline`: MinHash-LSH candidate generation + embedding cosine
//!   verification + Jaccard tiebreak (`HybridDedup::build / find / run_dedup`)
//! - `ast_tokenize`: regex-based AST-aware tokenization for Rust and Python
//!
//! Traceability: FR-AGP-017, FR-AGP-018 (triage dedup primitives),
//! audit recs #1-#5 from `AUDIT_BLOC_VS_2026_SOTA.md`.

pub mod adapter;
pub mod ast_tokenize;
pub mod backlog;
pub mod bloom;
pub mod claim;
pub mod classifier;
pub mod dedup;
pub mod embeddings;
pub mod engine;
pub mod hybrid_pipeline;
pub mod minhash;
pub mod repo_introspect;
pub mod router;

#[cfg(test)]
mod tests_dedup;

// Re-export the main engine surface so consumers can do:
//   use agileplus_triage::{SyncedItem, TriageRules, TriageOutcome, classify};
pub use engine::{classify, SyncedItem, TriageOutcome, TriageRule, TriageRules};

// Re-export BacklogStoreOps trait (used by adapter consumers)
pub use adapter::BacklogStoreOps;
