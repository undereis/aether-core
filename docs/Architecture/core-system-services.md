# Core System Services

Status: Phase 4 baseline.

Core System Services are the first real services registered through the Aether
Service Platform. They are intentionally thin: each service has a manifest,
descriptor, capability declaration, permission declaration, resource declaration,
health status, and ASB command route.

No business behavior, AI, agents, memory engine, knowledge graph, frontend,
desktop runtime, authentication, or production persistence is implemented in
this phase.

## Implemented Services

| Service | Manifest | ASB Route | Purpose |
| --- | --- | --- | --- |
| Telemetry Service | `core/services/telemetry-service/service.toml` | `system.telemetry` | Base telemetry contracts. |
| Configuration Service | `core/services/configuration-service/service.toml` | `system.configuration` | Local configuration contracts. |
| Health Service | `core/services/health-service/service.toml` | `system.health` | Health reporting and aggregation contracts. |
| Event Service | `core/services/event-service/service.toml` | `system.events` | Internal event routing contracts. |
| Service Inspector Service | `core/services/service-inspector/service.toml` | `system.inspector` | Service descriptor and capability inspection contracts. |

## Runtime Relationship

The `aether-system-services` crate loads the service manifests and creates
`ServiceDescriptor` values. The Kernel remains the owner of registration,
permissions, health aggregation, and the in-memory ASB instance.

## ASB Usage

Each core system service exposes a command handler through an ASB route. The
handler returns a small inspection response containing service identity, route,
command name, and health. This validates the communication contract without
adding domain behavior.

Services do not call each other directly. Any future service interaction must
use ASB event, request/reply, command, or notification contracts.

## Capabilities

- Telemetry Service: `telemetry.emit`, `telemetry.query`, `telemetry.health`
- Configuration Service: `config.read`, `config.validate`,
  `config.provider.local`
- Health Service: `health.report`, `health.aggregate`, `health.degradation`
- Event Service: `events.publish`, `events.subscribe`, `events.route`
- Service Inspector Service: `service.inspect`, `service.catalog`,
  `service.capabilities.query`

## Permissions

Permissions are declared in manifests and registered with the ASB permission
index by the Kernel:

- `event.publish`
- `event.subscribe`
- `config.read`
- `telemetry.emit`
- `service.inspect`
- `service.command`

## Resources

All Phase 4 services are low resource consumers. The Configuration Service
declares read-only filesystem access for local configuration providers. Other
base services declare no filesystem access and no network access.

## Validation

Phase 4 tests cover:

- official service map completeness;
- manifest loading;
- registration for each base service;
- capability queries;
- declared permissions;
- declared resources;
- health aggregation;
- no direct service-to-service coupling;
- ASB command route behavior;
- CLI inspection commands.
