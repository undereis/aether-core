# Aether Kernel

The Aether Kernel is the orchestration layer above the Core Runtime.

It does not replace the Runtime. The Runtime remains responsible for concrete
local bootstrap, event emission, module startup, module shutdown, and runtime
health. The Kernel coordinates those primitives through a higher-level contract
for registration, lifecycle, capability discovery, health reporting, telemetry,
and future plugin preparation.

## Responsibilities

- Own the kernel identity.
- Start and shutdown the underlying runtime in a controlled order.
- Register module descriptors before execution.
- Validate declared module dependencies during registration.
- Track module lifecycle status.
- Discover declared module capabilities.
- Expose kernel and module health reports.
- Emit foundational telemetry.

## Relationship With Runtime

`aether-runtime` remains the execution boundary. `aether-kernel` delegates module
loading and runtime health checks to it.

The Kernel adds orchestration state that the Runtime intentionally does not own:

- module registry
- dependency validation
- capability index
- lifecycle state snapshots
- kernel-level telemetry

This keeps the Runtime small and preserves the Phase 1 contract while allowing
the Kernel to evolve as the central coordination layer.

## Crate Boundaries

- `aether-core`: module contracts, capabilities, lifecycle status, and health.
- `aether-ids`: typed ID strategy for kernel, module, event, and capability IDs.
- `aether-events`: event contracts and `EventBusPort` abstraction.
- `aether-config`: configuration model and `ConfigProvider` abstraction.
- `aether-telemetry`: telemetry emitter and sink contracts.
- `aether-runtime`: concrete runtime bootstrap and module execution.
- `aether-kernel`: registry, lifecycle orchestration, health, and discovery.
- `aether-cli`: validation commands for runtime and kernel.

## Current Limits

Phase 2 does not implement plugins, AI, agents, OS capture, frontend, desktop UI,
business APIs, authentication, persistence models, or production observability.

The Kernel prepares contracts for those future capabilities without pretending
they exist in this phase.

## CLI Validation

```bash
cargo run -p aether-cli -- kernel status
cargo run -p aether-cli -- kernel health
cargo run -p aether-cli -- kernel modules
cargo run -p aether-cli -- kernel capabilities
```
