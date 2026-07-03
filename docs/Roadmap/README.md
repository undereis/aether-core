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

## Architecture Milestones

| Version | Milestone |
| --- | --- |
| `v0.2.0` | Kernel |
| `v0.3.0` | Service Platform |
| `v0.4.0` | Core System Services |
| `v0.4.5` | Kernel Decomposition |
| `v0.4.6` | Aether Engineering Protocol |

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
