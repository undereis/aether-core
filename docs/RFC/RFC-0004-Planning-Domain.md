# RFC-0004: Planning Domain

## 1. Status

Accepted.

This RFC is documentation-only and published in the
`v0.5.8-planning-domain-rfc` checkpoint at commit
`1ae21ac42a2a2fc0898ab639191e56c8df3a43f6`. It defines the future Planning
Domain for Aether but does not implement runtime behavior, a Planning Engine, a
Planning Service, a Planning Manager, crates, contracts, APIs, persistence,
inference, AI, agents, or tool execution. Planning is not implemented.

No functional behavior changes are introduced by this document.

## 2. Summary

The Planning Domain transforms an authorized goal, present context,
constraints, and policy limits into structured, inspectable possible futures.

Its output is a set of candidate plans containing subgoals, steps,
dependencies, preconditions, expected outputs, risks, budgets, fallback
options, and stop conditions.

The central boundary is:

```text
Planning structures possible futures.
Reasoning evaluates implications and uncertainty.
Decision chooses.
Action executes.
```

Planning does not execute work, choose the final alternative, persist Memory,
create Knowledge, assemble Context, learn, perceive raw input, or perform deep
reasoning.

## 3. Motivation

Planning belongs after Memory, Knowledge, and Context because useful plans need
clear access to prior experience, structured truth, and the authorized present
state without owning any of those responsibilities.

Memory may contribute references to prior goals, failures, constraints, and
successful workflows. Knowledge may contribute rules, facts, definitions, and
known dependencies. Context binds only the relevant, authorized, fresh signals
for the current operation. Planning consumes those bounded inputs to structure
possible futures.

Planning belongs before Reasoning and Decision in the conceptual pipeline
because candidate futures must exist before their implications can be deeply
evaluated or one alternative can be chosen. This ordering does not make the
runtime a rigid one-way chain. Optional, bounded assessment may refine a plan,
but ownership remains separate and cycles must be explicitly controlled.

Without a dedicated Planning Domain, Aether risks mixing objective
decomposition, inference, selection, execution, persistence, and automation in
one opaque component. That would create hidden cognitive state, unsafe action
chains, unbounded cost, and architecture that is difficult for humans and AI
coding agents to maintain.

## 4. Design Principles

- Planning structures possible futures.
- Reasoning evaluates implications and uncertainty.
- Decision chooses.
- Action executes.
- Planning is not execution.
- Planning is not Decision.
- Planning is not Memory.
- Planning is not Knowledge.
- Planning is not Context assembly.
- Planning is not deep Reasoning.
- Planning is not an agent loop.
- Plans must be traceable, bounded, inspectable, expirable, and policy-aware.
- Candidate plans remain proposals until Decision selects one.
- Rejected alternatives remain explainable within privacy boundaries.
- Efficient planning is bounded planning, not unsafe planning.
- Models may assist future inference but never own Planning.
- The Kernel coordinates; it does not plan.

## 5. Domain Definition

Planning is the cognitive domain responsible for representing goals as
structured candidate futures.

Planning consumes:

- an authorized planning intent;
- a goal and scope;
- an authorized Context view;
- supplied Memory and Knowledge references already bounded by Context;
- constraints and preconditions;
- policy decisions and permission boundaries;
- capability descriptions without invoking those capabilities;
- budget, horizon, freshness, and expiry limits;
- optional assessment references from future Reasoning contracts.

Planning produces:

- one or more candidate plans;
- subgoals and ordered or partially ordered steps;
- dependency and precondition relationships;
- expected outputs and milestones;
- explicit assumptions and uncertainty markers;
- risk registers and fallback options;
- budget, horizon, escalation, and stop-condition metadata;
- declarative information requirements when required inputs are missing;
- explanations and trace references.

Planning exposes inspectable candidate structures and lifecycle state. It does
not expose an executable action, mutate another cognitive domain, or conceal
state inside prompts, caches, model sessions, managers, or services.

## 6. Boundaries

### Planning vs Memory

Memory stores experience. Planning may receive authorized references to past
experience through Context, but it does not retrieve Memory directly, store
plans as Memory, or promote observations into persistent Memory.

### Planning vs Knowledge

Knowledge stores structured truth. Planning may consume authorized facts,
rules, definitions, constraints, and dependency references through Context. It
does not create, validate, invalidate, or mutate Knowledge.

### Planning vs Context

Context stores the present operational frame. Planning receives an authorized
Context view. It does not assemble, refresh, compress, persist, or mutate
Context.

### Planning vs Reasoning

Planning structures candidates. Reasoning evaluates assumptions, implications,
tradeoffs, uncertainty, and evidence. Structural decomposition and validation
belong to Planning; deep inference and epistemic evaluation belong to
Reasoning.

### Planning vs Decision

Planning proposes valid candidates. Decision chooses among valid alternatives.
Planning may reject structurally invalid or policy-denied candidates, but it
does not select the final plan.

### Planning vs Learning

Learning suggests patterns and improvements. Planning may emit observations
about plan structure or external outcomes, but it does not learn, change its own
behavior, or persist suggestions.

### Planning vs Perception

Perception transforms raw input into structured observations. Planning consumes
only authorized, structured signals and does not interpret raw input.

### Planning vs Prompt

A prompt is an inference representation. A plan is a governed cognitive
artifact. Prompt content, model conversation, and hidden provider state are not
Planning state.

### Planning vs Cache

Cache may optimize future computation. It must not become hidden plan state,
Memory, source of truth, or an untraceable substitute for a candidate plan.

### Planning vs Tool Execution

Planning may reference a declared tool capability, required permission,
expected output, cost class, and fallback. It never invokes the tool.

### Planning vs Agents

Planning may describe multiple steps without creating an agent loop. Agents do
not own Planning, and multi-step planning does not imply autonomous execution.

### Planning vs AI and Model Routing

Planning remains model-independent. Future model assistance and routing belong
behind approved Inference contracts and RFC-0009 governance; they are not part
of this RFC's implementation.

### Planning vs Policy Enforcement

Policies own behavioral constraints and authorization decisions. Planning
binds, applies, and explains supplied policy decisions. It does not become the
Policy Engine or silently override a denial.

### Planning vs Audit and Trace

Planning produces audit-friendly metadata and trace events. Audit and telemetry
infrastructure remain separate platform responsibilities.

## 7. Planning and Reasoning

Planning does not depend obligatorily on Reasoning.

The minimum Planning capability must be able to structure a goal, decompose it
into subgoals and steps, validate dependency shape, apply supplied constraints,
attach budgets, and produce candidate plans without deep inference.

Future Reasoning may evaluate:

- assumptions and weak premises;
- tradeoffs between candidates;
- uncertainty and missing evidence;
- consequences and risk interactions;
- contradictions between inputs;
- whether escalation is justified.

A future `ReasoningAssessment` is optional advisory input mediated by a typed
contract. It does not give Reasoning ownership of the plan, and Planning does
not call a Reasoning Service directly.

The Phase 4 Service Map lists `reasoning.execute` as a future Planning
requirement. RFC-0004 reinterprets that entry as an optional future capability
reachable only through ASB or Contract Bus, not as a rigid implementation
dependency. The basic Planning Domain must remain usable before RFC-0005 defines
Reasoning.

Any Planning/Reasoning refinement loop must be explicit and bounded by:

- a maximum iteration count;
- planning and inference budgets;
- correlation and causation identifiers;
- freshness and expiry checks;
- an escalation reason;
- stop conditions;
- a terminal outcome such as exposed, rejected, expired, or delegated.

Reasoning may evaluate a planning problem before candidate generation or assess
a candidate set afterward. Planning may refine candidates from that assessment.
Neither domain may recursively invoke the other, and an orchestrated refinement
must not become hidden state.

RFC-0005 retains ownership of deep inference, hypotheses, comparisons,
deductions, uncertainty evaluation, confidence interpretation, and explanation
of epistemic conclusions.

## 8. Planning and Decision

Planning proposes candidate plans. Decision chooses.

Planning may eliminate a candidate only when:

- its structure is invalid;
- a dependency cycle violates declared rules;
- required preconditions cannot be represented;
- its Context binding is stale or expired;
- it exceeds a hard budget or scope limit;
- a Policy decision explicitly denies it.

Every elimination must retain a sensitivity-safe reason and trace reference.
Planning must not rank a valid candidate as the final choice or convert a
candidate into executable intent.

`decision_status` is a future external reference owned by Decision lifecycle,
not a decision made by Planning. Planning may display that reference after a
Decision event but must not mutate it as if Planning had chosen.

Tradeoffs, constraints, invalid candidates, and alternatives rejected by
Decision should remain explainable without exposing sensitive content.

## 9. Planning and Context

Planning receives an authorized `ContextView` or equivalent future contract
result. It does not build or modify Context.

The Planning input must preserve:

- source references and provenance;
- scope and owner boundaries;
- freshness and expiry;
- compression state and disclosed information loss;
- visibility and sensitivity classifications;
- policy and permission constraints;
- trace, correlation, and causation links.

Stale or expired Context must block sensitive planning or produce an explicit
revalidation requirement. Planning must not silently use stale state.

When information is missing, Planning may emit a declarative information need
describing the required evidence, scope, reason, and sensitivity. Planning does
not satisfy that need by searching Context, Memory, Knowledge, files, tools, or
external systems directly.

## 10. Planning, Memory, and Knowledge

Planning may receive authorized Memory and Knowledge references through the
Context binding.

Memory can influence a plan with prior goals, failures, constraints, recovery
patterns, and observed outcomes. Knowledge can influence a plan with facts,
rules, definitions, relationships, and known dependencies.

Planning must not:

- call Memory or Knowledge services directly;
- perform retrieval itself;
- persist a plan as Memory;
- infer that a Memory signal is structured truth;
- invent, validate, or mutate Knowledge;
- copy sensitive source content into plan traces;
- continue using an invalidated or unauthorized reference.

Influence must remain traceable through references, provenance, confidence,
freshness, policy visibility, and sensitivity-safe explanations.

## 11. Planning and AEP-0016 Cognitive Traceability

AEP-0016 governs every future plan and Planning operation.

Conceptual events include:

- `PlanningRequestReceived`: records an authorized planning request.
- `PlanCandidateGenerated`: records creation of a candidate without raw plan
  content by default.
- `PlanConstraintApplied`: records a constraint and its source reference.
- `PlanRiskDetected`: records a risk classification and explanation reference.
- `PlanningEscalationRequested`: records why greater depth or assessment is
  requested.
- `PlanRefined`: records lineage from an earlier candidate and the refinement
  cause.
- `PlanRejected`: records whether rejection was structural, budgetary,
  contextual, policy-driven, or produced later by Decision.
- `PlanCandidateSetExposed`: records the candidate set handed to an authorized
  consumer.
- `PlanSelectedByDecision`: is produced or authorized by the future Decision
  boundary and referenced by Planning; Planning does not choose.
- `PlanExpired`: records expiry or discard.

Minimum conceptual metadata includes:

- plan id;
- request reference;
- goal reference;
- actor;
- owner;
- scope;
- Context references;
- provenance;
- policy decisions;
- constraints;
- lifecycle state;
- version;
- budget;
- confidence or confidence reference;
- rejected alternative references;
- correlation id;
- causation id;
- timestamps;
- expiry;
- explanation reference;
- sensitivity classification.

Events, logs, and traces must be metadata-first. They must not carry raw goals,
Context, rejected alternatives, tool inputs, Memory content, Knowledge evidence,
or sensitive plan details unless an explicit policy and visibility boundary
requires and authorizes that content.

## 12. Planning and RFC-0009 Efficient Intelligence

Efficient planning seeks the least depth sufficient for a safe, useful plan.

Future Planning should expose:

- explicit plan depth and horizon;
- maximum step and branch counts;
- iteration and refinement budgets;
- cost, token, latency, energy, and resource limits where applicable;
- escalation conditions and justification;
- fallback options;
- stop conditions;
- the reason a larger or deeper planning path was needed.

Simple goals should not produce elaborate plans without justification.
Over-budget or excessive plans should be reduced, rejected, or escalated
transparently. Reduction must disclose what was removed and must not hide risk,
policy, permission, provenance, or safety information.

Tool-first execution means a candidate plan may prefer a deterministic tool
capability when that capability is safer, cheaper, or more precise. Planning
records the capability, preconditions, permissions, expected output, fallback,
and budget. It does not execute the tool.

Cost and efficiency never override safety, policy, permissions, traceability,
auditability, owner boundaries, or required human review.

## 13. Planning Types

- Task planning structures work needed for a bounded task.
- Workflow planning structures coordinated stages without executing a workflow.
- Recovery planning defines alternatives after failed assumptions,
  dependencies, resources, or outcomes.
- Dependency planning models ordering and prerequisite relationships.
- Resource-aware planning binds declared resource and budget constraints.
- Policy-aware planning applies supplied policy decisions and required reviews.
- Uncertainty-aware planning makes assumptions, unknowns, and escalation needs
  visible without performing deep Reasoning.
- Tool-use planning without execution references tool capabilities and safety
  requirements but invokes nothing.
- Multi-step planning without agents structures multiple steps without creating
  autonomous loops.
- Fallback planning defines bounded alternatives and stop conditions.

These types are conceptual classifications, not enums, structs, APIs, schemas,
or runtime components.

## 14. Conceptual Plan Model

A future plan should conceptually expose:

- `plan_id`: stable identity for the candidate.
- `version`: immutable or traceable candidate version.
- `goal`: authorized desired outcome or reference.
- `scope`: bounded operational scope.
- `actor`: initiator or authorized system actor.
- `owner`: owner boundary for visibility and authority.
- `context_refs`: authorized Context and source references.
- `assumptions`: explicit premises that may require evaluation.
- `constraints`: structural, temporal, resource, and operational limits.
- `policy_constraints`: supplied policy decisions and required approvals.
- `steps`: ordered or partially ordered candidate steps.
- `dependencies`: relationships among steps, capabilities, and prerequisites.
- `preconditions`: requirements before a step can become eligible.
- `expected_outputs`: declared outcomes, not executed results.
- `risk_register`: identified risks and source references.
- `fallback_options`: bounded recovery candidates.
- `stop_conditions`: conditions that end, reject, expire, or escalate planning.
- `budget`: depth, horizon, iteration, cost, token, latency, energy, or resource
  limits where applicable.
- `confidence`: structural confidence or an external assessment reference; not
  deep epistemic confidence owned by Reasoning.
- `freshness`: Context and input validity metadata.
- `provenance`: origin and lineage references.
- `trace_links`: request, event, correlation, and causation references.
- `expiry`: point after which revalidation or discard is required.
- `decision_status`: external Decision reference, never Planning's choice.
- `explanation`: sensitivity-safe explanation of structure and influences.

This RFC does not define concrete types or serialization.

## 15. Conceptual Lifecycle

1. Receive planning intent.
2. Bind goal and authorized Context.
3. Validate freshness and scope.
4. Identify constraints and preconditions.
5. Generate candidate plans.
6. Validate structure, dependencies, risks, and budgets.
7. Apply supplied policy decisions.
8. Reduce or compact without hiding loss.
9. Expose candidate plans.
10. Hand candidates to the Decision Domain through a future contract.
11. Trace usage and externally reported outcome.
12. Expire or discard the plan.
13. Optionally emit observations through approved contracts, never Memories.

Lifecycle transitions must be attributable, versioned where needed, bounded by
policy, and visible through AEP-0016 trace metadata. A plan does not become an
action merely because it reached the exposed or selected state.

## 16. Future Manager and Service Shape

A future Planning Manager would govern the Planning Domain lifecycle. It may
coordinate registration, policy orchestration, health, budgets, expiry,
traceability, and bounded recovery. It must not generate candidate content,
execute tools, choose plans, persist Memory, call services directly, or become a
God Manager.

A future Planning Service would provide declared Planning capabilities such as
candidate generation, structural decomposition, validation, explanation, and
inspection. It must not choose among valid candidates, invoke tools, own policy,
call other services directly, or become a God Service.

The Kernel and entrypoints remain maestros. They coordinate generic platform
lifecycle and routing; they do not own goals, plans, reasoning, decisions, or
execution logic.

All future communication must use ASB or Contract Bus contracts. Logical
capability dependencies never permit direct service references.

## 17. Contracts and Events

Future conceptual contract vocabulary may include:

- `PlanningRequest`;
- `PlanningContextBinding`;
- `CandidatePlan`;
- `PlanCandidateSet`;
- `PlanConstraint`;
- `PlanRisk`;
- `PlanBudget`;
- `PlanFallback`;
- `PlanLifecycleState`;
- `ReasoningAssessmentRef`;
- `DecisionStatusRef`.

These names identify future contract responsibilities only. They do not create
Rust structs, traits, APIs, handlers, routes, schemas, or compatibility
commitments in this phase.

The conceptual events in Section 11 describe lifecycle observability. Event
ownership must follow domain boundaries: Decision owns selection, Policy owns
policy results, Context owns Context assembly and freshness, and Planning owns
candidate-plan lifecycle events.

## 18. Security and Privacy

Every planning request and plan must have an actor and owner boundary.

Future Planning must enforce through supplied policies and contracts:

- authorization and permission boundaries;
- data minimization;
- redaction and sensitivity classification;
- owner, project, user, session, and future tenant isolation;
- safe explanation without raw sensitive content;
- protection against prompt injection in supplied Context;
- protection against poisoned or stale plan inputs;
- no prompt dumping;
- no hidden Context;
- no cache as Memory;
- no plan leakage across scopes;
- no exposure of sensitive rejected alternatives;
- explicit review before sensitive or irreversible downstream action.

A plan is not authorization to execute. Selection by Decision is also not
execution authorization unless future Action policies and permissions approve
the operation.

## 19. Main-as-Maestro and AI-Maintainable Architecture

Planning must remain a cohesive domain, not a container for Context, Reasoning,
Decision, Action, automation, tools, agents, or persistence.

Future architecture should use small, legible contracts and cohesive modules
for candidate structure, lifecycle, constraints, risk, budget, trace, and
explanation. One file, service, or manager must not own all of these concerns by
convenience.

Failures should be small, isolated, traceable, and recoverable. A failed
candidate should not corrupt other candidates. A stale Context binding should
invalidate only affected plans. A failed optional assessment should leave a
visible, bounded outcome rather than hidden partial state.

Human engineers and AI coding agents must be able to determine from ownership
and contracts whether a change belongs to Planning, Reasoning, Decision, or
Action. Adding planning, reasoning, decision, and execution to the same module,
entrypoint, manager, service, or agent is prohibited.

## 20. Non-Goals

RFC-0004 does not:

- implement a Planning Engine;
- implement a Planner Service or Planning Service;
- create a planner crate;
- create a Planning Manager;
- create APIs;
- create database tables;
- create migrations;
- create schemas;
- create storage or persistence;
- implement cache;
- implement compression;
- implement model routing;
- implement agents;
- implement an AI runtime;
- execute tools;
- implement Memory;
- implement Knowledge;
- implement Context;
- implement Reasoning;
- implement Decision;
- implement Learning;
- implement Perception;
- implement Action;
- change Kernel, Manager, Service, Driver, Domain, Policy, ASB, or Contract Bus
  behavior;
- add dependencies;
- change runtime behavior.

## 21. Risks

- Planning becomes Reasoning.
- Planning becomes Decision.
- Planning and Reasoning form an unbounded conceptual or runtime cycle.
- Planning executes tools or automation.
- `planning.track` becomes a scheduler, workflow engine, persistence layer, or
  hidden state.
- Planning persists Memory.
- Planning invents or mutates Knowledge.
- Planning uses stale or expired Context.
- compression hides critical information loss.
- rejected alternatives disappear or become unexplainable.
- plans lack actor, owner, policy, provenance, or trace metadata.
- overplanning wastes cost, tokens, latency, energy, and attention.
- optimization bypasses safety, permission, policy, or human review.
- Planning becomes a God Domain.
- Planning Manager becomes a God Manager.
- Planning Service becomes a God Service.
- Service Map metadata is mistaken for existing Planning implementation.
- optional model assistance becomes mandatory or provider-owned Planning.
- plan confidence is mistaken for truth or deep Reasoning confidence.
- `decision_status` is mutated by Planning.
- plan candidates become implicit authorization to execute.

## 22. Future Test Strategy

Future implementation should include:

- no tool execution from Planning tests;
- no persistence from Planning tests;
- no Decision from Planning tests;
- trace required for plan generation tests;
- policy constraints applied tests;
- rejected alternatives traced tests;
- stale Context rejected or revalidated tests;
- over-budget plan reduced or rejected tests;
- no hidden state tests;
- no direct service-call tests;
- no prompt dump tests;
- cache-is-not-Memory tests;
- Planning/Reasoning cycle guard tests;
- no Planning/Decision merge tests;
- Kernel-does-not-own-Planning tests;
- Planning Manager does not execute capability tests;
- Planning Service does not choose or execute tests;
- sensitive-data leakage tests;
- correlation and causation continuity tests;
- expiry and invalidation tests;
- candidate isolation and bounded-failure tests.

Tests must cover allowed and denied paths. They must prove that structural
validation does not silently become deep Reasoning and that a selected plan
does not execute without the future Decision, Action, Policy, Permission, and
Driver boundaries.

This RFC does not implement tests.

## 23. Acceptance Criteria

RFC-0004 is ready for architecture review when it:

- defines the Planning Domain and its permanent boundaries;
- defines the conceptual Plan model;
- defines candidate-plan lifecycle;
- separates Planning from Reasoning;
- separates Planning from Decision and Action;
- treats Reasoning assessment as optional and contract-mediated;
- bounds every Planning/Reasoning refinement cycle;
- respects Context freshness, expiry, provenance, compression, and policy
  constraints;
- keeps Memory and Knowledge behind authorized Context references;
- complies with AEP-0016 traceability;
- applies RFC-0009 efficiency without weakening safety;
- defines future Manager and Service responsibilities without God components;
- defines contracts and events conceptually without creating code;
- defines security, privacy, risks, tests, open questions, and non-goals;
- keeps the Kernel coordination-only;
- requires ASB or Contract Bus and prohibits direct service calls;
- creates no runtime behavior;
- receives Architecture Guardian review.

Planning implementation remains blocked until this RFC is accepted, a future
implementation ADR is accepted, contracts and policies are defined, storage or
non-storage strategy is approved, test strategy is approved, and the
Architecture Guardian explicitly authorizes implementation.

## 24. Open Questions

- What is the exact future shape of `ReasoningAssessment`?
- How much of the Phase 4 Service Map should be revised so
  `reasoning.execute` cannot be mistaken for a rigid dependency?
- When does `planning.track` describe legitimate lifecycle observation, and
  when does it become Scheduler or Workflow ownership?
- How should rejected alternatives remain explainable without violating
  privacy?
- When does a plan become a workflow and leave the Planning Domain?
- How should structural confidence be represented without invading Reasoning?
- How should plan expiry bind to Context expiry and source invalidation?
- Which policy denials require human-visible explanation?
- Which plan classes require human review before Decision?
- What is the smallest useful Planning contract set for a future prototype?

## 25. Future Implementation Phases

The sequence below is conceptual and creates no implementation commitment:

1. RFC-0004 architecture review and acceptance.
2. Future Planning implementation ADR.
3. Future Planning contracts and compatibility rules.
4. Future Planning policies and permission model.
5. Future test harness and denied-path specifications.
6. Future Planning Manager skeleton after explicit approval.
7. Future Planning Service skeleton after explicit approval.
8. Future in-memory candidate-plan prototype after explicit approval.
9. Future AEP-0016 traceability integration.
10. Future RFC-0009 budget and efficiency integration.
11. Future compliance and Architecture Guardian review.

No phase may begin automatically from this RFC. Every implementation phase
requires a separate authorized scope and must preserve the boundaries defined
here.
