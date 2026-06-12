# Justfile — task runner for AgilePlus
# See https://just.systems/man/en/

set dotenv-load

default:
    @just --list

# Bootstrap tasks
bootstrap:
    # Placeholder for initialization tasks
    # Example: cargo install --path .

# Build command
build:
    cargo build --release

# Test command
test:
    cargo test --all-features

# Format code
fmt:
    cargo fmt --all

# Check formatting
fmt-check:
    cargo fmt --all --check

# Lint
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Audit dependencies
audit:
    cargo deny check advisories

# CI target (run all checks)
ci:
    fmt-check lint test build
