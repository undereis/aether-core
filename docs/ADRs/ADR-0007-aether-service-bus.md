# ADR-0007

## Aether Service Bus

## Status

Accepted.

## Context

Services in a cognitive operating system must not become a mesh of direct
in-process dependencies. Direct service knowledge would make supervision,
security, permissions, telemetry, and future distribution harder.

## Decision

Introduce the Aether Service Bus as the only service communication boundary.

The Phase 3 ASB is in-memory and supports:

- event publish
- event subscribe
- local request/reply
- service command routing by bus route
- service notification broadcast
- bus status inspection

The ASB enforces declared permissions for bus actions.
Commands are routed through bus routes rather than direct service references.

## Engineering Rule #002

No service may know directly about another service. All service communication
must pass through the Aether Service Bus.

## Consequences

- Service coupling is constrained at the platform boundary.
- Future transports can be introduced behind the ASB abstraction.
- Permission checks have a natural enforcement point.
- The initial implementation remains local and simple.
