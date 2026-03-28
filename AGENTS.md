# Agent Rules

**This project IS AgilePlus - the core project management platform.**

## Self-Reference

AgilePlus tracks its own work through its own system. All development on AgilePlus is managed internally via the agileplus CLI.

## Branch Discipline

- Feature branches in `repos/worktrees/AgilePlus/<category>/<branch>`
- Canonical repository tracks `main` only
- Return to `main` for merge/integration checkpoints

## Work Requirements

1. **Create spec for new work**: `agileplus specify --title "<feature>" --description "<desc>"`
2. **Update work package status**: `agileplus status <feature-id> --wp <wp-id> --state <state>`
3. **No code without corresponding AgilePlus spec**

## UTF-8 Encoding

All markdown files must use UTF-8.

## Specs Location

- kitty-specs/<feature-id>/spec.md
- kitty-specs/<feature-id>/plan.md
- kitty-specs/<feature-id>/tasks/WP*.md

## Dashboard Service Control

- Service endpoints now support process control for platform health. Implemented:
  - `POST /api/dashboard/services/:name/restart` (env configurable command via `AGILEPLUS_SERVICE_RESTART_CMD`)
  - `POST /api/dashboard/services/:name/toggle` (persisted config + health updates to dashboard state)
- These endpoint behaviors are covered by unit tests in `crates/agileplus-dashboard/src/routes.rs`.
- For deployments, configure full process supervisor behavior in your host environment (e.g., systemd/docker service, buildkite agents, etc.).
