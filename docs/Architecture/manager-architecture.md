# Manager Architecture

Managers hold operational intelligence that should not live inside the Kernel.

Each Manager has:

- `id`;
- `name`;
- `version`;
- `health`;
- `capabilities`;
- `manifest`.

Managers are not user-facing features. They are platform control components
used to keep Kernel responsibilities narrow.

## Managers Introduced

- `ServiceManager`: service registration, discovery, reload contract, inspect,
  health, and version.
- `LifecycleManager`: module descriptor registration, lifecycle transitions,
  module health, and module capability indexes.
- `ResourceManager`: future resource declaration and enforcement coordination.
- `TelemetryManager`: future telemetry routing, metrics, tracing, and health.
- `ConfigurationManager`: future configuration provider coordination.
- `PermissionManager`: future permission declaration and enforcement
  coordination.
- `HealthManager`: future health aggregation and controlled degradation.
- `DriverManager`: future driver registration, inspection, and health.
- `PluginManager`: future plugin registration, inspection, and health.

## Current Implementation

Phase 4.5 implements infrastructure only.

`ServiceManager` currently wraps the service registry and ASB permission
indexing. `LifecycleManager` currently wraps the module registry and lifecycle
state transitions.

The remaining Managers are descriptor-backed contracts with health,
capabilities, and manifests. They intentionally do not implement domain logic.

## Dependency Rule

Managers may coordinate platform contracts, but they must not bypass the Aether
Service Bus for service-to-service communication. Engineering Rule #002 remains
active.
