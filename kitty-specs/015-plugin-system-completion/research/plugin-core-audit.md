# Plugin Core Audit — T001 & T002

**Date**: 2026-05-04
**Spec**: 015-plugin-system-completion
**Work Package**: WP-001
**Tasks**: T001 (Audit), T002 (Design)

---

## T001: Audit Summary — Existing Plugin Interfaces

### 1. Repository Location

The `agileplus-plugin-core` repository referenced in the spec **does not exist** as a standalone GitHub repo (`gh api repos/KooshaPari/agileplus-plugin-core` returns 404). The local directories at `pheno/agileplus-plugin-core` and `HexaKit/agileplus-plugin-core` are empty (0 files).

**The actual implementation lives in the `PhenoPlugins` monorepo:**
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoPlugins/crates/pheno-plugin-core/`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoPlugins/crates/pheno-plugin-git/`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoPlugins/crates/pheno-plugin-sqlite/`

Additionally, there are two other relevant plugin infrastructure pieces:
- `/Users/kooshapari/CodeProjects/Phenotype/repos/libs/plugin-registry/` — standalone `Plugin` trait + `PluginRegistry` (async, tokio-based)
- `/Users/kooshapari/CodeProjects/Phenotype/repos/libs/plugin-integration/` — bridge layer wrapping `plugin-registry` into a `UnifiedPluginRegistry`
- `/Users/kooshapari/CodeProjects/Phenotype/repos/FocalPoint/crates/focus-plugin-sdk/` — `ConnectorPlugin` trait for WASM-based plugins (wasmtime)
- `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/crates/thegent-plugin-host/` — Go-based plugin host with `PluginLoaderPort` and `PluginExecutionPort`

### 2. What Traits Exist

#### 2a. `pheno-plugin-core` — Hexagonal Architecture Traits (Rust)

**File**: `crates/pheno-plugin-core/src/traits.rs`

| Trait | Extends | Sync/Async | Purpose |
|-------|---------|------------|---------|
| `AdapterPlugin` | `Send + Sync` | Sync | Base trait for all plugins: `name()`, `version()`, `initialize(config)`, `health_check()` |
| `VcsPlugin` | `AdapterPlugin` | Async (`#[async_trait]`) | Git VCS operations: worktree CRUD, branch ops, merge, artifact read/write/scan |
| `StoragePlugin` | `AdapterPlugin` | Async (`#[async_trait]`) | Database CRUD: features, work packages, audit entries |

**Supporting types** (all in `traits.rs`):
- `PluginConfig` — `{ name, version, adapter_config: serde_json::Value }`
- `WorktreeInfo` — `{ path, branch, feature_slug, wp_id }`
- `MergeResult` — `{ success, conflicts, merged_commit }`
- `ConflictInfo` — `{ path, ours, theirs }`
- `FeatureArtifacts` — `{ meta_json, audit_chain, evidence_paths }`

**File**: `crates/pheno-plugin-core/src/error.rs`

`PluginError` enum (9 variants):
- `Initialization`, `NotFound`, `AlreadyRegistered`, `AlreadyExists`, `Operation`, `Config`, `Io`, `Serialization`, `Execution`, `Validation`

**File**: `crates/pheno-plugin-core/src/registry.rs`

`PluginRegistry` struct:
- Uses `RwLock<HashMap<String, Arc<dyn VcsPlugin>>>` and `RwLock<HashMap<String, Arc<dyn StoragePlugin>>>`
- Methods: `new()`, `finalize()`, `is_finalized()`, `register_vcs()`, `vcs()`, `vcs_adapters()`, `register_storage()`, `storage()`, `storage_adapters()`, `health_check()`, `stats()`
- **Finalization pattern**: After `finalize()`, no new plugins can be registered
- Thread-safe via `Arc<RwLock<>>` interior mutability

#### 2b. `libs/plugin-registry` — Standalone Plugin Registry (Rust)

**File**: `libs/plugin-registry/src/plugin_trait.rs`

| Trait | Extends | Sync/Async | Purpose |
|-------|---------|------------|---------|
| `Plugin` | `Send + Sync` | Async (`#[async_trait]`) | Minimal lifecycle: `name()`, `version()`, `metadata()`, `initialize(config)`, `shutdown()` |

**Supporting types**:
- `PluginConfig` — `{ config: serde_json::Value, data_dir, host_version }`
- `PluginMetadata` — `{ name, version, min_host_version, description }`

**File**: `libs/plugin-registry/src/registry.rs`

`PluginRegistry` struct:
- Uses `Arc<RwLock<HashMap<String, Arc<dyn Plugin>>>>`
- Methods: `new()`, `len()`, `is_empty()`, `list_plugins()`, `get_all_metadata()`, `load()`, `initialize()`, `shutdown()`, `unload()`, `get()`
- **Version checking**: `initialize()` checks `min_host_version` against `host_version`
- Full lifecycle: load -> initialize -> shutdown -> unload
- Has unit tests (4 tests covering load/unload, duplicate, nonexistent, get)

#### 2c. `libs/plugin-integration` — Bridge Layer (Rust)

`UnifiedPluginRegistry` wraps `PluginRegistry` and provides:
- `load_plugin()` (load + initialize in one call)
- `list_plugins()`, `get_plugin()`, `shutdown_plugin()`, `unload_plugin()`

#### 2d. `focus-plugin-sdk` — WASM Plugin Trait (Rust)

**File**: `FocalPoint/crates/focus-plugin-sdk/src/plugin.rs`

| Trait | Extends | Sync/Async | Purpose |
|-------|---------|------------|---------|
| `ConnectorPlugin` | `Send + Sync` | Sync | WASM connector: `poll(config) -> Result<Vec<u8>, String>` returning NDJSON |

Uses wasmtime v44 for WASM execution.

#### 2e. `thegent-plugin-host` — Go Plugin Host

**File**: `thegent/crates/thegent-plugin-host/src/ports.rs`

| Trait | Extends | Purpose |
|-------|---------|---------|
| `PluginLoaderPort` | `Send + Sync + Debug` | `can_load(path)`, `load(manifest, path)` |
| `PluginExecutionPort` | `Send + Sync + Debug` | `execute(plugin, input)`, `is_available()` |

### 3. What's Implemented vs. Stubbed

#### `pheno-plugin-git` — IMPLEMENTED (with gaps)

**Status**: Functional implementation using git2.

Implemented:
- `GitAdapter` struct with `new()`, `from_cwd()`, `repo_path()`, `main_branch_name()`
- `AdapterPlugin` impl: `name()`, `version()`, `initialize()`
- `VcsPlugin` impl: All 12 methods implemented:
  - `create_worktree()` — creates branch + initializes new repo at `.worktrees/{wp_id}`
  - `list_worktrees()` — lists git worktrees, parses feature_slug/wp_id from names
  - `cleanup_worktree()` — prunes worktree + removes directory
  - `create_branch()` — creates branch from base
  - `checkout_branch()` — force checkout
  - `merge_to_target()` — full merge with conflict detection + commit
  - `detect_conflicts()` — diff-tree-based conflict detection
  - `read_artifact()`, `write_artifact()`, `artifact_exists()` — filesystem-based
  - `scan_feature_artifacts()` — scans `kitty-specs/{feature_slug}/`
- Unit tests: name/version test, artifact read/write test

**Gaps**:
- `create_worktree()` uses `Repository::init` (not `repo.add_worktree()`) — creates a separate repo clone, not a true git worktree
- `checkout_branch()` only calls `checkout_head()` without setting HEAD to the branch ref first
- `detect_conflicts()` returns all diff'd files as conflicts, not actual merge conflicts
- No authentication support (SSH keys, HTTPS tokens)
- No commit/push/fetch operations (clone, fetch, commit, push are in spec but not implemented)
- `Send`/`Sync` implemented via `unsafe impl` (should be safe since `PathBuf` is Send+Sync, but compiler can't auto-derive due to git2's raw pointers)
- Test coverage limited (2 integration tests)

#### `pheno-plugin-sqlite` — IMPLEMENTED (with gaps)

**Status**: Functional implementation using rusqlite.

Implemented:
- `SqliteStoragePlugin` struct with `new(path)`, `in_memory()`, `connection()`, `db_path()`
- WAL mode + foreign keys enabled
- Schema migration: creates `features`, `work_packages`, `audit_entries`, `plugin_metadata` tables
- `AdapterPlugin` impl: `name()`, `version()`, `initialize()`
- `StoragePlugin` impl: All 9 methods implemented:
  - `create_feature()`, `get_feature_by_slug()`, `get_feature_by_id()`, `update_feature_state()`, `list_all_features()`
  - `create_work_package()`, `get_work_package()`, `update_wp_state()`
  - `append_audit_entry()`, `get_audit_trail()`
- Unit tests: init test, feature CRUD, work package CRUD, audit CRUD

**Gaps**:
- No migration versioning (just `CREATE TABLE IF NOT EXISTS` — no up/down migration support)
- No transaction support (begin/commit/rollback exposed to callers)
- No backup operation
- No pagination/sorting on query methods
- Uses `std::sync::Mutex` (not async-aware) — blocks async runtime
- No connection pooling

#### `pheno-plugin-core` registry — IMPLEMENTED (with gaps)

**Status**: Functional but uses synchronous `RwLock` (not async).

**Gaps**:
- No async support (uses `std::sync::RwLock`, not `tokio::sync::RwLock`)
- No plugin discovery (no filesystem scanning)
- No dynamic loading (no `libloading` or dlopen)
- No version compatibility checking between plugin and host
- No `unload()` method (plugins can be registered but not removed)
- No `shutdown()` or graceful teardown for individual plugins
- `health_check()` iterates all plugins but doesn't aggregate results per-plugin

#### `libs/plugin-registry` — IMPLEMENTED (more complete lifecycle)

**Status**: More mature than pheno-plugin-core registry. Async-first with tokio.

**Gaps**:
- No plugin discovery
- No dynamic loading
- No TOML config validation

#### `thegent-plugin-host` — STUB (Go)

**Status**: Port traits defined but no implementation found in the Rust FFI layer.

### 4. Key Architectural Inconsistencies

1. **Two competing `Plugin` traits**: `pheno-plugin-core::traits::AdapterPlugin` vs `plugin-registry::plugin_trait::Plugin`. The `plugin-integration` crate attempts to bridge these but the bridge is incomplete.

2. **Two competing `PluginConfig` structs**: Different field names and semantics between the two crates.

3. **Two competing `PluginError` enums**: `pheno_plugin_core::error::PluginError` vs `plugin_registry::error::PluginError`.

4. **Two competing `PluginRegistry` structs**: `pheno_plugin_core::registry::PluginRegistry` (sync, type-specific maps) vs `plugin_registry::registry::PluginRegistry` (async, generic `dyn Plugin` map).

5. **Naming mismatch**: The spec references `agileplus-plugin-core` but the actual crate is named `pheno-plugin-core`. The `agileplus-plugin-core` repo doesn't exist.

6. **Sync vs Async**: `pheno-plugin-core` uses sync `RwLock` while `plugin-registry` uses async `tokio::sync::RwLock`. The `VcsPlugin`/`StoragePlugin` traits use `#[async_trait]` but the registry itself is sync.

---

## T002: Proposed Stable Plugin Trait Design

### Design Principles

1. **Single unified `Plugin` trait** — merge the two competing traits into one canonical interface
2. **Async-first** — use `tokio::sync::RwLock` throughout; all trait methods are async
3. **Version compatibility** — semver-based with min/max host version checking
4. **Lifecycle completeness** — full load -> init -> start -> stop -> unload cycle
5. **Type erasure via `dyn Plugin`** — single registry for all plugin types, with downcasting for type-specific operations
6. **TOML-based config** — validated at load time

### Proposed Core Trait

```rust
/// Core trait that all AgilePlus plugins must implement.
///
/// This trait provides the minimum interface for plugin lifecycle management.
/// Domain-specific capabilities (VCS, Storage, LLM) are provided by
/// separate supertraits.
#[async_trait::async_trait]
pub trait Plugin: Send + Sync + 'static {
    /// Returns the plugin name (e.g., "git", "sqlite").
    /// Must be unique across all loaded plugins.
    fn name(&self) -> &str;

    /// Returns the plugin version (semver).
    fn version(&self) -> &str;

    /// Returns the minimum host API version this plugin requires.
    /// The registry will reject plugins whose min_host_version exceeds
    /// the host's current API version.
    fn min_host_version(&self) -> Option<&str> {
        None
    }

    /// Returns the maximum host API version this plugin is compatible with.
    /// If `None`, no upper bound is enforced.
    fn max_host_version(&self) -> Option<&str> {
        None
    }

    /// Returns plugin metadata for display and discovery.
    fn metadata(&self) -> PluginMetadata;

    /// Initialize the plugin with the given configuration.
    ///
    /// Called after the plugin is loaded but before it is started.
    /// Use this for one-time setup that doesn't require runtime resources.
    ///
    /// # Errors
    /// Returns `PluginError::Initialization` if setup fails.
    async fn initialize(&self, config: PluginConfig) -> Result<(), PluginError>;

    /// Start the plugin and begin serving requests.
    ///
    /// Called after `initialize()` succeeds. The plugin should acquire
    /// any runtime resources (connections, threads, etc.) here.
    ///
    /// # Errors
    /// Returns `PluginError::Lifecycle` if the plugin cannot start.
    async fn start(&self) -> Result<(), PluginError>;

    /// Stop the plugin gracefully.
    ///
    /// The plugin should release all runtime resources and stop serving
    /// requests. The plugin remains loaded and can be restarted.
    ///
    /// # Errors
    /// Returns `PluginError::Lifecycle` if graceful shutdown fails.
    async fn stop(&self) -> Result<(), PluginError>;

    /// Perform a health check.
    ///
    /// Returns `Ok(())` if the plugin is healthy, or an error describing
    /// the issue. Called periodically by the registry.
    async fn health_check(&self) -> Result<(), PluginError> {
        Ok(())
    }

    /// Shut down the plugin and release all resources.
    ///
    /// Called before the plugin is unloaded. After this returns,
    /// the plugin instance is dropped.
    async fn shutdown(&self) -> Result<(), PluginError>;
}
```

### Proposed Lifecycle State Machine

```
                  ┌──────────┐
                  │ UNLOADED │
                  └────┬─────┘
                       │ load()
                       ▼
                  ┌──────────┐
                  │ LOADING  │
                  └────┬─────┘
                       │ initialize()
                       ▼
                  ┌──────────┐
        ┌────────│  LOADED  │────────┐
        │        └────┬─────┘        │
        │             │ start()      │ stop()
        │             ▼              │
        │        ┌──────────┐        │
        │        │ RUNNING  │────────┘
        │        └────┬─────┘
        │             │ error
        │             ▼
        │        ┌──────────┐
        └───────│  ERROR   │
         retry  └────┬─────┘
                    │ shutdown()
                    ▼
               ┌──────────┐
               │UNLOADING │
               └────┬─────┘
                    │ unload()
                    ▼
               ┌──────────┐
               │ UNLOADED │
               └──────────┘
```

States: `Unloaded` -> `Loading` -> `Loaded` -> `Running` -> `Stopping` -> `Loaded` (cycle) or `Error` -> `Unloading` -> `Unloaded`

### Proposed Supertraits for Domain-Specific Plugins

```rust
/// VCS plugin — extends base Plugin with version control operations.
#[async_trait::async_trait]
pub trait VcsPlugin: Plugin {
    async fn clone(&self, url: &str, path: &Path) -> Result<(), PluginError>;
    async fn fetch(&self, remote: &str) -> Result<(), PluginError>;
    async fn commit(&self, message: &str) -> Result<String, PluginError>;
    async fn push(&self, remote: &str, branch: &str) -> Result<(), PluginError>;
    async fn create_branch(&self, name: &str, base: &str) -> Result<(), PluginError>;
    async fn checkout_branch(&self, name: &str) -> Result<(), PluginError>;
    async fn merge(&self, source: &str, target: &str) -> Result<MergeResult, PluginError>;
    async fn create_worktree(&self, feature: &str, wp: &str) -> Result<PathBuf, PluginError>;
    async fn list_worktrees(&self) -> Result<Vec<WorktreeInfo>, PluginError>;
    async fn cleanup_worktree(&self, path: &Path) -> Result<(), PluginError>;
}

/// Storage plugin — extends base Plugin with persistence operations.
#[async_trait::async_trait]
pub trait StoragePlugin: Plugin {
    async fn migrate(&self, direction: MigrationDirection) -> Result<(), PluginError>;
    async fn transaction<F, T>(&self, f: F) -> Result<T, PluginError>
    where
        F: FnOnce(&Transaction) -> Result<T, PluginError> + Send;
    async fn backup(&self, path: &Path) -> Result<(), PluginError>;
    // CRUD operations for domain types...
}
```

### Proposed PluginRegistry Design

```rust
/// Thread-safe plugin registry with full lifecycle management.
pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, PluginEntry>>>,
    config: RegistryConfig,
}

struct PluginEntry {
    plugin: Arc<dyn Plugin>,
    state: PluginState,
    config: PluginConfig,
}

enum PluginState {
    Loading,
    Loaded,
    Running,
    Stopping,
    Error(String),
    Unloading,
}

impl PluginRegistry {
    pub fn new(config: RegistryConfig) -> Self;

    /// Discover plugins in configured directories.
    pub async fn discover(&self) -> Result<Vec<PluginManifest>, PluginError>;

    /// Load and initialize a plugin.
    pub async fn load(&self, manifest: PluginManifest) -> Result<(), PluginError>;

    /// Start a loaded plugin.
    pub async fn start(&self, name: &str) -> Result<(), PluginError>;

    /// Stop a running plugin.
    pub async fn stop(&self, name: &str) -> Result<(), PluginError>;

    /// Shutdown and unload a plugin.
    pub async fn unload(&self, name: &str) -> Result<(), PluginError>;

    /// Get a running plugin by name.
    pub async fn get(&self, name: &str) -> Result<Arc<dyn Plugin>, PluginError>;

    /// Get a typed reference to a plugin (downcast).
    pub async fn get_as<T: Plugin + 'static>(&self, name: &str) -> Result<Arc<T>, PluginError>;

    /// List all plugins and their states.
    pub async fn list(&self) -> Vec<PluginInfo>;

    /// Health check all running plugins.
    pub async fn health_check_all(&self) -> HashMap<String, Result<(), PluginError>>;
}
```

### Version Compatibility Strategy

1. **Host API version**: The registry declares a current API version (e.g., `"1.0.0"`)
2. **Plugin declares**: `min_host_version` and optional `max_host_version`
3. **Compatibility check at load time**:
   - Plugin's `min_host_version` must be <= host's current API version
   - Plugin's `max_host_version` (if set) must be >= host's current API version
4. **Semver rules**: Pre-release versions (`0.x.y`) are treated as unstable; minor version bumps within same major are backward-compatible

### Migration Path from Current Code

1. **Consolidate** `pheno-plugin-core` and `libs/plugin-registry` into a single `agileplus-plugin-core` crate
2. **Rename** `AdapterPlugin` -> `Plugin` (the unified trait)
3. **Add** `start()`/`stop()` lifecycle methods to the base trait
4. **Replace** sync `RwLock` with `tokio::sync::RwLock` in the registry
5. **Add** `PluginState` enum and state machine enforcement
6. **Add** version compatibility checking at load time
7. **Add** `discover()` method for filesystem-based plugin discovery
8. **Unify** `PluginConfig` and `PluginError` types
9. **Update** `pheno-plugin-git` and `pheno-plugin-sqlite` to implement the new unified `Plugin` trait
10. **Deprecate** `libs/plugin-integration` (functionality merged into the unified registry)

---

## Summary of Findings

| Aspect | Current State | Gap | Priority |
|--------|--------------|-----|----------|
| Base `Plugin` trait | Two competing traits (`AdapterPlugin`, `Plugin`) | Unify into single trait | HIGH |
| Lifecycle | `initialize()` only; no start/stop | Add full lifecycle (load/init/start/stop/unload) | HIGH |
| Registry | Two competing registries | Unify into single async registry | HIGH |
| Version checking | None | Add semver compatibility at load time | HIGH |
| Plugin state machine | None (only `finalize()` guard) | Add proper state enum + transitions | MEDIUM |
| Plugin discovery | None | Add filesystem scanning | MEDIUM |
| Config validation | Raw `serde_json::Value` | Add TOML schema with validation | MEDIUM |
| Git plugin | Implemented with gaps | Add auth, fetch, push, commit; fix worktree | HIGH |
| SQLite plugin | Implemented with gaps | Add migrations, transactions, backup | HIGH |
| Error types | Two competing `PluginError` enums | Unify into single error type | MEDIUM |
| thegent-plugin-host | Port traits only (Go) | Implement FFI bridge to Rust plugin-core | LOW (WP-004) |
