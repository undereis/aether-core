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

## Phase 4.5 Contract Bus Base

The ASB now includes a base Contract Bus abstraction for typed internal
contracts:

- `BusContract`;
- `ContractRequest`;
- `ContractReply`;
- `ContractHandler`;
- `AetherContractBus`.

The current implementation is still in-memory and local. The Contract Bus base
does not introduce business contracts; it only prepares a typed routing surface
so future services can communicate through versioned internal contracts instead
of ad hoc command names.

## Permission Enforcement

The Kernel-owned `ServiceBusController` approves identities, permissions, and
route ownership. Services receive only an identity-bound `ServiceBusClient`;
they cannot alter grants, choose another caller identity, or replace a route.

The client checks approved permissions before service actions:

- `event.publish` is required to publish events and notifications.
- `event.subscribe` is required to subscribe to events.
- `service.command` is required for request/reply and service commands.

No service action is executed without the required approved permission.
Duplicate route registration and notification identity spoofing are rejected.

## Boundary Rule

Services do not receive the administrative controller or direct references to
other services. The bus client is the mediated data-plane boundary; the
controller remains inside the Kernel control plane.

## Current Limits

The Phase 3 ASB is synchronous and local. It does not provide distributed
delivery guarantees, durable queues, authentication, encryption, retries,
backpressure, schema negotiation, or production transport adapters.
