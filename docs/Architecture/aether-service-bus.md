# Aether Service Bus

The Aether Service Bus, or ASB, is the official internal communication layer for
services.

The initial implementation is in-memory. Its abstraction is designed so future
implementations can target NATS, Redis Streams, gRPC, local IPC, or cloud buses
through a new ADR.

## Supported Phase 3 Operations

- Publish event.
- Subscribe event.
- Local request/reply.
- Service command routing by bus route.
- Service notification broadcast.
- Bus status inspection.

## Permission Enforcement

ASB checks declared permissions before service actions:

- `event.publish` is required to publish events and notifications.
- `event.subscribe` is required to subscribe to events.
- `service.command` is required for request/reply and service commands.

No service action should be executed without the required declared permission.

## Boundary Rule

Services do not receive direct references to other services and do not route
commands by direct service identity. The bus is the mediator and routing
boundary.

## Current Limits

The Phase 3 ASB is synchronous and local. It does not provide distributed
delivery guarantees, durable queues, authentication, encryption, retries,
backpressure, or production transport adapters.
