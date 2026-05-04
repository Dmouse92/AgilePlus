---
spec_id: eco-003
title: Circular Dependency Resolution
created_at: 2026-05-03T00:00:00Z
priority: low
status: planned
type: operational
---

# Work Packages: Circular Dependency Resolution

**Inputs:** `meta.json`, `spec.md` in `eco-003-circular-dep-resolution/`.
**Prerequisites:** Tach is already installed and CI checks are live (per spec frontmatter: "Enabled tach CI checks -- done"). `task quality` in `repos/CLAUDE.md` already enforces circular-dep boundaries. Cross-project decomposition governed by phenotype-shared workspace and `project_decomposition_governance.md`.
**Primary scope:** Verify and maintain circular-dependency enforcement across the Rust workspace via tach.
**Secondary scope:** Cross-project decomposition patterns (phenotype-shared workspace).

Per global governance: planner WPs equip implementers. No code -- documentation and acceptance criteria only. Acceptance criteria checklists drive implementation verification.

---

## What

This spec addresses circular dependency resolution in the AgilePlus Rust workspace. The spec frontmatter indicates the primary work is already complete: tach CI checks are live and circular-dep enforcement is part of `task quality` (defined in `repos/CLAUDE.md`). Cross-project decomposition is handled by the phenotype-shared workspace and `project_decomposition_governance.md`.

The spec itself is a stub -- the problem statement reads "Enabled tach CI checks -- done" with no further functional requirements, target users, or acceptance criteria defined in the spec body. This tasks.md captures the verification and maintenance work to ensure the existing enforcement remains effective.

## Why

Circular dependencies between crates create compilation coupling, prevent parallel builds, and make it impossible to reason about individual crate boundaries. Tach enforces module-level dependency direction at CI time, ensuring the hexagonal architecture's port/adapter boundaries are respected. Even though the primary work is complete, ongoing maintenance is required as new crates are added to the workspace (many crates in `Cargo.toml` are currently commented out pending implementation).

---

## Work Packages

### WP01: Verify Tach CI Enforcement Coverage (Priority: P0)

**Goal:** Confirm that tach CI checks cover all active crates in the AgilePlus workspace and that no circular dependencies exist in the current codebase.
**Independent Test:** Run `tach check` locally and in CI; verify zero violations across all active crates. Introduce a deliberate circular import and verify tach catches it.
**Prompt:** `tasks/WP01-verify-tach-coverage.md`
**Estimated:** ~150 lines, 4-5 subtasks

#### Acceptance Criteria
1. `tach check` passes with zero violations across all active crates in the workspace.
2. All active crates (libs/, agileplus/, apps/) are listed in the tach configuration.
3. A deliberate circular import between two crates is detected by tach within CI.
4. Tach CI check is a required status check on PRs touching crate boundaries.

#### Included Subtasks
1.1 Audit current tach configuration; verify all active workspace members are listed.
1.2 Run `tach check` locally; document any existing violations.
1.3 Fix any existing circular dependency violations found.
1.4 Add a test case: introduce a deliberate circular import in a throwaway branch, verify tach CI fails.
1.5 Verify tach CI check is configured as a required status check in branch protection.

---

### WP02: Maintain Tach Config as New Crates Are Added (Priority: P1)

**Goal:** As commented-out crates in `Cargo.toml` are uncommented and implemented, tach configuration is updated to include them with correct dependency boundaries.
**Independent Test:** Uncomment a crate in `Cargo.toml`, add it to tach config, run `tach check`; verify the new crate has no circular deps.
**Prompt:** `tasks/WP02-maintain-tach-config.md`
**Estimated:** ~100 lines, 3-4 subtasks

#### Acceptance Criteria
1. A process exists (documented in `docs/` or as a checklist) for adding new crates to tach configuration.
2. Each new crate added to the workspace is accompanied by a tach config update in the same PR.
3. No crate is merged to main without tach coverage.

#### Included Subtasks
2.1 Document the tach config update process in `docs/` (checklist format).
2.2 Add a PR template reminder: "If adding a new crate, update tach config."
2.3 Verify the checklist is referenced in `docs/agents/governance-constraints.md`.
2.4 Audit commented-out crates in `Cargo.toml`; pre-compute their expected tach boundaries for future use.

---

## Dependencies

- WP01 should complete before WP02 (verify existing state before defining maintenance process).
- No dependencies on other specs.

## Risks

- **Tach config drift:** As crates are uncommented from `Cargo.toml`, tach config may not be updated in sync. Mitigation: PR template reminder + CI enforcement.
- **Cross-project circular deps:** Tach only covers intra-workspace deps; cross-project circular dependencies are governed by phenotype-shared workspace rules and `project_decomposition_governance.md`.
