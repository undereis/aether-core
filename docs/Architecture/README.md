# Architecture

Aether is organized as a modular monorepo with separately evolvable platform
areas:

- `core`: Rust runtime foundation.
- `backend`: Python FastAPI service boundary.
- `frontend`: future Next.js user interface.
- `desktop`: future Tauri desktop shell.
- `infrastructure`: operational topology and deployment notes.
- `docker`: local container support.
- `docs`: governance, standards, and technical records.

The intended architecture style is Clean Architecture with DDD boundaries and
event-driven integration where it becomes justified. The current foundation does
not define business domains yet.

The native Rust runtime is described in `docs/Architecture/core-runtime.md`.
The kernel orchestration layer is described in
`docs/Architecture/aether-kernel.md`.
Module lifecycle rules are described in
`docs/Architecture/module-lifecycle.md`.
Telemetry is described in `docs/Architecture/telemetry.md`.
The Aether Service Platform is described in
`docs/Architecture/aether-service-platform.md`.
The Aether Service Bus is described in
`docs/Architecture/aether-service-bus.md`.
Service, permission, and resource models are described in
`docs/Architecture/service-model.md`,
`docs/Architecture/permission-model.md`, and
`docs/Architecture/resource-model.md`.
The official service map is described in
`docs/Architecture/aether-official-service-map.md`.
The Phase 4 core system services are described in
`docs/Architecture/core-system-services.md`.

## Dependency Direction

Outer layers may depend on inner abstractions. Inner layers must not depend on
transport, framework, storage, or UI details.

## Integration Direction

Synchronous APIs are allowed for platform health and version checks. Future
cross-module workflows should prefer events once domain capabilities exist.
Service-to-service communication must pass through the Aether Service Bus.
