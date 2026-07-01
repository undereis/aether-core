# Aether Session Handoff

Last updated: 2026-07-01

## Current Status

The Aether foundation is complete through Phase 4 and ready for review.

- Phase 0: approved.
- Foundation hardening: completed.
- Phase 1 Core Runtime in Rust: implemented and validated.
- Phase 2 Aether Kernel: implemented and validated.
- Phase 3 Aether Service Platform: implemented, validated, committed, and tagged.
- Phase 4 Official Service Map + Core System Services: implemented and pending
  final review.

The repository has an official checkpoint through Phase 3:

- branch: `master`
- commit: `fb854cef781cd0457356bf95544bdec528fe48e3`
- tag: `v0.3.0-service-platform`

Phase 4 is implemented but not committed. Check `git status --short` at the
start of the next session before any action.

## What Was Done

### Phase 0: Foundation

- Created the professional monorepo structure.
- Configured Rust workspace.
- Configured Python 3.12 backend workspace with `uv`.
- Added minimal FastAPI backend with:
  - `/health`
  - `/version`
  - typed settings
  - structured logging
  - dependency injection container
- Configured Docker Compose for PostgreSQL, Redis, and Qdrant.
- Added engineering governance:
  - README
  - LICENSE
  - `.env.example`
  - `.editorconfig`
  - `.gitignore`
  - `.pre-commit-config.yaml`
  - Makefile
  - docs structure
- Added initial documentation:
  - Architecture
  - ADRs
  - Engineering Constitution
  - Coding Standards
  - Roadmap
  - Project Structure
  - Tech Stack
  - Decision Log

### Foundation Hardening

- Created `docs/ADRs/ADR-0002-runtime-version-strategy.md`.
- Established the runtime strategy: production runtimes should use LTS versions.
- Analyzed local Node.js:
  - installed version: `v26.3.0`
  - status: Current, not LTS
  - recommended LTS: Node.js v24
  - no automatic Node version changes were made
- Reviewed and aligned Phase 0 documentation.

### Phase 1: Core Runtime in Rust

Implemented the native Rust core under `core/crates`:

- `aether-events`
  - base event types
  - event IDs
  - event source
  - payload and metadata
  - in-memory publish/subscribe event bus
- `aether-config`
  - local runtime config
  - TOML loading
  - default config
- `aether-logging`
  - structured log records
  - log levels
  - JSON log sink
  - in-memory log sink
- `aether-core`
  - module identifiers
  - module descriptors
  - module health enum
  - `AetherModule` lifecycle trait
- `aether-runtime`
  - runtime bootstrap
  - start/stop lifecycle
  - module loading
  - health checks
  - event emission
- `aether-cli`
  - `validate`
  - `version`
  - `help`

Created Phase 1 documentation:

- `docs/Architecture/core-runtime.md`
- `docs/ADRs/ADR-0003-core-runtime-rust.md`

### Phase 2: Aether Kernel

Implemented the Kernel layer above the Runtime:

- `aether-ids`
  - typed ID prefixes:
    - `evt_`
    - `mod_`
    - `ker_`
    - `cap_`
  - generated `UUIDv7` suffix support
  - stable suffix validation
- `aether-core`
  - module capabilities
  - module dependencies
  - lifecycle states:
    - `Created`
    - `Registered`
    - `Initializing`
    - `Running`
    - `Degraded`
    - `Stopping`
    - `Stopped`
    - `Failed`
- `aether-events`
  - `EventBusPort` abstraction
  - current in-memory bus preserved
  - event IDs expose typed `evt_` representation
- `aether-config`
  - `ConfigProvider` abstraction
  - static provider
  - TOML file provider
- `aether-telemetry`
  - telemetry records
  - telemetry signals for logs, metrics, and traces
  - telemetry emitter
  - in-memory sink
  - structured-logging sink
- `aether-kernel`
  - kernel identity
  - controlled startup
  - safe shutdown
  - module registry
  - dependency validation
  - lifecycle tracking
  - capability discovery
  - health checks
  - kernel telemetry
- `aether-cli`
  - `kernel status`
  - `kernel health`
  - `kernel modules`
  - `kernel capabilities`

Created Phase 2 documentation:

- `docs/Architecture/aether-kernel.md`
- `docs/Architecture/module-lifecycle.md`
- `docs/Architecture/telemetry.md`
- `docs/ADRs/ADR-0004-aether-kernel.md`
- `docs/ADRs/ADR-0005-typed-id-strategy.md`

Updated supporting docs:

- `README.md`
- `core/README.md`
- `docs/Architecture/README.md`
- `docs/Project-Structure/README.md`
- `docs/Tech-Stack/README.md`
- `docs/Roadmap/README.md`
- `docs/Decision-Log/README.md`
- `docs/ADRs/README.md`

### Phase 3: Aether Service Platform

Implemented the Kernel-controlled service platform:

- `aether-permissions`
  - internal permission model
  - initial permissions:
    - `event.publish`
    - `event.subscribe`
    - `config.read`
    - `telemetry.emit`
    - `service.command`
    - `service.inspect`
- `aether-resources`
  - CPU, memory, storage, network, and filesystem resource declarations
- `aether-service`
  - service manifest loading from TOML
  - service descriptor
  - service registry
  - capability provider lookup
  - capability dependent lookup
  - declared permission checks
  - service health aggregation
- `aether-service-bus`
  - in-memory Aether Service Bus
  - event publish and subscribe
  - request/reply
  - service command routing by bus route, not direct service identity
  - service notifications
  - bus status
  - permission enforcement for bus actions
- `aether-kernel`
  - owns Service Registry
  - owns ASB instance
  - exposes service registration and health aggregation
- `aether-cli`
  - `service list`
  - `service inspect`
  - `service capabilities`
  - `service health`
  - `bus status`

Created Phase 3 documentation:

- `docs/Architecture/aether-service-platform.md`
- `docs/Architecture/aether-service-bus.md`
- `docs/Architecture/service-model.md`
- `docs/Architecture/permission-model.md`
- `docs/Architecture/resource-model.md`
- `docs/ADRs/ADR-0006-aether-service-platform.md`
- `docs/ADRs/ADR-0007-aether-service-bus.md`
- `CHANGELOG.md`

Updated Engineering Constitution with Rule #002: services must communicate only
through ASB.

### Phase 4: Official Service Map + Core System Services

Implemented the official service map and first base system services:

- `docs/Architecture/aether-official-service-map.md`
  - Foundation Services
  - Cognitive Services
  - AI Services
  - Interaction Services
  - Device Services
  - Automation Services
  - Enterprise Services
  - 52 planned services total
- `aether-system-services`
  - typed official service map constants
  - manifest loading for core system services
  - Kernel registration helper
  - ASB command handler registration
  - no direct service-to-service coupling checks
- Core service manifests:
  - `core/services/telemetry-service/service.toml`
  - `core/services/configuration-service/service.toml`
  - `core/services/health-service/service.toml`
  - `core/services/event-service/service.toml`
  - `core/services/service-inspector/service.toml`
- Core system services:
  - Telemetry Service
  - Configuration Service
  - Health Service
  - Event Service
  - Service Inspector Service
- `aether-cli`
  - `service map`
  - `system services`
  - `system health`
  - `system inspect`

Created Phase 4 documentation:

- `docs/Architecture/core-system-services.md`
- `docs/ADRs/ADR-0008-official-service-map.md`
- `docs/ADRs/ADR-0009-core-system-services.md`

Updated supporting docs:

- `README.md`
- `core/README.md`
- `CHANGELOG.md`
- `docs/ADRs/README.md`
- `docs/Architecture/README.md`
- `docs/Project-Structure/README.md`
- `docs/Roadmap/README.md`
- `docs/Tech-Stack/README.md`

## Implemented Events

The event model includes:

- `SystemStarted`
- `SystemStopped`
- `ModuleLoaded`
- `ModuleFailed`
- `ConfigLoaded`
- `HealthCheckRequested`
- `HealthCheckCompleted`

Each event contains:

- `id`
- `timestamp`
- `source`
- `event_type`
- `payload`
- `metadata`

## Validation Status

The final Phase 3 validation passed:

- `make validate`
- `cargo fmt --all -- --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo run -p aether-cli -- validate`
- `cargo run -p aether-cli -- version`
- `cargo run -p aether-cli -- kernel status`
- `cargo run -p aether-cli -- kernel health`
- `cargo run -p aether-cli -- kernel modules`
- `cargo run -p aether-cli -- kernel capabilities`
- `cargo run -p aether-cli -- service list`
- `cargo run -p aether-cli -- service inspect`
- `cargo run -p aether-cli -- service capabilities`
- `cargo run -p aether-cli -- service health`
- `cargo run -p aether-cli -- service map`
- `cargo run -p aether-cli -- system services`
- `cargo run -p aether-cli -- system health`
- `cargo run -p aether-cli -- system inspect`
- `cargo run -p aether-cli -- bus status`

Validation coverage includes:

- Ruff
- Black
- MyPy
- Pytest
- Cargo fmt check
- Cargo Clippy with warnings denied
- Cargo check
- Cargo test
- Python backend build
- Tauri CLI check
- Docker validation
- Backend smoke test
- Core CLI validation
- Kernel CLI validation
- Service CLI validation
- System service CLI validation
- Bus CLI validation

Rust workspace test count after Phase 4: 77 tests passed.

## Current Git State

Phase 4 has not been committed.

Suggested Phase 4 commit and tag after review:

```bash
git add .
git commit -m "feat(system): add official service map and core system services"
git tag v0.4.0-core-system-services
```

## Important Local Environment Notes

- macOS: 26.5.1 arm64.
- Rust/Cargo: 1.96.1.
- Python: 3.12.13.
- Docker: 29.6.1.
- Docker Compose: 5.2.0.
- Colima: 0.10.3.
- Node.js: 26.3.0 Current, not LTS.
- pnpm: 11.9.0.
- Bun: 1.3.14.
- Tauri CLI: 2.11.4.

Port notes:

- `127.0.0.1:8000` was already occupied by a local Python process.
- Aether backend defaults to `127.0.0.1:18000`.
- The existing service on port 8000 was not changed or stopped.

Docker notes:

- PostgreSQL, Redis, and Qdrant were running and validated during previous
  validation.
- If a new session starts fresh, run `make docker-up` before `make validate` if
  containers are not already running.

## Explicitly Not Implemented

The following remain out of scope and were not implemented:

- AI
- agents
- embeddings
- memory engine
- knowledge graph
- chat
- frontend functionality
- desktop functionality
- OS-level event capture
- authentication
- business APIs
- business database schema
- production OpenTelemetry
- plugins
- NATS messaging
- direct service-to-service communication

## Next Recommended Steps

The next section should begin with Phase 4 review, not Phase 5 implementation.

Recommended order:

1. Review the official service map and classification choices.
2. Review the five Phase 4 manifests and their capability/permission/resource
   declarations.
3. Review `aether-system-services` public API and ASB handler registration.
4. Review Engineering Rule #002 enforcement for core system services.
5. Create a commit and tag for Phase 4 after review.
6. Only after review approval, define the formal scope for Phase 5.

## Useful Commands

```bash
make validate
cargo check --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo clippy --workspace --all-targets -- -D warnings
cargo run -p aether-cli -- validate
cargo run -p aether-cli -- version
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
make docker-up
make docker-validate
make backend-smoke
```

## Current Architectural Position

Aether now has a validated native Rust foundation for internal runtime
contracts, local configuration, structured logs, module lifecycle, internal
events, typed IDs, telemetry abstraction, a Kernel orchestration layer, and the
Aether Service Platform. Phase 4 adds the official service map and the first
core system service base. It is intentionally not yet a product runtime, AI
system, agent system, desktop shell, memory engine, knowledge graph, or data
platform.
