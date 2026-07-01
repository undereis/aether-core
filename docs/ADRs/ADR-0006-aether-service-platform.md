# ADR-0006

## Aether Service Platform

## Status

Accepted.

## Context

The Kernel established controlled runtime orchestration, module registration,
lifecycle, capabilities, health, telemetry, and typed IDs. Aether now needs a
platform layer where internal resources can be modeled as services rather than
generic modules.

The service model must support future decades of evolution without introducing
AI, agents, authentication, business schemas, or UI behavior in Phase 3.

## Decision

Introduce the Aether Service Platform with these crates:

- `aether-service`
- `aether-service-bus`
- `aether-permissions`
- `aether-resources`

The Kernel owns the Service Registry and ASB instance. Services declare
capabilities, required capabilities, requested permissions, resources,
lifecycle, health, and a source manifest.

## Consequences

- New platform resources can be registered and inspected as services.
- Service dependencies are expressed through capabilities, not direct service
  references.
- Health aggregation can summarize platform state.
- Phase 1/2 module compatibility is preserved.
- No AI, agents, authentication, business persistence, frontend, or Memory
  Engine behavior is introduced.
