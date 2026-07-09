# CDR-0001: Cognitive Design Review

Status: Accepted and published.

Phase: Cognitive Era architecture review.

Reconciliation note: CDR-0001 was accepted and published in
`v0.5.0-cognitive-design-review`. This lifecycle update does not implement any
cognitive domain or runtime behavior.

Official statement:

> The Cognitive Core owns intelligence. Models only provide inference.

## Purpose

This Cognitive Design Review defines how Aether thinks.

It does not implement Memory, Knowledge, AI, agents, services, managers,
drivers, APIs, persistence, or user-facing behavior. It establishes the
architecture vocabulary and boundaries required before any Cognitive Era
implementation can begin.

Aether is not a chatbot, not an LLM wrapper, and not a GPT-dependent
application. Aether is a cognitive operating system. Models are replaceable
inference mechanisms inside a larger cognitive architecture.

## Section 1: Cognitive Philosophy

Software executes instructions inside a defined environment. It can be useful,
reliable, and powerful without understanding goals, context, memory,
knowledge, decisions, or learning.

A virtual assistant responds to user requests. It may automate tasks or provide
answers, but it often depends on scripted integrations, narrow context, or a
single conversational surface.

A chatbot is an interaction pattern. It uses conversation as the primary user
interface. A chatbot can be backed by rules, retrieval, LLMs, or workflows, but
the chat surface itself is not a cognitive architecture.

An LLM is an inference model. It predicts, transforms, summarizes, classifies,
or generates text and other modalities based on model training and input
context. It does not own durable memory, policy authority, provenance,
execution rights, or platform lifecycle.

A cognitive system coordinates perception, memory, knowledge, context,
planning, reasoning, decision, action, and learning. It separates what it sees,
what it remembers, what it knows, what is currently true, what it intends to do,
how it evaluates alternatives, what it chooses, what it executes, and what it
learns afterward.

A cognitive operating system is a cognitive system with platform boundaries. It
does not merely answer; it supervises cognitive services, enforces policies,
tracks capabilities, exposes contracts, coordinates lifecycle, records
telemetry, and keeps inference providers replaceable.

Aether belongs only to the last category because it is built around Kernel,
Managers, Services, Drivers, Domains, Policies, ASB, Contract Bus, manifests,
capabilities, permissions, resources, health, and architectural governance. The
conversation interface, future LLMs, and future agents are clients or
execution mechanisms, not the center of intelligence.

## Section 2: Cognitive Pipeline

The official Cognitive Pipeline is:

```text
User Input
  -> Perception Domain
  -> Working Memory
  -> Memory Domain
  -> Knowledge Domain
  -> Context Domain
  -> Planning Domain
  -> Reasoning Domain
  -> Decision Domain
  -> Action Domain
  -> Learning Domain
  -> Memory Update
```

User Input is any authorized input entering the cognitive system. It may later
come from chat, voice, files, screen context, automation events, APIs, or other
approved sources.

Perception Domain transforms raw input into structured observations. It detects
what kind of signal entered the system and prepares it for cognition.

Working Memory holds the active cognitive frame for the current operation. It
is temporary and not persistent.

Memory Domain retrieves relevant past experience and manages future memory
contracts. It handles experience, not structured truth.

Knowledge Domain provides structured knowledge such as entities, relationships,
facts, rules, evidence, provenance, and confidence.

Context Domain builds the current state of user, task, project, environment,
session, conversation, time, and execution.

Planning Domain receives goals and produces candidate plans, subgoals, tasks,
dependencies, milestones, priorities, and recovery paths.

Reasoning Domain evaluates memory, knowledge, context, goals, policies,
constraints, and inference outputs. It produces hypotheses, comparisons,
deductions, explanations, syntheses, uncertainty assessments, and confidence
signals.

Decision Domain chooses among alternatives. It never executes actions and it
must respect Policies.

Action Domain executes approved actions through explicit services, drivers, or
future automation boundaries. It does not reason, decide, or learn.

Learning Domain observes outcomes and produces suggestions, patterns, and
insights. It never changes the system by itself.

Memory Update persists approved memory changes only after policy evaluation and
provenance capture.

## Section 3: Domain Boundaries

### Perception

Perception begins when authorized raw input enters the cognitive system. It ends
when the input has been transformed into structured observations.

Perception owns normalization, modality identification, extraction, signal
classification, and observation metadata.

Perception does not own memory persistence, truth management, planning,
reasoning, decisions, or actions.

### Memory

Memory begins when the system needs to store or retrieve experience. It ends
when relevant memory candidates, memory records, or memory updates are returned
with provenance and retention metadata.

Memory owns experience, recall, retention, expiry, indexing, retrieval,
versioning, and memory provenance.

Memory does not own structured truth, global context, planning, reasoning,
policy decisions, or action execution.

### Knowledge

Knowledge begins when information must become structured truth candidates or
validated knowledge. It ends when entities, relationships, facts, rules,
evidence, confidence, and provenance are exposed through contracts.

Knowledge owns graph structure, facts, rules, evidence, provenance, and
confidence.

Knowledge does not own raw experience, transient context, planning, inference
execution, or action.

### Context

Context begins when the system needs a current-state frame. It ends when the
active user, task, project, environment, session, conversation, time, and
execution state are assembled.

Context owns the present. It binds memory and knowledge to the current
situation.

Context does not own long-term storage, canonical knowledge, planning logic,
decision authority, or execution.

### Planning

Planning begins with a goal or requested outcome. It ends with a plan,
subgoals, tasks, dependencies, constraints, milestones, priorities, and recovery
paths.

Planning owns future structure.

Planning does not decide final execution, perform actions, learn from outcomes,
or persist memory.

### Reasoning

Reasoning begins when the system must evaluate meaning, alternatives,
uncertainty, or explanation. It ends with hypotheses, comparisons, deductions,
explanations, syntheses, confidence, and uncertainty.

Reasoning owns cognitive evaluation.

Reasoning does not execute actions, persist learning, bypass policies, or
become a model provider.

### Decision

Decision begins when alternatives exist. It ends when an alternative is chosen,
rejected, deferred, or escalated.

Decision owns selection.

Decision does not execute, reason deeply, learn, or override Policies.

### Action

Action begins after an approved decision produces an executable intent. It ends
when the action succeeds, fails, is denied, or is rolled back through an
approved path.

Action owns execution only.

Action does not reason, decide, learn, or create hidden state.

### Learning

Learning begins after observations, outcomes, or repeated patterns are detected.
It ends with suggestions, insights, candidate patterns, or proposed memory and
knowledge updates.

Learning owns suggested adaptation.

Learning does not persist changes, alter behavior automatically, execute
actions, or bypass Policies.

## Section 4: Memory Model

Working Memory is the temporary active frame used during a cognitive operation.
It is not persistent. It may contain the current prompt, active observations,
retrieved candidates, intermediate reasoning notes, current plan state, and
short-lived decision context.

Episodic Memory stores experiences and events. It answers what happened, when
it happened, under what context, and with what outcome.

Semantic Memory stores learned meanings and durable conceptual information. It
does not become Knowledge automatically; promotion into Knowledge requires
evidence, provenance, and confidence rules.

Procedural Memory stores reusable procedures, workflows, and task patterns. It
must remain policy-governed because procedures can later influence action.

Project Memory stores project-specific facts, decisions, conventions, files,
constraints, and history. It must be separable by project boundary.

User Memory stores user preferences, stable facts, repeated patterns, and
personal context. It is privacy-sensitive and must be governed by Privacy
Policy, Memory Policy, Retention Policy, and Provenance Policy.

System Memory stores platform operational facts, configuration history, service
state summaries, and architectural decisions. It must not mix with user-private
memory.

Long-Term Memory is persistent memory. It may include episodic, semantic,
procedural, project, user, or system memory after policy approval.

Persistence depends on memory type and policy. Working Memory is never
persistent by definition. Long-Term Memory is persistent by definition.

Retention defines how long memory may live. It must be explicit and
policy-governed.

Indexing makes memories retrievable through approved keys, metadata, semantic
search, temporal references, project boundaries, or future vector indexes.

Retrieval returns memory candidates with provenance, confidence or relevance
signals, and policy-filtered visibility.

Expiration removes or deactivates memory according to retention rules,
privacy rules, and data lifecycle policy.

Versioning records how memory changed over time, especially when memory
influences decisions or future automation.

Provenance records where a memory came from, when it was created, what evidence
supports it, and which policy allowed its persistence.

## Section 5: Knowledge Model

Knowledge is not memory. Memory stores experience. Knowledge stores structured
claims about the world or system.

An Entity is a typed object or concept known to the system.

A Relationship connects entities with a named and typed meaning.

A Fact is a structured claim that may be supported, contradicted, revised, or
deprecated.

A Rule is a reusable constraint or inference pattern. Rules must not become
hidden policy enforcement.

Evidence is support for a fact, relationship, or rule.

Source identifies where evidence came from.

Confidence represents the system's assessed reliability of a claim.

Provenance records origin, transformation path, timestamps, and authority.

Knowledge Graph is the structured representation of entities, relationships,
facts, rules, evidence, sources, confidence, and provenance.

Knowledge without evidence is not accepted Knowledge. It may be a hypothesis,
memory candidate, or reasoning artifact, but not canonical knowledge.

## Section 6: Context Model

User Context describes the relevant user state, preferences, constraints, and
authorized memory visible to the current task.

Task Context describes the current task, goal, input, required output,
constraints, and progress.

Project Context describes project-specific state, files, architecture,
decisions, conventions, and active boundaries.

Environment Context describes runtime environment, local machine constraints,
available tools, service state, permissions, resources, and operational
conditions.

Session Context describes the current session state, recent actions, current
working assumptions, and temporary state.

Conversation Context describes the active conversational exchange. It is not
the whole cognitive state.

Temporal Context describes current time, deadlines, recency, ordering, and
time-sensitive validity.

Execution Context describes current authorized execution scope, tool access,
permissions, pending operations, and safety constraints.

These contexts coexist through composition. Context must remain a current-state
view, not a generic database. Persistent facts belong in Memory or Knowledge.

## Section 7: Planning Model

Planning receives goals and produces plans.

A Goal is a desired outcome.

A Subgoal is a smaller outcome that supports a larger goal.

A Task is an executable or inspectable unit of work.

A Dependency is a required relationship between goals, tasks, capabilities, or
resources.

A Constraint limits valid plans.

A Milestone marks a meaningful progress boundary.

Priority orders competing work.

A Recovery Plan describes what to do when a task, dependency, resource, or
assumption fails.

Planning must produce inspectable artifacts. It must not secretly execute work.

## Section 8: Reasoning Model

Reasoning receives Memory, Knowledge, Context, Goal, Policies, Constraints, and
Inference.

Reasoning produces hypotheses, comparisons, deductions, explanations,
syntheses, uncertainty, and confidence.

Hypotheses are possible interpretations or solutions.

Comparisons evaluate alternatives against evidence and constraints.

Deductions derive consequences from available facts and rules.

Explanations make reasoning inspectable to users, services, and future audit
systems.

Syntheses combine multiple signals into coherent answers or proposals.

Uncertainty records what is unknown, ambiguous, weakly supported, or time
sensitive.

Confidence records how strongly the system supports an output.

Reasoning may call an Inference Provider, but inference is not the owner of
reasoning. The Reasoning Domain frames the request, interprets the result, and
keeps the cognitive architecture stable across model changes.

## Section 9: Decision Model

Decision chooses. It never executes.

Decision receives alternatives, reasoning outputs, context, risk signals,
policy results, confidence, and constraints.

Decision criteria include safety, policy compliance, user intent, evidence
strength, reversibility, resource cost, privacy impact, operational risk, and
expected value.

Every relevant decision must pass through Policies before it can produce an
executable intent.

## Section 10: Action Model

Action executes approved decisions.

Action never reasons.

Action never learns.

Action never decides.

Action must use explicit service, driver, bus, or future automation contracts.
External IO must pass through Driver boundaries and policy/resource constraints
when those phases exist.

Action outcomes must be observable through telemetry, audit, health, and future
learning inputs where appropriate.

## Section 11: Learning Model

Learning never alters the system by itself.

Learning produces observations, patterns, suggestions, and insights.

An observation is a recorded signal about what happened.

A pattern is a repeated structure detected across observations or memories.

A suggestion is a proposed change, memory update, knowledge promotion,
procedure, or automation candidate.

An insight is a higher-level interpretation that may inform future reasoning or
planning.

Persistence, promotion, automation, or behavior change requires Policies.
Learning without policy is prohibited.

## Section 12: Inference Model

LLM is not intelligence.

An Inference Provider is a replaceable component that executes model-backed
inference requests.

A Model Provider supplies access to one or more models.

Local Models run on local infrastructure or device boundaries.

Remote Models run through external providers and require future privacy,
security, telemetry, and network policies.

Future Models may include multimodal, symbolic, neuro-symbolic, specialized, or
domain-specific inference systems.

The correct flow is:

```text
Reasoning
  -> Inference Provider
  -> Model
  -> Reasoning
```

The Reasoning Domain owns the cognitive task. The Inference Provider performs a
bounded inference operation. The model is replaceable. Aether must be able to
replace GPT, local models, or future models without changing the cognitive
architecture.

## Section 13: Future Cognitive Managers

Memory Manager will coordinate memory registration, retention, retrieval
policies, indexing contracts, versioning, and memory health.

Knowledge Manager will coordinate knowledge schema boundaries, graph health,
evidence requirements, confidence updates, and provenance rules.

Context Manager will coordinate current-state assembly, context freshness,
context visibility, and context health.

Planning Manager will coordinate goal decomposition, plan lifecycle, dependency
validation, milestone state, and recovery planning.

Reasoning Manager will coordinate reasoning requests, inference provider
selection boundaries, uncertainty tracking, and reasoning health.

Decision Manager will coordinate alternative selection, policy checks, decision
records, and escalation paths.

Learning Manager will coordinate pattern detection, suggestion review,
promotion workflows, and learning health.

Perception Manager will coordinate perception providers, observation contracts,
modality handling, and perception health.

Managers coordinate cognitive domains. They do not execute user-facing
capabilities directly and must not bypass Services, Policies, ASB, or Contract
Bus.

## Section 14: Future Cognitive Services

Memory Service will execute memory capabilities such as memory write, read,
retrieve, index, expire, and version.

Knowledge Service will execute knowledge capabilities such as entity
management, relationship queries, evidence linking, and confidence updates.

Context Service will execute context capabilities such as context read,
snapshot, refresh, and scoped update.

Planning Service will execute planning capabilities such as plan creation,
task decomposition, dependency analysis, and recovery plan generation.

Reasoning Service will execute reasoning capabilities such as hypothesis
generation, comparison, explanation, synthesis, uncertainty analysis, and
inference orchestration.

Decision Service will execute decision capabilities such as alternative
evaluation, policy-gated selection, deferral, rejection, and escalation.

Learning Service will execute learning capabilities such as observation
analysis, pattern discovery, suggestion generation, and promotion proposals.

Perception Service will execute perception capabilities such as input
normalization, signal extraction, modality classification, and observation
creation.

Services communicate only through ASB or Contract Bus. Service-to-service
direct calls remain prohibited.

## Section 15: Cognitive Policies

Memory Policy governs memory capture, retrieval, retention, update, expiry,
visibility, and deletion.

Learning Policy governs what learning may observe, what suggestions may be
created, and what promotion workflow is required.

Privacy Policy governs user-sensitive data, consent boundaries, redaction,
locality, and visibility.

Retention Policy governs lifespan, expiration, archival, deletion, and legal or
user-defined retention constraints.

Reasoning Policy governs allowed reasoning inputs, uncertainty requirements,
model-use boundaries, and explanation requirements.

Action Policy governs whether a decision may become an action and under what
authorization, reversibility, and safety constraints.

Evidence Policy governs what evidence is required for knowledge, decisions, and
memory promotion.

Provenance Policy governs origin tracking, transformation history, source
trust, and auditability.

## Section 16: Anti-Patterns

Chatbot First is prohibited. Conversation is an interface, not the architecture.

LLM First is prohibited. Models provide inference; they do not own cognition.

Memory inside LLM is prohibited. Durable memory must be platform-owned,
inspectable, policy-governed, and model-independent.

Knowledge without Evidence is prohibited. Unsupported claims are hypotheses or
memory candidates, not accepted knowledge.

Learning without Policy is prohibited. Learning can suggest; Policies decide
what may change.

Action without Authorization is prohibited. Execution requires explicit
decision and policy approval.

Kernel owning cognition is prohibited. Cognitive logic belongs to cognitive
domains, managers, services, and policies.

Service calling Service directly is prohibited. Communication must pass through
ASB or Contract Bus.

Hidden State is prohibited. Important state must be discoverable, inspectable,
owned, and governed.

Implicit Context is prohibited. Context must be explicit, scoped, current, and
auditable where needed.

## Section 17: Cognitive Principles

- Memory stores experience.
- Knowledge stores truth.
- Context stores the present.
- Planning creates futures.
- Reasoning creates hypotheses.
- Decision chooses.
- Action executes.
- Learning suggests.
- Policies govern.
- Inference accelerates.
- Provenance explains origin.
- Evidence supports knowledge.
- Confidence expresses uncertainty.
- Managers coordinate.
- Services execute capabilities.
- Drivers touch the outside world.
- The Kernel coordinates the platform.

## Section 18: Implementation Roadmap

The official recommended RFC order for Era II is:

1. RFC-0001 Memory Domain
2. RFC-0002 Knowledge Domain
3. RFC-0003 Context Domain
4. RFC-0004 Planning Domain
5. RFC-0005 Reasoning Domain
6. RFC-0006 Decision Domain
7. RFC-0007 Learning Domain
8. RFC-0008 Perception Domain

Memory comes first because future cognitive work needs explicit rules for
experience, persistence, retention, retrieval, and provenance.

Knowledge follows because structured truth must be separated from memory before
reasoning depends on it.

Context follows because the current-state model must bind user, task, project,
environment, session, conversation, time, and execution.

Planning, Reasoning, Decision, Learning, and Perception then become safer to
define because their inputs, outputs, boundaries, and policy requirements are
clearer.

## Section 19: Risks

- LLM becomes the center of the system.
- Memory and Knowledge mix into one unclear storage layer.
- Context becomes a generic database.
- Learning changes behavior without approval.
- Reasoning executes actions directly.
- Decision bypasses Policies.
- Action contains hidden reasoning.
- Knowledge lacks evidence or confidence.
- Memory lacks provenance or retention.
- Services call each other directly.
- Managers accumulate domain algorithms.
- Kernel grows into cognitive ownership.
- Inference providers leak policy-sensitive data.
- Model-specific assumptions enter core contracts.
- Conversation history is mistaken for full context.
- Working Memory is accidentally persisted.
- Long-Term Memory is updated without review.
- External IO bypasses Drivers.
- Policy logic is scattered inside services.
- Cognitive domains become circularly dependent.

## Section 20: Acceptance Criteria

The Cognitive Era may begin implementation only when:

- CDR-0001 is approved.
- The target domain RFC is approved.
- The Architecture Guardian approves the phase boundary.
- AEPs are respected.
- Architecture Constitution v2 is respected.
- The Domain is defined before code.
- Public contracts are defined before implementation.
- Policies are defined or explicitly deferred with justification.
- Risks are documented.
- Test strategy is documented.
- Service communication path uses ASB or Contract Bus.
- Kernel remains coordination-only.
- Models remain replaceable inference mechanisms.

Until these criteria are satisfied, Memory, Knowledge, Context, Planning,
Reasoning, Decision, Action, Learning, AI, and agents must not be implemented.
