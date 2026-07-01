# Aether

The Cognitive Operating System.

Aether is a long-term platform foundation owned by NeuroForge Labs. Phase 0
contains no product features, no agents, no embeddings, no memory engine, no
chat, no business APIs, and no authentication. Its purpose is to establish the
engineering baseline for future decades of work.

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
```
