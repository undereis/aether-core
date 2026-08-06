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

- CDR-0001 accepted and published in `v0.5.0-cognitive-design-review`.
- Official cognitive architecture defined and checkpointed before
  implementation.
- Cognitive Pipeline defined before any Era II implementation.
- Memory, Knowledge, Context, Planning, Reasoning, Decision, Action, Learning,
  and Inference boundaries documented.
- Era II implementation remains blocked until CDR and domain RFC approval.

## RFC-0001: Memory Domain

- RFC-0001 Memory Domain Revision 1 accepted and published in
  `v0.5.1-memory-domain-rfc`.
- Memory philosophy, boundaries, types, lifecycle, provenance, confidence,
  retention, forgetting, retrieval, promotion, and storage strategy defined for
  architecture review.
- RFC-0001 Revision 1 defines Cognitive Memory Metabolism, Memory Score
  Model, Cognitive DNA, Memory Relationship Model, Cognitive Memory Events,
  Memory Evolution, Conflict Resolution, Explainability, Cognitive Retrieval,
  Cognitive Memory Principles, and Cognitive Memory Ecosystem.
- AEP-0016 Cognitive Traceability accepted as official governance for future
  cognitive traceability.
- Memory implementation remains blocked until RFC approval, ADR approval,
  public contracts, policies, and test strategy are complete.

## ADR-0010: Memory Domain Implementation Architecture

- ADR-0010 Memory Domain Implementation Architecture accepted and published in
  `v0.5.2-memory-implementation-adr`.
- Future Memory implementation architecture proposed without code changes.
- Proposed future crates, contracts, policies, cognitive memory events, storage,
  indexing, traceability, explainability, testing, and phased implementation
  plan documented.
- Memory implementation remains blocked until ADR-0010 is approved and the
  required contracts, policies, storage strategy, and test strategy are
  accepted.

## ARR-0001: Architecture Readiness Review

- ARR-0001 Architecture Readiness Review completed and published in
  `v0.5.3-architecture-readiness-review`.
- Current architecture reviewed for readiness to proceed to RFC-0002 Knowledge
  Domain as a documentation-only phase.
- Decision completed as ready with minor cautions.

## RFC-0002: Knowledge Domain

- RFC-0002 Knowledge Domain accepted and published in
  `v0.5.4-knowledge-domain-rfc`.
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

- RFC-0003 Context Domain accepted and published as a documentation-only
  cognitive-domain RFC in `v0.5.7-context-domain-rfc`.
- Context defined as present-state assembly, separate from Memory experience
  and Knowledge structured truth.
- Context boundaries, lifecycle, freshness, traceability, policies, events,
  contracts, manager/service responsibilities, efficient intelligence
  considerations, risks, anti-patterns, and future implementation phases
  documented.
- Context implementation remains blocked until RFC approval, implementation
  ADR approval, contracts, policies, storage/non-storage strategy, and test
  strategy are accepted.

## RFC-0004: Planning Domain

- RFC-0004 Planning Domain is accepted and published as documentation-only
  architecture in `v0.5.8-planning-domain-rfc`.
- Planning structures possible futures; Reasoning evaluates implications and
  uncertainty; Decision chooses; Action executes.
- Planning boundaries, candidate Plan model, lifecycle, traceability,
  efficiency, risks, non-goals, and future test strategy are defined for
  future implementation governance.
- Planning remains unimplemented.
- Planning implementation remains blocked until a future implementation ADR,
  contracts, policies, storage/non-storage strategy, test strategy, and
  Architecture Guardian authorization.

## RFC-0005: Reasoning Domain

- RFC-0005 Reasoning Domain is a Draft documentation-only architecture artifact.
- Reasoning evaluates implications, relationships, hypotheses, evidence,
  contradictions, and uncertainty within an authorized scope.
- Assessments are structured, explainable, and non-binding; Decision chooses
  and Action executes.
- Reasoning remains unimplemented. No Reasoning Engine, Service, Manager, API,
  storage, provider runtime, or agent has been created.
- The final architectural audit of the RFC-0005 Draft has been completed, and
  the internal blockers it identified have been corrected.
- RFC-0005 remains not published, not tagged, and not implemented. Any future
  checkpoint, tag, or publication requires separate explicit authorization and
  a controlled task.
- Service Map reconciliation remains required before RFC-0005 may be promoted
  to Accepted or Published. No cognitive implementation has started.
- RFC-0006 Decision Domain remains reserved for a later documentation phase.

## AEP-0016: Cognitive Traceability

- AEP-0016 Cognitive Traceability accepted and published in
  `v0.5.5-cognitive-traceability-aep`.
- Future Memory and Knowledge implementations must comply with AEP-0016.
- Future Context, Reasoning, Planning, Decision, Learning, and Action designs
  must consider AEP-0016 from the start.
- Traceability governance remains documentation-only until future
  implementation phases define concrete contracts and tests.

## RFC-0009: Efficient Intelligence & Energy-Aware Architecture

- RFC-0009 Efficient Intelligence & Energy-Aware Architecture accepted and
  published as a transversal architecture direction in
  `v0.5.6-efficient-intelligence-rfc`.
- RFC-0005 Reasoning Domain is a Draft, documentation-only, not published, not
  implemented, and under architectural review pending a future checkpoint and
  publication. RFC-0006 through RFC-0008 remain reserved for the approved
  cognitive domain sequence.
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
| `v0.5.1` | Memory Domain RFC |
| `v0.5.2` | Memory Implementation Architecture ADR |
| `v0.5.3` | Architecture Readiness Review |
| `v0.5.4` | Knowledge Domain RFC |
| `v0.5.5` | Cognitive Traceability AEP |
| `v0.5.6` | Efficient Intelligence transversal RFC |
| `v0.5.7` | Context Domain RFC |
| `v0.5.8` | Planning Domain RFC |

Tags `v0.5.0` through `v0.5.8` are published on the remote. RFC-0004 Planning
Domain is accepted and published as documentation-only architecture. RFC-0005
Reasoning Domain is in Draft documentation review and is not published or
implemented. RFC-0006 Decision Domain remains reserved. No cognitive
implementation has started.

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
