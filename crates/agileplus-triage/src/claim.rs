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
    /// Optional reason (e.g. `"task:abc-123"`, `"branch:feat/x"`).
    pub reason: Option<String>,
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
        reason: Option<String>,
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
