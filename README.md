# Aether

**A modular, security-conscious foundation for long-lived cognitive systems.**

Aether is an architecture and platform R&D project maintained by NeuroForge Labs. It establishes the engineering foundation for future cognitive capabilities while keeping runtime, policy, service, and domain boundaries explicit.

> **Current maturity:** foundation and architecture research. Aether is not yet a production AI product, autonomous agent platform, or end-user application.

## What is implemented

- A native Rust runtime and orchestration kernel.
- Module registration, dependency validation, lifecycle states, health checks, and capability discovery.
- A manager layer, driver contracts, domain boundaries, policy contracts, and typed identifiers.
- An in-memory event bus, Aether Service Bus, and Contract Bus foundation.
- Service manifests, permissions, resources, telemetry, and health aggregation.
- Five base system services: Telemetry, Configuration, Health, Event, and Service Inspector.
- A Python 3.12 FastAPI foundation with health, version, configuration, logging, and dependency injection.
- Local PostgreSQL, Redis, and Qdrant infrastructure through Docker Compose.
- Validation tooling for linting, type checking, tests, builds, and smoke checks.

## What is planned

The following areas are documented or reserved for future phases and are **not implemented as cognitive runtime capabilities**:

- Memory, Knowledge, Context, Planning, Reasoning, Decision, Learning, and Perception domains.
- AI providers, agents, model routing, and inference orchestration.
- Authentication, multi-tenancy, enterprise services, and production persistence.
- Next.js frontend, Tauri desktop application, device integration, and automation.

Architecture documents and RFCs define boundaries before implementation. Their presence must not be interpreted as working product functionality.

## Architecture

```text
FastAPI foundation
        |
Aether Kernel
        |
Managers + Runtime + Policies
        |
Service Platform + ASB / Contract Bus
        |
Core system services and driver/domain contracts
```

The Rust workspace currently contains 18 focused crates, including:

- `aether-runtime`, `aether-kernel`, and `aether-managers`
- `aether-service`, `aether-service-bus`, and `aether-system-services`
- `aether-events`, `aether-telemetry`, and `aether-config`
- `aether-permissions`, `aether-resources`, and `aether-policies`
- `aether-domains`, `aether-drivers`, and `aether-ids`

See [`docs/Project-Structure/README.md`](docs/Project-Structure/README.md) for ownership rules and the authoritative repository structure.

## Security and engineering principles

- Rust `unsafe_code` is forbidden across the workspace.
- Services communicate through explicit bus contracts instead of direct coupling.
- Permissions, resource declarations, provenance, and policy boundaries are first-class architectural concerns.
- Cognitive outputs are designed to remain bounded, traceable, explainable, and non-binding until an authorized decision and action boundary exists.
- Runtime dependencies follow an LTS/stable strategy recorded in ADRs.
- Development infrastructure credentials are local defaults only and must never be used in production.

See [`SECURITY.md`](SECURITY.md) for reporting and deployment guidance.

## Technology

| Area | Technology |
| --- | --- |
| Core runtime | Rust 2024 edition |
| Backend | Python 3.12, FastAPI, Pydantic |
| Infrastructure | Docker Compose, PostgreSQL, Redis, Qdrant |
| Tooling | Cargo, uv, Ruff, Black, mypy, pytest, pre-commit |
| Future UI | Next.js and Tauri (reserved, not implemented) |

## Quick start

Prerequisites: Rust 1.90+, Python 3.12, `uv`, `pnpm`, and Docker.

```bash
make setup
make lint
make type-check
make test
make build
```

Start and validate local infrastructure:

```bash
make docker-up
make docker-validate
```

Run the backend:

```bash
make backend-dev
```

The local backend defaults to `127.0.0.1:18000`.

## Validation

Continuous integration validates both language workspaces:

- Rust formatting, Clippy, tests, and workspace build.
- Python dependency locking, Ruff, Black, mypy, pytest, and package build.

Local full validation remains available through:

```bash
make validate
```

## Roadmap and governance

- [`docs/Roadmap/README.md`](docs/Roadmap/README.md)
- [`docs/Architecture/README.md`](docs/Architecture/README.md)
- [`docs/ADRs/README.md`](docs/ADRs/README.md)
- [`CHANGELOG.md`](CHANGELOG.md)

Foundation phases through 4.6 are complete. Later cognitive-domain material is architecture-first and proceeds through explicit RFC, ADR, review, and checkpoint gates.

## Maintainer

**Ramon Mascarenha Reis**  
Cybersecurity, network infrastructure, and secure systems architecture.

## License

Apache License 2.0. See [`LICENSE`](LICENSE).
