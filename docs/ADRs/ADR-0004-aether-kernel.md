# ADR-0004

## Aether Kernel

## Status

Accepted.

## Context

The Phase 1 Core Runtime established events, configuration, logging, module
contracts, runtime bootstrap, and a minimal CLI. Aether now needs a higher-level
orchestration layer that can manage controlled startup, shutdown, module
registration, dependency validation, capability discovery, health checks, and
future plugin preparation.

The Runtime should remain small and executable. The Kernel should coordinate
runtime primitives without absorbing every low-level responsibility.

## Decision

Create `aether-kernel` as a Rust crate above `aether-runtime`.

The Kernel owns:

- kernel identity
- module registry
- dependency validation
- lifecycle tracking
- capability discovery
- kernel health reporting
- kernel telemetry

The Runtime remains responsible for:

- runtime start and stop
- concrete module loading
- event emission
- runtime health checks

## Consequences

- Phase 1 compatibility is preserved.
- Module orchestration has a dedicated boundary.
- Future plugins can integrate through registration and capability contracts.
- Kernel state can evolve without bloating the Runtime.
- No AI, agents, UI, OS capture, or business behavior is introduced by this ADR.
