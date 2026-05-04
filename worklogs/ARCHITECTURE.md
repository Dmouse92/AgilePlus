# Architecture Decisions and Research

Category: ARCHITECTURE
Projects: [cross-repo]

---

## 2026-05-04 — [cliproxyapi-plusplus] WP-001 audit + provider abstraction design (T001/T002)

**Context:** WP-001 of spec `017-cli-tools-consolidation` — audit existing CLI tooling and design the provider abstraction for the unified LLM proxy.

**Work done:**
- Audited `cliproxyapi-plusplus` (~3,993 .go files, ~200K LOC). Go proxy that exposes OpenAI-compatible HTTP surface and translates to 15+ vendor backends via OAuth/CLI-token auth.
- Mapped auth backends: `antigravity, claude, codex, copilot, cursor, gemini, gitlab, iflow, kilo, kimi, kiro, qwen, vertex` (13 vendor + `base/empty/synthesizer` infra).
- Mapped translators: `acp, antigravity, claude, codex, gemini, gemini-cli, kiro, openai` (8 wire-format adapters).
- Found rate limiting is solid (`ratelimit/` sliding-window, RPM/TPM/RPD/TPD, per-credential cooldown, `RequestRetry`/`MaxRetryCredentials`/`MaxRetryInterval` config).
- Inventoried the other 6 spec-scoped repos: `agentapi-plusplus` (PTY-driving HTTP API, orthogonal to cliproxy), `forgecode` (Rust git-workflow CLI), `helios-cli` (Rust codex fork), and `Cmdra/thegent-sharecli/thegent-subprocess` (all empty submodule placeholders — cannot work on WP-003/005/006 until cloned).
- Key finding: WP-001 acceptance criteria assume "REST API key" adapters for Mistral/Cohere/Groq/Ollama/local, none of which exist. However Mistral/Groq/Ollama are OpenAI-compatible so they need only ~50 LOC each to wrap the existing OpenAI translator. Cohere needs a thin translator (~M effort).
- Designed typed `Provider` interface (`pkg/llmproxy/provider/provider.go`), `Credential`/`CredentialSource` contracts, `Router` + fallback chain, normalized `ErrorClass` taxonomy, and per-provider adapter strategy table.
- Confirmed agentapi-plusplus integration is a BaseURL contract only (no shared library needed).

**Artifacts:**
- Audit + design doc: `kitty-specs/017-cli-tools-consolidation/research/cliproxy-audit.md` (339 lines)
- tasks.md: T001 and T002 checked off

**Outcome:** WP-001 T001 + T002 complete. Unblocks T003-T011 (adapter implementation, routing, rate-limit pipeline, agentapi hook, test coverage).

---

## 2026-05-04 — ARCHITECTURE.md skeleton replaced with real content

**Context:** `repos/ARCHITECTURE.md` was a placeholder with no real system knowledge. The file described "AgilePlus" as if it were the shelf itself, listed fictional repos, and contained generic skeleton bullet points rather than factual content about the actual polyrepo collection.

**Work done:**

- Surveyed top-level `repos/` directory to identify canonical project directories vs. worktree directories (pattern: `<project>-wtrees/`).
- Read `AgilePlus/README.md`, `crates/README.md`, `pheno/README.md`, `kitty-specs/021-polyrepo-ecosystem-stabilization/tasks.md`, `AgilePlus/openapi.yaml`, `agileplus-mcp/pyproject.toml`, and CI workflow listings to extract real system structure.
- Identified three cross-cutting roles:
  1. **phenoShared + phenotype-infrakit** as the shared Rust foundation.
  2. **AgilePlus** (CLI + REST + gRPC + Python MCP bridge) as the work-tracking spine.
  3. **cheap-llm-mcp + AgentMCP + agent-user-status** as the agent automation layer.
- Documented the polyrepo shelf model (not a monorepo; each repo is standalone), the four component categories, the runtime data flow through the AgilePlus stack and cross-repo product stack, and the standard release flow (specify -> worktree -> validate -> PR -> publish -> propagate -> record).
- Removed all placeholder text and skeleton markers.

**Key architectural facts recorded:**

- AgilePlus has 25 Rust workspace crates in `AgilePlus/crates/` covering domain, CLI, API, gRPC, SQLite, git, GitHub, NATS, p2p, telemetry, cache, sync, events, graph, import, triage, subcmds, dashboard, artifacts, benchmarks, and test support.
- MCP bridge (`agileplus-mcp`) is Python/FastMCP 3.0, bridging to the Rust gRPC layer.
- Proto definitions are at the shelf root (`buf.yaml`, `buf.gen.yaml`).
- Release uses `cliff.toml` for changelogs, repo-local `publish.yml` for crate publication, and per-repo deploy workflows for web properties.
- Worktrees are the standard isolation mechanism for feature work; canonical directories stay on `main`.

**File modified:** `/Users/kooshapari/CodeProjects/Phenotype/repos/ARCHITECTURE.md`
