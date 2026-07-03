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
The Phase 4.5 Kernel decomposition is described in
`docs/Architecture/kernel-architecture.md`,
`docs/Architecture/manager-architecture.md`,
`docs/Architecture/driver-architecture.md`,
`docs/Architecture/domain-architecture.md`, and
`docs/Architecture/policy-architecture.md`.
The Phase 4.6 Architecture Constitution and governance documents are described
in `docs/Architecture/Aether-Architecture-Constitution-v2.md`,
`docs/Architecture/Architecture-Bible.md`,
`docs/Architecture/Architecture-Evolution-Roadmap.md`, and
`docs/Architecture/Architectural-Gravity.md`.
The active engineering protocols are indexed in `docs/AEP/README.md`.
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

## Architecture Milestones

| Version | Milestone | Architectural Outcome |
| --- | --- | --- |
| `v0.2.0` | Kernel | Kernel orchestration above the Core Runtime. |
| `v0.3.0` | Service Platform | Service Model, manifests, permissions, resources, and ASB. |
| `v0.4.0` | Core System Services | Official service map and base system services. |
| `v0.4.5` | Kernel Decomposition | Manager, Driver, Domain, Policy layers and Contract Bus base. |
| `v0.4.6` | Aether Engineering Protocol | Architecture Constitution v2, AEP governance, Architecture Bible, and Architecture Guardian. |

## Foundation Era Closure

Phase 4.5 closes the Foundation Era structurally. Phase 4.6 closes it
constitutionally by defining how future work must proceed. The architecture now
has the platform boundaries and governance required for future cognitive
capabilities without introducing those capabilities yet.
