# Aether

The Cognitive Operating System.

Aether is a long-term platform foundation owned by NeuroForge Labs. Phase 0
contains no product features, no agents, no embeddings, no memory engine, no
chat, no business APIs, and no authentication. Its purpose is to establish the
engineering baseline for future decades of work.

The Foundation Era is complete through Phase 4.5. Future approved phases will
build cognitive capabilities on top of the Kernel, Manager Layer, Service
Platform, Driver contracts, Domain model, Policy model, ASB, and Contract Bus
base established here.

## Phase 0 Scope

- Professional monorepo structure.
- Rust workspace for the future core runtime.
- Python 3.12 workspace managed by `uv`.
- Minimal FastAPI backend with health, version, config loading, logging, and
  dependency injection.
- Docker Compose infrastructure for PostgreSQL, Redis, and Qdrant.
- Engineering documentation, coding standards, ADRs, and decision log.
- Local validation commands through `make`.

## Phase 1 Scope

- Native Rust core runtime foundation.
- Base internal event types and in-memory event bus.
- Local runtime configuration.
- Structured logging primitives.
- Module lifecycle contract.
- Runtime bootstrap.
- Minimal CLI validation command.

Phase 1 still excludes AI, agents, chat, business APIs, UI, OS capture, and
desktop functionality.

## Phase 2 Scope

- Aether Kernel as the orchestration layer above the Runtime.
- Module registry with dependency validation.
- Explicit module lifecycle states.
- Capability declaration and discovery.
- Kernel health checks.
- EventBus, ConfigProvider, telemetry, and typed ID abstractions.
- Kernel CLI validation commands.

Phase 2 still excludes AI, agents, chat, OS capture, business APIs, UI,
authentication, plugins, and desktop functionality.

## Phase 3 Scope

- Aether Service Platform as the Kernel-controlled service layer.
- Service Model and TOML Service Manifest.
- Service Registry and health aggregation.
- Aether Service Bus for internal service communication.
- Capability, permission, and resource models for services.
- CLI inspection commands for services and bus status.

Phase 3 still excludes AI, agents, authentication, Memory Engine, business
database schemas, frontend, desktop functionality, and production sandboxing.

## Phase 4 Scope

- Official Service Map across Foundation, Cognitive, AI, Interaction, Device,
  Automation, and Enterprise layers.
- Base core system services:
  - Telemetry Service
  - Configuration Service
  - Health Service
  - Event Service
  - Service Inspector Service
- Declarative service manifests under `core/services`.
- `aether-system-services` crate for loading and registering base services.
- CLI inspection commands for the service map and core system services.

Phase 4 still excludes AI, agents, Memory Engine, Knowledge Graph,
authentication, frontend, desktop functionality, and business persistence.

## Phase 4.5 Scope

- Kernel decomposition to prevent growth into a God Object.
- Official Manager Layer infrastructure.
- Driver Layer contracts.
- Domain Layer contracts.
- Policy Layer contracts.
- Contract Bus base on top of the ASB.
- Public compatibility facade preserved on the Kernel.

Phase 4.5 closes the Foundation Era. It still excludes AI, agents, Memory
Engine, Knowledge Graph, authentication, frontend, desktop functionality, and
business persistence.

## Quick Start

```bash
make setup
make lint
make type-check
make test
make build
make docker-up
make docker-validate
make backend-dev
```

In another shell:

```bash
make backend-validate
```

The local backend defaults to `127.0.0.1:18000` to avoid colliding with common
development services on port `8000`. Override with `BACKEND_PORT=...` only when
the target port is known to be free.

## Repository Layout

See `docs/Project-Structure/README.md` for the authoritative structure and
ownership rules.

## Runtime Policy

Production runtime versions follow the LTS strategy documented in
`docs/ADRs/ADR-0002-runtime-version-strategy.md`.

## Core Runtime CLI

```bash
cargo run -p aether-cli -- validate
cargo run -p aether-cli -- kernel status
cargo run -p aether-cli -- kernel health
cargo run -p aether-cli -- kernel modules
cargo run -p aether-cli -- kernel capabilities
cargo run -p aether-cli -- service list
cargo run -p aether-cli -- service inspect
cargo run -p aether-cli -- service capabilities
cargo run -p aether-cli -- service health
cargo run -p aether-cli -- service map
cargo run -p aether-cli -- system services
cargo run -p aether-cli -- system health
cargo run -p aether-cli -- system inspect
cargo run -p aether-cli -- bus status
```
