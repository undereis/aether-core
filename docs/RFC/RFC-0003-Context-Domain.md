# RFC-0003: Context Domain

## 1. Status

Accepted and published.

Reconciliation note: RFC-0003 was accepted and published in
`v0.5.7-context-domain-rfc`. It remains architecture documentation only;
Context and all related runtime capabilities remain unimplemented.

This RFC is documentation-only. It defines the future Context Domain for
Aether, but it does not implement Context, crates, services, managers,
policies, contracts, APIs, storage, cache, compression, model routing, AI,
agents, or runtime behavior.

No functional behavior changes are introduced by this document.

## 2. Summary

The Context Domain is responsible for assembling, bounding, updating,
reducing, and explaining the present state of Aether for an authorized
cognitive operation.

Context is the current operational frame. It composes relevant signals about
the user, task, project, session, conversation, environment, permissions,
time, execution scope, tools, and operational state.

Context is not Memory. Context is not Knowledge. Context is not a plan,
reasoning result, decision, learning artifact, perception output, AI agent,
prompt dump, chat history, or generic database.

## 3. Motivation

Aether needs a Context Domain because future cognitive work requires an
explicit, bounded, traceable view of the present before Planning, Reasoning,
Decision, Learning, or Action can safely operate.

Without a dedicated Context Domain, Aether risks:

- treating long prompts as system state;
- mixing Memory, Knowledge, and current task state;
- losing track of why specific information influenced an operation;
- leaking sensitive information into model prompts, logs, traces, or tools;
- creating hidden cognitive state;
- using stale, excessive, or unauthorized context;
- increasing token, cost, latency, and energy usage without improving utility.

Context gives Aether a disciplined way to answer:

- What is relevant now?
- Why is it relevant?
- Who or what is allowed to see it?
- How fresh is it?
- What was omitted?
- What was compressed?
- Which policies allowed its use?
- How can the active context be explained after the fact?

## 4. Design Principles

### Present-State Assembly

Context stores the present. It assembles a current-state frame for a specific
authorized operation.

### Bounded Context

Every context must have a clear scope. Aether must know which user, task,
project, session, conversation, environment, time window, execution boundary,
and tool boundary the context applies to.

### Traceable Context

Every future context unit and context assembly operation must be traceable
under AEP-0016. The system must be able to explain where context came from,
why it was included, what policy allowed it, and which operation consumed it.

### Policy-Aware Context

Context must never bypass policies, permissions, privacy boundaries, resource
limits, safety checks, or auditability.

### Efficient Context

Context should contain the least sufficient state for the operation. It should
avoid prompt dumping, unnecessary token usage, stale data, redundant inputs,
and expensive processing when a smaller context would be sufficient.

### Context Is Not Memory

Memory stores experience. Context selects and assembles what is relevant now.
Context must not become permanent storage.

### Context Is Not Knowledge

Knowledge stores structured truth. Context may consume relevant Knowledge
through future contracts, but it does not own durable truth.

### AI-Maintainable Architecture

Future Context implementation must be modular, explicit, and easy for humans
and Codex to inspect. Large hidden files, implicit flows, and God objects are
architectural failures.

### No Prompt Dumping

Context must not become a habit of sending everything to a model. Relevant
state must be selected, reduced, scoped, and traceable.

## 5. Domain Definition

The Context Domain owns current-state assembly.

It begins when Aether needs a present-state frame for an authorized cognitive
operation. It ends when the current user, task, project, session,
conversation, environment, permission, temporal, execution, tool, and
operational signals have been selected, bounded, validated, reduced where
appropriate, and exposed through future contracts.

The Context Domain is responsible for:

- assembling current state;
- delimiting context scope;
- validating policy and permission boundaries;
- tracking freshness and expiration;
- selecting relevant Memory and Knowledge candidates through future contracts;
- reducing or compressing context when appropriate;
- explaining why context was included or excluded;
- tracing context usage;
- preventing hidden, stale, excessive, or unauthorized state.

The Context Domain is not responsible for:

- storing durable experience;
- storing structured truth;
- producing plans;
- reasoning over alternatives;
- selecting decisions;
- executing actions;
- learning patterns;
- perceiving raw input;
- owning model inference;
- owning cache as hidden state.

## 6. Context Model

A future context unit is conceptual only in this RFC. It should include:

- context id: a typed identity for the assembled context frame;
- scope: the user, task, project, session, conversation, environment, and
  execution boundary;
- task/session binding: the operation that required this context;
- sources: Memory candidates, Knowledge claims, current input, tool state,
  environment state, permission state, and other authorized sources;
- freshness: when each signal was last validated and when it becomes stale;
- provenance: where each signal came from;
- confidence: confidence or reliability of context signals where applicable;
- policy constraints: rules that allow, deny, redact, or limit use;
- expiry: when the context should be discarded or reassembled;
- compression state: whether context was reduced, summarized, filtered, or
  otherwise transformed;
- trace links: correlation, causation, request, and operation metadata;
- explanation: why this context was assembled and how it should be used.

This RFC does not create structs, traits, schemas, or APIs.

## 7. Context Types

Future Context may include several composed context types.

User Context describes the current user state, preferences, constraints,
authorized memory visibility, privacy boundary, and relevant operating mode.

Session Context describes the current session state, recent actions, active
assumptions, temporary constraints, and transient working state.

Task Context describes the active goal, input, requested output, progress,
constraints, risk level, and completion boundary.

Project Context describes project-specific state such as active files,
architecture, conventions, decisions, phase, roadmap, and approved boundaries.

Environment Context describes runtime environment, local machine constraints,
available services, service health, operating system details, and resource
state.

Permission Context describes the permissions, policies, owner boundaries, and
authorization rules relevant to the operation.

Tool Context describes available tools, tool capabilities, tool permissions,
recent tool outputs, and tool execution constraints.

Execution Context describes authorized execution scope, pending operations,
side-effect boundaries, rollback expectations, and safety constraints.

Temporal Context describes current time, deadlines, recency, ordering,
expiration, and time-sensitive validity.

Conversation Context describes the active conversational exchange. It is not
the whole cognitive state and must not be treated as durable memory.

Document Context describes currently relevant document fragments, document
state, references, and source boundaries for the active operation.

Safety/Policy Context describes risk classification, data sensitivity,
required approvals, policy decisions, and audit requirements.

## 8. Context Lifecycle

The future Context lifecycle should be explicit and traceable:

1. Perceive inputs.
2. Select relevant signals.
3. Assemble context.
4. Validate policies.
5. Compress or reduce where appropriate.
6. Expose context to the authorized cognitive operation.
7. Trace usage.
8. Expire or discard the context.
9. Optionally emit observations to future Memory or Learning through approved
   contracts, without persisting context by itself.

Context lifecycle transitions must be attributable to a request, operation,
policy, event, manager, service, user, or system trigger.

## 9. Context Boundaries

### Context vs Memory

Memory is historical experience. Context is current state.

Memory may provide candidates for Context, but Context decides what is active
now for a specific task, session, project, environment, conversation, time, or
execution scope. Context must not become permanent storage.

### Context vs Knowledge

Knowledge is durable structured truth or structured truth candidates. Context
may use Knowledge to assemble the present state, but Context does not own
Knowledge and does not create canonical truth.

### Context vs Planning

Planning creates futures. Context provides the current frame from which future
plans may be generated.

### Context vs Reasoning

Reasoning creates hypotheses, explanations, comparisons, and syntheses.
Context provides present-state input to Reasoning but does not reason by
itself.

### Context vs Decision

Decision chooses between alternatives. Context informs Decision but does not
choose.

### Context vs Learning

Learning suggests patterns and improvements. Context may provide observations,
but it does not learn, persist behavior changes, or mutate the system.

### Context vs Perception

Perception transforms raw input into structured observations. Context selects
and assembles current-state signals, including perception outputs when
authorized.

### Context vs Prompt

A prompt is a representation sent to an inference provider. Context is the
governed current-state frame. Prompt context is not the Context Domain.

### Context vs Cache

Cache may optimize repeated work in future phases. Cache must never become
hidden context, hidden memory, or an untraceable source of truth.

## 10. Context Assembly

Context assembly must be an explicit, authorized, policy-aware operation.

Future context assembly should:

- identify the operation requesting context;
- determine scope and owner boundaries;
- select only relevant signals;
- consult Memory and Knowledge only through future contracts;
- validate permissions and policies;
- detect stale or conflicting signals;
- reduce unnecessary content;
- record trace metadata;
- explain included and omitted signals.

Context assembly must not indiscriminately pull all available memory,
knowledge, conversation history, files, tool output, or environmental state.

## 11. Context Freshness

Context is time-sensitive.

Future Context design must track:

- when each signal was observed;
- when each source was last validated;
- how long the context remains valid;
- whether the context is stale;
- whether revalidation is required;
- which operations consumed stale or refreshed context;
- whether freshness rules differ by domain, user, project, policy, or risk.

Stale context must be visible. It must not silently influence sensitive
operations.

## 12. Context Compression

Context compression is a future controlled capability.

Its purpose is to reduce token usage, cost, latency, and compute load while
preserving correctness, safety, traceability, and useful state.

Future compression must:

- preserve critical information;
- identify what was removed;
- explain why reduction was safe;
- retain trace links to source context;
- avoid hiding uncertainty or loss;
- avoid removing policy, permission, safety, or provenance signals;
- allow escalation when compression risk is too high.

This RFC does not implement compression.

## 13. Context Traceability

AEP-0016 governs future Context design.

Every future context operation must be able to answer:

- where did this context come from?
- why was this source included?
- why was another source omitted?
- which policy allowed or denied it?
- who or what requested it?
- which operation used it?
- was it compressed, filtered, redacted, or summarized?
- when did it expire?
- which trace, correlation, and causation ids connect it to the operation?
- how can the used context be explained without exposing sensitive data?

Events, logs, traces, and audit trails must avoid leaking sensitive content by
default.

## 14. Context Policies

Future Context policies may include:

- Privacy Policy: controls access to personal, project, enterprise, or
  sensitive information;
- Retention Policy: controls whether traces, summaries, or context fragments
  may be retained;
- Permission Policy: controls which operations may access which context;
- Source Trust Policy: controls source reliability and source eligibility;
- Context Freshness Policy: controls staleness, expiration, and revalidation;
- Compression Safety Policy: controls what may be reduced or summarized;
- Sensitive Data Minimization Policy: limits exposure of sensitive content;
- Tool Access Boundary Policy: controls which tools may see or use context.

Policies must govern context behavior before implementation details are chosen.

## 15. Context Events

Future context events may include:

- ContextAssemblyRequested: emitted when an operation requests current-state
  assembly;
- ContextAssembled: emitted when a scoped context frame is produced;
- ContextSourceSelected: emitted when a source is included in context;
- ContextCompressed: emitted when context is reduced, summarized, or filtered;
- ContextExpired: emitted when context is no longer valid;
- ContextRejectedByPolicy: emitted when policy blocks context use;
- ContextUsedByOperation: emitted when an authorized operation consumes
  context.

Event payloads must be metadata-first and sensitivity-safe. They must not leak
raw sensitive content by default.

## 16. Context Contracts

Future conceptual contracts may include:

- ContextAssemblyRequest;
- ContextAssemblyResult;
- ContextSourceDescriptor;
- ContextScope;
- ContextPolicyDecision;
- ContextTrace.

Contracts should define operation intent, scope, actor, owner boundary,
policy context, source descriptors, freshness requirements, compression state,
trace ids, and explainability outputs.

This RFC does not create real contracts.

## 17. Context Manager

The future Context Manager governs the Context Domain.

It should coordinate:

- context assembly lifecycle;
- policy orchestration;
- freshness validation;
- visibility boundaries;
- context health;
- traceability expectations;
- compression and reduction rules;
- coordination with Memory and Knowledge through Contract Bus boundaries.

The Context Manager must not store durable memory, own Knowledge, produce
plans, reason, decide, learn, perceive raw input, execute actions, or call
services directly.

## 18. Context Service

The future Context Service executes context capabilities.

It may eventually expose authorized capabilities such as context assembly,
context inspection, context refresh, context reduction, context explanation,
and context health reporting.

The Context Service must communicate through ASB or Contract Bus. It must not
call Memory, Knowledge, Planning, Reasoning, Decision, Learning, Perception, or
Action services directly.

The Context Service must not become a God Service.

## 19. Storage/Non-Storage Strategy

Context is primarily ephemeral.

It should be assembled for an operation, used within its scope, and expired or
discarded when no longer valid.

Context must not persist itself as Memory. Context must not become durable
Knowledge. Future implementations may retain minimal traces, audit metadata,
context summaries, or policy decisions only when explicitly authorized by
future ADRs, policies, contracts, and storage strategy.

Any future persistence requires:

- implementation ADR;
- approved storage strategy;
- policy model;
- retention rules;
- privacy and redaction rules;
- traceability design;
- tests for hidden state and leakage.

Cache must not become hidden memory.

## 20. Efficient Intelligence Considerations

RFC-0009 applies directly to Context.

Future Context design should:

- avoid prompt dumping;
- select only relevant context;
- reduce duplicate or stale signals;
- support controlled context compression;
- consider cache only when policy-aware and traceable;
- expose cost, token, energy, and latency implications where appropriate;
- prefer deterministic tools when they produce better context than long model
  reasoning;
- require justification before escalating to larger models;
- preserve safety, policies, permissions, traceability, and auditability.

Efficiency is never permission to weaken governance.

## 21. Main-as-Maestro and AI-Maintainability

Future Context implementation must follow Main-as-Maestro, not
Main-as-God-File.

The Kernel and main entrypoints should orchestrate. They must not own Context
logic.

Context must remain modular:

- responsibilities are separated by manager, service, contract, policy, and
  future store boundary;
- future files should stay small, cohesive, and inspectable;
- contracts should be explicit before implementation;
- failure should be small, isolated, traceable, and recoverable;
- humans and Codex should be able to reason about the system without hidden
  flows or implicit state.

## 22. Security And Privacy

Context is high-risk because it can combine sensitive signals from many
sources.

Future Context implementation must address:

- data minimization;
- PII and sensitive data handling;
- owner and tenant isolation;
- project isolation;
- permission checks;
- tool access limits;
- policy-aware retrieval;
- redaction;
- prompt injection through context sources;
- stale or poisoned context;
- cross-owner leakage;
- audit trails without raw sensitive data;
- explainability without overexposure.

## 23. Testing Strategy

Future Context implementation should include:

- context assembly tests;
- boundary tests;
- policy rejection tests;
- freshness tests;
- stale context tests;
- compression loss tests;
- traceability tests;
- no-hidden-state tests;
- sensitive-data leakage tests;
- no direct service-call tests;
- kernel-does-not-own-context tests;
- Context-is-not-Memory tests;
- Context-is-not-Knowledge tests;
- prompt-dumping prevention tests;
- cache-is-not-hidden-memory tests.

This RFC does not implement tests.

## 24. Implementation Phases

Future implementation should proceed only after this RFC is reviewed and
accepted.

Recommended future phases:

1. RFC approval.
2. Context implementation ADR.
3. Context contracts.
4. Context policies.
5. Context Manager skeleton.
6. Context Service skeleton.
7. In-memory/context assembly prototype.
8. Traceability integration.
9. Compression strategy.
10. Cache strategy.
11. Storage/audit strategy.
12. Compliance review.

Each phase should define acceptance criteria before implementation begins.

## 25. Risks

Key risks:

- Context becomes a generic database;
- Context becomes chat history;
- Context becomes prompt dumping;
- Context persists data without Memory policies;
- Context becomes hidden state;
- Context bypasses privacy or permission policies;
- Context directly owns Memory or Knowledge;
- Context calls services directly;
- context compression hides critical information loss;
- cache becomes invisible memory;
- stale context influences sensitive decisions;
- Context Manager becomes a God Manager;
- Context Service becomes a God Service;
- efficiency pressure weakens safety or auditability.

## 26. Anti-Patterns

Prohibited anti-patterns:

- Context as database;
- Context as chat history;
- Context as prompt dump;
- Context as hidden state;
- Context bypassing policies;
- Context directly owning Memory or Knowledge;
- Context Service calling services directly;
- compression hiding critical loss;
- cache becoming invisible memory;
- Context God Manager;
- Context God Service;
- Kernel owning context logic;
- LLM-owned context.

## 27. Open Questions

Future architecture work must answer:

- What is the first minimal Context contract?
- Which context sources are allowed in the first implementation?
- Which Context policies are required before any prototype?
- How should context freshness be represented?
- How should compressed context explain omitted information?
- Which traces are required without leaking sensitive data?
- What minimal audit metadata is safe to retain?
- How should Context notify Planning and Reasoning without direct service
  calls?
- Which context operations require human approval?
- How will Context interact with future execution modes from RFC-0009?

## 28. Acceptance Criteria

RFC-0003 can be considered ready for review when it:

- clearly defines Context as present-state assembly;
- separates Context from Memory, Knowledge, Planning, Reasoning, Decision,
  Learning, Perception, prompts, and cache;
- includes policy, freshness, traceability, compression, security, and
  efficiency considerations;
- respects CDR-0001;
- respects AEP-0016;
- respects RFC-0009;
- defines future events, contracts, manager, service, testing strategy, risks,
  anti-patterns, and non-goals;
- does not start implementation.

Future Context implementation remains blocked until this RFC is approved, an
implementation ADR is approved, contracts are defined, policies are defined,
storage/non-storage strategy is accepted, test strategy is accepted, and the
Architecture Guardian approves.

## 29. Non-Goals

This RFC does not:

- implement Context;
- implement a Context Engine;
- create crates;
- create services;
- create managers;
- create policies;
- create contracts;
- create structs;
- create traits;
- create APIs;
- create database tables;
- create migrations;
- create schemas;
- create storage;
- implement Memory;
- implement Knowledge;
- implement Planning;
- implement Reasoning;
- implement Decision;
- implement Learning;
- implement Perception;
- implement AI;
- implement agents;
- implement model routing;
- implement cache;
- implement compression;
- change runtime behavior;
- refactor existing architecture.
