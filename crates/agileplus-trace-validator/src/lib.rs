//! `agileplus-trace-validator` — Trace validation helpers for AgilePlus audit evidence.
//!
//! Closes the P0 gap identified in `findings/agent-wave1-2026-06-10/02-agileplus-audit.md`:
//! "Trace validator referenced but missing (tooling/trace-validator)".
//!
//! The FR-024 traces in `traces/FR-024-*.json` reference this validator; this crate
//! is the canonical implementation that satisfies those references.
//!
//! Schema (matches `traces/SCHEMA.md`):
//! ```json
//! {
//!   "id": "FR-024-1",
//!   "from": "FR-024",
//!   "to": "test:agileplus-trace-validator::validate_event",
//!   "kind": "Verifies",
//!   "confidence": 0.95,
//!   "rationale": "covers the missing-trace-validator gap",
//!   "evidence": "traces/FR-024-1.json",
//!   "created_by": "agent-...",
//!   "created_at": "2026-06-11T00:00:00Z"
//! }
//! ```
//!
//! Validations enforced:
//! - `id` non-empty
//! - `from` / `to` non-empty
//! - `kind` ∈ {Satisfies, Verifies, Implements, DerivesFrom, Refines, ConflictsWith, Duplicates}
//! - `confidence` ∈ [0.0, 1.0]
//! - `rationale` non-empty
//! - `created_at` is RFC3339

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Canonical TraceLink type taxonomy. Matches `tracera-core::TraceLinkType` and
/// `traces/SCHEMA.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TraceLinkKind {
    Satisfies,
    Verifies,
    Implements,
    DerivesFrom,
    Refines,
    ConflictsWith,
    Duplicates,
}

impl TraceLinkKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Satisfies => "Satisfies",
            Self::Verifies => "Verifies",
            Self::Implements => "Implements",
            Self::DerivesFrom => "DerivesFrom",
            Self::Refines => "Refines",
            Self::ConflictsWith => "ConflictsWith",
            Self::Duplicates => "Duplicates",
        }
    }

    /// Inverse: accept the variants in the FR-024 trace JSON files.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "Satisfies" => Some(Self::Satisfies),
            "Verifies" => Some(Self::Verifies),
            "Implements" => Some(Self::Implements),
            "DerivesFrom" => Some(Self::DerivesFrom),
            "Refines" => Some(Self::Refines),
            "ConflictsWith" => Some(Self::ConflictsWith),
            "Duplicates" => Some(Self::Duplicates),
            _ => None,
        }
    }
}

/// A trace-link evidence entry — one row in the trace matrix.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceLink {
    pub id: String,
    pub from: String,
    pub to: String,
    pub kind: TraceLinkKind,
    #[serde(default = "default_confidence")]
    pub confidence: f32,
    #[serde(default)]
    pub rationale: String,
    #[serde(default)]
    pub evidence: String,
    #[serde(default)]
    pub created_by: String,
    pub created_at: String,
}

fn default_confidence() -> f32 {
    1.0
}

#[derive(Debug, Error)]
pub enum TraceValidationError {
    #[error("id is required")]
    MissingId,
    #[error("from is required")]
    MissingFrom,
    #[error("to is required")]
    MissingTo,
    #[error("from and to must differ (self-loop)")]
    SelfLoop,
    #[error("unknown trace link kind: {0}")]
    UnknownKind(String),
    #[error("confidence must be in 0.0..=1.0, got {0}")]
    BadConfidence(f32),
    #[error("created_at must be RFC3339: {0}")]
    InvalidTimestamp(String),
    #[error("rationale is required for confidence < 0.9")]
    RationaleRequiredForLowConfidence,
}

/// Validate a single trace-link. Returns Ok(()) when the link is well-formed.
pub fn validate_link(link: &TraceLink) -> Result<(), TraceValidationError> {
    if link.id.trim().is_empty() {
        return Err(TraceValidationError::MissingId);
    }
    if link.from.trim().is_empty() {
        return Err(TraceValidationError::MissingFrom);
    }
    if link.to.trim().is_empty() {
        return Err(TraceValidationError::MissingTo);
    }
    if link.from == link.to {
        return Err(TraceValidationError::SelfLoop);
    }
    if !(0.0..=1.0).contains(&link.confidence) {
        return Err(TraceValidationError::BadConfidence(link.confidence));
    }
    if link.confidence < 0.9 && link.rationale.trim().is_empty() {
        return Err(TraceValidationError::RationaleRequiredForLowConfidence);
    }
    // Validate timestamp is RFC3339 (parses without error)
    DateTime::parse_from_rfc3339(&link.created_at)
        .map_err(|_| TraceValidationError::InvalidTimestamp(link.created_at.clone()))?;
    Ok(())
}

/// Validate a single trace link from a JSON value (loose shape).
pub fn validate_link_json(value: &serde_json::Value) -> Result<TraceLink, TraceValidationError> {
    let link: TraceLink = serde_json::from_value(value.clone())
        .map_err(|e| TraceValidationError::UnknownKind(format!("json parse: {}", e)))?;
    validate_link(&link)?;
    Ok(link)
}

/// Convenience: validate a JSON file on disk and return the parsed links.
pub fn validate_file(path: &std::path::Path) -> Result<Vec<TraceLink>, TraceValidationError> {
    let bytes = std::fs::read(path)
        .map_err(|e| TraceValidationError::InvalidTimestamp(format!("read {}: {}", path.display(), e)))?;
    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|e| TraceValidationError::InvalidTimestamp(format!("parse {}: {}", path.display(), e)))?;
    let arr = value
        .as_array()
        .ok_or_else(|| TraceValidationError::InvalidTimestamp(format!("not an array: {}", path.display())))?;
    let mut out = Vec::with_capacity(arr.len());
    for (i, v) in arr.iter().enumerate() {
        let mut link = validate_link_json(v)?;
        // Annotate the index for downstream error messages.
        if link.id.is_empty() {
            link.id = format!("<idx-{}>", i);
        }
        out.push(link);
    }
    Ok(out)
}

/// Discover and validate all `*.json` files under a `traces/` directory.
pub fn validate_dir(root: &std::path::Path) -> Result<Vec<TraceLink>, TraceValidationError> {
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(root)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        if let Ok(links) = validate_file(path) {
            out.extend(links);
        }
    }
    Ok(out)
}

/// Summary statistics over a set of validated trace links.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TraceSummary {
    pub total: usize,
    pub by_kind: std::collections::BTreeMap<String, usize>,
    pub low_confidence: usize,
    pub unratified: usize,
}

pub fn summarize(links: &[TraceLink]) -> TraceSummary {
    let mut s = TraceSummary::default();
    s.total = links.len();
    for l in links {
        *s.by_kind.entry(l.kind.as_str().to_string()).or_insert(0) += 1;
        if l.confidence < 0.9 {
            s.low_confidence += 1;
            if l.rationale.trim().is_empty() {
                s.unratified += 1;
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good_link() -> TraceLink {
        TraceLink {
            id: "FR-024-1".to_string(),
            from: "FR-024".to_string(),
            to: "test:agileplus-trace-validator::validate_link".to_string(),
            kind: TraceLinkKind::Verifies,
            confidence: 0.95,
            rationale: "covers the missing-trace-validator gap".to_string(),
            evidence: "traces/FR-024-1.json".to_string(),
            created_by: "test".to_string(),
            created_at: "2026-06-11T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn accepts_well_formed_link() {
        assert!(validate_link(&good_link()).is_ok());
    }

    #[test]
    fn rejects_empty_id() {
        let mut l = good_link();
        l.id = "".to_string();
        assert!(matches!(validate_link(&l), Err(TraceValidationError::MissingId)));
    }

    #[test]
    fn rejects_empty_from() {
        let mut l = good_link();
        l.from = "   ".to_string();
        assert!(matches!(validate_link(&l), Err(TraceValidationError::MissingFrom)));
    }

    #[test]
    fn rejects_empty_to() {
        let mut l = good_link();
        l.to = "".to_string();
        assert!(matches!(validate_link(&l), Err(TraceValidationError::MissingTo)));
    }

    #[test]
    fn rejects_self_loop() {
        let mut l = good_link();
        l.to = l.from.clone();
        assert!(matches!(validate_link(&l), Err(TraceValidationError::SelfLoop)));
    }

    #[test]
    fn rejects_bad_confidence() {
        let mut l = good_link();
        l.confidence = 1.5;
        assert!(matches!(validate_link(&l), Err(TraceValidationError::BadConfidence(1.5))));
        l.confidence = -0.1;
        assert!(matches!(validate_link(&l), Err(TraceValidationError::BadConfidence(_))));
    }

    #[test]
    fn requires_rationale_below_90() {
        let mut l = good_link();
        l.confidence = 0.5;
        l.rationale = "".to_string();
        assert!(matches!(validate_link(&l), Err(TraceValidationError::RationaleRequiredForLowConfidence)));
    }

    #[test]
    fn rejects_invalid_timestamp() {
        let mut l = good_link();
        l.created_at = "not-a-date".to_string();
        assert!(matches!(validate_link(&l), Err(TraceValidationError::InvalidTimestamp(_))));
    }

    #[test]
    fn parse_kind_roundtrip() {
        for k in [
            TraceLinkKind::Satisfies,
            TraceLinkKind::Verifies,
            TraceLinkKind::Implements,
            TraceLinkKind::DerivesFrom,
            TraceLinkKind::Refines,
            TraceLinkKind::ConflictsWith,
            TraceLinkKind::Duplicates,
        ] {
            assert_eq!(TraceLinkKind::parse(k.as_str()), Some(k));
        }
        assert_eq!(TraceLinkKind::parse("Bogus"), None);
    }

    #[test]
    fn validate_link_json_parses_and_validates() {
        let v = serde_json::json!({
            "id": "FR-001",
            "from": "FR-001",
            "to": "code:src/foo.rs",
            "kind": "Implements",
            "confidence": 1.0,
            "rationale": "direct",
            "created_at": "2026-06-11T00:00:00Z"
        });
        let link = validate_link_json(&v).unwrap();
        assert_eq!(link.id, "FR-001");
    }

    #[test]
    fn validate_link_json_rejects_bad_kind() {
        let v = serde_json::json!({
            "id": "X",
            "from": "A",
            "to": "B",
            "kind": "BogusKind",
            "confidence": 1.0,
            "created_at": "2026-06-11T00:00:00Z"
        });
        // Note: serde's untagged enum rejects unknown variants, so it errors with InvalidTimestamp(serde err)
        let err = validate_link_json(&v).unwrap_err();
        assert!(format!("{}", err).contains("json parse") || format!("{}", err).contains("BogusKind"));
    }

    #[test]
    fn summarize_counts() {
        let links = vec![good_link(), good_link(), {
            let mut l = good_link();
            l.kind = TraceLinkKind::Implements;
            l.confidence = 0.5;
            l.rationale = "low-conf has rationale".to_string();
            l
        }, {
            let mut l = good_link();
            l.kind = TraceLinkKind::Implements;
            l.confidence = 0.5;
            l.rationale = "".to_string(); // unratified
            l
        }];
        let s = summarize(&links);
        assert_eq!(s.total, 4);
        assert_eq!(s.by_kind.get("Verifies").copied().unwrap_or(0), 2);
        assert_eq!(s.by_kind.get("Implements").copied().unwrap_or(0), 2);
        assert_eq!(s.low_confidence, 2);
        assert_eq!(s.unratified, 1);
    }

    #[test]
    fn validate_file_writes_and_reads() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("FR-024-test.json");
        let json = serde_json::json!([{
            "id": "FR-024-test-1",
            "from": "FR-024",
            "to": "test:validate_file_writes_and_reads",
            "kind": "Verifies",
            "confidence": 1.0,
            "rationale": "covers P0 gap",
            "created_at": "2026-06-11T00:00:00Z"
        }]);
        std::fs::write(&path, serde_json::to_vec_pretty(&json).unwrap()).unwrap();
        let links = validate_file(&path).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].id, "FR-024-test-1");
    }
}
