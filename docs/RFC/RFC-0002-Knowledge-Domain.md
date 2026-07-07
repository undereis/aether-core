# RFC-0002: Knowledge Domain

## 1. Status

Draft.

## 2. Summary

The Knowledge Domain defines how Aether stores, organizes, versions, audits,
relates, validates, invalidates, and retrieves structured knowledge.

Knowledge is responsible for structured truth and structured truth candidates.
It is not raw experience, current context, chat history, prompt context,
runtime cache, graph storage, vector search, or model inference output.

This RFC is conceptual only. It does not implement Knowledge, Memory, Context,
AI, agents, crates, structs, traits, APIs, databases, migrations, graph
storage, vector indexes, runtime behavior, or service behavior.

The fundamental boundary is:

```text
Memory stores experience.
Knowledge stores structured truth.
Context stores the present.
```

## 3. Motivation

Aether needs Knowledge as a separate cognitive domain because experience,
truth, and present state must not collapse into one storage mechanism.

Without a Knowledge Domain:

- LLM output could become unverified knowledge.
- Vector search could be mistaken for truth.
- A database schema could become the cognitive model.
- Graph edges could exist without evidence or provenance.
- Memory records could be promoted without validation.
- Context could become persistent historical state.
- Contradictions could be hidden or overwritten.
- Reasoning would lack a reliable structured substrate.
- Future decisions could depend on claims that cannot be audited.

Knowledge gives Aether a durable, inspectable, evidence-backed, versioned, and
policy-governed representation of structured truth while keeping models
replaceable and storage technology behind contracts.

## 4. Design Principles

- Knowledge stores structured truth.
- Knowledge must be provenance-aware.
- Knowledge must be confidence-aware.
- Knowledge must be versioned.
- Knowledge must be auditable.
- Knowledge must support contradiction.
- Knowledge must support invalidation.
- Knowledge must be LLM-independent.
- Knowledge must not own Memory.
- Knowledge must not own Context.
- Knowledge must not execute Actions.
- Knowledge must not live in the Kernel.
- Knowledge must not be defined by storage technology.

Additional principles:

- Knowledge retrieval returns structured claims, not unquestioned certainty.
- Knowledge confidence is an explanation surface, not absolute truth.
- Knowledge relationships are domain concepts; graph storage is optional.
- Knowledge updates require contracts, policy context, provenance, and trace
  metadata.
- Knowledge must preserve Engineering Rule #002: no direct service-to-service
  communication.

## 5. Domain Definition

The Knowledge Domain owns the conceptual model for structured knowledge.

Knowledge is:

- structured facts;
- validated claims;
- relationships;
- rules;
- schemas;
- ontologies;
- concepts;
- entities;
- evidence links;
- truth candidates;
- contradiction sets;
- versioned assertions;
- domain models.

Knowledge is not:

- raw memory;
- chat history;
- prompt context;
- vector database;
- graph database;
- LLM hidden state;
- user preference memory;
- temporary session state;
- action plan;
- decision result;
- runtime cache.

Knowledge begins when information must be represented as a structured claim,
entity, relationship, rule, schema, ontology, or evidence-backed assertion.
Knowledge ends when it exposes those structures through future contracts to
Context, Reasoning, Planning, Decision, Learning, or other domains.

## 6. Memory vs Knowledge

Memory stores experience.

Knowledge stores structured truth.

Memory can generate candidates for Knowledge, but Memory never promotes
Knowledge by itself. A memory may say that an event happened, a user said
something, a tool produced a result, or an observation was captured. Knowledge
may later represent a structured claim derived from that memory only when the
claim has provenance, evidence, confidence, versioning, and policy approval.

Knowledge does not copy Memory without provenance. It must preserve evidence
links to Memory or other sources and must be able to explain when a knowledge
claim originated from memory-derived experience.

Knowledge does not delete Memory. Memory forgetting does not automatically
delete Knowledge without policy evaluation, because a knowledge claim may also
be supported by other evidence. Knowledge invalidation does not automatically
delete Memory, because memory remains a historical record of experience.

Memory may contain weak signals, conflicting observations, stale events, or
personal preferences. Knowledge requires structure, validation, provenance,
confidence, and versioning before it may be used as structured truth.

## 7. Knowledge vs Context

Context stores the present.

Knowledge stores durable structured truth or structured truth candidates.

Context may use Knowledge to assemble the current user, task, project,
environment, session, conversation, temporal, and execution state. Knowledge
does not assemble Context. Context does not become Knowledge automatically.

Prompt context is not Knowledge. Session state is not Knowledge. Current task
state is not Knowledge. A conversation snippet can become evidence for a
Knowledge candidate only through explicit contracts, provenance, policy
approval, and traceability.

Future Context components may consult Knowledge through Contract Bus
contracts. They must not call a Knowledge Service directly.

## 8. Knowledge vs Reasoning

Knowledge provides structured material for Reasoning.

Reasoning interprets, compares, combines, and derives conclusions using Memory,
Knowledge, Context, goals, constraints, policies, and future inference
providers.

Knowledge does not reason by itself. It may store derived knowledge only when a
future contract captures provenance, confidence, evidence, versioning, and
policy approval. A reasoning result must not write Knowledge directly without
a Knowledge contract and policy checks.

Reasoning can propose hypotheses. Knowledge can store a hypothesis only as an
explicit Knowledge type with uncertainty, evidence, provenance, and status.

## 9. Knowledge vs Learning

Learning may propose updates to Knowledge.

Learning does not own Knowledge. Knowledge does not learn by itself.

Learning can detect patterns, observations, outdated claims, contradictions,
or possible ontology changes. Those outputs remain suggestions until a future
Knowledge contract validates policy, provenance, evidence, confidence,
traceability, and versioning.

No learning output may silently mutate Knowledge.

## 10. Knowledge Sources

Promoted Memory can provide evidence-backed candidates. Risk: experience can be
personal, stale, partial, or sensitive. Requirement: retain memory evidence
links, owner boundary, confidence, provenance, and policy visibility.

User-confirmed facts can provide strong evidence. Risk: users can be mistaken
or can confirm a claim in a narrow scope. Requirement: record actor, scope,
timestamp, confidence, and any later correction path.

Imported documents can provide structured evidence. Risk: documents can be
outdated, private, malicious, or partially parsed. Requirement: source
reference, document provenance, import policy, evidence hash or reference, and
visibility boundary.

System configuration can provide system-level facts. Risk: configuration may
change or include secrets. Requirement: redact sensitive values and version
configuration-derived claims.

Verified external sources can support knowledge. Risk: remote data may change,
be unavailable, or have uncertain authority. Requirement: source reliability,
retrieval timestamp, evidence retention, and invalidation path.

Domain rules can provide reusable structured constraints. Risk: rules can be
confused with Policies or become too rigid. Requirement: separate domain rules
from behavior-governing Policies.

Human-approved ontology can provide schema and relationship semantics. Risk:
ontology decisions can become global too early. Requirement: version ontology
changes and define scope.

Tool outputs can become candidates. Risk: tools may be wrong, partial, or
authorized only for a narrow context. Requirement: tool identity, input scope,
output reference, timestamp, and policy context.

Future enterprise connectors can provide organizational knowledge. Risk:
tenant leakage and authorization errors. Requirement: tenant, owner, policy,
audit, and provenance boundaries.

Future structured datasets can provide bulk claims. Risk: stale data,
licensing, schema drift, and hidden bias. Requirement: dataset version,
source, import contract, and evidence visibility.

## 11. Knowledge Record Architecture

Future Knowledge records should be described conceptually by:

- `KnowledgeId`: stable typed identity for a knowledge record.
- `KnowledgeType`: fact, claim, entity, relationship, rule, schema, ontology,
  concept, evidence link, contradiction, hypothesis, or related type.
- `KnowledgeState`: lifecycle state such as candidate, proposed, validated,
  verified, active, conflicting, deprecated, invalidated, archived, or deleted.
- `KnowledgeClaim`: structured assertion represented by the record.
- `KnowledgeContent`: normalized claim content or structured representation.
- `KnowledgeEntity`: typed object, person, project, concept, component, or
  other entity referenced by claims.
- `KnowledgeRelation`: typed relationship between entities or records.
- `KnowledgeRule`: domain rule or structured constraint, distinct from Policy.
- `KnowledgeSchema`: shape definition for records, entities, relationships, or
  imported knowledge.
- `KnowledgeOntology`: versioned vocabulary of entity and relationship
  semantics.
- `KnowledgeSource`: source boundary such as memory, document, user, tool,
  system, connector, or dataset.
- `KnowledgeOwner`: user, project, system, organization, or tenant boundary.
- `KnowledgeProvenance`: origin, transformation path, actor, contract,
  timestamp, and lineage.
- `KnowledgeEvidence`: supporting evidence references and reliability signals.
- `KnowledgeConfidence`: human-readable and future score-backed confidence
  state.
- `KnowledgeValidity`: temporal, scope, and lifecycle validity.
- `KnowledgeVersion`: immutable version identity and supersession trail.
- `KnowledgeLineage`: prior versions, derived records, and transformation
  path.
- `KnowledgeScope`: user, project, system, organization, domain, or tenant
  scope.
- `KnowledgePrivacyLevel`: sensitivity and visibility classification.
- `KnowledgePolicyBinding`: policies and policy decisions governing use.
- `KnowledgeTrace`: request, actor, correlation, causation, policy result, and
  emitted event trail.
- `KnowledgeContradictionSet`: explicit set of related contradictory claims.
- `KnowledgeVerificationStatus`: validation and verification state.

These are conceptual names only. They do not define real structs, traits,
schemas, APIs, or serialization formats.

## 12. Knowledge Types

Fact is a structured claim treated as currently supported within a scope.

Claim is an assertion that may be unverified, supported, conflicting,
deprecated, or invalidated.

Entity is a typed object or concept known to the system.

Relationship is a typed connection between entities or records.

Rule is a domain rule or inference constraint, not a behavior-governing
Policy.

ProcedureReference points to procedural knowledge without executing it.

Schema describes the structure of knowledge records, imports, or entity types.

Ontology describes versioned entity and relationship vocabulary.

Concept describes an abstract idea or classification target.

Classification assigns an entity, claim, or concept to a category.

Constraint describes a structured limitation relevant to Planning, Reasoning,
or Decision.

Definition defines a term, component, concept, or domain vocabulary.

ObservationSummary is a structured summary derived from evidence or memory,
not raw memory itself.

EvidenceLink references evidence supporting or weakening a claim.

Contradiction records an explicit conflict between claims.

Hypothesis records an uncertain structured possibility.

VerifiedKnowledge is knowledge that passed defined verification criteria.

DeprecatedKnowledge is retained knowledge that should not be used as current
truth but remains available for history, audit, or explanation.

## 13. Knowledge States

Conceptual lifecycle states:

- Candidate: possible knowledge entered the domain.
- Proposed: candidate has enough shape for review.
- Validated: structure, provenance, evidence, and policy checks passed.
- Verified: higher-confidence verification was satisfied.
- Active: claim is available for ordinary policy-aware retrieval.
- Conflicting: claim is in an unresolved contradiction set.
- Deprecated: claim should no longer be treated as current.
- Invalidated: claim is known to be false, revoked, expired, or superseded.
- Archived: claim is retained outside default retrieval paths.
- Deleted: recoverable content has been removed where policy allows.

Allowed transition families:

- Candidate to Proposed after structure and source checks.
- Proposed to Validated after evidence, provenance, scope, and policy checks.
- Validated to Verified after confidence threshold, human confirmation, or
  source reliability requirements.
- Validated or Verified to Active after policy approval.
- Active to Conflicting when contradiction is detected.
- Active, Verified, Validated, or Conflicting to Deprecated when superseded or
  no longer reliable.
- Any non-deleted state to Invalidated when evidence, policy, source
  revocation, user correction, expiry, or domain rule change requires it.
- Deprecated or Invalidated to Archived when retention allows historical
  preservation.
- Archived, Deprecated, Invalidated, or Active to Deleted only when policy,
  compliance, retention, audit, and evidence constraints allow deletion.

Prohibited transitions:

- Deleted to Active.
- Candidate directly to Active without validation and policy checks.
- Any persistent state without provenance, owner, evidence status, and policy
  binding.
- Conflicting to Verified without explicit contradiction resolution.
- Invalidation to deletion when audit or compliance requires tombstone or
  retained redacted metadata.

Human confirmation or evidence threshold may be required for Verified, Active,
contradiction resolution, broad visibility, ontology changes, and claims that
affect future actions.

## 14. Knowledge Promotion

Knowledge promotion is how information becomes structured Knowledge.

Possible flows:

```text
Memory -> Knowledge Candidate
Document -> Knowledge Candidate
User Confirmation -> Knowledge Candidate
Tool Output -> Knowledge Candidate
Human Review -> Validated Knowledge
Policy Approval -> Active Knowledge
```

Memory does not promote Knowledge by itself. LLMs do not promote Knowledge by
themselves. Vector similarity does not promote Knowledge by itself.

Promotion requires:

- explicit contract;
- policy checks;
- source and owner boundary;
- provenance;
- evidence;
- confidence;
- versioning;
- traceability;
- contradiction check;
- sensitive-data controls.

Promotion may be automated only in future low-risk scopes after policies,
contracts, tests, and audit surfaces exist.

## 15. Knowledge Confidence

Knowledge confidence describes the reliability of a claim within scope. It is
not absolute truth.

Suggested confidence states:

- Unverified: structure exists but support is not validated.
- Weak: limited, indirect, stale, or uncertain support exists.
- Inferred: derived by reasoning, extraction, or model assistance and not yet
  independently confirmed.
- Supported: evidence supports the claim within scope.
- UserConfirmed: an authorized user confirmed the claim.
- SystemVerified: Aether verified the claim through approved internal
  evidence or system sources.
- ExternallyVerified: approved external source evidence supports the claim.
- Conflicting: claim conflicts with another claim or evidence.
- Deprecated: claim should not be used as current truth.

Future numeric scores may exist, but they must remain explainable and must not
replace human-readable confidence states.

## 16. Knowledge Evidence

Evidence is the support, challenge, or context for a knowledge record.

The evidence model should include:

- evidence source;
- evidence type;
- evidence timestamp;
- evidence owner;
- evidence reliability;
- evidence chain;
- evidence hash or reference;
- evidence visibility;
- evidence retention;
- evidence conflicts.

Evidence may come from memory, documents, tool outputs, user confirmation,
system configuration, external sources, datasets, or future enterprise
connectors.

Evidence must not always expose raw content. For sensitive evidence, the
Knowledge record should reference redacted evidence metadata or authorized
inspection contracts.

## 17. Knowledge Provenance

Every Knowledge record must answer:

- Where did it come from?
- Who or what created it?
- When was it created?
- By which contract?
- With which evidence?
- With which confidence?
- Which policy allowed it?
- Which previous version or source record preceded it?
- Which trace or correlation id produced it?
- Which domain originated it?

Provenance is mandatory for persistent Knowledge. Knowledge without provenance
may exist only as a transient candidate and must not become Active.

## 18. Knowledge Versioning

Knowledge must be versioned because truth changes, scope changes, evidence
changes, and ontology changes.

Versioning requirements:

- immutable versions for meaningful claim changes;
- supersession links when a newer claim replaces an older claim;
- deprecation when a claim should no longer be used as current;
- invalidation when a claim is wrong, revoked, expired, or contradicted beyond
  acceptable use;
- conceptual rollback for restoring earlier versions under policy;
- lineage from source to current record;
- compatibility rules for schemas and ontologies;
- audit history for mutations;
- optional semantic versioning for schemas and ontologies.

Updating Knowledge should create traceable version history, not silent
mutation, when the change affects meaning, confidence, scope, evidence,
validity, privacy, or lifecycle state.

## 19. Knowledge Contradiction Model

Knowledge may contain controlled contradictions.

Contradiction is not failure by itself. It is a visible state that tells
Reasoning, Context, Planning, Decision, and future Learning that claims must be
handled carefully.

Contradiction relationships include:

- contradicts;
- supports;
- weakens;
- supersedes;
- invalidates;
- refines;
- depends_on;
- derived_from.

Contradictions must not be erased automatically. They should preserve the
claims, evidence, provenance, confidence, source boundaries, and policy
visibility needed to explain why conflict exists.

Resolution may deprecate, invalidate, supersede, split scope, lower confidence,
or keep multiple claims active under different scopes.

## 20. Knowledge Relationship Model

Knowledge relationships connect structured records without defining storage
technology.

Future relationship kinds:

- is_a;
- part_of;
- related_to;
- depends_on;
- supports;
- contradicts;
- causes;
- prevents;
- requires;
- defines;
- references;
- derived_from;
- supersedes;
- invalidates;
- evidence_for.

Each relationship must define direction, meaning, scope, confidence,
provenance, evidence, version, and policy visibility.

Knowledge relationships differ from Memory relationships. Memory relationships
link experiences for recall, explanation, provenance, and Knowledge candidacy.
Knowledge relationships link structured claims, entities, rules, and evidence
inside the structured truth domain.

A future graph layer may store these relationships, but graph storage is a
detail, not the domain definition.

## 21. Knowledge Retrieval

Knowledge retrieval returns policy-visible structured claims and explanation
metadata.

Retrieval strategies include:

- retrieve by id;
- retrieve by entity;
- retrieve by relation;
- retrieve by claim;
- retrieve by confidence;
- retrieve by scope;
- retrieve by provenance;
- retrieve by evidence;
- retrieve by validity;
- retrieve by contradiction set;
- retrieve by current task needs.

Retrieval must be policy-aware, traceable, and explainable. A retrieval result
should explain what was returned, why it matched, what evidence supports it,
what confidence applies, whether contradictions exist, and what policy
visibility constrained the result.

Vector similarity may assist discovery, but it must never be the source of
truth.

## 22. Knowledge Validation

Knowledge validation checks whether a candidate can become trusted structured
knowledge within a scope.

Validation dimensions:

- structure validation;
- evidence validation;
- provenance validation;
- contradiction validation;
- confidence validation;
- policy validation;
- scope validation;
- freshness validation;
- source reliability validation.

Validation may produce Proposed, Validated, Conflicting, Deprecated, or
Rejected outcomes depending on evidence, policy, scope, and contradiction
state.

Validation must not be hidden inside LLM output. A model may assist extraction
or classification only through a future Inference Provider and only as one
traceable input.

## 23. Knowledge Invalidation

Invalidation marks Knowledge as no longer valid for ordinary use.

Knowledge may be invalidated by:

- new evidence;
- contradiction;
- policy;
- source revocation;
- user correction;
- expiry;
- domain rule change.

Invalidation is not necessarily deletion. Invalidated Knowledge may remain
available for audit, historical explanation, contradiction analysis,
provenance lineage, or policy-required tombstones.

Invalidation should emit future events, update indexes, preserve traceability,
and notify dependent domains only through explicit contracts.

## 24. Knowledge Forgetting And Deletion

Knowledge must respect privacy, retention, deletion, redaction, tombstones,
audit trail, evidence unlinking, relationship cleanup, index cleanup, and
future graph cleanup.

Forgetting removes Knowledge from active cognitive use. Deletion removes
recoverable content where policy allows it. Neither operation may silently
break provenance, audit, relationship integrity, or contradiction history.

This RFC does not promise absolute hard delete. Policy, audit, compliance,
legal, or integrity requirements may require tombstones, redacted metadata,
minimal retention, evidence references, relationship invalidation, or
non-content audit records.

Forgetting and deletion must account for:

- relational metadata;
- lifecycle state;
- evidence references;
- relationship indexes;
- semantic indexes;
- future graph storage;
- object or filesystem evidence blobs;
- audit trail visibility;
- downstream Context, Reasoning, Planning, Decision, and Memory references.

## 25. Knowledge Policies

Future policies:

- KnowledgePolicy governs creation, update, retrieval, lifecycle transitions,
  visibility, and allowed operations.
- KnowledgePromotionPolicy governs candidate promotion and Active status.
- KnowledgeValidationPolicy governs structure, evidence, confidence, scope,
  and verification requirements.
- KnowledgeRetrievalPolicy governs who may retrieve which claims and for what
  purpose.
- KnowledgePrivacyPolicy governs sensitive claims, evidence redaction,
  owner boundaries, and disclosure.
- KnowledgeRetentionPolicy governs lifespan, archival, deletion, tombstones,
  and retention class.
- KnowledgeInvalidationPolicy governs invalidation triggers and effects.
- KnowledgeContradictionPolicy governs detection, visibility, escalation, and
  resolution of conflicts.
- KnowledgeEvidencePolicy governs required evidence quality and visibility.
- KnowledgeProvenancePolicy governs origin, transformation path, lineage, and
  source reliability.
- KnowledgeTraceabilityPolicy governs correlation, causation, explanation,
  audit, and redaction of trace metadata.
- KnowledgeUpdatePolicy governs mutation, versioning, supersession, and
  compatibility.

Policies govern behavior. They do not execute Knowledge capabilities.

## 26. Knowledge Events

Future Knowledge events:

- KnowledgeCandidateCreated: emitted when a candidate enters the domain;
  payload should include identifiers, source reference, owner boundary, and
  trace metadata.
- KnowledgeProposed: emitted when a candidate becomes reviewable; payload
  should include claim type and provenance reference.
- KnowledgeValidated: emitted after validation succeeds; payload should include
  validation result and evidence references.
- KnowledgeVerified: emitted after stronger verification; payload should
  include verification status and policy result.
- KnowledgeActivated: emitted when a record becomes active for retrieval;
  payload should include lifecycle state and visibility scope.
- KnowledgeRetrieved: emitted after retrieval; payload should include request
  id, result identifiers, policy visibility, and explanation reference.
- KnowledgeUpdated: emitted after a versioned update; payload should include
  old and new version references.
- KnowledgeDeprecated: emitted when a record should no longer be current.
- KnowledgeInvalidated: emitted when a record is no longer valid.
- KnowledgeArchived: emitted when a record leaves default retrieval paths.
- KnowledgeDeleted: emitted when recoverable content is removed where policy
  allows.
- KnowledgeContradictionDetected: emitted when conflict is found; payload must
  use sensitivity-safe metadata.
- KnowledgeConfidenceChanged: emitted when confidence changes.
- KnowledgeEvidenceLinked: emitted when evidence is linked or unlinked.
- KnowledgeTraceRecorded: emitted or recorded when trace metadata is captured.
- KnowledgePolicyDenied: emitted when policy blocks an operation.

Knowledge event payloads must avoid raw sensitive content by default. Prefer
identifiers, lifecycle state, policy result, evidence references, redacted
reason codes, trace identifiers, and content hashes where appropriate.

## 27. Knowledge Contracts

Future contracts are conceptual. They are not real structs, traits, APIs, or
schemas in this RFC.

### CreateKnowledgeCandidate

Objective: create a structured candidate from an authorized source.

Conceptual input: actor, owner boundary, source reference, claim draft,
evidence references, scope, policy context, trace context.

Conceptual output: candidate identifier, state, provenance summary, policy
result, event references.

Policy checks: KnowledgePolicy, KnowledgePrivacyPolicy,
KnowledgeEvidencePolicy, KnowledgeProvenancePolicy, KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeCandidateCreated, KnowledgePolicyDenied,
KnowledgeTraceRecorded.

Risks: hidden knowledge, weak evidence, sensitive data capture, LLM output
treated as truth.

### PromoteKnowledge

Objective: move a candidate toward validated or active knowledge.

Conceptual input: candidate id, promotion target, evidence, actor, confidence,
scope, policy context, trace context.

Conceptual output: new lifecycle state, promotion rationale, version reference.

Policy checks: KnowledgePromotionPolicy, KnowledgeValidationPolicy,
KnowledgePrivacyPolicy, KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeProposed, KnowledgeValidated, KnowledgeActivated,
KnowledgePolicyDenied, KnowledgeTraceRecorded.

Risks: premature promotion, Memory becoming Knowledge automatically, vector
similarity treated as truth.

### ValidateKnowledge

Objective: validate structure, evidence, provenance, confidence, scope, and
policy fit.

Conceptual input: candidate or record id, validation purpose, evidence set,
policy context.

Conceptual output: validation status, failure reasons, confidence impact.

Policy checks: KnowledgeValidationPolicy, KnowledgeEvidencePolicy,
KnowledgeProvenancePolicy.

Events emitted: KnowledgeValidated, KnowledgePolicyDenied,
KnowledgeTraceRecorded.

Risks: accepting unsupported facts, hidden validation logic.

### VerifyKnowledge

Objective: apply stronger verification through human confirmation, trusted
source, system check, or approved external evidence.

Conceptual input: record id, verification method, verifier, evidence,
policy context.

Conceptual output: verification status, confidence update, trace metadata.

Policy checks: KnowledgeValidationPolicy, KnowledgeEvidencePolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeVerified, KnowledgeConfidenceChanged,
KnowledgeTraceRecorded.

Risks: false authority, stale source, verification leaking evidence.

### RetrieveKnowledge

Objective: retrieve policy-visible structured claims.

Conceptual input: retrieval intent, actor, owner boundary, query, scope,
context summary, policy context, trace context.

Conceptual output: records, evidence summaries, confidence, contradictions,
explanation, policy visibility.

Policy checks: KnowledgeRetrievalPolicy, KnowledgePrivacyPolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeRetrieved, KnowledgePolicyDenied,
KnowledgeTraceRecorded.

Risks: unauthorized retrieval, stale indexes, sensitive evidence leakage.

### UpdateKnowledge

Objective: create a versioned mutation for a knowledge record.

Conceptual input: record id, proposed changes, reason, actor, evidence,
policy context, trace context.

Conceptual output: new version, lifecycle state, provenance update.

Policy checks: KnowledgeUpdatePolicy, KnowledgeProvenancePolicy,
KnowledgeEvidencePolicy, KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeUpdated, KnowledgeConfidenceChanged when applicable,
KnowledgeTraceRecorded.

Risks: silent mutation, broken lineage, policy bypass.

### DeprecateKnowledge

Objective: mark a record as no longer current while preserving history.

Conceptual input: record id, reason, actor, superseding record when available,
policy context.

Conceptual output: deprecated state, version reference, lineage update.

Policy checks: KnowledgeUpdatePolicy, KnowledgeRetentionPolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeDeprecated, KnowledgeTraceRecorded.

Risks: active retrieval of deprecated records, lost audit explanation.

### InvalidateKnowledge

Objective: mark a claim invalid due to evidence, correction, policy, expiry,
or contradiction.

Conceptual input: record id, invalidation reason, evidence, actor or system
trigger, policy context.

Conceptual output: invalidated state, evidence references, affected relations.

Policy checks: KnowledgeInvalidationPolicy, KnowledgeEvidencePolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeInvalidated, KnowledgeConfidenceChanged,
KnowledgeTraceRecorded.

Risks: treating invalidation as deletion, hiding useful contradiction history.

### ArchiveKnowledge

Objective: move a record out of default retrieval while retaining it for
history, audit, or compliance.

Conceptual input: record id, archive reason, retention class, policy context.

Conceptual output: archived state, retention reference, audit marker.

Policy checks: KnowledgeRetentionPolicy, KnowledgePrivacyPolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeArchived, KnowledgeTraceRecorded.

Risks: archived claims still used as active truth.

### DeleteKnowledge

Objective: remove recoverable content where policy allows.

Conceptual input: record id or scope, deletion authority, reason,
policy context.

Conceptual output: deletion result, tombstone when required, index cleanup,
relationship cleanup, evidence unlinking.

Policy checks: KnowledgeRetentionPolicy, KnowledgePrivacyPolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeDeleted, KnowledgeTraceRecorded.

Risks: deleting audit evidence, incomplete index or graph cleanup, promising
hard delete where compliance requires retention.

### LinkKnowledgeEvidence

Objective: link evidence to a knowledge record.

Conceptual input: record id, evidence reference, evidence type, reliability,
visibility, policy context.

Conceptual output: evidence link id, updated evidence profile.

Policy checks: KnowledgeEvidencePolicy, KnowledgePrivacyPolicy,
KnowledgeProvenancePolicy.

Events emitted: KnowledgeEvidenceLinked, KnowledgeTraceRecorded.

Risks: evidence leakage, weak evidence treated as verification.

### LinkKnowledgeRelationship

Objective: create a typed relationship between knowledge records or entities.

Conceptual input: source, target, relation kind, direction, evidence,
confidence, policy visibility.

Conceptual output: relationship id, relationship profile, trace metadata.

Policy checks: KnowledgePolicy, KnowledgeEvidencePolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeUpdated, KnowledgeTraceRecorded.

Risks: accidental graph-driven architecture, hidden relationship semantics.

### DetectKnowledgeContradiction

Objective: identify conflicting claims, evidence, scope, or confidence.

Conceptual input: record id or candidate set, contradiction criteria, scope,
policy context.

Conceptual output: contradiction report, related records, confidence impact.

Policy checks: KnowledgeContradictionPolicy, KnowledgePrivacyPolicy,
KnowledgeEvidencePolicy.

Events emitted: KnowledgeContradictionDetected, KnowledgeTraceRecorded.

Risks: false conflict, private evidence disclosed through metadata.

### ResolveKnowledgeContradiction

Objective: resolve, preserve, scope, deprecate, or invalidate conflicting
claims.

Conceptual input: contradiction set, proposed resolution, evidence, actor,
policy context.

Conceptual output: resolution result, updated states, version references.

Policy checks: KnowledgeContradictionPolicy, KnowledgeValidationPolicy,
KnowledgeEvidencePolicy, KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeUpdated, KnowledgeInvalidated,
KnowledgeConfidenceChanged, KnowledgeTraceRecorded.

Risks: erasing useful uncertainty, premature resolution.

### ExplainKnowledge

Objective: explain why a Knowledge record exists, what supports it, and how it
may be used.

Conceptual input: record id, explanation purpose, actor, visibility scope,
policy context.

Conceptual output: provenance, evidence, confidence, version, contradictions,
lifecycle, policy boundaries.

Policy checks: KnowledgeTraceabilityPolicy, KnowledgePrivacyPolicy,
KnowledgeRetrievalPolicy.

Events emitted: KnowledgeTraceRecorded.

Risks: explanation leaking sensitive evidence or source content.

### TraceKnowledge

Objective: expose trace and audit timeline for a record or operation.

Conceptual input: record id or request id, trace scope, actor, policy context.

Conceptual output: redacted trace timeline, correlation and causation chain,
policy results.

Policy checks: KnowledgeTraceabilityPolicy, KnowledgePrivacyPolicy.

Events emitted: KnowledgeTraceRecorded when trace access must be audited.

Risks: trace metadata exposing sensitive events.

### ListKnowledgeLineage

Objective: list versions, supersessions, derivations, and invalidation path.

Conceptual input: record id, lineage scope, policy context.

Conceptual output: lineage entries, versions, source references, policy
visibility.

Policy checks: KnowledgeRetrievalPolicy, KnowledgePrivacyPolicy,
KnowledgeTraceabilityPolicy.

Events emitted: KnowledgeTraceRecorded when required.

Risks: lineage revealing private sources.

### ListKnowledgeByEntity

Objective: retrieve claims and relationships for an entity.

Conceptual input: entity id or entity selector, scope, policy context.

Conceptual output: entity-linked records, relationships, confidence,
contradictions.

Policy checks: KnowledgeRetrievalPolicy, KnowledgePrivacyPolicy.

Events emitted: KnowledgeRetrieved, KnowledgeTraceRecorded.

Risks: entity profile overexposure, stale relationship index.

### ListKnowledgeByScope

Objective: retrieve records visible within a user, project, system,
organization, tenant, or domain scope.

Conceptual input: scope selector, retrieval purpose, policy context.

Conceptual output: policy-visible records, evidence summaries, confidence,
lifecycle state.

Policy checks: KnowledgeRetrievalPolicy, KnowledgePrivacyPolicy,
KnowledgeRetentionPolicy.

Events emitted: KnowledgeRetrieved, KnowledgeTraceRecorded.

Risks: cross-owner leakage, broad retrieval without explanation.

## 28. Knowledge Storage Strategy

This RFC does not implement storage.

Future storage strategy:

Relational storage stores metadata, ownership, versioning, provenance,
lifecycle, policy bindings, and audit trail. It should be the initial
audit-friendly source for structured metadata.

Graph layer stores relationships, contradictions, dependencies, ontologies,
and entity graph only when relationships exceed simple relational modeling.
Graph storage is not mandatory at the beginning.

Vector index supports semantic discovery only. It is never source of truth.

Object or filesystem storage stores large evidence blobs or imported documents
only when required and only behind Drivers, Policies, and Resource boundaries.

Knowledge is not defined by storage. Relational storage, graph layer, vector
index, and object storage must remain behind contracts, store ports, Drivers
when external IO is involved, Policies, and Resource boundaries.

## 29. Knowledge Indexing Strategy

Future indexes:

- entity index;
- relation index;
- claim index;
- provenance index;
- evidence index;
- confidence index;
- validity index;
- scope index;
- policy visibility index;
- contradiction index;
- version index;
- semantic index.

Stale indexes are a first-class risk. Any lifecycle change involving
deprecation, invalidation, archival, deletion, privacy restriction, evidence
unlinking, relationship cleanup, or scope change must define index update,
index invalidation, or index quarantine behavior.

Indexes are retrieval aids. They do not own truth.

## 30. Traceability And Explainability

AEP-0016 Cognitive Traceability is still Proposed, but this RFC treats it as
mandatory design input.

Every Knowledge record and future Knowledge operation must be traceable,
explainable, and auditable.

Every operation should record:

- origin;
- request;
- actor;
- owner;
- policy result;
- lifecycle transition;
- confidence;
- evidence;
- version;
- contradiction status;
- reason for retrieval;
- reason for mutation;
- emitted events;
- correlation id;
- causation id.

Traceability must avoid leaking sensitive content. Logs, traces, events, and
indexes should prefer identifiers, redacted metadata, policy decision
references, evidence references, content hashes, correlation ids, and causation
ids over raw content.

Raw evidence or claim content should be available only through authorized
inspection contracts and policy-aware retrieval.

## 31. LLM Independence

Knowledge never depends directly on GPT, Claude, Gemini, local models, or any
provider-specific model.

Future LLM or model-backed inference may assist with:

- extraction;
- summarization;
- classification;
- contradiction detection;
- ontology suggestion;
- retrieval ranking;
- explanation drafting.

Those operations must pass through a future Inference Provider. A model can
produce suggestions, candidate structures, or explanation drafts, but it never
owns Knowledge and is never source of truth without evidence, provenance,
policy, validation, and traceability.

The Cognitive Core owns intelligence. Models only provide inference.

## 32. Security And Privacy

Knowledge is security-sensitive because structured truth can reveal identity,
behavior, project state, organizational structure, private evidence, and
decision-relevant facts.

Future implementation must consider:

- user knowledge isolation;
- project knowledge isolation;
- system knowledge isolation;
- future enterprise tenant isolation;
- consent;
- redaction;
- sensitive evidence protection;
- policy-aware retrieval;
- prompt injection via knowledge;
- Knowledge poisoning;
- false knowledge injection;
- stale knowledge;
- unauthorized promotion;
- unauthorized retrieval;
- cross-owner leakage;
- audit trail without raw content leakage.

Knowledge must not be hidden from inspection by authorized users and policies.
At the same time, inspection must not disclose sensitive evidence outside its
owner, scope, or policy boundary.

## 33. Testing Strategy

Future Knowledge implementation must define tests before runtime behavior.

Required test categories:

- unit tests;
- contract tests;
- policy tests;
- promotion tests;
- validation tests;
- invalidation tests;
- contradiction tests;
- confidence tests;
- provenance tests;
- evidence tests;
- versioning tests;
- lineage tests;
- retrieval tests;
- traceability tests;
- sensitive-data leakage tests;
- policy bypass tests;
- stale index tests;
- no direct service-call tests;
- kernel-does-not-own-knowledge tests;
- LLM-does-not-own-knowledge tests;
- Knowledge-is-not-Memory tests;
- Knowledge-is-not-Context tests.

Tests must prove both allowed and denied paths. They must prove that graph
storage, vector search, and model assistance never become source of truth.

## 34. Implementation Phases

Recommended future phases, without executing them now:

- Phase 6.1: Knowledge Contracts
  - Completion criteria: typed contract names, request and response concepts,
    error vocabulary, versioning expectations, and contract tests approved.
- Phase 6.2: Knowledge Policies
  - Completion criteria: promotion, validation, retrieval, privacy, retention,
    invalidation, contradiction, evidence, provenance, traceability, and update
    policies defined with allow, deny, and not-applicable tests.
- Phase 6.3: Knowledge Manager Skeleton
  - Completion criteria: manager descriptor, capabilities, health, manifest,
    registration, and coordination routes without storage or knowledge
    behavior.
- Phase 6.4: Knowledge Service Skeleton
  - Completion criteria: service manifest, capabilities, permissions,
    resources, health, ASB or Contract Bus routes, and no direct service
    calls.
- Phase 6.5: In-memory Knowledge Store
  - Completion criteria: storage port and in-memory adapter for tests only,
    with no production persistence assumption.
- Phase 6.6: Knowledge Lifecycle + Events
  - Completion criteria: allowed transitions, denied transitions, event
    concepts, event privacy rules, and stale index prevention tests.
- Phase 6.7: Provenance + Evidence
  - Completion criteria: evidence references, provenance lineage, confidence
    states, and sensitive evidence redaction tests.
- Phase 6.8: Traceability + Explainability
  - Completion criteria: trace metadata, explanation surfaces, correlation,
    causation, redaction, and audit-safe event payload tests.
- Phase 6.9: Relational Metadata Prototype
  - Completion criteria: metadata persistence behind store contracts,
    migrations reviewed by ADR or implementation plan, and policy-aware tests.
- Phase 6.10: Relationship + Contradiction Prototype
  - Completion criteria: relationship contracts, contradiction detection and
    resolution flows, and proof that storage graph does not define the domain.
- Phase 6.11: Semantic Index Prototype
  - Completion criteria: semantic index behind store contracts, stale index
    invalidation, privacy-filtered retrieval, and proof vector search is not
    truth.
- Phase 6.12: Knowledge Compliance Review
  - Completion criteria: architecture audit, security audit, privacy audit,
    AEP compliance, no direct service calls, Kernel boundary intact, full
    validation passing, and explicit confirmation that no AI or agent ownership
    exists.

These phases are recommendations. Future architecture review may split,
merge, or reorder them if it preserves the domain boundaries.

## 35. Interaction With Future Domains

Memory is a source of experience and evidence candidates. It does not promote
Knowledge directly.

Context consumes relevant Knowledge for current situation assembly. It does
not own durable Knowledge.

Reasoning consumes and interprets Knowledge. It does not write Knowledge
directly without contracts and policies.

Planning consumes Knowledge constraints, rules, definitions, and known
dependencies. It does not mutate Knowledge.

Decision consumes Knowledge confidence, evidence, contradiction status, and
policy results. It does not execute actions.

Learning may propose Knowledge updates, contradictions, deprecations, or new
candidate relationships. It does not persist Knowledge itself.

Action is never called directly by Knowledge.

Inference assists only. It never owns Knowledge.

All interactions must use ASB or Contract Bus contracts.

## 36. Risks

- Knowledge becomes a generic database.
- Knowledge becomes a graph database.
- Knowledge becomes vector search.
- Knowledge becomes LLM memory.
- Knowledge becomes Context.
- Knowledge becomes Reasoning.
- Knowledge becomes too rigid and blocks uncertainty.
- Knowledge accepts facts without evidence.
- Contradictions are erased too early.
- Confidence is treated as absolute truth.
- Stale knowledge influences future reasoning.
- Knowledge poisoning introduces false claims.
- Prompt injection influences persisted claims.
- Unauthorized promotion creates hidden truth.
- Unauthorized retrieval leaks sensitive claims.
- Traceability is insufficient.
- Policy exists on paper but lacks enforcement.
- Graph layer is introduced prematurely.
- Storage schemas drive the domain model.
- Evidence links leak sensitive Memory.
- Excess complexity delays the first safe prototype.

## 37. Open Questions

Questions for future ADRs or implementation reviews:

- Will Knowledge Manager and Knowledge Service be separated exactly as proposed
  for Memory?
- What is the first real storage implementation?
- When will graph layer become necessary?
- How will evidence be stored?
- How will contradictions be resolved?
- Who can promote Knowledge?
- Which operations require human approval?
- How will Knowledge be exported or imported?
- How does Knowledge integrate with enterprise connectors?
- How will ontology versioning work?
- How will Knowledge integrate with AEP-0016 if it remains Proposed?
- Which confidence states require numeric scoring, if any?
- How will Knowledge invalidation notify Context and Reasoning without direct
  service calls?

## 38. Acceptance Criteria

RFC-0002 can be considered ready for review when it:

- clearly differentiates Memory, Knowledge, and Context;
- defines Knowledge without depending on graph, vector, or LLM providers;
- includes provenance, evidence, confidence, and versioning;
- includes contradiction model;
- includes invalidation model;
- includes policies;
- includes events;
- includes contracts;
- includes storage strategy;
- includes testing strategy;
- includes risks;
- respects ARR-0001;
- respects AEP-0016 as mandatory design input while Proposed;
- does not start implementation.

Future Knowledge implementation remains blocked until RFC-0002 is approved,
an implementation ADR is approved, contracts are defined, policies are
defined, storage strategy is accepted, test strategy is accepted, AEP-0016 is
formally accepted, and the Architecture Guardian approves.

RFC-0002 may be reviewed while AEP-0016 remains Proposed. RFC-0002 may also
be checkpointed while AEP-0016 remains Proposed. No real implementation of
Memory or Knowledge may begin until AEP-0016 Cognitive Traceability is
formally accepted.

## 39. Non-Goals

RFC-0002 does not:

- implement Knowledge;
- implement Memory;
- implement Context;
- create crates;
- create real contracts;
- create structs;
- create traits;
- create APIs;
- create databases;
- create migrations;
- create graph storage;
- create vector indexes;
- create AI;
- create agents;
- alter runtime behavior;
- alter the Kernel;
- alter Managers;
- alter Services;
- alter Drivers;
- alter Policies;
- alter public contracts;
- alter CDR-0001;
- alter RFC-0001;
- alter ADR-0010;
- alter ARR-0001.
