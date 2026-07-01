# Core Runtime

The Aether Core Runtime is the native Rust foundation for local execution.

## Responsibilities

- Define internal event contracts.
- Provide an initial in-memory event bus.
- Load local runtime configuration.
- Emit structured logs through explicit sinks.
- Define the standard module lifecycle trait.
- Bootstrap the runtime and validate module health.
- Provide a minimal CLI for local validation.

## Crate Boundaries

- `aether-events`: base event types and event bus.
- `aether-config`: local configuration types and TOML loading.
- `aether-logging`: structured log records, logger, and sinks.
- `aether-core`: module identifiers, descriptors, health, and lifecycle trait.
- `aether-runtime`: runtime bootstrap and module orchestration.
- `aether-cli`: minimal validation CLI.

## Phase 1 Limits

Phase 1 does not implement AI, agents, UI, OS-level capture, persistence,
business workflows, or desktop integration. The runtime only establishes the
contracts and local bootstrap path those future modules will use.

## Future Integration

Future backend, desktop, and local-capture modules will use the runtime through
typed module contracts and internal events. Cross-module behavior should remain
event-driven where asynchronous decoupling is required, while direct calls are
reserved for simple lifecycle and health contracts.

