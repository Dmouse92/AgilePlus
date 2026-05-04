# AgilePlus Rust Crates

Rust workspace crates for the AgilePlus project management system. Covers domain logic, CLI, API server, gRPC, storage adapters, VCS integrations, and shared configuration.

**26 crates:** `agileplus-domain`, `agileplus-cli`, `agileplus-api`, `agileplus-grpc`, `agileplus-sqlite`, `agileplus-git`, `agileplus-github`, `agileplus-plane`, `agileplus-nats`, `agileplus-p2p`, `agileplus-telemetry`, `agileplus-events`, `agileplus-graph`, `agileplus-cache`, `agileplus-sync`, `agileplus-import`, `agileplus-triage`, `agileplus-subcmds`, `agileplus-dashboard`, `agileplus-artifacts`, `agileplus-benchmarks`, `agileplus-fixtures`, `agileplus-contract-tests`, `agileplus-integration-tests`, `phenotype-config`.

Many crates are currently commented out in the workspace `Cargo.toml` pending implementation. Active development happens in `libs/` and `agileplus/`.

```bash
cargo build --workspace   # build all crates
cargo test --workspace    # run all tests
cargo clippy --all        # lint
```
