# Service Model

Aether services are Kernel-controlled platform resources.

## Descriptor Fields

A service descriptor contains:

- `id`
- `name`
- `version`
- `description`
- `owner`
- `capabilities`
- `permissions`
- `dependencies`
- `resources`
- `lifecycle_status`
- `health_status`
- `manifest`

## Service Manifest

Service manifests are declarative TOML documents. They describe service intent
without instantiating business behavior.

```toml
[service]
name = "telemetry-service"
version = "0.1.0"
description = "Base telemetry service"
owner = "neuroforge-labs"

[capabilities]
provides = ["telemetry.emit", "telemetry.query"]
requires = ["events.publish"]

[permissions]
requested = ["event.publish", "config.read"]

[resources]
cpu_class = "low"
memory_class = "low"
storage_class = "none"
network = false
```

## Dependencies

Service dependencies are modeled as required capabilities, not direct service
references. This supports Engineering Rule #002 and allows the Kernel to resolve
providers without coupling services to each other.

## Compatibility

The module model remains available for Phase 1/2 compatibility. Service is the
preferred abstraction for new platform resources introduced from Phase 3 onward.
