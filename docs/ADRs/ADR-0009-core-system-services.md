# ADR-0009

## Core System Services

## Status

Accepted.

## Context

Phase 3 introduced the Aether Service Platform, Service Model, manifests,
permissions, resources, health aggregation, and the ASB. Phase 4 needs the first
real services, but must not implement business logic, AI, agents, memory,
frontend, desktop runtime, or authentication.

## Decision

Introduce `aether-system-services` as the crate that owns the declarative base
for Phase 4 system services:

- Telemetry Service
- Configuration Service
- Health Service
- Event Service
- Service Inspector Service

Each service is loaded from a TOML manifest under `core/services`, converted
into a `ServiceDescriptor`, registered through the Kernel, and exposed through
an ASB command route.

The Kernel continues to own the Service Registry and the ASB instance. The new
crate does not become an alternate runtime or a parallel registry.

## Consequences

- Core services can be inspected through the CLI.
- Manifests become the source of truth for service declarations.
- Health aggregation now has real registered services.
- ASB route handlers validate the communication model without adding domain
  behavior.
- Future phases can expand service internals while preserving the same
  manifest, permission, resource, and ASB boundaries.

## Phase 4.5 Addendum

Phase 4.5 refines internal ownership without changing the Phase 4 service
model. The Kernel remains the public coordination facade. Service registry,
service discovery, service inspection, service health aggregation, and ASB
permission registration are now owned by `ServiceManager`.
