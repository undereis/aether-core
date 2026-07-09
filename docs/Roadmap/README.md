# Roadmap

## Phase 0: Foundation

- Toolchain validation.
- Monorepo structure.
- Backend health/version surface.
- Local infrastructure containers.
- Engineering governance.

## Phase 1: Core Runtime

- Rust core runtime crates.
- Base event contracts and in-memory event bus.
- Local configuration and structured logging.
- Module lifecycle contract.
- Runtime bootstrap and validation CLI.

## Phase 2: Aether Kernel

- Kernel orchestration above the Runtime.
- Module registry and dependency validation.
- Lifecycle states and health reporting.
- Capability declaration and discovery.
- EventBus and ConfigProvider abstractions.
- Telemetry abstraction.
- Typed ID strategy.

## Phase 3: Aether Service Platform

- Service Model and Service Manifest.
- Service Registry owned by the Kernel.
- Aether Service Bus.
- Advanced capability model.
- Initial permission model.
- Initial resource model.
- Service lifecycle supervision.
- Health aggregation.
- Service and bus inspection CLI.

## Phase 4: Official Service Map + Core System Services

- Official service map across platform layers.
- Base Telemetry, Configuration, Health, Event, and Service Inspector services.
- Declarative service manifests under `core/services`.
- `aether-system-services` crate for loading and registering base services.
- CLI inspection for service map and core system services.

## Phase 4.5: Kernel Decomposition

- Kernel responsibilities reduced to coordination.
- Manager Layer made official.
- Driver Layer contracts introduced.
- Domain Layer contracts introduced.
- Policy Layer contracts introduced.
- Contract Bus base introduced on top of the ASB.
- Foundation Era completed.

## Phase 4.6: Aether Engineering Protocol

- Aether Engineering Protocols established.
- Architecture Constitution v2 created.
- Architecture Bible created.
- Architecture Evolution Roadmap created.
- Architectural Gravity classification created.
- Architecture Guardian role instituted.

## CDR-0001: Cognitive Design Review

- CDR-0001 drafted.
- Official cognitive architecture defined for review.
- Cognitive Pipeline defined before any Era II implementation.
- Memory, Knowledge, Context, Planning, Reasoning, Decision, Action, Learning,
  and Inference boundaries documented.
- Era II implementation remains blocked until CDR and domain RFC approval.

## RFC-0001: Memory Domain

- RFC-0001 Memory Domain drafted.
- Memory philosophy, boundaries, types, lifecycle, provenance, confidence,
  retention, forgetting, retrieval, promotion, and storage strategy defined for
  architecture review.
- RFC-0001 Revision 1 drafted with Cognitive Memory Metabolism, Memory Score
  Model, Cognitive DNA, Memory Relationship Model, Cognitive Memory Events,
  Memory Evolution, Conflict Resolution, Explainability, Cognitive Retrieval,
  Cognitive Memory Principles, and Cognitive Memory Ecosystem.
- AEP-0016 Cognitive Traceability accepted as official governance for future
  cognitive traceability.
- Memory implementation remains blocked until RFC approval, ADR approval,
  public contracts, policies, and test strategy are complete.

## ADR-0010: Memory Domain Implementation Architecture

- ADR-0010 Memory Domain Implementation Architecture drafted.
- Future Memory implementation architecture proposed without code changes.
- Proposed future crates, contracts, policies, cognitive memory events, storage,
  indexing, traceability, explainability, testing, and phased implementation
  plan documented.
- Memory implementation remains blocked until ADR-0010 is approved and the
  required contracts, policies, storage strategy, and test strategy are
  accepted.

## ARR-0001: Architecture Readiness Review

- ARR-0001 Architecture Readiness Review drafted.
- Current architecture reviewed for readiness to proceed to RFC-0002 Knowledge
  Domain as a documentation-only phase.
- Decision drafted as ready with minor cautions.

## RFC-0002: Knowledge Domain

- RFC-0002 Knowledge Domain drafted.
- Knowledge defined as structured truth, separate from Memory experience and
  Context present-state assembly.
- Evidence, provenance, confidence, versioning, contradiction, invalidation,
  policies, events, contracts, storage strategy, indexing strategy,
  traceability, explainability, risks, and future implementation phases defined
  for architecture review.
- Knowledge implementation remains blocked until RFC approval, implementation
  ADR approval, contracts, policies, storage strategy, and test strategy are
  accepted.

## RFC-0003: Context Domain

- RFC-0003 Context Domain drafted as a documentation-only cognitive-domain
  RFC.
- Context defined as present-state assembly, separate from Memory experience
  and Knowledge structured truth.
- Context boundaries, lifecycle, freshness, traceability, policies, events,
  contracts, manager/service responsibilities, efficient intelligence
  considerations, risks, anti-patterns, and future implementation phases
  documented.
- Context implementation remains blocked until RFC approval, implementation
  ADR approval, contracts, policies, storage/non-storage strategy, and test
  strategy are accepted.

## AEP-0016: Cognitive Traceability

- AEP-0016 Cognitive Traceability accepted.
- Future Memory and Knowledge implementations must comply with AEP-0016.
- Future Context, Reasoning, Planning, Decision, Learning, and Action designs
  must consider AEP-0016 from the start.
- Traceability governance remains documentation-only until future
  implementation phases define concrete contracts and tests.

## RFC-0009: Efficient Intelligence & Energy-Aware Architecture

- RFC-0009 Efficient Intelligence & Energy-Aware Architecture drafted as a
  transversal architecture direction.
- RFC-0004 through RFC-0008 remain reserved for the approved cognitive domain
  sequence.
- Future cognitive and intelligence work should consider cost, tokens, latency,
  energy, model routing, cache, context compression, tool-first execution, and
  safety governance from the design phase.
- No functional implementation, dependency, routing layer, cache, model
  selection, or execution mode is introduced by RFC-0009.

## Architecture Milestones

| Version | Milestone |
| --- | --- |
| `v0.2.0` | Kernel |
| `v0.3.0` | Service Platform |
| `v0.4.0` | Core System Services |
| `v0.4.5` | Kernel Decomposition |
| `v0.4.6` | Aether Engineering Protocol |
| `v0.5.0` | Cognitive Design Review |

## Era Roadmap

The formal Era roadmap is maintained in
`docs/Architecture/Architecture-Evolution-Roadmap.md`.

## Later Phases

- Era II: Cognitive Core.
- Era III: Intelligence.
- Era IV: Automation.
- Era V: Enterprise.
- Era VI: Distributed.

Each phase requires updated ADRs before implementation.
