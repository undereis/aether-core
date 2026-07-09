# Changelog

## Unreleased

- Reconciled governance status, remote checkpoint records, and the approved
  cognitive RFC sequence before RFC-0004 Planning Domain.

No unreleased product, runtime, or architecture implementation changes.

## v0.5.7-context-domain-rfc

- Accepted and published RFC-0003 Context Domain as architecture documentation.
- Defined Context as authorized present-state assembly without implementing a
  Context Engine or runtime behavior.

## v0.5.6-efficient-intelligence-rfc

- Accepted and published RFC-0009 Efficient Intelligence & Energy-Aware
  Architecture as a transversal architecture direction.
- Preserved RFC-0003 through RFC-0008 for the approved cognitive-domain
  sequence without implementing routing, cache, compression, AI, or agents.

## v0.5.5-cognitive-traceability-aep

- Accepted and published AEP-0016 Cognitive Traceability as official governance
  for future cognitive design and implementation.

## v0.5.4-knowledge-domain-rfc

- Accepted and published RFC-0002 Knowledge Domain as architecture
  documentation.
- Defined Knowledge as structured truth without implementing Knowledge,
  storage, graph, vector search, AI, or agents.

## v0.5.3-architecture-readiness-review

- ARR-0001 Architecture Readiness Review drafted.

## v0.5.2-memory-implementation-adr

- ADR-0010 Memory Domain Implementation Architecture drafted.

## v0.5.1-memory-domain-rfc

- RFC-0001 Revision 1 Memory Domain Architectural Review drafted.
- RFC-0001 Memory Domain drafted.
- Proposed AEP-0016 Cognitive Traceability.

## v0.5.0-cognitive-design-review

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
