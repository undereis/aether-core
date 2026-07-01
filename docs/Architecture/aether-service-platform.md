# Aether Service Platform

The Aether Service Platform, or ASP, is the internal service layer controlled by
the Kernel.

ASP introduces services as the progressively preferred abstraction above the
Phase 1/2 module model. Modules remain supported for compatibility, but new
internal platform resources should be modeled as services when they need
capabilities, permissions, resources, health, and bus communication.

## Responsibilities

- Define the service descriptor model.
- Load declarative service manifests.
- Register services through the Kernel.
- Track service capabilities, permissions, resources, lifecycle, and health.
- Aggregate service platform health.
- Provide an official internal communication path through the Aether Service Bus.

## Kernel Relationship

The Kernel owns the Service Registry and the in-memory ASB instance.

The Runtime still executes lower-level module lifecycle. ASP adds a controlled
service platform above the Kernel and below future product features.

## Engineering Rule #002

No service may know or call another service directly.

All service communication must pass through the Aether Service Bus. Direct
service references, direct service method calls, or direct service-owned channel
sharing are architectural violations.

## Current Limits

Phase 3 does not implement AI, agents, authentication, business persistence,
frontend, desktop UI, Memory Engine, or production service sandboxing.
