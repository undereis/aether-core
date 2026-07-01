# Roadmap

## Phase 0: Foundation

- Toolchain validation.
- Monorepo structure.
- Backend health/version surface.
- Local infrastructure containers.
- Engineering governance.

## Phase 1: Core Runtime

- Rust core runtime crates.
- Base event contracts and in-memory event bus.
- Local configuration and structured logging.
- Module lifecycle contract.
- Runtime bootstrap and validation CLI.

## Phase 2: Aether Kernel

- Kernel orchestration above the Runtime.
- Module registry and dependency validation.
- Lifecycle states and health reporting.
- Capability declaration and discovery.
- EventBus and ConfigProvider abstractions.
- Telemetry abstraction.
- Typed ID strategy.

## Phase 3: Aether Service Platform

- Service Model and Service Manifest.
- Service Registry owned by the Kernel.
- Aether Service Bus.
- Advanced capability model.
- Initial permission model.
- Initial resource model.
- Service lifecycle supervision.
- Health aggregation.
- Service and bus inspection CLI.

## Phase 4: Official Service Map + Core System Services

- Official service map across platform layers.
- Base Telemetry, Configuration, Health, Event, and Service Inspector services.
- Declarative service manifests under `core/services`.
- `aether-system-services` crate for loading and registering base services.
- CLI inspection for service map and core system services.

## Later Phases

- Memory engine.
- Knowledge graph.
- AI integration.
- Agent orchestration.
- Product UI and desktop shell.

Each phase requires updated ADRs before implementation.
