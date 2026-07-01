# ADR-0008

## Official Service Map

## Status

Accepted.

## Context

Aether is a cognitive operating system, not a single application. Without an
official service map, future phases could add isolated features that bypass the
Kernel, the Service Model, permissions, resources, health, and ASB contracts.

## Decision

Create the Official Service Map as the planning baseline for all future Aether
services.

The map groups services by layers:

- Foundation Services
- Cognitive Services
- AI Services
- Interaction Services
- Device Services
- Automation Services
- Enterprise Services

Each service is classified, assigned capabilities, required capabilities,
probable permissions, probable resources, priority, probable implementation
phase, risks, and allowed dependencies.

Engineering Rule #002 remains mandatory: no service may know or call another
service directly; all communication must pass through the Aether Service Bus.

## Consequences

- Future work has a stable architectural vocabulary.
- Service responsibilities can evolve without becoming application features.
- Dependencies are discussed as capability contracts, not object references.
- The map is intentionally broader than Phase 4 implementation.
