//! Resource claim primitives with TTL and heartbeat.
//!
//! A claim is a record that an agent has exclusive ownership of a
//! resource (repo, branch, worktree, or subproject) for `ttl_seconds`
//! (default 3600). Agents must heartbeat before the TTL elapses or the
//! claim reverts to the global pool.
//!
//! Traceability: FR-AGP-019 (resource claim primitive)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of resource being claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ClaimKind {
    /// A repository (full directory).
    Repo,
    /// A git branch within a repo.
    Branch,
    /// A git worktree within a repo.
    Worktree,
    /// A subproject (logical grouping within a repo).
    Subproject,
}

/// State of a claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimState {
    /// Currently valid and held.
    Active,
    /// Grace period: TTL elapsed but a heartbeat was received; in reclamation.
    Draining,
    /// Returned to the global pool (expired or released).
    Expired,
}

/// Structured reason why a claim was taken.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum ClaimReason {
    /// Linked to a work-package id.
    #[serde(rename = "task_ref")]
    TaskRef(String),
    /// Linked to a branch (e.g. `feat/login`).
    #[serde(rename = "branch")]
    Branch(String),
    /// Linked to a subproject.
    #[serde(rename = "subproject")]
    Subproject(String),
    /// For ephemeral runs.
    #[serde(rename = "wip_run")]
    WipRun(String),
    /// Free-form human-entered reason.
    #[serde(rename = "manual")]
    Manual(String),
}

impl Default for ClaimReason {
    fn default() -> Self {
        ClaimReason::Manual(String::new())
    }
}

impl ClaimReason {
    /// Discriminator string used for SQL storage (e.g. `task_ref`).
    pub fn kind_str(&self) -> Option<String> {
        Some(
            match self {
                ClaimReason::TaskRef(_) => "task_ref",
                ClaimReason::Branch(_) => "branch",
                ClaimReason::Subproject(_) => "subproject",
                ClaimReason::WipRun(_) => "wip_run",
                ClaimReason::Manual(_) => "manual",
            }
            .to_string(),
        )
    }

    /// The inner value string.
    pub fn value(&self) -> Option<String> {
        Some(match self {
            ClaimReason::TaskRef(v)
            | ClaimReason::Branch(v)
            | ClaimReason::Subproject(v)
            | ClaimReason::WipRun(v)
            | ClaimReason::Manual(v) => v.clone(),
        })
    }
}

/// A claim record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub resource: String,
    pub kind: ClaimKind,
    pub agent_id: String,
    pub created_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
    pub ttl_seconds: i64,
    pub state: ClaimState,
    /// Structured reason why the claim was taken.
    pub reason: ClaimReason,
}

impl Claim {
    /// Seconds since last heartbeat; negative if in the future (clock skew).
    /// Truncated to whole seconds (see `is_expired` for sub-second precision).
    pub fn age_seconds(&self, now: DateTime<Utc>) -> i64 {
        (now - self.last_heartbeat).num_seconds()
    }

    /// True if TTL has elapsed (still recoverable via grace period).
    /// Uses millisecond precision so sub-second TTLs work reliably.
    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        (now - self.last_heartbeat).num_milliseconds() > self.ttl_seconds * 1000
    }
}

/// Errors that can occur during claim operations.
#[derive(Debug, thiserror::Error)]
pub enum ClaimError {
    #[error("claim not found: {0}")]
    NotFound(String),

    #[error("wrong owner: expected {expected}, actual {actual}")]
    WrongOwner { expected: String, actual: String },

    #[error("wrong state")]
    WrongState,
}

/// Common interface for in-memory and persistent claim stores.
pub trait ClaimStoreTrait {
    /// Issue a new claim. Returns `None` if the resource is already
    /// actively claimed by a different claim id.
    fn claim(
        &mut self,
        id: &str,
        resource: &str,
        kind: ClaimKind,
        agent: &str,
        ttl_seconds: i64,
        reason: ClaimReason,
    ) -> Option<Claim>;

    /// Refresh `last_heartbeat` to now. Returns `true` if the claim was
    /// found and refreshed.
    fn heartbeat(&mut self, id: &str) -> bool;

    /// Release a claim (active release by the owner). Returns `true` if
    /// the claim was found and released.
    fn release(&mut self, id: &str) -> bool;

    /// Reap all claims whose TTL has elapsed. Removes them from the store
    /// and frees the resource. Returns the number of reaped claims.
    fn reap_expired(&mut self, now: DateTime<Utc>) -> usize;

    /// All claims regardless of state.
    fn all(&self) -> Vec<Claim>;

    /// Active claims only.
    fn active(&self) -> Vec<Claim>;

    /// Lookup by resource.
    fn lookup(&self, kind: ClaimKind, resource: &str) -> Option<Claim>;

    /// Transfer a claim: mark the old claim as `Draining`, create a new
    /// claim with `to_id` and `to_agent`, inheriting resource+kind+ttl+reason.
    fn claim_transfer(
        &mut self,
        from_id: &str,
        to_id: &str,
        to_agent: &str,
    ) -> Result<Claim, ClaimError>;
}

/// In-memory store of claims. Use external locking for thread-safety in
/// production deployments.
#[derive(Debug, Default, Clone)]
pub struct ClaimStore {
    claims: HashMap<String, Claim>,
    by_resource: HashMap<(ClaimKind, String), String>,
}

impl ClaimStore {
    /// New empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// All claims regardless of state.
    pub fn all(&self) -> Vec<Claim> {
        self.claims.values().cloned().collect()
    }

    /// Active claims only.
    pub fn active(&self) -> Vec<Claim> {
        self.claims
            .values()
            .filter(|c| c.state == ClaimState::Active)
            .cloned()
            .collect()
    }

    /// Issue a new claim. Returns `None` if the resource is already
    /// actively claimed by a different claim id.
    pub fn claim(
        &mut self,
        id: &str,
        resource: &str,
        kind: ClaimKind,
        agent: &str,
        ttl_seconds: i64,
        reason: ClaimReason,
    ) -> Option<Claim> {
        let resource_key = (kind, resource.to_string());
        if let Some(existing_id) = self.by_resource.get(&resource_key) {
            if existing_id != id
                && self
                    .claims
                    .get(existing_id)
                    .map(|c| c.state == ClaimState::Active)
                    .unwrap_or(false)
            {
                return None;
            }
        }
        let now = Utc::now();
        let c = Claim {
            id: id.to_string(),
            resource: resource.to_string(),
            kind,
            agent_id: agent.to_string(),
            created_at: now,
            last_heartbeat: now,
            ttl_seconds,
            state: ClaimState::Active,
            reason,
        };
        self.claims.insert(id.to_string(), c.clone());
        self.by_resource.insert(resource_key, id.to_string());
        Some(c)
    }

    /// Refresh `last_heartbeat` to now. Returns `true` if the claim was
    /// found and refreshed.
    pub fn heartbeat(&mut self, id: &str) -> bool {
        if let Some(c) = self.claims.get_mut(id) {
            c.last_heartbeat = Utc::now();
            true
        } else {
            false
        }
    }

    /// Release a claim (active release by the owner). Returns `true` if
    /// the claim was found and released.
    pub fn release(&mut self, id: &str) -> bool {
        if let Some(c) = self.claims.remove(id) {
            self.by_resource.remove(&(c.kind, c.resource));
            true
        } else {
            false
        }
    }

    /// Reap all claims whose TTL has elapsed. Removes them from the store
    /// and frees the resource. Returns the number of reaped claims.
    pub fn reap_expired(&mut self, now: DateTime<Utc>) -> usize {
        let mut reaped = 0;
        let expired: Vec<String> = self
            .claims
            .iter()
            .filter(|(_, c)| c.is_expired(now))
            .map(|(id, _)| id.clone())
            .collect();
        for id in expired {
            if let Some(c) = self.claims.remove(&id) {
                self.by_resource.remove(&(c.kind, c.resource));
                reaped += 1;
            }
        }
        reaped
    }

    /// Lookup by resource.
    pub fn lookup(&self, kind: ClaimKind, resource: &str) -> Option<Claim> {
        self.by_resource
            .get(&(kind, resource.to_string()))
            .and_then(|id| self.claims.get(id))
            .cloned()
    }
}

impl ClaimStoreTrait for ClaimStore {
    fn claim(
        &mut self,
        id: &str,
        resource: &str,
        kind: ClaimKind,
        agent: &str,
        ttl_seconds: i64,
        reason: ClaimReason,
    ) -> Option<Claim> {
        self.claim(id, resource, kind, agent, ttl_seconds, reason)
    }

    fn heartbeat(&mut self, id: &str) -> bool {
        self.heartbeat(id)
    }

    fn release(&mut self, id: &str) -> bool {
        self.release(id)
    }

    fn reap_expired(&mut self, now: DateTime<Utc>) -> usize {
        self.reap_expired(now)
    }

    fn all(&self) -> Vec<Claim> {
        self.all()
    }

    fn active(&self) -> Vec<Claim> {
        self.active()
    }

    fn lookup(&self, kind: ClaimKind, resource: &str) -> Option<Claim> {
        self.lookup(kind, resource)
    }

    fn claim_transfer(
        &mut self,
        from_id: &str,
        to_id: &str,
        to_agent: &str,
    ) -> Result<Claim, ClaimError> {
        let mut old = self
            .claims
            .get(from_id)
            .cloned()
            .ok_or_else(|| ClaimError::NotFound(from_id.to_string()))?;
        if old.state != ClaimState::Active {
            return Err(ClaimError::WrongState);
        }
        // (a) mark the old claim as Draining.
        old.state = ClaimState::Draining;
        self.claims.insert(from_id.to_string(), old.clone());
        // (b) create a new claim inheriting everything.
        let now = Utc::now();
        let new_claim = Claim {
            id: to_id.to_string(),
            resource: old.resource.clone(),
            kind: old.kind,
            agent_id: to_agent.to_string(),
            created_at: now,
            last_heartbeat: now,
            ttl_seconds: old.ttl_seconds,
            state: ClaimState::Active,
            reason: old.reason.clone(),
        };
        self.claims.insert(to_id.to_string(), new_claim.clone());
        self.by_resource
            .insert((new_claim.kind, new_claim.resource.clone()), to_id.to_string());
        // (c) return the new claim.
        Ok(new_claim)
    }
}
