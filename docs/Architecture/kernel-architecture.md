# Kernel Architecture

The Aether Kernel is the coordination layer of the cognitive operating system.
It must stay small, explicit, and free of business rules.

## Rule

The Kernel never executes business logic.

It only:

- starts the platform;
- shuts the platform down safely;
- supervises runtime state;
- registers descriptors through managers;
- discovers platform metadata through managers;
- coordinates recovery paths;
- emits foundational telemetry.

Operational intelligence belongs to Managers. Service behavior belongs to
Services. Native adaptation belongs to Drivers. Long-lived responsibility
boundaries belong to Domains. Behavioral constraints belong to Policies.

## Current Ownership

In Phase 4.5 the Kernel owns:

- `KernelId`;
- `AetherRuntime`;
- `ManagerRegistry`;
- `LifecycleManager`;
- `ServiceManager`;
- `TelemetryEmitter`;
- kernel lifecycle status.

The Kernel no longer owns the module registry or service registry directly.
Those are delegated to Managers while public compatibility methods remain
available on `AetherKernel`.

## Compatibility Facade

The following Kernel methods remain available for earlier phases:

- `register_module`;
- `load_module`;
- `modules`;
- `capabilities`;
- `register_service`;
- `services`;
- `service_health`;
- `service_registry`;
- `service_bus`.

Internally these methods delegate to `LifecycleManager` and `ServiceManager`.
This keeps Fases 1-4 compatible while preventing Kernel growth into a God
Object.

## Non-Goals

Phase 4.5 does not add AI, agents, Memory Engine, Knowledge Engine,
authentication, frontend, desktop, persistence, or real OS capture.
