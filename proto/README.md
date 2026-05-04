# AgilePlus Protocol Buffer Definitions

gRPC service and message definitions for AgilePlus, organized by API version.

**Services (v1):**
- `core.proto` -- core domain messages and RPCs
- `agents.proto` -- agent lifecycle and task management
- `integrations.proto` -- external service integrations (GitHub, Plane, NATS)
- `common.proto` -- shared types and enums

Code generation uses `buf` (preferred) or `protoc` directly. See the root `AgilePlus` repo for generation scripts.

```bash
buf generate    # from project root
```
