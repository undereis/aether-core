# ADR-0005

## Typed ID Strategy

## Status

Accepted.

## Context

Aether will eventually coordinate many internal entities across runtime,
kernel, modules, capabilities, events, plugins, and external integrations.
Plain strings are easy to create but become ambiguous over time.

The project needs a simple strategy for readable, type-oriented identifiers
without introducing a distributed ID service or persistence dependency in Phase
2.

## Decision

Introduce `aether-ids` with prefix-based typed identifiers.

Initial prefixes:

- `evt_` for events
- `mod_` for modules
- `ker_` for kernels
- `cap_` for capabilities

Phase 3 and Phase 4.5 extend the same strategy with:

- `svc_` for services
- `mgr_` for managers
- `drv_` for drivers
- `dom_` for domains
- `pol_` for policies

Typed IDs support generated `UUIDv7` suffixes and stable human-provided suffixes
for local descriptors.

The Phase 2 implementation does not force a full migration of Phase 1 IDs where
that would be unnecessarily disruptive. It introduces the strategy and applies
it where safe:

- Kernel IDs use `ker_`.
- Capability IDs use `cap_`.
- Module IDs can be created with `mod_`.
- Event IDs expose a typed `evt_` representation while retaining the Phase 1
  event ID contract.

## Consequences

- New components can use predictable typed identifiers.
- Logs, events, telemetry, and diagnostics become easier to scan.
- The strategy stays local and dependency-free beyond Rust crate dependencies.
- Future migrations should be documented in new ADRs when they change serialized
  contracts.
