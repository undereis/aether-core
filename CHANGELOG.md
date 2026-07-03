# Changelog

## Unreleased

- CDR-0001 Cognitive Design Review drafted.

## v0.4.6-aether-engineering-protocol

- Established the Aether Engineering Protocols.
- Added Architecture Constitution v2.
- Added Architecture Bible.
- Added Architecture Evolution Roadmap.
- Added Architectural Gravity classification.
- Instituted the Architecture Guardian role.
- Documented that future phases must obey AEP governance.

## v0.4.5-kernel-decomposition

- Phase 4.5 approved.
- Foundation Era completed.
- Kernel Decomposition completed.
- Manager Layer made official.
- Driver Layer made official.
- Domain Layer made official.
- Policy Layer made official.
- Contract Bus Base introduced.
- Added the Manager Layer infrastructure.
- Moved module registry and lifecycle ownership behind `LifecycleManager`.
- Moved service registry and service health ownership behind `ServiceManager`.
- Added Driver, Domain, and Policy layer contracts.
- Added typed IDs for managers, drivers, domains, and policies.
- Added the Contract Bus base on top of the existing ASB.
- Added Phase 4.5 architecture documentation.

## v0.4.0-core-system-services

- Added the official Aether service map across Foundation, Cognitive, AI,
  Interaction, Device, Automation, and Enterprise layers.
- Added base manifests for Telemetry, Configuration, Health, Event, and Service
  Inspector services.
- Added `aether-system-services` for loading and registering core system
  services through the Kernel and ASB.
- Added CLI commands for service map and core system service inspection.
- Added Phase 4 architecture docs and ADRs.

## v0.3.0-service-platform

- Added Aether Service Platform foundation.
- Added Service Model and Service Manifest support.
- Added Aether Service Bus with in-memory implementation.
- Added internal permission and resource models.
- Added service registry queries and health aggregation.
- Added service and bus inspection commands to the CLI.

## v0.2.0-kernel

- Added Aether Kernel orchestration layer.
- Added module registry, lifecycle tracking, health checks, telemetry, and typed
  ID strategy.

## v0.1.0-foundation

- Established the initial Aether monorepo foundation, backend health surface,
  Docker infrastructure, Rust core runtime, and engineering documentation.
