# Tasks: Polyrepo Ecosystem Stabilization

## Phase 1: Immediate (Days 1-7) — Stop the Bleeding

## WP-01: Close/merge 10 open PRs in phenotype-infrakit
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/
**Depends on:** none
**Effort:** L
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The close/merge 10 open prs in phenotype-infrakit scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

<!-- Reconciled 2026-04-28: PRs #544-#563 verified earlier as 13 MERGED + 3 CLOSED + 3 not-found.
     All items effectively done; ticked. See spec-stale-checkbox-pattern.md. -->

### Tasks
- T0101 [x] PR #544: Workspace stabilization — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0102 [x] PR #553: Gitignore + test-infra — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0103 [x] PR #554: Workspace restructuring — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0104 [x] PR #557: String compression (zstd) — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0105 [x] PR #558: Builder derive macro — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0106 [x] PR #559: Shared config implementation — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0107 [x] PR #560: ADR-015 crate org guidelines — merge (docs only) (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0108 [x] PR #561: Health checker with timeout — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0109 [x] PR #562: Error core layered types — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0110 [x] PR #563: Test infrastructure utilities — review and merge (verified merged) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`

## WP-02: Delete 8 obvious test/typo repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/, /Users/kooshapari/CodeProjects/Phenotype/repos/tehgent/, /Users/kooshapari/CodeProjects/Phenotype/repos/BytePort-TestPortfolio/, /Users/kooshapari/CodeProjects/Phenotype/repos/Byteport-TestZip/, /Users/kooshapari/CodeProjects/Phenotype/repos/P2/, /Users/kooshapari/CodeProjects/Phenotype/repos/Tokn/, /Users/kooshapari/CodeProjects/Phenotype/repos/argisexec/, /Users/kooshapari/CodeProjects/Phenotype/repos/acp/
**Depends on:** none
**Effort:** L
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The delete 8 obvious test/typo repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0201 [ ] agentapi-deprec (deprecated, replaced by plusplus) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0202 [ ] tehgent (typo of thegent) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0203 [ ] BytePort-TestPortfolio (test artifact) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0204 [ ] Byteport-TestZip (test artifact) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0205 [ ] P2 (placeholder) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0206 [ ] Tokn (truncated name) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0207 [ ] argisexec (typo/abbrev) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`
- T0208 [ ] acp (ambiguous) `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-deprec/`

## WP-03: Clean 22 GB build artifacts locally
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/target/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/node_modules/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/.venv/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/*.log
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The clean 22 gb build artifacts locally scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0301 [ ] `rm -rf heliosCLI/bazel-*` (~30 GB savings) `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T0302 [ ] `rm -rf */node_modules` (~5 GB savings) `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T0303 [ ] `rm -rf */.venv` (~3 GB savings) `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T0304 [ ] `cargo clean` in workspace target (~1.5 GB savings) `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T0305 [ ] Delete all `.log` files at shelf root (~200 MB) `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`

## WP-04: Enforce .gitignore across 9 cloned repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/, /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/, /Users/kooshapari/CodeProjects/Phenotype/repos/thegent/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosApp/, /Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus/, /Users/kooshapari/CodeProjects/Phenotype/repos/cliproxyapi-plusplus/, /Users/kooshapari/CodeProjects/Phenotype/repos/cloud/, /Users/kooshapari/CodeProjects/Phenotype/repos/agent-wave/
**Depends on:** none
**Effort:** L
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The enforce .gitignore across 9 cloned repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0401 [ ] phenotype-infrakit: Add target/, *.log to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0402 [ ] AgilePlus: Add target/, .venv/, __pycache__/ to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0403 [ ] thegent: Add node_modules/, .venv/, target/ to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T0404 [ ] heliosCLI: Add bazel-*, target/ to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T0405 [ ] heliosApp: Add node_modules/, dist/ to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosApp/`
- T0406 [ ] agentapi-plusplus: Add node_modules/, dist/ to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus/`
- T0407 [ ] cliproxyapi-plusplus: Verify .gitignore completeness `/Users/kooshapari/CodeProjects/Phenotype/repos/cliproxyapi-plusplus/`
- T0408 [ ] cloud: Add .next/, node_modules/ to .gitignore `/Users/kooshapari/CodeProjects/Phenotype/repos/cloud/`
- T0409 [ ] agent-wave: Verify .gitignore completeness `/Users/kooshapari/CodeProjects/Phenotype/repos/agent-wave/`

## WP-05: Set up org-level .github repo with reusable workflows
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/package.json
**Depends on:** none
**Effort:** XL
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The set up org-level .github repo with reusable workflows scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0501 [ ] Create github.com/KooshaPari/.github repo `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0502 [ ] Move 32 workflow files from shelf root to .github/workflows/ `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0503 [ ] Create reusable ci-rust.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0504 [ ] Create reusable ci-python.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0505 [ ] Create reusable ci-typescript.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0506 [ ] Create reusable ci-go.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0507 [ ] Create reusable security.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0508 [ ] Create reusable publish.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0509 [ ] Create reusable docs.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0510 [ ] Create reusable release.yml workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T0511 [ ] Update all active repos to reference org workflows `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`

## WP-06: Audit and enrich 35 AgilePlus specs
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/, /Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/
**Depends on:** none
**Effort:** L
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The audit and enrich 35 agileplus specs scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0601 [ ] Audit all 35 specs for completeness (spec.md, plan.md, tasks.md, research.md) `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0602 [ ] Enrich spec 005 (heliosApp) with plan, tasks, research `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0603 [ ] Enrich spec 006 (heliosCLI) with plan, tasks, research `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0604 [ ] Enrich spec 007 (thegent) with plan, tasks, research `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0605 [ ] Enrich spec 012 (portfolio triage) with audit findings `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0606 [ ] Enrich spec 013 (infrakit stabilization) with audit findings `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0607 [ ] Create spec 021 (this spec) with full stabilization plan `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0608 [ ] Update worklog with audit findings `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`

## WP-07: Establish worktree discipline
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/WORKTREES.md, /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/.git/worktrees/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The establish worktree discipline scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0701 [ ] Document worktree rules in WORKTREES.md `/Users/kooshapari/CodeProjects/Phenotype/repos/WORKTREES.md`
- T0702 [ ] Clean empty worktree directories (docs/, infrastructure/, phenotype-errors/) `/Users/kooshapari/CodeProjects/Phenotype/repos/WORKTREES.md`
- T0703 [ ] Investigate cache-adapter-impl worktree (detached HEAD?) `/Users/kooshapari/CodeProjects/Phenotype/repos/WORKTREES.md`
- T0704 [ ] Merge or close phenotype-crypto-complete worktree `/Users/kooshapari/CodeProjects/Phenotype/repos/WORKTREES.md`
- T0705 [ ] Document maximum 3 concurrent worktrees per repo rule `/Users/kooshapari/CodeProjects/Phenotype/repos/WORKTREES.md`

## WP-08: Run cargo fmt && cargo clippy on phenotype-infrakit
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The run cargo fmt && cargo clippy on phenotype-infrakit scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0801 [ ] `cargo fmt` across workspace `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0802 [ ] `cargo clippy --workspace -- -D warnings` `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0803 [ ] Fix all clippy warnings `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0804 [ ] Verify all tests pass: `cargo test --workspace` `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`

## WP-09: Commit all dirty files across 9 repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/, /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/, /Users/kooshapari/CodeProjects/Phenotype/repos/thegent/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosApp/, /Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus/, /Users/kooshapari/CodeProjects/Phenotype/repos/cliproxyapi-plusplus/, /Users/kooshapari/CodeProjects/Phenotype/repos/cloud/
**Depends on:** none
**Effort:** L
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The commit all dirty files across 9 repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T0901 [ ] phenotype-infrakit: Commit 8 dirty files (session docs, worklog, new sources) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T0902 [ ] AgilePlus: Commit 28 dirty files (cleanup, deleted workflows) `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T0903 [ ] thegent: Commit 4 dirty files (WORKLOG.md, Cargo.toml, CODEOWNERS) `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T0904 [ ] heliosCLI: Commit 8 dirty session doc files `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T0905 [ ] heliosApp: Commit CLAUDE.md, PR_SUMMARY.md, WORKLOG.md `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosApp/`
- T0906 [ ] agentapi-plusplus: Commit WORKLOG.md `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus/`
- T0907 [ ] cliproxyapi-plusplus: Commit PLAN.md `/Users/kooshapari/CodeProjects/Phenotype/repos/cliproxyapi-plusplus/`
- T0908 [ ] cloud: Commit 2 plan files `/Users/kooshapari/CodeProjects/Phenotype/repos/cloud/`

## WP-10: Return canonical repos to main
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/thegent/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosApp/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The return canonical repos to main scope is reflected accurately in the updated task plan with no untracked blockers left in scope.


### Tasks
- T1001 [ ] thegent: Merge `refactor/cleanup-error-variants` → main `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T1002 [ ] heliosApp: Merge `feat/fix-typescript-vite-federation` → main `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosApp/`
- T1003 [ ] heliosCLI: Merge `refactor/decouple-harness-crates` → main `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T1004 [ ] Verify all repos on main branch `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`

---

## Phase 2: Short-term (Weeks 2-3) — Consolidate and Deduplicate

## WP-11: Merge 15 duplicate repos into 8 targets
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contracts/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-error-core/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-errors/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-error-macros/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-ports-canonical/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-port-traits/, /Users/kooshapari/CodeProjects/Phenotype/repos/thegent-plugin-host/, /Users/kooshapari/CodeProjects/Phenotype/repos/forgecode-fork/, /Users/kooshapari/CodeProjects/Phenotype/repos/hexagon-rust/, /Users/kooshapari/CodeProjects/Phenotype/repos/agileplus-agents/, /Users/kooshapari/CodeProjects/Phenotype/repos/agileplus-mcp/, /Users/kooshapari/CodeProjects/Phenotype/repos/router-docs/, /Users/kooshapari/CodeProjects/Phenotype/repos/fixit/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-config-loader/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-shared-config/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-async-traits/, /Users/kooshapari/CodeProjects/Phenotype/repos/bifrost-routing/, /Users/kooshapari/CodeProjects/Phenotype/repos/bifrost-routing-backup/, /Users/kooshapari/CodeProjects/Phenotype/repos/vibeproxy-monitoring-unified/
**Depends on:** none
**Effort:** XL
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The merge 15 duplicate repos into 8 targets scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

<!-- Reconciled 2026-04-28: 14/15 source repos already 404 (deleted) or archived.
     Only vibeproxy-monitoring-unified remains active (despite spec note);
     deferred pending user review (recently pushed, may have new purpose). -->

### Tasks
- T1101 [x] phenotype-contract + phenotype-contracts → phenotype-contracts (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1102 [x] phenotype-error-core + phenotype-errors + phenotype-error-macros → phenotype-error-core (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1103 [x] phenotype-ports-canonical + phenotype-port-traits → phenotype-contracts (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1104 [x] thegent-plugin-host → thegent/apps/plugin-host (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1105 [x] forgecode-fork → forgecode (or delete) (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1106 [x] hexagon-rust → hexagon-rs (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1107 [x] agileplus-agents → AgilePlus/packages/agents (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1108 [x] agileplus-mcp → AgilePlus/packages/mcp (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1109 [x] router-docs → phenotype-hub/docs/ (archived) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1110 [x] FixitGo + FixitRs → fixit (single repo) (both 404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1111 [x] phenotype-config-loader → phenotype-config-core (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1112 [x] phenotype-shared-config → phenotype-config-core (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1113 [x] phenotype-async-traits → phenotype-contracts (404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1114 [x] bifrost-routing + bifrost-routing-backup → bifrost (both 404 — deleted) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`
- T1115 [ ] vibeproxy-monitoring-unified (NOT archived despite spec note — active 2026-04-27, needs review) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-contract/`

## WP-12: Archive 4 odin-* course repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/odin-dash/, /Users/kooshapari/CodeProjects/Phenotype/repos/odin-TTT/, /Users/kooshapari/CodeProjects/Phenotype/repos/odin-library/, /Users/kooshapari/CodeProjects/Phenotype/repos/odin-recipes/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The archive 4 odin-* course repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

<!-- Reconciled 2026-04-28: all 4 verified archived via gh api. -->

### Tasks
- T1201 [x] odin-dash → archive (verified archived) `/Users/kooshapari/CodeProjects/Phenotype/repos/odin-dash/`
- T1202 [x] odin-TTT → archive (verified archived) `/Users/kooshapari/CodeProjects/Phenotype/repos/odin-dash/`
- T1203 [x] odin-library → archive (verified archived) `/Users/kooshapari/CodeProjects/Phenotype/repos/odin-dash/`
- T1204 [x] odin-recipes → archive (verified archived) `/Users/kooshapari/CodeProjects/Phenotype/repos/odin-dash/`

## WP-13: Move personal repos to separate org
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/, /Users/kooshapari/CodeProjects/Phenotype/repos/dotfiles/, /Users/kooshapari/CodeProjects/Phenotype/repos/vibeproxy/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The move personal repos to separate org scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T1301 [ ] Create separate GitHub org or use personal account `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/`
- T1302 [ ] Move koosha-portfolio `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/`
- T1303 [ ] Move dotfiles `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/`
- T1304 [ ] Move vibeproxy (after audit) `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/`
- T1305 [ ] Remove from local shelf `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/`
- T1306 [ ] Exclude from CI/CD and AgilePlus tracking `/Users/kooshapari/CodeProjects/Phenotype/repos/koosha-portfolio/`

## WP-14: Set up GitHub Packages for @phenotype/*
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/package.json
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The set up github packages for @phenotype/* scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T1401 [ ] Configure npm scope @phenotype/* `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T1402 [ ] Set up publishing workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T1403 [ ] Publish first package `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T1404 [ ] Verify installation from GitHub Packages `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`

## WP-15: Set up PyPI publishing for phenotype-*
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/pyproject.toml, /Users/kooshapari/CodeProjects/Phenotype/repos/*/setup.py
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The set up pypi publishing for phenotype-* scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T1501 [ ] Configure PyPI project `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T1502 [ ] Set up publishing workflow `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T1503 [ ] Publish first package `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T1504 [ ] Verify installation from PyPI `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`

## WP-16: Complete phenotype-infrakit Phase 3 (performance)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The complete phenotype-infrakit phase 3 (performance) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T1601 [ ] Performance benchmarks for all crates `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T1602 [ ] Optimize hot paths `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T1603 [ ] Document performance characteristics `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`

## WP-17: Complete AgilePlus Phase 3 (governance)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The complete agileplus phase 3 (governance) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T1701 [ ] Implement policy rules `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T1702 [ ] Set up evidence evaluation `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
- T1703 [ ] Complete governance enforcement `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`

## WP-18: Distribute base templates to all active repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md, /Users/kooshapari/CodeProjects/Phenotype/repos/*/CLAUDE.md, /Users/kooshapari/CodeProjects/Phenotype/repos/*/README.md
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The distribute base templates to all active repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.


### Tasks
- T1801 [ ] Create base AGENTS.md template `/Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md`
- T1802 [ ] Create base CLAUDE.md template `/Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md`
- T1803 [ ] Create base README.md template `/Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md`
- T1804 [ ] Distribute to all ~190 active repos `/Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md`
- T1805 [ ] Verify template adoption `/Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md`

---

## Phase 3: Medium-term (Weeks 4-6) — Build Auxiliary Infrastructure

## WP-19: Create SDK monorepo (phenotype-sdk)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/packages/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/python/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The create sdk monorepo (phenotype-sdk) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T1901 [ ] Create phenotype-sdk repo structure `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/`
- T1902 [ ] Move packages/pheno-* into phenotype-sdk/packages/ `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/`
- T1903 [ ] Move python/pheno-* into phenotype-sdk/python/ `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/`
- T1904 [ ] Set up workspace configuration `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/`
- T1905 [ ] Configure publishing for all packages `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-sdk/`

## WP-20: Set up docs federation (VitePress hub)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/, /Users/kooshapari/CodeProjects/Phenotype/repos/thegent/docs/, /Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/docs/, /Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/docs/, /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/docs/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The set up docs federation (vitepress hub) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2001 [ ] Configure phenodocs as federation hub `/Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/`
- T2002 [ ] Add thegent/docs/ as source `/Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/`
- T2003 [ ] Add AgilePlus/docs/ as source `/Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/`
- T2004 [ ] Add heliosCLI/docs/ as source `/Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/`
- T2005 [ ] Add phenotype-infrakit/docs/ as source `/Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/`
- T2006 [ ] Deploy to docs.phenotype.dev `/Users/kooshapari/CodeProjects/Phenotype/repos/phenodocs/`

## WP-21: Implement health check pattern
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/src/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/app/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/services/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The implement health check pattern scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2101 [ ] Define /health endpoint standard `/Users/kooshapari/CodeProjects/Phenotype/repos/*/src/`
- T2102 [ ] Implement in all services `/Users/kooshapari/CodeProjects/Phenotype/repos/*/src/`
- T2103 [ ] Set up health monitoring `/Users/kooshapari/CodeProjects/Phenotype/repos/*/src/`

## WP-22: Set up Sentry for all production services
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The set up sentry for all production services scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2201 [ ] Configure Sentry projects `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T2202 [ ] Add Sentry SDK to all services `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T2203 [ ] Set up alerting rules `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`

## WP-23: Complete thegent Phase 3 (memory)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/thegent/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The complete thegent phase 3 (memory) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2301 [ ] Implement memory layer `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T2302 [ ] Cross-platform integration `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T2303 [ ] Testing and documentation `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`

## WP-24: Complete heliosCLI Phase 2 (sandboxing)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The complete helioscli phase 2 (sandboxing) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2401 [ ] Implement sandboxing `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T2402 [ ] Security review `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`
- T2403 [ ] Testing and documentation `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosCLI/`

## WP-25: Archive 11 low-signal personal projects
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/, /Users/kooshapari/CodeProjects/Phenotype/repos/QuadSGM/, /Users/kooshapari/CodeProjects/Phenotype/repos/Kogito/, /Users/kooshapari/CodeProjects/Phenotype/repos/Tossy/, /Users/kooshapari/CodeProjects/Phenotype/repos/Frostify/, /Users/kooshapari/CodeProjects/Phenotype/repos/AppGen/, /Users/kooshapari/CodeProjects/Phenotype/repos/TripleM/, /Users/kooshapari/CodeProjects/Phenotype/repos/Project-Spyn/, /Users/kooshapari/CodeProjects/Phenotype/repos/ssToCal-front/, /Users/kooshapari/CodeProjects/Phenotype/repos/BytePortfolio/, /Users/kooshapari/CodeProjects/Phenotype/repos/agentapi/
**Depends on:** none
**Effort:** XL
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The archive 11 low-signal personal projects scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2501 [ ] heliosBench → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2502 [ ] QuadSGM → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2503 [ ] Kogito → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2504 [ ] Tossy → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2505 [ ] Frostify → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2506 [ ] AppGen → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2507 [ ] TripleM → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2508 [ ] Project-Spyn → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2509 [ ] ssToCal-front → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2510 [ ] BytePortfolio → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`
- T2511 [ ] agentapi → archive `/Users/kooshapari/CodeProjects/Phenotype/repos/heliosBench/`

## WP-26: Split phenotype-infrakit into 3 workspaces (optional)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The split phenotype-infrakit into 3 workspaces (optional) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.


### Tasks
- T2601 [ ] core workspace (contracts, errors) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T2602 [ ] runtime workspace (event-sourcing, cache, state-machine) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T2603 [ ] tools workspace (policy-engine, validation) `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T2604 [ ] Update downstream consumers `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`

---

## Phase 4: Long-term (Weeks 7-12) — Full Ecosystem Stabilization

## WP-27: Complete thegent Phase 4 (cross-platform)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/thegent/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The complete thegent phase 4 (cross-platform) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2701 [ ] Cross-platform integration `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T2702 [ ] Testing across platforms `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`
- T2703 [ ] Documentation `/Users/kooshapari/CodeProjects/Phenotype/repos/thegent/`

## WP-28: Complete phenotype-infrakit Phase 4 (enterprise)
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/
**Depends on:** none
**Effort:** S
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The complete phenotype-infrakit phase 4 (enterprise) scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2801 [ ] Enterprise features `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T2802 [ ] Performance optimization `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T2803 [ ] Documentation `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`

## WP-29: Set up artifact storage and retention policies
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The set up artifact storage and retention policies scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T2901 [ ] Configure GitHub Actions cache (30 days) `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T2902 [ ] Configure GitHub Releases (permanent) `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T2903 [ ] Configure GHCR (90 days) `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`
- T2904 [ ] Configure S3/GitHub Pages for benchmarks `/Users/kooshapari/CodeProjects/Phenotype/repos/.github/workflows/`

## WP-30: Implement template versioning and distribution
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/, /Users/kooshapari/CodeProjects/Phenotype/repos/templates/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The implement template versioning and distribution scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T3001 [ ] Define versioning scheme (1.0 → 1.1 quarterly) `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3002 [ ] Create template registry `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3003 [ ] Implement scaffolding CLI `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3004 [ ] Set up template testing CI `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`

## WP-31: Clone and onboard remaining ~200 repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The clone and onboard remaining ~200 repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T3101 [ ] Systematic clone of all GitHub repos `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3102 [ ] Add AGENTS.md, CLAUDE.md, README.md where missing `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3103 [ ] Set up docs/sessions/ directories `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3104 [ ] Verify git health `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`

## WP-32: Full CI/CD coverage across all active repos
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The full ci/cd coverage across all active repos scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T3201 [ ] Verify all repos reference org workflows `/Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/`
- T3202 [ ] Fix any CI failures `/Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/`
- T3203 [ ] Set up branch protection rules `/Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/`
- T3204 [ ] Configure required status checks `/Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/`

## WP-33: Governance audit — verify compliance
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md, /Users/kooshapari/CodeProjects/Phenotype/repos/*/CLAUDE.md, /Users/kooshapari/CodeProjects/Phenotype/repos/*/docs/sessions/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The governance audit — verify compliance scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T3301 [ ] Check all repos for AGENTS.md `/Users/kooshapari/CodeProjects/Phenotype/repos/*/AGENTS.md`
- T3302 [ ] Check all repos for CLAUDE.md `/Users/kooshapari/CodeProjects/Phenotype/repos/*/CLAUDE.md`
- T3303 [ ] Check all repos for docs/sessions/ `/Users/kooshapari/CodeProjects/Phenotype/repos/*/docs/sessions/`
- T3304 [ ] Verify CI/CD passing `/Users/kooshapari/CodeProjects/Phenotype/repos/*/.github/workflows/`
- T3305 [ ] Verify no dirty files on main `/Users/kooshapari/CodeProjects/Phenotype/repos/*/`
- T3306 [ ] Generate compliance report `/Users/kooshapari/CodeProjects/Phenotype/repos/worklogs/`

## WP-34: Performance benchmarks and optimization report
**File Scope:** /Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/, /Users/kooshapari/CodeProjects/Phenotype/repos/*/
**Depends on:** none
**Effort:** M
### Acceptance Criteria
- All listed items in this work package are completed, archived, or explicitly reconciled.
- The performance benchmarks and optimization report scope is reflected accurately in the updated task plan with no untracked blockers left in scope.

### Tasks
- T3401 [ ] Run benchmarks across all crates `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T3402 [ ] Document performance characteristics `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T3403 [ ] Identify optimization opportunities `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`
- T3404 [ ] Create optimization roadmap `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-infrakit/`

