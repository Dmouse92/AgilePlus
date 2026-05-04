---
spec_id: eco-004
title: Hexagonal Migration
created_at: 2026-05-03T00:00:00Z
priority: low
status: planned
type: operational
---

# Work Packages: Hexagonal Migration

**Inputs:** `meta.json`, `spec.md` in `eco-004-hexagonal-migration/`.
**Prerequisites:** AgilePlus is already hexagonal (24-crate workspace, ports-and-adapters pattern; see `AgilePlus/CLAUDE.md` "Architecture"). Org-wide hexagonal mandate lives in `Phenotype/repos/thegent/docs/governance/23_ARCHITECTURAL_GOVERNANCE.md`. Spec frontmatter states: "No migration required; spec captured a status check, not work."
**Primary scope:** Confirm AgilePlus compliance with hexagonal architecture principles; document the existing architecture for reference.
**Secondary scope:** Org-wide hexagonal governance for other Phenotype repos.

Per global governance: planner WPs equip implementers. No code -- documentation and acceptance criteria only. Acceptance criteria checklists drive implementation verification.

---

## What

This spec addresses hexagonal architecture migration for AgilePlus. The spec frontmatter and problem statement both confirm that AgilePlus is already compliant: it uses a 24-crate workspace with ports-and-adapters pattern (see `AgilePlus/CLAUDE.md`). The spec itself is a status check rather than a work item -- no migration is required.

The org-wide hexagonal mandate is documented in `Phenotype/repos/thegent/docs/governance/23_ARCHITECTURAL_GOVERNANCE.md` and applies to all Phenotype repos. This tasks.md captures the verification and documentation work to confirm compliance and provide a reference for future crate implementations.

## Why

Hexagonal architecture (ports and adapters) isolates domain logic from infrastructure concerns, enabling testability, independent deployability, and clean separation between business rules and external systems. AgilePlus was designed hexagonal from the start, but as new crates are uncommented and implemented (many are currently commented out in `Cargo.toml`), each must follow the same pattern. This spec serves as a compliance checkpoint and reference.

---

## Work Packages

### WP01: Document Existing Hexagonal Architecture (Priority: P0)

**Goal:** The existing hexagonal architecture of AgilePlus is documented with a crate-by-crate map showing port/adapter boundaries, dependency direction, and domain isolation.
**Independent Test:** A new developer reads the documentation and can identify which crates are domain, which are adapters, and which are ports; they can trace a request from API entry point through domain logic to infrastructure.
**Prompt:** `tasks/WP01-document-hexagonal-architecture.md`
**Estimated:** ~200 lines, 4-5 subtasks

#### Acceptance Criteria
1. A document exists (in `docs/` or `docs/adr/`) mapping each active crate to its hexagonal role (domain, port, adapter, application).
2. Dependency direction is documented: domain has zero dependencies on infrastructure; adapters depend on domain ports.
3. The document includes a Mermaid diagram showing the layered architecture.
4. Commented-out crates in `Cargo.toml` are annotated with their intended hexagonal role for future implementation.

#### Included Subtasks
1.1 Inventory all active crates in `libs/`, `agileplus/`, `apps/` and classify each as domain/port/adapter/application.
1.2 Create a Mermaid diagram showing dependency direction between layers.
1.3 Document the ports-and-adapters pattern as applied in AgilePlus (with code references).
1.4 Annotate commented-out crates in `Cargo.toml` with intended hexagonal roles.
1.5 Publish the document in `docs/adr/` or `docs/specs/`.

---

### WP02: Verify Hexagonal Compliance for New Crates (Priority: P1)

**Goal:** As crates are uncommented from `Cargo.toml` and implemented, each is verified to follow hexagonal architecture principles before merge.
**Independent Test:** Pick a commented-out crate (e.g., `agileplus-domain`), implement it following hexagonal principles, and verify it has zero dependencies on infrastructure crates.
**Prompt:** `tasks/WP02-verify-new-crate-compliance.md`
**Estimated:** ~150 lines, 3-4 subtasks

#### Acceptance Criteria
1. A checklist exists for verifying hexagonal compliance of new crates (domain has no infra deps, adapters implement ports, etc.).
2. At least one uncommented crate is verified against the checklist as a pilot.
3. The checklist is referenced in `docs/agents/governance-constraints.md`.

#### Included Subtasks
2.1 Create a hexagonal compliance checklist (domain isolation, port interfaces, adapter implementations).
2.1 Pick a pilot crate (e.g., `agileplus-domain`) and verify it against the checklist.
2.3 Document any deviations found and remediation steps.
2.4 Add the checklist reference to `docs/agents/governance-constraints.md`.

---

## Dependencies

- WP01 should complete before WP02 (document existing state before verifying new crates).
- No dependencies on other specs.

## Risks

- **Spec is a no-op:** The spec frontmatter explicitly states "No migration required." The primary risk is spending effort on work that has no forward value. Mitigation: keep WPs scoped to documentation/verification only.
- **Future crate drift:** As commented-out crates are implemented, they may not follow hexagonal patterns if not checked. Mitigation: compliance checklist + governance-constraints reference.
