# RFC-0001: Memory Domain

Status: Accepted and published, Revision 1.

Review name: RFC-0001 Revision 1 Memory Domain Architectural Review.

Phase: Cognitive Era domain definition.

Related baseline:

- CDR-0001: Cognitive Design Review
- AEP-0004: Architecture First
- AEP-0005: Domain First
- AEP-0011: Policies Govern Behavior
- AEP-0013: Everything Has Contracts
- AEP-0016: Cognitive Traceability, accepted governance

Reconciliation note: RFC-0001 Revision 1 was accepted and published in
`v0.5.1-memory-domain-rfc`. AEP-0016 was still a proposal during the original
review and was later accepted in `v0.5.5-cognitive-traceability-aep`. Memory
remains unimplemented.

## 1. Executive Summary

The Memory Domain defines how Aether records, retrieves, retains, forgets,
archives, versions, and explains experience.

Memory is not a database table, not a vector index, not a prompt cache, and not
an LLM-owned hidden state. Memory is a cognitive domain governed by provenance,
confidence, retention, privacy, evidence, policies, and explicit contracts.

The Memory Domain exists to preserve experience in a way that remains
inspectable, policy-governed, recoverable, auditable, and useful to future
Context, Reasoning, Planning, Decision, Learning, and Knowledge workflows.

This RFC does not implement Memory Engine behavior. It defines the domain
boundary and future contracts required before implementation may begin.

## 2. Memory Philosophy

Memory stores experience.

Knowledge stores structured truth.

Context stores the present.

The Memory Domain records what happened, what was observed, what was provided,
what was inferred, what was confirmed, what changed, and what may be useful
again later. Memory is allowed to contain uncertainty. It may include
conflicting signals, weak evidence, temporary observations, and deprecated
records, provided their status, provenance, and confidence are explicit.

Memory is not automatically true. A memory can be observed, inferred, stale,
conflicting, personal, project-specific, procedural, or system-level. Knowledge
begins only when memory-derived or source-derived information becomes
structured, evidence-backed, connected, and validated.

Memory must remain model-independent. An inference provider may help summarize,
classify, or retrieve memory candidates in the future, but it must never own
durable memory.

## 3. Domain Boundaries

The Memory Domain owns:

- memory capture rules;
- memory record structure;
- memory lifecycle states;
- retention and expiry semantics;
- forgetting semantics;
- memory versioning;
- memory provenance;
- memory confidence;
- memory evidence links;
- memory retrieval contracts;
- memory promotion proposals;
- memory archive semantics;
- memory audit requirements.

The Memory Domain does not own:

- canonical truth;
- knowledge graph structure;
- current-state context assembly;
- planning algorithms;
- reasoning algorithms;
- action execution;
- policy enforcement engines;
- model inference providers;
- external IO drivers;
- user interface behavior.

Memory ends and Knowledge begins when an experience is transformed into a
structured claim with entities, relationships, facts, rules, evidence,
confidence, and provenance suitable for the Knowledge Domain.

Memory ends and Context begins when historical or retained records are selected
and assembled into the current state of a task, session, user, project,
environment, conversation, time, or execution scope.

Memory may provide inputs to Knowledge and Context, but it must not collapse
into either domain.

## 4. Memory Types

Working Memory is temporary cognitive state for the current operation. It is
not persistent and must not be treated as Long-Term Memory.

Episodic Memory records events and experiences. It answers what happened, when,
where, under which context, and with what outcome.

Semantic Memory records remembered meanings, preferences, and conceptual
associations that are not yet formal Knowledge. It may later contribute to
Knowledge promotion.

Procedural Memory records repeatable procedures, workflows, habits, and
execution patterns. It must be policy-governed because procedures can influence
future action.

Project Memory records project-specific history, architecture, decisions,
constraints, conventions, and events. It must remain scoped to project
boundaries.

User Memory records user-specific preferences, facts, behavior patterns, and
personal context. It is privacy-sensitive by default.

System Memory records platform-level operational experience, architectural
history, system decisions, runtime observations, and service-state summaries.
It must not mix with private user memory.

Long-Term Memory is persistent memory approved for durable retention.

Archived Memory is retained memory moved out of active retrieval paths while
remaining available for audit, restoration, or low-frequency retrieval.

Expired Memory is memory whose retention window has ended. Expired memory must
not be used for ordinary retrieval unless a policy explicitly allows a
transitional or audit path.

## 5. Memory Lifecycle

Captured means a candidate memory has entered the Memory Domain.

Processed means the candidate has been normalized, summarized, classified, and
prepared for indexing or rejection.

Indexed means the memory can be retrieved through approved retrieval paths.

Promoted means the memory has been approved for a stronger retention class,
broader visibility, or possible Knowledge consideration.

Archived means the memory is no longer active by default but is retained for
audit, restoration, compliance, or low-frequency retrieval.

Expired means the memory exceeded its retention window and must be hidden,
archived, forgotten, or deleted according to policy.

Forgotten means the memory is intentionally removed from active cognitive use
while an audit marker may remain where policy requires.

Deleted means the memory content and recoverable payload are removed according
to policy. A minimal tombstone may remain only when required for audit,
integrity, or compliance.

Valid lifecycle transitions must be defined before implementation. No future
service may move memory between states without declared permission and policy
evaluation.

## 6. Memory Record

A future Memory Record should conceptually contain:

- `id`: typed memory identifier;
- `type`: memory type;
- `content`: original or normalized content;
- `summary`: concise representation for inspection and retrieval;
- `source`: origin of the memory;
- `owner`: user, project, system, or service ownership boundary;
- `created_at`: creation timestamp;
- `updated_at`: last update timestamp;
- `valid_from`: start of semantic validity;
- `valid_until`: end of semantic validity when known;
- `confidence`: confidence state;
- `provenance`: origin and transformation trail;
- `evidence`: supporting evidence references;
- `tags`: retrieval and organization labels;
- `relationships`: links to related memories or future knowledge entities;
- `version`: memory version;
- `retention_policy`: governing retention rule;
- `privacy_level`: sensitivity and visibility classification.

This model is conceptual. It does not define a concrete Rust struct, database
schema, API, or serialization contract in this RFC.

## 7. Provenance Model

Every memory must know where it came from.

Every memory must know when it appeared.

Every memory must know who or which system created it.

Every memory must know what evidence supports it.

Every memory must state whether it was observed, provided by a user, imported,
derived, inferred, summarized, transformed, or promoted.

Every memory must carry a confidence state.

Provenance must include enough information to explain why a memory exists and
whether it should be trusted, retrieved, retained, promoted, archived, or
forgotten.

Provenance must not be optional for persistent memory.

## 8. Confidence Model

Observed means the memory came from a direct observation or direct input.

User Confirmed means the user explicitly confirmed the memory.

Inferred means the memory was derived through reasoning, summarization,
classification, or pattern detection.

Weak Signal means the memory is supported by limited, indirect, incomplete, or
low-confidence evidence.

Conflicting means the memory conflicts with another memory, knowledge claim, or
context signal.

Deprecated means the memory should no longer be used as current or reliable,
but may remain for history, audit, or explanation.

Confidence is not a single numeric truth value in this RFC. Future
implementation may add numeric scoring, but the domain must preserve human
readable confidence states.

## 9. Retention Model

Temporary memory lives only for the active operation or session segment.
Working Memory belongs here.

Short-term memory persists briefly to support recent task continuity,
conversation continuity, or near-term work.

Long-term memory persists across sessions after policy approval.

Archived memory persists outside default active retrieval paths.

Expired memory has exceeded its allowed retention window and must not remain
active.

Forgotten memory has been intentionally removed from cognitive use. Depending
on policy, a non-content tombstone may remain for audit.

Retention must be explicit. Any memory without a retention rule is invalid for
future persistent storage.

## 10. Forgetting Model

Aether must be able to forget.

Forgetting is an explicit, policy-governed, auditable transition that removes a
memory from active cognitive use. It may be initiated by user request, privacy
policy, retention expiry, conflict resolution, project boundary change, legal
requirement, or system governance.

Forgetting must define:

- target memory scope;
- requester or system trigger;
- policy authority;
- timestamp;
- audit trail;
- whether content is archived, tombstoned, or deleted;
- impact on relationships and indexes;
- impact on future retrieval.

Forgetting must not silently corrupt Knowledge. If a memory supports a future
knowledge claim, the Knowledge Domain must later receive a provenance or
evidence invalidation signal through an explicit contract.

## 11. Memory Retrieval

Semantic search retrieves memories by meaning. Future implementation may use
Qdrant or another vector index, but semantic search must remain a retrieval
strategy, not the definition of memory.

Keyword search retrieves memories by exact or lexical terms.

Time-based retrieval retrieves memories by creation time, validity windows,
recency, session time, project timeline, or event order.

Source-based retrieval retrieves memories by origin, creator, tool, service,
import path, document, or observation channel.

Relationship-based retrieval retrieves memories connected to other memories,
future knowledge entities, projects, tasks, decisions, or events.

Context-aware retrieval retrieves memories filtered or ranked by current user,
task, project, environment, session, conversation, temporal, and execution
context.

Every retrieval result should be explainable with source, confidence,
provenance, relevance basis, and policy visibility.

## 12. Memory Promotion

Promotion is the process of moving memory to a stronger retention, visibility,
or cognitive significance level.

A temporary memory may be promoted to short-term memory when it supports active
task continuity.

A short-term memory may be promoted to long-term memory when it is stable,
useful, policy-approved, and sufficiently supported.

A memory may become a Knowledge candidate only when it has evidence,
provenance, confidence, and a structured claim suitable for Knowledge Domain
review.

Promotion must depend on Policy. Learning may suggest promotion, Reasoning may
support promotion, and users may request promotion, but policy approval is
required before durable promotion.

## 13. Memory vs Knowledge

Memory is registered experience.

Knowledge is derived, validated, structured, connected information.

Memory may say: "The user said they prefer concise reports during this
project."

Knowledge may say: "Project Aether uses Rust for the Core Runtime, according
to ADR-0003, accepted as an architectural decision."

Memory can contain uncertainty, conflict, stale observations, and personal
experience. Knowledge must have evidence, provenance, structure, and confidence
appropriate for canonical use.

The Memory Domain stores the experience. The Knowledge Domain owns the graph,
entities, relationships, facts, rules, confidence, and evidence rules for
structured truth.

## 14. Memory vs Context

Memory is historical.

Context is current.

Memory can provide candidates for current Context, but Context decides what is
active now. A user preference stored in memory is not automatically current
context unless the Context Domain selects it for the active task, session,
project, environment, or execution scope.

Context must not become permanent storage. Memory must not become the current
state manager.

## 15. Memory Manager

The future Memory Manager will coordinate Memory Domain governance.

Expected responsibilities:

- memory lifecycle coordination;
- retention rule coordination;
- policy evaluation orchestration;
- memory index health coordination;
- memory version coordination;
- archive and forgetting workflows;
- provenance requirements;
- memory capability discovery;
- memory health aggregation;
- coordination with Context, Knowledge, Reasoning, Learning, and Audit
  boundaries.

The Memory Manager must not store memory records directly as hidden state. It
must not bypass services, policies, ASB, Contract Bus, or storage contracts.

## 16. Memory Service

The future Memory Service will execute declared memory capabilities.

Expected responsibilities:

- create memory records;
- retrieve memory records;
- update memory records;
- index memory records;
- archive memory records;
- forget memory records;
- promote memory records;
- link related memories;
- explain memory origin and use;
- expose memory health and inspection surfaces.

The Memory Service must declare capabilities, permissions, resources,
dependencies, lifecycle status, health status, and communication routes through
ASB or Contract Bus.

The Memory Service must not call Knowledge, Context, Reasoning, or Learning
services directly.

## 17. Memory Policies

Memory Policy governs capture, update, retrieval, promotion, archiving,
forgetting, deletion, visibility, and lifecycle transitions.

Privacy Policy governs sensitive user data, consent, redaction, locality,
visibility, and personal memory boundaries.

Retention Policy governs memory lifespan, expiration, archival, deletion, and
retention class.

Forgetting Policy governs explicit forgetting requests, deletion semantics,
tombstones, audit markers, and downstream invalidation signals.

Promotion Policy governs when memory may move to stronger retention or become a
Knowledge candidate.

Provenance Policy governs origin tracking, transformation history, source
trust, and explanation requirements.

Evidence Policy governs what evidence is required for promotion, retrieval
confidence, conflict handling, and Knowledge candidacy.

## 18. Memory Contracts

Future conceptual contracts:

CreateMemory captures a candidate memory with source, owner, type, content,
retention intent, privacy level, evidence, and provenance.

RetrieveMemory returns policy-visible memory candidates using semantic,
keyword, time-based, source-based, relationship-based, or context-aware
retrieval.

UpdateMemory creates a new version or lifecycle transition for an existing
memory.

ArchiveMemory moves memory out of active retrieval while retaining it under
archive rules.

ForgetMemory removes memory from active cognitive use and applies forgetting
policy.

PromoteMemory requests stronger retention, broader cognitive use, or Knowledge
candidacy.

LinkMemory relates memory records to other memories, future knowledge entities,
projects, tasks, decisions, events, or evidence.

ExplainMemory returns why a memory exists, where it came from, what supports
it, how confident it is, what policies govern it, and why it was retrieved.

These are conceptual contracts only. Concrete request and response types require
a future implementation RFC or ADR before code is created.

## 19. Storage Strategy

This RFC does not implement storage.

The recommended future storage strategy is:

- PostgreSQL for memory metadata, ownership, lifecycle, retention, confidence,
  provenance, policy references, and audit-friendly relational records;
- Qdrant for vector retrieval and semantic search indexes;
- a future graph store or graph layer for rich relationships between memories,
  entities, evidence, decisions, projects, and knowledge candidates;
- filesystem or object storage for large blobs only when needed and only behind
  Driver, Policy, and Resource boundaries.

Storage must remain an implementation detail behind Memory Domain contracts.
The Memory Domain must not be defined by PostgreSQL, Qdrant, a graph database,
or an LLM context window.

## 20. Risks

- Memory becomes a generic database.
- Memory mixes with Knowledge and weakens truth boundaries.
- Memory stores sensitive data without policy approval.
- Memory lacks provenance.
- Memory lacks expiration.
- Memory lacks forgetting.
- Memory becomes dependent on an LLM.
- Memory becomes invisible to users.
- Working Memory is accidentally persisted.
- Long-Term Memory is promoted too easily.
- Archived Memory remains active through retrieval bugs.
- Expired Memory is still used by Reasoning.
- Forgetting leaves stale vector or relationship indexes.
- Confidence is treated as truth.
- Semantic search returns plausible but unsupported memories.
- User Memory and Project Memory are mixed.
- System Memory exposes private user data.
- Memory relationships become an accidental Knowledge Graph.
- Memory implementation bypasses ASB or Contract Bus.

## 21. Anti-Patterns

LLM-owned memory is prohibited. Durable memory must be platform-owned,
inspectable, policy-governed, and model-independent.

Hidden memory is prohibited. Important memory must be discoverable and
explainable through future contracts.

Unversioned memory is prohibited for persistent records. Changes must preserve
history or policy-approved tombstones.

Untrusted memory is prohibited for cognitive use. Confidence and provenance
must be explicit.

Memory without source is prohibited for persistent storage.

Memory without retention is prohibited.

Memory as Knowledge Graph is prohibited. Memory may link to future knowledge,
but it does not own canonical truth.

Context stored as permanent memory is prohibited. Context is current state;
Memory is history.

## 22. Acceptance Criteria

Future Memory implementation may begin only when:

- RFC-0001 is approved;
- a Memory implementation ADR is created and accepted;
- the Memory Domain is defined and still aligned with CDR-0001;
- public Memory contracts are defined;
- required Policies are defined or explicitly deferred with justification;
- storage strategy is approved;
- test strategy is defined;
- privacy and provenance risks are documented;
- AEPs are respected;
- Architecture Constitution v2 is respected;
- Engineering Rule #002 remains enforced;
- no service-to-service direct calls are introduced;
- Kernel remains coordination-only;
- Memory remains model-independent.

Until these criteria are satisfied, no Memory Engine, Memory Service, Memory
Manager implementation, database schema, vector index, API, or runtime behavior
may be created.

## 23. Cognitive Memory Metabolism

Cognitive Memory Metabolism describes how experience becomes useful cognition
over time.

Aether memory should not be treated as inert storage. Memory enters the system,
is digested into structured records, is indexed for retrieval, is reinforced or
weakened by future evidence, may become a Knowledge candidate, may be archived,
may expire, and may be forgotten.

The metabolic stages are:

- ingestion: authorized experience enters as a candidate memory;
- digestion: the candidate is normalized, summarized, classified, and checked
  for provenance and policy requirements;
- assimilation: the memory becomes retrievable inside approved boundaries;
- reinforcement: repeated use, user confirmation, evidence, or successful
  outcomes can strengthen confidence or retention class;
- decay: age, conflict, irrelevance, policy expiry, or weak evidence can reduce
  active retrieval priority;
- excretion: expired, forgotten, deleted, or invalidated memory leaves active
  cognition through explicit policy-governed transitions.

Memory metabolism must be observable. Future implementations must be able to
explain why a memory became active, stronger, weaker, archived, forgotten, or
deleted.

## 24. Memory Score Model

The Memory Score Model defines conceptual scoring dimensions used to rank,
retrieve, promote, archive, or decay memories.

This RFC does not define numeric formulas. It defines score categories that
future implementations must preserve as explainable dimensions:

- relevance score: how strongly the memory relates to the current query,
  task, project, or context;
- confidence score: how trustworthy the memory is based on source,
  confirmation, evidence, and conflict state;
- recency score: how time-relevant the memory is;
- frequency score: how often the memory or related pattern has been useful;
- provenance score: how clear and trustworthy the origin trail is;
- policy score: whether policy permits use, promotion, retention, or
  disclosure;
- sensitivity score: how privacy-sensitive the memory is;
- decay score: whether the memory should become less active over time;
- utility score: whether the memory has improved reasoning, planning,
  decision, or user outcomes.

Scores must be inspectable. A future retrieval result must not only say that a
memory matched; it must explain which score dimensions contributed.

## 25. Cognitive DNA

Cognitive DNA is the stable identity and trace profile of a memory.

It is not biological language and not a mystical concept. It is the minimal set
of attributes that lets Aether understand what a memory is, where it belongs,
how it behaves, and how it can evolve.

Conceptual Cognitive DNA includes:

- memory identity;
- memory type;
- owner boundary;
- source boundary;
- provenance lineage;
- confidence state;
- retention class;
- privacy level;
- relationship profile;
- policy bindings;
- version lineage;
- retrieval profile;
- evolution history.

Cognitive DNA must travel with persistent memory. If a memory is summarized,
linked, archived, promoted, deprecated, or forgotten, its Cognitive DNA must
remain sufficient to explain the lifecycle transition.

## 26. Memory Relationship Model

Memory relationships connect experiences without turning Memory into a
Knowledge Graph.

Relationships are memory-local links used for recall, explanation, conflict
detection, provenance, and future Knowledge candidacy. Canonical entities,
facts, and rules remain owned by the Knowledge Domain.

Relationship kinds may include:

- derived_from: one memory was transformed from another;
- supports: one memory supports another memory or future claim;
- contradicts: one memory conflicts with another;
- updates: one memory revises another;
- duplicates: two memories appear to represent the same experience;
- related_to: memories share context, task, project, source, or concept;
- caused_by: one experience appears to have caused or triggered another;
- part_of: a memory belongs to a larger event, session, project, or workflow;
- evidence_for: a memory serves as evidence for a future Knowledge candidate;
- invalidates: a memory weakens or invalidates another memory.

Relationships must have provenance, direction, type, confidence, and policy
visibility. Hidden relationship graphs are prohibited.

## 27. Cognitive Memory Events

Cognitive Memory Events are future observable events emitted when memory
lifecycle or cognitive significance changes.

This RFC does not add event types to code. It defines future event concepts:

- MemoryCaptured;
- MemoryProcessed;
- MemoryIndexed;
- MemoryRetrieved;
- MemoryPromoted;
- MemoryArchived;
- MemoryExpired;
- MemoryForgotten;
- MemoryDeleted;
- MemoryLinked;
- MemoryConflictDetected;
- MemoryConfidenceChanged;
- MemoryExplained;
- MemoryPolicyDenied.

Memory events must not leak sensitive content by default. Event payloads should
favor identifiers, lifecycle state, policy result, provenance references, and
audit metadata over raw memory content.

## 28. Memory Evolution Model

Memory evolves as experience, evidence, context, policy, and time change.

Evolution paths include:

- reinforcement through confirmation or successful reuse;
- weakening through age, low utility, or weak evidence;
- conflict through contradictory memories or Knowledge claims;
- promotion into stronger retention or Knowledge candidacy;
- summarization into lower-cost representations;
- archival into non-active retention;
- deprecation when no longer reliable;
- forgetting or deletion through explicit policy-governed workflows.

Evolution must be versioned. A memory should not silently mutate in place when
the change affects provenance, confidence, retention, privacy, or cognitive
meaning.

## 29. Memory Conflict Resolution

Memory conflict is expected. Aether must support conflicting experience
without collapsing it into false certainty.

Conflict resolution may use:

- source trust;
- user confirmation;
- recency;
- evidence strength;
- policy authority;
- project boundary;
- owner boundary;
- Knowledge validation;
- observed outcome;
- explicit deprecation.

Conflict states must be visible to Context, Reasoning, Decision, and future
Knowledge workflows. Reasoning may use conflicting memories as hypotheses, but
Decision must not treat unresolved conflicts as settled truth.

## 30. Memory Explainability

Every persistent memory should be explainable.

Memory Explainability must answer:

- why does this memory exist?
- where did it come from?
- who or what created it?
- when was it created and updated?
- what evidence supports it?
- what confidence state does it have?
- what policies govern it?
- why was it retrieved?
- why was it promoted, archived, forgotten, or deleted?
- what other memories or future knowledge candidates is it related to?

Explainability is not optional decoration. It is the review surface that makes
memory safe for long-lived cognitive use.

## 31. Cognitive Retrieval Capabilities

Cognitive retrieval is more than search.

Future retrieval capabilities should include:

- recall by meaning;
- recall by exact terms;
- recall by time and event sequence;
- recall by project, task, owner, and session;
- recall by source and provenance;
- recall by relationship;
- recall by confidence and conflict state;
- recall by policy visibility;
- recall by cognitive utility;
- recall by explanation path;
- recall by decay or archival status where policy permits.

Retrieval must produce candidates, not unquestioned truth. Context and
Reasoning decide how candidates are used inside the current cognitive flow.

## 32. Cognitive Memory Principles

- Memory is experience, not truth.
- Memory must be traceable.
- Memory must be explainable.
- Memory must be policy-governed.
- Memory must be model-independent.
- Memory must support forgetting.
- Memory must decay when appropriate.
- Memory must preserve provenance.
- Memory must expose confidence.
- Memory must support conflict.
- Memory must be versioned.
- Memory relationships must not become hidden Knowledge.
- Retrieval returns candidates, not certainty.
- Promotion requires policy.
- Persistent memory without retention is invalid.

## 33. Cognitive Memory Ecosystem

The Memory Domain participates in a broader cognitive ecosystem.

Perception may produce observations that become memory candidates.

Context may request memories that are relevant to the current state.

Knowledge may consume memory-derived candidates only through evidence and
provenance-aware promotion.

Planning may use memories about previous goals, failures, constraints, and
successful workflows.

Reasoning may use memory candidates as experiential evidence, hypotheses, or
counterexamples.

Decision may consider memories only after policy, confidence, and conflict
state are visible.

Action may create outcomes that become future memory candidates, but Action
does not write memory directly without an approved contract.

Learning may detect patterns across memories and suggest promotion,
deprecation, or procedural memory candidates, but Learning does not persist
changes by itself.

Audit and Telemetry must be able to observe memory lifecycle transitions
without exposing sensitive content.

The Memory ecosystem must preserve Engineering Rule #002: cognitive services
communicate through ASB or Contract Bus, not direct service references.
