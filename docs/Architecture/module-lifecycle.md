# Module Lifecycle

Aether modules move through explicit lifecycle states tracked by the Kernel.

## States

- `Created`: descriptor exists but has not been registered.
- `Registered`: descriptor is accepted by the Kernel registry.
- `Initializing`: startup is in progress.
- `Running`: module is available.
- `Degraded`: module is partially available.
- `Stopping`: shutdown is in progress.
- `Stopped`: module stopped cleanly.
- `Failed`: lifecycle execution failed.

## Current Transition Rules

The Phase 2 transition matrix is intentionally conservative:

- `Created` -> `Registered` or `Initializing`
- `Registered` -> `Initializing` or `Stopped`
- `Initializing` -> `Running`, `Degraded`, or `Failed`
- `Running` -> `Degraded`, `Stopping`, or `Failed`
- `Degraded` -> `Running`, `Stopping`, or `Failed`
- `Stopping` -> `Stopped` or `Failed`
- `Stopped` -> `Initializing`
- `Failed` -> `Initializing`

The Kernel uses these states for module orchestration. The same enum may also
represent the Kernel bootstrap status where that is useful.

## Health Mapping

Lifecycle is not identical to health, but the Kernel derives a baseline health
view from lifecycle status:

- `Running` maps to healthy.
- `Registered`, `Initializing`, and `Degraded` map to degraded.
- `Created`, `Stopping`, `Stopped`, and `Failed` map to unhealthy.

Future phases may let modules report richer health details, but the current
contract is enough to validate controlled lifecycle behavior.
