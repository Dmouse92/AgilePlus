# Duplication Worklog

## 2026-05-04 — [cross-repo] de-duplicate docker-compose.yml + docker-compose.plane.yml across shelf

- Investigated 14 candidate compose files at the shelf root and inside `pheno/`, `HexaKit/`, `PhenoDevOps/` (top-level + their `agileplus/` sub-dirs).
- SHA-256 comparison: all 7 copies of `docker-compose.yml` (rust-builder + python-mcp services) were byte-identical; all 7 copies of `docker-compose.plane.yml` (Plane.so + Dragonfly stack, 6 services) were byte-identical.
- Designated `repos/docker-compose.yml` and `repos/docker-compose.plane.yml` as canonical.
- Added a header comment to each canonical file documenting orchestrated services, mirrored locations, and edit discipline.
- Replaced 12 duplicate copies (6 per family) with relative symlinks to the canonical files in `pheno/`, `pheno/agileplus/`, `HexaKit/`, `HexaKit/agileplus/`, `PhenoDevOps/`, `PhenoDevOps/agileplus/`.
- Left untouched (different content / different purpose):
  - `*/tests/integration/docker-compose.test.yml` — full-stack integration test rig (3 identical copies, intentional sibling under repo-specific test trees).
  - `PhenoDevOps/docker/docker-compose.yml` — Phenotype storage stack (Dragonfly, QuestDB, Qdrant, Meilisearch, NATS, MinIO).
  - `helios-router/docker-compose.yaml` and entries under `*-wtrees/` worktrees (worktrees mirror their canonical and re-resolve via the symlinks already in their working trees once they pull).
- Net effect: 12 duplicated YAML files (~50KB) collapsed to 2 canonical sources; future edits propagate automatically.

## 2026-05-04 — [agentapi-plusplus] remove nested repository copy

- Investigated duplicate tree at `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus/agentapi-plusplus/` against canonical outer checkout `/Users/kooshapari/CodeProjects/Phenotype/repos/agentapi-plusplus/`.
- Evidence: inner tree had no `.git`, was tracked by the outer repository, was first added by `ab571f7` (`Replay: phenotype-config SDK integration + docs scaffold (#287)`), lacked `.github/`, and contained older/smaller `go.mod`, `go.sum`, `README.md`, and server implementation snapshots.
- Existing docs already noted the nested checkout as a known issue in `docs/sessions/20260428-taskfile-agentapi-plusplus/05_KNOWN_ISSUES.md`.
- Removed the nested duplicate tree and removed the stale Taskfile exclusion `:!:agentapi-plusplus/**` so future Go formatting covers the canonical repo directly.
