# ADR-0010

## Memory Domain Implementation Architecture

## Status

Accepted and published.

Reconciliation note: ADR-0010 was accepted and published in
`v0.5.2-memory-implementation-adr`. AEP-0016 was proposed when this decision was
written and was later accepted in `v0.5.5-cognitive-traceability-aep`.
Historical proposal references below preserve that review context. Current and
future Memory work must comply with the accepted AEP-0016. Memory remains
unimplemented.

## Context

CDR-0001 defined the cognitive architecture of Aether and established that the
Cognitive Core owns intelligence while models only provide inference.

RFC-0001 defined the Memory Domain as a policy-governed cognitive domain for
experience, retention, retrieval, provenance, confidence, forgetting,
explainability, and evolution.

AEP-0016 proposed Cognitive Traceability as a future governance protocol for
traceable, explainable, and auditable cognitive components.

This ADR defines the future implementation architecture for the Memory Domain.
It does not create code, crates, services, managers, schemas, APIs, structs,
traits, migrations, runtime behavior, Memory Engine behavior, Knowledge
behavior, AI behavior, or agent behavior.

## Decision

The Memory Domain will be implemented as an independent cognitive domain,
governed by a future Memory Manager, executed by a future Memory Service,
protected by Memory Policies, accessed through typed contracts, and
communicated exclusively through ASB or Contract Bus.

Memory will not be a generic database.

Memory will not be Knowledge.

Memory will not be Context.

Memory will not be LLM-owned memory.

Memory will not live in the Kernel.

The Kernel will remain a coordinator. Managers will coordinate domain
governance. Services will execute declared memory capabilities. Policies will
govern sensitive behavior. Storage will remain an implementation detail behind
contracts.

## Proposed Future Crates

The future implementation may introduce these crates, subject to approval at
the implementation phase:

- `core/crates/aether-memory`
  - Owns Memory Domain models, lifecycle vocabulary, and domain-level
    invariants.
  - Should be introduced only when memory concepts need concrete type
    definitions.
- `core/crates/aether-memory-contracts`
  - Owns typed Contract Bus request and response surfaces for memory
    operations.
  - Should be introduced before runtime behavior so callers depend on
    contracts, not implementation.
- `core/crates/aether-memory-policies`
  - Owns memory-specific policy contracts and policy evaluation vocabulary.
  - May initially contain interfaces and policy result types before full
    enforcement exists.
- `core/crates/aether-memory-store`
  - Owns storage ports and adapters for in-memory, PostgreSQL, Qdrant, and
    future relationship storage.
  - Should be deferred until contracts and policies are stable enough to avoid
    storage-driven architecture.

To reduce risk, implementation should begin with contracts and policies before
creating durable storage. `aether-memory-store` can be delayed until an
in-memory prototype proves lifecycle, traceability, and policy boundaries.

## Memory Manager Architecture

The future Memory Manager governs the Memory Domain.

The Memory Manager does not store memory records directly.

The Memory Manager does not call Services directly.

The Memory Manager coordinates through ASB or Contract Bus.

Future responsibilities:

- lifecycle coordination;
- policy orchestration;
- health aggregation;
- retention coordination;
- forgetting coordination;
- promotion coordination;
- relationship coordination;
- traceability coordination;
- event coordination;
- storage abstraction coordination;
- compliance coordination.

The Memory Manager may supervise contracts, health, and policy flow, but it
must not execute retrieval algorithms, store durable memory, call LLM providers,
or become a hidden Memory Engine.

## Memory Service Architecture

The future Memory Service executes declared memory capabilities.

The Memory Service does not govern the Memory Domain.

The Memory Service does not call Knowledge, Context, Reasoning, Learning, or
Action services directly.

Future responsibilities:

- create memory;
- retrieve memory;
- update memory;
- archive memory;
- expire memory;
- forget memory;
- delete memory;
- promote memory;
- link memory;
- score memory;
- explain memory;
- trace memory;
- emit memory events.

The Memory Service must declare capabilities, permissions, resources,
dependencies, lifecycle status, health status, and communication routes through
the Service Model. Any communication with other cognitive areas must use ASB or
Contract Bus contracts.

## Memory Contracts

Future contracts are conceptual in this ADR. They are not Rust structs, APIs,
or serialized schemas yet.

### CreateMemory

Objective: capture an authorized memory candidate.

Conceptual input: actor, owner boundary, memory type, content reference,
source, evidence, retention intent, privacy level, policy context, trace
context.

Conceptual output: memory identifier, lifecycle state, provenance summary,
policy result, emitted event references.

Policy checks: MemoryPolicy, PrivacyPolicy, RetentionPolicy, ProvenancePolicy,
EvidencePolicy, TraceabilityPolicy.

Events emitted: MemoryCaptured, MemoryPolicyDenied when denied, and
MemoryTraceRecorded when trace metadata is recorded.

Risks: hidden memory, sensitive data retention, missing provenance, content
stored without retention.

### RetrieveMemory

Objective: return policy-visible memory candidates.

Conceptual input: retrieval intent, actor, owner boundary, query, context
summary, retrieval strategy, policy context, trace context.

Conceptual output: ranked candidates, score dimensions, explanation, policy
visibility, trace metadata.

Policy checks: MemoryPolicy, PrivacyPolicy, RetrievalPolicy,
TraceabilityPolicy.

Events emitted: MemoryRetrieved, MemoryPolicyDenied, MemoryTraceRecorded.

Risks: retrieval of sensitive memory, stale indexes, unexplained ranking,
confusing candidates with truth.

### UpdateMemory

Objective: create a new version or allowed mutation for a memory record.

Conceptual input: memory identifier, proposed changes, reason, actor, evidence,
policy context, trace context.

Conceptual output: new version reference, lifecycle state, provenance update,
policy result.

Policy checks: MemoryPolicy, PrivacyPolicy, ProvenancePolicy, EvidencePolicy,
TraceabilityPolicy.

Events emitted: MemoryProcessed, MemoryConfidenceChanged when applicable,
MemoryTraceRecorded.

Risks: silent mutation, provenance loss, overwritten history.

### ArchiveMemory

Objective: move a memory out of active retrieval while preserving approved
audit or restoration paths.

Conceptual input: memory identifier, archive reason, actor, retention class,
policy context.

Conceptual output: archived state, archive timestamp, audit reference.

Policy checks: RetentionPolicy, PrivacyPolicy, MemoryPolicy,
TraceabilityPolicy.

Events emitted: MemoryArchived, MemoryTraceRecorded.

Risks: archived memory remaining active, audit trail leaking content.

### ExpireMemory

Objective: mark memory as past its retention window.

Conceptual input: memory identifier, retention rule, scheduler or policy
trigger, trace context.

Conceptual output: expired state and next required action.

Policy checks: RetentionPolicy, ForgettingPolicy, PrivacyPolicy.

Events emitted: MemoryExpired, MemoryTraceRecorded.

Risks: expired memory still retrieved, downstream indexes left stale.

### ForgetMemory

Objective: remove memory from active cognitive use through an explicit and
auditable transition that covers metadata, lifecycle state, vector indexes,
relationships, audit trail, and future storage layers.

Conceptual input: target scope, requester, reason, owner boundary, policy
context, trace context.

Conceptual output: forgotten state, tombstone reference when allowed, lifecycle
update, relationship handling result, index cleanup result, and storage cleanup
or redaction result.

Policy checks: ForgettingPolicy, PrivacyPolicy, RetentionPolicy,
TraceabilityPolicy.

Events emitted: MemoryForgotten, MemoryTraceRecorded.

Risks: incomplete forgetting, lingering vector entries, invalid relationships,
metadata that still exposes sensitive content, and audit requirements that
prevent absolute content removal.

This contract must not promise universal hard delete. Policy, compliance, or
audit requirements may require tombstones, redaction, content hashing,
relationship invalidation, or minimal retained metadata.

### DeleteMemory

Objective: remove recoverable memory content according to policy across
metadata, lifecycle state, vector indexes, relationships, audit trail, and
future storage layers.

Conceptual input: memory identifier or scope, deletion authority, reason,
policy context.

Conceptual output: deletion result, lifecycle update, index cleanup result,
relationship cleanup or invalidation result, storage cleanup result, and
allowed tombstone metadata.

Policy checks: ForgettingPolicy, RetentionPolicy, PrivacyPolicy,
TraceabilityPolicy.

Events emitted: MemoryDeleted, MemoryTraceRecorded.

Risks: deleting required audit evidence, retaining content after deletion.

Deletion means removal of recoverable memory content where policy allows it. It
does not guarantee absolute erasure when policy, compliance, or audit trail
requires tombstones, redacted metadata, minimal retention, or non-content audit
records.

### PromoteMemory

Objective: request stronger retention, broader cognitive use, or Knowledge
candidacy.

Conceptual input: memory identifier, promotion target, evidence, confidence,
actor or learning suggestion, policy context.

Conceptual output: promoted state or denied result, promotion rationale.

Policy checks: PromotionPolicy, EvidencePolicy, PrivacyPolicy,
ProvenancePolicy, TraceabilityPolicy.

Events emitted: MemoryPromoted, MemoryPolicyDenied, MemoryTraceRecorded.

Risks: weak memories becoming durable, memory being treated as Knowledge.

### LinkMemory

Objective: create an explicit relationship between memories or approved
external references.

Conceptual input: source memory, target reference, relationship kind,
direction, confidence, evidence, policy visibility.

Conceptual output: relationship identifier, updated relationship profile,
trace metadata.

Policy checks: MemoryPolicy, PrivacyPolicy, EvidencePolicy,
TraceabilityPolicy.

Events emitted: MemoryLinked, MemoryTraceRecorded.

Risks: hidden relationship graph, accidental Knowledge Graph ownership.

### ScoreMemory

Objective: calculate explainable score dimensions for retrieval, promotion,
retention, conflict, or decay.

Conceptual input: memory identifier or candidate set, scoring purpose,
context summary, policy context.

Conceptual output: score dimensions and explanation.

Policy checks: RetrievalPolicy, PromotionPolicy, PrivacyPolicy when visibility
affects scoring.

Events emitted: MemoryTraceRecorded and optionally MemoryConfidenceChanged.

Risks: scores treated as truth, opaque ranking, policy effects hidden.

### TraceMemory

Objective: expose lifecycle, provenance, policy, and correlation history for a
memory operation or record.

Conceptual input: memory identifier, trace scope, actor, policy visibility.

Conceptual output: redacted trace timeline, correlation and causation chain,
policy results.

Policy checks: TraceabilityPolicy, PrivacyPolicy, Audit policy when present.

Events emitted: MemoryTraceRecorded when the trace access itself must be
audited.

Risks: trace output leaking sensitive content.

### ExplainMemory

Objective: explain why a memory exists, why it was retrieved or changed, and
what supports it.

Conceptual input: memory identifier, explanation purpose, actor, context
summary, policy context.

Conceptual output: provenance, evidence, confidence, score dimensions,
relationships, lifecycle state, policy boundaries.

Policy checks: TraceabilityPolicy, PrivacyPolicy, RetrievalPolicy.

Events emitted: MemoryExplained, MemoryTraceRecorded.

Risks: explanation exposing raw memory content beyond authorization.

### DetectMemoryConflict

Objective: identify conflicting memories, evidence, confidence, or lifecycle
state.

Conceptual input: memory identifier or candidate set, conflict criteria,
context summary, policy context.

Conceptual output: conflict report, related memories, confidence impact,
recommended next action.

Policy checks: ConflictPolicy, PrivacyPolicy, EvidencePolicy.

Events emitted: MemoryConflictDetected, MemoryTraceRecorded.

Risks: false conflict, private memory disclosed through conflict metadata.

### ResolveMemoryConflict

Objective: resolve or mark a conflict through policy-governed action.

Conceptual input: conflict identifier, proposed resolution, evidence, actor,
policy context.

Conceptual output: resolution result, updated confidence or lifecycle state,
trace metadata.

Policy checks: ConflictPolicy, EvidencePolicy, PrivacyPolicy,
TraceabilityPolicy.

Events emitted: MemoryConfidenceChanged, MemoryProcessed,
MemoryTraceRecorded.

Risks: premature resolution, erasing useful uncertainty.

### SummarizeMemory

Objective: produce an approved summary without replacing provenance or
original content references.

Conceptual input: memory identifier or group, summary purpose, allowed
visibility, policy context.

Conceptual output: summary, source references, confidence and trace metadata.

Policy checks: PrivacyPolicy, ProvenancePolicy, TraceabilityPolicy,
RetrievalPolicy.

Events emitted: MemoryProcessed, MemoryTraceRecorded.

Risks: lossy summary treated as original memory, LLM-generated text persisted
without provenance.

### ListMemoryTimeline

Objective: expose memory lifecycle and version history in order.

Conceptual input: memory identifier or owner scope, time range, visibility
scope, policy context.

Conceptual output: timeline entries, lifecycle transitions, version lineage,
event references.

Policy checks: PrivacyPolicy, TraceabilityPolicy, RetrievalPolicy.

Events emitted: MemoryTraceRecorded when required by audit policy.

Risks: timeline metadata revealing sensitive events.

## Memory Record Architecture

Future technical concepts:

- `MemoryId`: stable typed identity for a memory record.
- `MemoryType`: working, episodic, semantic, procedural, project, user,
  system, long-term, archived, or expired classification.
- `MemoryState`: lifecycle state such as captured, processed, indexed,
  promoted, archived, expired, forgotten, or deleted.
- `MemoryContent`: original or normalized content reference, potentially
  redacted or externalized.
- `MemorySummary`: compact inspectable representation used for review and
  retrieval.
- `MemorySource`: origin boundary such as user, service, import, observation,
  action outcome, or system event.
- `MemoryOwner`: user, project, system, organization, or service boundary.
- `MemoryProvenance`: origin, transformation path, timestamps, authority, and
  lineage.
- `MemoryEvidence`: supporting references, observations, sources, or future
  knowledge links.
- `MemoryConfidence`: human-readable confidence state plus future explainable
  score dimensions.
- `MemoryScore`: decomposed scoring profile for relevance, confidence,
  freshness, usage, evidence, policy, and trust.
- `MemoryRelationship`: typed directional link with confidence, provenance,
  visibility, and policy boundary.
- `MemoryRetention`: retention class, expiry, archival, deletion, and
  tombstone rules.
- `MemoryPrivacyLevel`: sensitivity and visibility classification.
- `MemoryVersion`: version identity, lineage, reason for change, and affected
  fields.
- `MemoryCognitiveDNA`: traceable identity profile for origin, owner,
  evidence, policy, retention, version, hash, timestamps, lineage, and
  relationship profile.
- `MemoryTrace`: correlation, causation, actor, request, policy result,
  lifecycle transition, and emitted events.
- `MemoryPolicyBinding`: policies and policy decisions that govern the record.

These names are conceptual. They do not define concrete structs in this ADR.

## Memory Lifecycle Architecture

Allowed lifecycle states:

- Captured
- Processed
- Indexed
- Promoted
- Archived
- Expired
- Forgotten
- Deleted

Allowed transition families:

- Captured to Processed after validation, normalization, provenance capture,
  and policy checks.
- Processed to Indexed after indexing requirements and visibility constraints
  are satisfied.
- Indexed to Promoted after PromotionPolicy, EvidencePolicy, PrivacyPolicy,
  and RetentionPolicy approval.
- Indexed or Promoted to Archived after retention, user, compliance, or
  operational policy approval.
- Indexed, Promoted, or Archived to Expired when the retention window ends.
- Captured, Processed, Indexed, Promoted, Archived, or Expired to Forgotten
  when ForgettingPolicy allows removal from active cognition.
- Forgotten or Expired to Deleted when deletion is policy-approved and audit
  requirements are satisfied.

Prohibited transitions:

- Deleted to any active state.
- Forgotten to Indexed or Promoted without explicit restoration architecture
  and policy approval.
- Captured directly to Promoted without processing and policy approval.
- Any state to Indexed without provenance and retention.
- Any persistent state without owner, source, and policy binding.

Policy approval is required for persistence, indexing, promotion, archiving,
forgetting, deletion, visibility expansion, relationship creation, and any
transition involving sensitive memory.

## Cognitive Memory Metabolism Architecture

Future implementation should translate memory metabolism into contracts,
events, policies, and tests:

- Capture maps to CreateMemory and MemoryCaptured.
- Validation maps to policy checks, provenance checks, and MemoryPolicyDenied
  when rejected.
- Normalization maps to Processed state and MemoryProcessed.
- Classification maps to MemoryType, privacy level, retention class, and
  policy bindings.
- Indexing maps to MemoryIndexed and index health checks.
- Relationship Linking maps to LinkMemory and MemoryLinked.
- Promotion maps to PromoteMemory and MemoryPromoted.
- Usage maps to RetrieveMemory, MemoryRetrieved, score updates, and trace
  events.
- Evolution maps to UpdateMemory, ScoreMemory, DetectMemoryConflict, and
  MemoryConfidenceChanged.
- Retention maps to ExpireMemory, ArchiveMemory, and retention tests.
- Archival maps to MemoryArchived.
- Forgetting maps to ForgetMemory, DeleteMemory, and index cleanup tests.

Each stage must be traceable and must have tests for allowed and denied paths.

## Memory Score Architecture

Future scoring dimensions:

- `confidence`
- `evidence`
- `freshness`
- `relevance`
- `usage`
- `relationship_density`
- `conflict_score`
- `trust_score`
- `policy_score`
- `overall_memory_score`

Scores must be explainable and auditable. This ADR does not define numeric
formulas. The first implementation should return decomposed score dimensions,
not only a final number. `overall_memory_score` must be derived from visible
dimensions and must not be treated as truth.

## Cognitive DNA Architecture

Cognitive DNA is the traceable identity profile of a memory.

Future Cognitive DNA should include:

- origin;
- creator;
- source;
- evidence;
- confidence;
- visibility;
- owner;
- policy;
- retention;
- version;
- hash;
- created_at;
- updated_at;
- lineage;
- relationship_profile.

Cognitive DNA must remain sufficient to explain lifecycle transitions even when
content is summarized, archived, forgotten, or deleted. Sensitive fields must
be redacted from logs and broad events.

## Memory Relationship Architecture

Future relationship kinds:

- parent;
- child;
- related;
- supports;
- contradicts;
- depends_on;
- derived_from;
- supersedes;
- duplicates;
- references;
- invalidates;
- evidence_for.

Each relationship must declare direction, confidence, provenance, policy
visibility, and reason. Memory relationships are experience links for recall,
explanation, provenance, conflict detection, and Knowledge candidacy.

They are not the Knowledge Graph. The Knowledge Domain owns canonical entities,
facts, rules, and validated truth relationships. Memory may provide evidence or
candidates to Knowledge only through explicit contracts.

## Cognitive Memory Events

Future memory events:

- MemoryCaptured: emitted after a candidate enters the Memory Domain; payload
  should include identifiers, owner boundary, source reference, and trace
  context, not raw sensitive content.
- MemoryProcessed: emitted after normalization, classification, or versioned
  update; payload should include lifecycle state and provenance reference.
- MemoryIndexed: emitted after approved indexes are updated; payload should
  include index types and freshness metadata.
- MemoryRetrieved: emitted after retrieval; payload should include request
  identifier, candidate identifiers, score references, and policy visibility.
- MemoryPromoted: emitted after approved promotion; payload should include
  promotion target and policy result.
- MemoryArchived: emitted after archival; payload should include archive reason
  and retention reference.
- MemoryExpired: emitted when retention expires; payload should include rule
  reference and next required action.
- MemoryForgotten: emitted when memory leaves active cognitive use; payload
  should avoid recoverable content.
- MemoryDeleted: emitted when recoverable content is deleted; payload should
  include tombstone reference only when allowed.
- MemoryLinked: emitted after a relationship is created; payload should include
  relationship kind and policy visibility.
- MemoryConflictDetected: emitted when conflict is detected; payload should
  include conflict references and sensitivity-safe metadata.
- MemoryConfidenceChanged: emitted when confidence changes; payload should
  include old and new confidence states when policy allows.
- MemoryExplained: emitted when explanation is generated; payload should record
  explanation access, not expose full explanation by default.
- MemoryPolicyDenied: emitted when policy blocks a memory operation; payload
  should include policy identity, result, and redacted reason.
- MemoryTraceRecorded: emitted or recorded when trace metadata is captured.

Memory event payloads must be designed for privacy. Identifiers, trace
references, lifecycle states, and policy results are preferred over raw memory
content.

## Policy Architecture

Future policy responsibilities:

- MemoryPolicy governs capture, update, retrieval, lifecycle transitions,
  visibility, and allowed operations.
- PrivacyPolicy governs sensitive data, consent, redaction, isolation, and
  disclosure boundaries.
- RetentionPolicy governs lifespan, expiry, archival, tombstones, and deletion
  timing.
- ForgettingPolicy governs explicit forgetting requests, deletion semantics,
  downstream invalidation, and audit markers.
- PromotionPolicy governs stronger retention, broader visibility, and
  Knowledge candidacy.
- ProvenancePolicy governs origin capture, lineage, source trust, and
  transformation history.
- EvidencePolicy governs support requirements, evidence quality, conflict
  handling, and Knowledge candidacy.
- TraceabilityPolicy governs trace metadata, explanation requirements,
  correlation, causation, and audit visibility.
- RetrievalPolicy governs who may retrieve which memories for which purpose.
- ConflictPolicy governs detection, visibility, escalation, resolution, and
  preservation of uncertainty.

Policies govern behavior. They do not execute memory capabilities.

## Storage Architecture

Future storage strategy:

- PostgreSQL stores metadata, lifecycle, ownership, provenance, retention,
  confidence, policy references, and audit-friendly relational records.
- Qdrant stores semantic and vector retrieval indexes.
- A future graph layer stores relationships, evidence links, knowledge
  candidates, and contradiction graphs when memory relationships exceed simple
  relational modeling.
- Filesystem or object storage stores large blobs only when required, always
  behind Driver, Policy, and Resource boundaries.

Storage is a detail behind Memory contracts. The Memory Domain must not be
defined by PostgreSQL, Qdrant, a graph database, or an LLM context window.
PostgreSQL, Qdrant, graph storage, and filesystem or object storage are always
implementation details behind contracts, store ports, Drivers when external IO
is involved, Policies, and Resource boundaries.

The initial implementation should favor an in-memory store until contracts,
policies, lifecycle, events, traceability, and tests are stable.

## Indexing Architecture

Future indexes:

- metadata index;
- semantic index;
- time index;
- source index;
- relationship index;
- confidence index;
- policy visibility index;
- retention index.

Stale index risk must be treated as a first-class failure mode. Every lifecycle
transition that affects retrieval must define index update, index invalidation,
or index quarantine behavior. Tests must cover forgotten, deleted, expired,
archived, and privacy-restricted records to ensure stale index entries cannot
surface hidden memory.

## ASB / Contract Bus Integration

All Memory operations must pass through ASB or Contract Bus.

Direct calls are prohibited between:

- Memory Service;
- Knowledge Service;
- Context Service;
- Reasoning Service;
- Learning Service;
- Action Service.

Future Contract Bus messages should carry:

- contract name;
- contract version;
- request id;
- actor;
- owner boundary;
- policy context;
- trace context;
- correlation id;
- causation id.

Contract Bus is the preferred surface for typed memory operations. ASB remains
the communication boundary and compatibility layer.

## Traceability And Explainability

This ADR originally incorporated AEP-0016 while it was proposed. AEP-0016 is
now accepted governance, and every future Memory implementation must comply
with it directly.

Every memory and every memory operation must be traceable, explainable, and
auditable.

Every operation must record:

- origin;
- request;
- actor;
- owner;
- policy result;
- lifecycle transition;
- confidence;
- evidence;
- reason for retrieval;
- reason for mutation;
- emitted events;
- correlation id;
- causation id.

Logs and events must avoid leaking sensitive content. They should prefer
redacted metadata, identifiers, policy decision references, content hashes, and
trace identifiers. Raw content should appear only behind authorized inspection
contracts and policy checks.

## LLM Independence

Memory never depends directly on GPT, Claude, Gemini, local models, or any
specific provider.

Future LLM or model-backed inference may assist with:

- classification;
- summarization;
- retrieval ranking;
- conflict explanation;
- relationship suggestions.

These operations must pass through a future Inference Provider. Models may
produce suggestions or derived artifacts, but they must never own durable
memory. Persistent memory remains platform-owned, policy-governed, traceable,
and inspectable.

## Domain Boundaries

Memory stores experience.

Knowledge stores structured truth.

Context stores the present.

Mandatory limits:

- Memory does not create the Knowledge Graph.
- Memory does not assemble Context.
- Memory does not do Reasoning.
- Memory does not execute Action.
- Memory does not learn by itself.
- Memory does not decide by itself.
- Memory does not call LLMs directly.
- Memory does not live in the Kernel.

Memory may provide candidates, evidence, history, or experience through
contracts. Other domains decide how those candidates are used within their own
boundaries.

## Security And Privacy Considerations

Future implementation must treat memory as privacy-sensitive by default.

Required considerations:

- sensitive data classification;
- user memory isolation;
- project memory isolation;
- system memory isolation;
- consent capture for durable user memory;
- future redaction;
- explicit retention;
- explicit forgetting;
- audit trail without raw content leakage;
- protection against hidden memory;
- protection against prompt injection through memory content;
- protection against retrieval of sensitive memory outside authorized context.

User, project, and system memory boundaries must not be mixed. Memory retrieval
must be scoped by owner, actor, policy, and current context.

## Testing Strategy

Future testing should include:

- unit tests;
- contract tests;
- policy tests;
- lifecycle transition tests;
- memory score tests;
- Cognitive DNA tests;
- relationship tests;
- retrieval tests;
- traceability tests;
- provenance tests;
- forgetting tests;
- conflict resolution tests;
- storage integration tests;
- ASB and Contract Bus tests;
- sensitive-data leakage tests;
- policy bypass tests;
- forgetting completeness tests;
- stale vector index tests;
- no direct service-call tests;
- kernel-does-not-own-memory tests.

Tests must prove allowed and denied paths. They must also prove that Memory,
Knowledge, Context, Reasoning, Learning, and Action remain separated by bus
contracts.

## Implementation Phases

Recommended future phases:

- Phase 5.1: Memory Contracts
  - Completion criteria: typed contract names, request and response concepts,
    error vocabulary, compatibility expectations, and contract tests approved.
- Phase 5.2: Memory Policies
  - Completion criteria: Memory, Privacy, Retention, Forgetting, Promotion,
    Provenance, Evidence, Traceability, Retrieval, and Conflict policy
    contracts defined with allow, deny, and not-applicable tests.
- Phase 5.3: Memory Manager Skeleton
  - Completion criteria: manager descriptor, capabilities, health, manifest,
    registration, and coordination routes without storage or memory behavior.
- Phase 5.4: Memory Service Skeleton
  - Completion criteria: service manifest, capabilities, permissions,
    resources, health, ASB and Contract Bus routes, and no direct service
    calls.
- Phase 5.5: In-memory Memory Store
  - Completion criteria: storage port and in-memory adapter for tests only,
    with no production persistence assumption.
- Phase 5.6: Memory Lifecycle + Events
  - Completion criteria: allowed transitions, denied transitions, event
    emission concepts, and stale index prevention tests.
- Phase 5.7: Traceability + Explainability
  - Completion criteria: trace metadata, explanation surfaces, redaction rules,
    and audit-safe event payload tests.
- Phase 5.8: PostgreSQL Metadata Prototype
  - Completion criteria: metadata persistence behind store contracts,
    migrations reviewed by ADR or implementation plan, and policy-aware tests.
- Phase 5.9: Qdrant Retrieval Prototype
  - Completion criteria: semantic index behind store contracts, stale index
    invalidation, and privacy-filtered retrieval tests.
- Phase 5.10: Relationship + Conflict Prototype
  - Completion criteria: relationship contracts, conflict detection and
    resolution flow, and proof that Memory has not become Knowledge Graph.
- Phase 5.11: Memory Compliance Review
  - Completion criteria: architecture audit, security audit, privacy audit,
    AEP compliance, no direct service calls, Kernel boundary intact, and full
    validation passing.

## Migration And Compatibility

There is no migration now because Memory has not been implemented.

Future changes to Memory contracts require:

- compatibility plan;
- versioning;
- ADR;
- migration strategy;
- tests.

Breaking contract changes must be exceptional and must preserve existing
memory visibility, provenance, retention, forgetting, and audit guarantees.

## Risks

- Overengineering.
- Memory becomes a generic database.
- Memory becomes Knowledge.
- Context is persisted incorrectly.
- LLM-owned memory.
- Hidden memory.
- Storage leakage.
- Policy bypass.
- Retrieval without explanation.
- Incomplete forgetting.
- Stale vector index.
- Weak provenance.
- Relationship graph accidentally becomes Knowledge Graph.
- Scores become treated as absolute truth.
- Events leak sensitive data.
- Kernel grows to own Memory logic.
- Managers execute memory behavior instead of coordination.
- Services call each other directly.

## Consequences

Benefits:

- traceability;
- domain separation;
- LLM independence;
- testability;
- auditability;
- safer future evolution;
- explicit storage boundaries;
- clearer privacy and retention governance.

Costs:

- greater initial complexity;
- more documentation;
- more contracts;
- more tests;
- more discipline;
- slower first implementation phase.

The costs are acceptable because Memory is a high-risk cognitive domain. Aether
must prefer stable cognitive foundations over quick feature delivery.

## Acceptance Criteria

Memory implementation can start only when:

- ADR-0010 is approved;
- contracts are defined;
- policies are defined;
- test strategy is approved;
- storage strategy is approved;
- AEP-0016 is accepted and mandatory;
- the Architecture Guardian approves;
- no AEP violation exists;
- no ambiguity remains between Memory, Knowledge, and Context;
- Kernel remains coordination-only;
- services communicate only through ASB or Contract Bus;
- no Memory Engine, Knowledge behavior, AI behavior, or agent behavior is
  introduced before its approved phase.
