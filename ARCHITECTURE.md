# Architecture

## System overview
Phenotype repos is a polyrepo workspace, not a single application. It combines product repos, shared Rust foundations, language-specific app surfaces, docs/governance, and automation tooling that are meant to evolve together.

The main layers are:
- Product and workflow repos: end-user CLIs, MCP services, dashboards, and local workflow tools.
- Shared foundations: reusable Rust crates and support packages that encode domain, transport, storage, and orchestration primitives.
- Docs and governance: specs, plans, ADRs, worklogs, and repo-local READMEs that define intent and release rules.

## Component ownership

### Shared Rust foundations
- `phenoShared/` owns the reusable Rust workspace that most Phenotype Rust crates build on.
- `phenoShared/crates/` contains the core reusable layers: domain, application, ports, config, error, event-sourcing, cache, policy, state-machine, health, HTTP, PostgreSQL, Redis, and NanoVMs client support.
- `crates/` at the workspace root provides AgilePlus-specific Rust crates for domain logic, CLI, API, gRPC, storage adapters, telemetry, sync, import, triage, and test fixtures.

### Product and app surfaces
- `AgilePlus/` owns the spec-driven work-tracking product and its root Rust workspace.
- `pheno-cli/` owns the org-wide release governance CLI.
- `thegent/` owns the Rust/Python task runner and agent-oriented orchestration surface.
- `phenoData/` owns the data-layer workspace.
- `PhenoMCP/` owns MCP-facing integration surfaces.
- `PhenoRuntime/` owns runtime orchestration and execution primitives.
- `FocalPoint/` owns the operational platform and supporting Rust tooling.
- `helioscope/` owns the helios CLI/app surface.
- `cheap-llm-mcp/` owns the low-cost MCP model gateway used for bulk reasoning and subagents.
- `agileplus-agents/` owns AgilePlus agent orchestration helpers.
- `agileplus-mcp/` owns the MCP service surface for AgilePlus integrations.

### Docs and governance
- `docs/`, `kitty-specs/`, `worklogs/`, and repo-local root docs (`README.md`, `PRD.md`, `PLAN.md`, `ADR.md`, `USER_JOURNEYS.md`) own design intent, implementation specs, and delivery history.
- Repo READMEs are the canonical entry point for per-project details; this document only records cross-repo boundaries.

## Runtime data flow
```text
user intent
  -> CLI / MCP / app surface
  -> repo-specific orchestration layer
  -> shared Rust or support libraries
  -> storage, VCS, network, or model adapters
  -> external systems and persisted artifacts
```

Typical examples:
- `pheno-cli` drives release, publish, and audit workflows across repositories.
- `AgilePlus` collects feature specs and work-package state, then updates local files and optional sync targets.
- `thegent`, `agileplus-agents`, and `cheap-llm-mcp` route agent requests through task execution, model access, and tool orchestration.
- Shared crates in `phenoShared` keep domain rules, transport contracts, and infrastructure adapters aligned across products.

## Release flow
1. Work starts in the owning repo or shared crate, with intent captured in the repo README/specs and the relevant AgilePlus or governance artifact.
2. Implementation changes are made in a branch or worktree, with shared crates updated first when multiple repos depend on the same behavior.
3. Local quality checks run in the owning repo before integration.
4. Release metadata, changelogs, and worklogs are updated in the repo that owns the shipped surface.
5. Changes merge back to the canonical branch, then downstream repos consume the new version or commit via dependency updates, workspace member changes, or synchronized follow-up PRs.

## Repository detail map
- `README.md` — workspace-level overview and release discipline for AgilePlus.
- `phenoShared/README.md` — shared Rust infrastructure toolkit and crate catalog.
- `pheno-cli/README.md` — org-wide release governance CLI.
- `phenoData/README.md` — data-layer workspace and storage-oriented components.
- `thegent/README.md` — Rust/Python task runner and orchestration platform.
- `cheap-llm-mcp/README.md` — cheap model gateway for agent subtasks.
- `FocalPoint/README.md` — platform and tooling surface for broader Phenotype operations.
- `helioscope/README.md` — helios CLI/app surface.
- `PhenoMCP/README.md` — MCP integration surface.
- `PhenoRuntime/README.md` — runtime execution surface.
- `pheno/README.md` — release governance CLI lineage and workflow automation.
- `phenoForge/README.md` — build/task orchestration runtime.
- `PhenoKits/`, `PhenoPlugins/`, `PhenoProc/`, `PhenoVCS/`, and related repos own specialized platform capabilities; see each repo README for local boundaries.

## Key invariants
- Shared behavior belongs in shared crates before it is duplicated in product repos.
- Repo READMEs/specs define the owning repo’s contract; this file only explains how the repos relate.
- Release flow should stay forward-only: update the owner, validate locally, then propagate downstream consumers.
- Keep language-specific surfaces thin and let shared libraries carry the stable contracts.
