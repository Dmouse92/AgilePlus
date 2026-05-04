# Remediation Playbook — Low-Risk Infra Fixes
Generated: 2026-05-04

Total repos scanned: 112

## Priority 1 — Documentation

### Missing README.md (1)
- `AgilePlus` (Rust) — create README.md >=20 lines
  - Source hints: CLAUDE.md exists
  - Include: cargo build/test/check commands

### Stub README.md (1)
- `AppGen` (Node) — 12_STUB lines; expand purpose, install, usage, tests, CI

## Priority 2 — .gitignore coverage

Missing .gitignore (7):
- `Benchora` (Rust) — add standard rust .gitignore
- `hwLedger` (Other) — add standard other .gitignore
- `localbase3` (Other) — add standard other .gitignore
- `PhenoCompose` (Node) — add standard node .gitignore
- `PhenoProject` (Other) — add standard other .gitignore
- `rich-cli-kit` (Rust) — add standard rust .gitignore
- `thegent-dispatch` (Rust) — add standard rust .gitignore

## Priority 3 — Nested git repos / gitlinks

Repos with nested .git entries (14):
- `PhenoProc` — 26 nested entries
- `DataKit` — 6 nested entries
- `AuthKit` — 4 nested entries
- `Sidekick` — 3 nested entries
- `AgilePlus` — 2 nested entries
- `McpKit` — 2 nested entries
- `PhenoDevOps` — 2 nested entries
- `thegent` — 2 nested entries
- `Conft` — 1 nested entries
- `FocalPoint` — 1 nested entries
- `PhenoKits` — 1 nested entries
- `ResilienceKit` — 1 nested entries
- `TestingKit` — 1 nested entries
- `Tracely` — 1 nested entries

Action: verify each nested entry is intentional. Convert to submodule if intended, remove from parent index if accidental, or ignore local worktree folders.

## Priority 4 — Lockfile policy

Repos without common lockfile (33):
- `hwLedger` (Other)
- `localbase3` (Other)
- `PhenoCompose` (Node)
- `PhenoProject` (Other)
- `agent-user-status` (Python)
- `AgentMCP` (Other)
- `AuthKit` (Python)
- `cheap-llm-mcp` (Python)
- `Conft` (Other)
- `DataKit` (Other)
- `Dino` (Other)
- `dinoforge-packs` (Other)
- `DINOForge-UnityDoorstop` (Other)
- `foqos-private` (Other)
- `heliosBench` (Node)
- `Httpora` (Python)
- `McpKit` (Python)
- `nanovms` (Node)
- `ObservabilityKit` (Other)
- `PhenoHandbook` (Node)
- `PhenoSpecs` (Other)
- `phenotype-hub` (Other)
- `phenotype-infra` (Other)
- `phenotype-omlx` (Python)
- `phenoXdd` (Other)
- `Pine` (Other)
- `PlatformKit` (Other)
- `portage` (Python)
- `QuadSGM` (Python)
- `ResilienceKit` (Other)
- `TestingKit` (Other)
- `vibeproxy` (Other)
- `vibeproxy-monitoring-unified` (Other)

Action: enforce per-language policy: Node apps commit lockfile; Rust binary/workspace apps commit Cargo.lock; libraries may omit Cargo.lock intentionally.

## Priority 5 — Rust dependency hygiene

Repos mixing `log` and `tracing` in root Cargo.toml:
- `HexaKit` — standardize workspace logging dependency
- `PhenoObservability` — standardize workspace logging dependency
- `pheno` — standardize workspace logging dependency

## Priority 6 — Build artifacts

~40 Rust repos have `target/` directories. Ensure `.gitignore` includes `/target/`; do not delete without checking local in-progress build artifacts.

## Suggested branch names

- Documentation: `docs/readme-expansion-20260504`
- Gitignore cleanup: `chore/gitignore-coverage-20260504`
- Nested git cleanup: `chore/nested-git-remediation-20260504`
- Rust dependency hygiene: `chore/rust-logging-deps-20260504`

## Safe next edits (low risk)

1. Add README.md for `AgilePlus` using CLAUDE.md as source.
2. Expand `AppGen` README.md to >=20 lines.
3. Add `.gitignore` to: `Benchora`, `hwLedger`, `localbase3`, `PhenoCompose`, `PhenoProject`, `rich-cli-kit`, `thegent-dispatch`
4. Add `/target/` to `.gitignore` for Rust repos missing it.
