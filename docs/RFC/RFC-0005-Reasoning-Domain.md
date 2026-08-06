# RFC-0005: Reasoning Domain

## 1. Status

- Status: Draft.
- Category: Cognitive Architecture.
- Scope: Documentation-only.
- Publication: Not published.
- Checkpoint: None.
- Implementation: Not implemented.

This RFC defines architecture boundaries for a future Reasoning Domain. It does
not create runtime behavior, contracts, APIs, storage, services, managers,
providers, agents, or executable reasoning capabilities. It is pending
architectural review and a future checkpoint and publication.

## 2. Summary

Reasoning is the cognitive domain responsible for evaluating implications,
relationships, hypotheses, evidence, contradictions, and uncertainty within an
authorized scope, producing structured, explainable, and non-binding
assessments.

Reasoning sits between candidate formation and operational choice:

```text
Planning structures possible futures.
Reasoning evaluates implications and uncertainty.
Decision chooses.
Action executes.
```

Reasoning may compare evidence, identify gaps, test assumptions, expose risks,
and produce a non-binding recommendation. It MUST NOT choose, authorize, or
execute an operational action.

## 3. Motivation

Aether needs explicit ownership for cognitive evaluation. Evidence comparison,
implication analysis, contradiction detection, hypothesis assessment, and
uncertainty handling are durable architectural responsibilities even when the
underlying strategy or inference provider changes.

Without a Reasoning Domain, those responsibilities could become hidden inside:

- Planning, causing plans to silently choose or justify themselves;
- Decision, obscuring the boundary between assessment and selection;
- providers, making the cognitive architecture model-owned;
- prompts, creating implicit and unversioned contracts;
- agents, combining evaluation with autonomous execution;
- tools, mistaking deterministic output for cognitive ownership;
- application services, producing direct coupling and inconsistent policy;
- caches, preserving untraceable conclusions as hidden state.

Dedicated ownership makes assessments bounded, inspectable, policy-aware,
provider-independent, failure-isolated, and reusable only under explicit rules.
It also allows Decision to consume assessments without surrendering its own
authority to choose.

## 4. Architectural Principles

- Reasoning evaluates; Decision chooses.
- Every assessment MUST be non-binding.
- Cross-domain communication MUST use versioned contracts through ASB or
  Contract Bus.
- Direct service calls are prohibited.
- Reasoning MUST consume only authorized context and references.
- Evidence is not automatically truth.
- Inference is not automatically Knowledge.
- Confidence is not certainty.
- Policies MUST precede data exposure, provider use, reuse, and escalation.
- Every request MUST be bounded by finite limits and explicit stop conditions.
- Reasoning failure MUST remain isolated and recoverable.
- Providers perform inference; they do not own the Reasoning Domain.
- Explanations MUST be structured, safe, and appropriate to the caller.
- Private chain-of-thought is not a domain artifact and MUST NOT be required,
  persisted, logged, exposed, or transported.
- The Kernel coordinates the platform; it does not reason.
- Main-as-Maestro applies: entrypoints orchestrate lifecycle and MUST NOT become
  reasoning executors or God Files.
- Human and AI maintainability require cohesive ownership, explicit contracts,
  small future modules, and inspectable failure paths.

## 5. Definition and Ownership

The Reasoning Domain owns the meaning and lifecycle of structured cognitive
assessment.

It owns:

- implication analysis within an authorized scope;
- comparison of supplied evidence, alternatives, and candidate futures;
- evaluation of assumptions and constraints;
- contradiction, inconsistency, and information-gap detection;
- explicit uncertainty representation;
- assessment confidence semantics;
- alternative interpretations and counterfactual analysis;
- non-binding recommendations and recommendation strength;
- safe explanation requirements;
- reasoning budgets, stop reasons, and escalation requests;
- assessment traceability and lifecycle semantics.

Ownership remains separated as follows:

- Planning generates and owns possible-future structures.
- Reasoning evaluates candidates, implications, evidence, and uncertainty.
- Decision owns selection, rejection, deferral, and operational choice.
- Action owns authorized execution.
- Context owns authorized present-state assembly.
- Memory owns experience and retrieval candidates.
- Knowledge owns structured truth and truth candidates.
- Policies own behavioral constraints and allow/deny outcomes.
- Providers own bounded inference execution behind replaceable adapters.

Reasoning may recommend. A recommendation is an assessment output, not a
decision, authorization, command, plan mutation, or action intent.

## 6. Invariants

- Reasoning MUST NOT choose the final operational action.
- Reasoning MUST NOT authorize execution.
- Reasoning MUST NOT call tools directly.
- Reasoning MUST NOT persist Memory.
- Reasoning MUST NOT mutate Knowledge.
- Reasoning MUST NOT assemble or mutate Context.
- Reasoning MUST NOT silently own or rewrite Planning artifacts.
- Reasoning MUST NOT learn or mutate future behavior by itself.
- Reasoning MUST preserve provenance, sensitivity, and authorization
  boundaries received from upstream contracts.
- Every assessment MUST be explicitly non-binding.
- Every request MUST have finite depth, iteration, time, cost, token, or other
  applicable resource limits.
- Every assessment MUST expose uncertainty, completion state, and stop reason.
- Partial results MUST be distinguishable from complete results.
- Stale results MUST NOT be silently reused.
- Every cross-domain interaction MUST use contracts through ASB or Contract
  Bus.
- Every provider output MUST be treated as untrusted until validated.
- Every lifecycle transition MUST be traceable under AEP-0016.
- Private chain-of-thought MUST NOT be a required input, output, trace, event,
  log, cache entry, or audit record.

## 7. Domain Boundaries

### 7.1 Memory

- Allowed input: authorized references to experiential evidence, confidence,
  conflict state, provenance, freshness, and policy visibility.
- Allowed output: assessment references or explicit promotion suggestions.
- Prohibited behavior: direct retrieval, persistence, promotion, deletion, or
  retention mutation.
- Responsibility owner: Memory owns experience and its lifecycle.
- Interaction: preferably through Context; otherwise through an explicit
  Memory contract mediated by ASB or Contract Bus.
- Failure behavior: unavailable, stale, or unauthorized memory becomes a
  visible gap, stop reason, or bounded degraded assessment.

### 7.2 Knowledge

- Allowed input: authorized claims, entities, relationships, rules, evidence,
  contradiction status, confidence, provenance, validity, and version.
- Allowed output: hypotheses, contradiction findings, or update proposals.
- Prohibited behavior: creating, validating, promoting, invalidating, or
  mutating official Knowledge.
- Responsibility owner: Knowledge owns structured truth and truth candidates.
- Interaction: through Context references or explicit Knowledge contracts.
- Failure behavior: unsupported or conflicting claims remain uncertain; they
  MUST NOT be treated as settled truth.

### 7.3 Context

- Allowed input: an authorized, bounded, fresh `ContextView` with visibility,
  expiry, provenance, and policy constraints.
- Allowed output: explicit requests for additional information and context-use
  trace references.
- Prohibited behavior: assembly, refresh, compression, retrieval, persistence,
  or mutation of Context.
- Responsibility owner: Context owns present-state assembly.
- Interaction: through a versioned Context contract.
- Failure behavior: missing, stale, contaminated, or expired Context MAY stop
  assessment or produce an explicit partial result.

### 7.4 Planning

- Allowed input: candidate plan references, assumptions, constraints, risks,
  dependencies, budgets, and requested assessment questions.
- Allowed output: findings, implication analysis, contradictions, and explicit
  refinement requests.
- Prohibited behavior: silently rewriting, selecting, executing, or owning a
  plan.
- Responsibility owner: Planning owns candidate future structures.
- Interaction: through versioned request and assessment contracts.
- Failure behavior: Planning MAY continue in degraded mode without advanced
  assessment when policy and risk permit.

### 7.5 Decision

- Allowed input: a scoped request for evaluation criteria or alternatives.
- Allowed output: structured, non-binding assessments and recommendations.
- Prohibited behavior: final selection, rejection, authorization, or creation
  of executable intent.
- Responsibility owner: Decision owns operational choice.
- Interaction: Decision consumes assessments through Contract Bus contracts.
- Failure behavior: Decision MAY reject partial, stale, policy-denied, or
  insufficiently supported assessments.

### 7.6 Action

- Allowed input: none for direct execution.
- Allowed output: no action command or executable intent.
- Prohibited behavior: invoking actions, tools, drivers, workflows, agents, or
  external IO.
- Responsibility owner: Action owns execution after Decision and policy gates.
- Interaction: Reasoning and Action MUST NOT call one another directly.
- Failure behavior: an assessment never becomes execution merely because it
  contains a recommendation.

### 7.7 Learning

- Allowed input: policy-authorized learning suggestions or pattern references
  presented as evidence candidates.
- Allowed output: observations about uncertainty, contradictions, or possible
  learning signals.
- Prohibited behavior: training, adaptation, persistence, or behavior mutation.
- Responsibility owner: Learning owns suggested adaptation.
- Interaction: explicit contracts with policy and provenance metadata.
- Failure behavior: unvalidated learning signals remain hypotheses.

### 7.8 Perception

- Allowed input: authorized structured observations with modality, source,
  provenance, confidence, and sensitivity.
- Allowed output: requests to clarify ambiguous observations.
- Prohibited behavior: raw sensor acquisition or observation normalization.
- Responsibility owner: Perception owns transformation of raw input into
  structured observations.
- Interaction: through Context or explicit observation contracts.
- Failure behavior: uncertain or corrupted observations MUST remain visibly
  uncertain and MAY stop assessment.

### 7.9 Prompt

- Allowed input: no prompt is a canonical domain contract.
- Allowed output: provider adapters MAY derive prompts from approved contracts.
- Prohibited behavior: treating prompt text, chat history, or model session
  state as the Reasoning Domain or durable assessment.
- Responsibility owner: future Prompt or provider adapter boundaries own
  provider-specific representation.
- Interaction: contracts remain provider-independent.
- Failure behavior: prompt construction failure is a provider-boundary failure,
  not a change to Reasoning semantics.

### 7.10 Cache

- Allowed input: traceable assessment candidates with compatible scope,
  version, policy, freshness, sensitivity, and provenance.
- Allowed output: an explicitly identified cache hit or miss.
- Prohibited behavior: becoming hidden Memory, Knowledge, Context, or source of
  truth.
- Responsibility owner: future cache infrastructure owns storage mechanics;
  Reasoning policies govern eligibility.
- Interaction: behind explicit contracts and policy gates.
- Failure behavior: invalid, stale, or unavailable cache falls back safely and
  MUST NOT alter domain correctness.

### 7.11 Tools

- Allowed input: deterministic results supplied through an authorized caller or
  contract, with source and execution metadata.
- Allowed output: an explicit request that another authorized domain consider a
  tool operation.
- Prohibited behavior: direct tool invocation or execution orchestration.
- Responsibility owner: Action or approved service boundaries own execution.
- Interaction: Reasoning evaluates supplied tool results as evidence; it does
  not execute tools.
- Failure behavior: missing tool output becomes a gap, not permission to call a
  tool.

### 7.12 Policies

- Allowed input: policy decisions, constraints, redaction rules, allowed
  strategies, provider limits, and escalation rules.
- Allowed output: traceable policy-evaluation requests and policy findings.
- Prohibited behavior: replacing, bypassing, or silently interpreting policy as
  optional advice.
- Responsibility owner: Policies govern behavior.
- Interaction: through policy contracts and future enforcement boundaries.
- Failure behavior: denial, unavailable policy, or ambiguous authorization
  fails closed when sensitivity or risk requires it.

### 7.13 Audit

- Allowed input: audit requirements and visibility constraints.
- Allowed output: metadata-first trace references, lifecycle transitions,
  policy results, budget state, and stop reasons.
- Prohibited behavior: exposing private chain-of-thought or unnecessary
  sensitive content.
- Responsibility owner: Audit and Telemetry own durable audit and operational
  observation surfaces.
- Interaction: through events and explicit audit contracts.
- Failure behavior: mandatory audit failure MUST stop sensitive assessment or
  produce a policy-defined degraded outcome.

### 7.14 AI Providers

- Allowed input: bounded, minimized, policy-approved inference requests.
- Allowed output: untrusted inference candidates with provider metadata.
- Prohibited behavior: domain ownership, decision, action, Memory persistence,
  Knowledge mutation, or contract definition.
- Responsibility owner: providers execute replaceable inference strategies.
- Interaction: only through future provider adapters and inference contracts.
- Failure behavior: timeout, refusal, malformed output, or provider outage is
  isolated and MAY trigger policy-approved fallback or escalation.

### 7.15 Agents

- Allowed input: an agent MAY request assessment through an authorized contract.
- Allowed output: a non-binding assessment subject to the agent's own policy and
  action boundaries.
- Prohibited behavior: allowing agents to own Reasoning, bypass policies, or
  turn recommendations into automatic execution.
- Responsibility owner: future Agent governance owns autonomous orchestration.
- Interaction: through Contract Bus with actor, owner, scope, and sensitivity.
- Failure behavior: malicious, over-broad, or unauthorized requests are denied
  and traced.

## 8. Planning and Reasoning

Planning owns candidate future structures. Reasoning evaluates candidate
assumptions, evidence, implications, tradeoffs, contradictions, and uncertainty.

Planning MAY operate without Reasoning in a policy-approved degraded mode.
Reasoning is an optional assessor, not a mandatory central dependency. It MUST
NOT become a single gateway through which every plan must pass.

Reasoning never owns or rewrites a plan silently. Findings return through
contracts. A request for plan refinement MUST be explicit, traceable, and
handled by Planning. Recursive direct invocation is prohibited.

Every Planning/Reasoning cycle MUST define:

- an iteration budget;
- a depth budget;
- a token budget where model inference is used;
- a latency budget;
- a cost budget;
- an energy or compute class budget;
- repetition and loop detection;
- result deduplication;
- correlation and causation identifiers;
- request and assessment expiry;
- explicit stop conditions;
- explicit escalation conditions.

The cycle stops on completion, policy denial, invalid scope, cancellation,
expiry, repeated state, exhausted budget, timeout, provider failure, prohibited
exposure, or irreducible uncertainty. Escalation requires policy approval,
remaining budget, and a traceable justification.

## 9. Reasoning and Decision

Reasoning evaluates. Decision chooses.

Reasoning MAY rank evidence strength, risks, uncertainties, interpretations, or
candidate implications. It MAY produce a recommendation, but that recommendation
MUST be non-binding. Decision owns selection, rejection, deferral, escalation,
and the operational choice record.

Policies MAY invalidate, deny, constrain, or remove options. Policies MUST NOT
select the final operational option among remaining policy-valid alternatives.
Decision exclusively owns that choice. If policy leaves zero valid
alternatives, the result is denial, rejection, or escalation, not a Decision
made by Policy or Reasoning.

Reasoning MAY report policy findings, but it MUST NOT convert them into an
operational choice. Recommendation strength remains non-binding and is not
decision authority.

Every future assessment SHOULD carry a conceptual invariant equivalent to:

```text
non_binding: true
```

This RFC does not define or implement that field. The invariant prevents
Reasoning from becoming a hidden Decision Engine.

## 10. Reasoning and Context

Reasoning receives an authorized `ContextView`. It does not assemble, retrieve,
refresh, compress, expand, persist, or mutate Context.

Arbitrary information retrieval is prohibited. Additional information requires
an explicit contract request to the owning domain. Provenance, freshness,
expiry, visibility, redaction, sensitivity, owner, and policy constraints MUST
propagate into the assessment.

Missing, stale, contaminated, expired, or unauthorized Context MAY stop the
lifecycle. Context minimization remains mandatory: only information necessary
for the authorized question should be exposed.

## 11. Reasoning, Memory, and Knowledge

Reasoning has no mandatory direct access to Memory or Knowledge. Authorized
references SHOULD arrive through Context whenever that preserves domain and
policy boundaries.

Reasoning does not persist Memory and does not promote claims into official
Knowledge. An observation, memory, inference, hypothesis, recommendation, and
fact remain distinct artifact classes. Promotion, validation, retention,
invalidation, and mutation belong to the owning domains and Policies.

Reuse of Memory or Knowledge references requires visible provenance, freshness,
confidence or contradiction state where applicable, scope compatibility, and
authorization. Reasoning MAY propose a Memory or Knowledge change through an
explicit contract, but the proposal remains non-binding.

## 12. Conceptual Request Contract

`ReasoningRequest` is a conceptual future contract. It is not a struct, API,
schema, serialization format, or implementation commitment.

| Field | Classification | Purpose |
| --- | --- | --- |
| `reasoning_request_id` | Required | Stable request identity. |
| `contract_version` | Required | Contract compatibility boundary. |
| `actor` | Required | Requesting user, service, or governed system actor. |
| `owner` | Required | Data and operation ownership boundary. |
| `authorization_ref` | Required | Versioned, non-secret reference to the authorization context governing actor, owner, scope, data, providers, strategies, and exposure. |
| `goal` | Optional | Goal that frames the assessment. |
| `question` | Required | Explicit question to evaluate. |
| `reasoning_scope` | Required | Bounded subject and permitted evaluation scope. |
| `context_ref` | Required | Authorized ContextView reference. |
| `plan_candidate_refs` | Optional | Candidate plans supplied for assessment. |
| `evaluated_subject_refs` | Required | Subjects, alternatives, or claims to evaluate. |
| `evidence_refs` | Optional | Authorized evidence references. |
| `assumptions` | Optional | Declared assumptions to test or retain. |
| `constraints` | Optional | Structural, temporal, resource, or domain constraints. |
| `policy_constraints` | Required | Applicable policy decisions and boundaries. |
| `requested_reasoning_modes` | Optional | Non-binding capability hints, not algorithm selection. |
| `maximum_depth` | Required | Finite depth limit. |
| `iteration_budget` | Required | Finite iteration limit. |
| `token_budget` | Optional | Model-token ceiling when applicable. |
| `latency_budget` | Required | Maximum elapsed-time class or duration. |
| `cost_budget` | Optional | Monetary or provider-cost ceiling. |
| `energy_budget` | Optional | Compute or energy class ceiling. |
| `confidence_threshold` | Deferred/Open | Requires a future confidence taxonomy. |
| `freshness_requirements` | Required | Source and Context freshness constraints. |
| `trace_context` | Required | AEP-0016 trace linkage and policy-safe metadata. |
| `correlation_id` | Required | End-to-end operation correlation. |
| `causation_id` | Required | Immediate causal predecessor. |
| `idempotency_key` | Required | Duplicate-request protection. |
| `expiry` | Required | Request validity boundary. |
| `requested_output_format` | Optional | Contract-level output shape preference. |
| `sensitivity_classification` | Required | Exposure, provider, trace, and cache boundary. |

Required fields are conceptual invariants. Optional fields depend on request
class. Deferred fields require later ADR or contract decisions. No final
serialization, language, or schema is selected here.

`authorization_ref` references authorization metadata; it MUST NOT contain
credentials, tokens, secrets, or copied sensitive authorization material. It
MUST be validated before processing and propagated safely into assessment and
trace metadata. Expired, revoked, or scope-incompatible authorization stops the
lifecycle. `actor`, `owner`, `reasoning_scope`, and `authorization_ref` are
related but non-interchangeable boundaries. Their final serialization remains
Deferred/Open.

## 13. Conceptual Assessment Contract

`ReasoningAssessment` is a conceptual future contract. It is not a struct, API,
schema, storage record, or implementation commitment.

| Field | Classification | Purpose |
| --- | --- | --- |
| `assessment_id` | Required | Stable assessment identity. |
| `contract_version` | Required | Assessment contract compatibility. |
| `request_id` | Required | Source request reference. |
| `actor` | Required | Requesting actor reference preserved from the authorized request. |
| `owner` | Required | Ownership boundary under which the assessment was produced. |
| `reasoning_scope` | Required | Evaluated scope; it MUST NOT exceed the authorized request scope. |
| `authorization_ref` | Required | Non-secret reference to the authorization context governing production, exposure, and reuse. |
| `policy_context_ref` | Required | Applicable policy-context reference without copied sensitive policy data. |
| `evaluated_subject_refs` | Required | Subjects actually evaluated. |
| `findings` | Required | Structured assessment findings. |
| `implications` | Required | Consequences derived within scope. |
| `contradictions` | Optional | Conflicts found in evidence or assumptions. |
| `uncertainty` | Required | Explicit unknowns, ambiguity, and support limits. |
| `evidence_strength` | Required | Qualified support assessment, not truth. |
| `confidence` | Required | Defined confidence state with interpretation. |
| `assumptions_used` | Required | Assumptions retained or tested. |
| `risks` | Optional | Identified risk statements and severity. |
| `unresolved_questions` | Optional | Missing information or unresolved issues. |
| `alternative_interpretations` | Optional | Plausible competing interpretations. |
| `recommendation` | Optional | Non-binding advisory output. |
| `recommendation_strength` | Optional | Qualified support, not decision authority. |
| `non_binding` | Required | Invariant that MUST be conceptually true. |
| `policy_findings` | Required | Policy results affecting evaluation or exposure. |
| `provenance` | Required | Origin and transformation references. |
| `freshness` | Required | Freshness state of used inputs and output. |
| `partial_result` | Required | Explicit completion marker. |
| `trace_links` | Required | AEP-0016 event and audit references. |
| `budget_consumed` | Required | Depth, iteration, token, time, cost, and energy use. |
| `stop_reason` | Required | Why evaluation ended. |
| `expiry` | Required | Reuse validity boundary. |
| `safe_explanation` | Required | Policy-safe structured explanation. |
| `sensitivity_classification` | Required | Exposure, retention, and reuse boundary. |

A recommendation is not a Decision. Confidence is not probability unless a
future contract formally defines and calibrates it. Partial assessments MUST
NOT be treated as complete. Stale assessments MUST NOT be silently reused.
`safe_explanation` is not private chain-of-thought.

Every assessment MUST preserve the actor, ownership, scope, authorization,
policy, and sensitivity boundaries under which it was produced. It MUST NOT
broaden visibility, scope, or authority. `authorization_ref` is not Decision or
execution authority, and consumers MUST revalidate authorization whenever
policy requires it. Revoked, expired, or incompatible authorization prevents
exposure or reuse. Assessment ownership does not give Reasoning ownership of an
evaluated plan, evidence, Knowledge, Memory, or Decision. Authorization metadata
MUST remain traceable without copying secrets.

## 14. Reasoning Modes

Future Reasoning capabilities MAY declare these conceptual modes:

- deductive: derives implications from supplied rules and premises;
- inductive: identifies bounded patterns without asserting certainty;
- abductive: proposes plausible explanations for observations;
- causal: evaluates supplied causal hypotheses and dependencies;
- comparative: compares alternatives against shared criteria;
- constraint-based: evaluates consistency with explicit constraints;
- counterfactual: evaluates bounded alternative conditions;
- temporal: evaluates ordering, duration, recency, and time-dependent effects;
- risk-oriented: emphasizes hazards, reversibility, and failure impact;
- consistency checking: identifies contradictions and incompatible claims.

Modes are declared capabilities or non-binding hints. Policies MAY permit,
deny, constrain, or require them. They remain provider-independent and MAY be
implemented by different future strategies. This RFC does not define an
algorithm for any mode.

## 15. Lifecycle

The conceptual Reasoning lifecycle is:

1. Receive `ReasoningRequest` through Contract Bus.
2. Validate contract version.
3. Validate actor, owner, and `authorization_ref`.
4. Validate scope and policy constraints.
5. Validate budgets and expiry.
6. Bind the authorized `ContextView` reference.
7. Validate freshness and provenance.
8. Identify the question, assumptions, evidence, and constraints.
9. Select an allowed strategy or provider path.
10. Evaluate evidence, relationships, candidates, and implications.
11. Detect contradictions, unsupported claims, and information gaps.
12. Update explicit uncertainty and confidence state.
13. Apply stop, fallback, and escalation rules.
14. Produce a `ReasoningAssessment`.
15. Apply exposure, minimization, and redaction policies.
16. Return the assessment through Contract Bus.
17. Emit policy-safe trace events.
18. Expire or discard transient evaluation state.

Stop conditions include:

- policy denial;
- invalid, expired, revoked, or incompatible authorization;
- expired request;
- stale, contaminated, or unauthorized Context;
- insufficient provenance;
- repetition or duplicate state;
- loop detection;
- depth, iteration, token, latency, cost, or energy budget exhaustion;
- timeout;
- provider failure;
- prohibited exposure;
- irreducible uncertainty;
- cancellation.

Every stop MUST produce an explicit stop reason. A partial assessment MUST be
marked as partial. No lifecycle state may remain hidden in a provider session,
prompt transcript, manager, cache, or global mutable object.

## 16. AEP-0016 Traceability

AEP-0016 governs every future Reasoning request, assessment, transition, and
cross-domain interaction.

Every event uses a metadata-first envelope containing event identity, contract
version, timestamp, actor, owner, authorization reference, subject references,
policy references, sensitivity, correlation ID, causation ID, current budget
state, lifecycle state, and applicable stop or escalation reason. Payloads MUST
avoid raw sensitive content and MUST NOT contain private chain-of-thought.

| Event | Purpose | Additional minimum metadata |
| --- | --- | --- |
| `ReasoningRequestReceived` | Record accepted ingress. | Request and scope references. |
| `ReasoningScopeValidated` | Record scope and authorization result. | Validation and denial reasons. |
| `ReasoningContextBound` | Record authorized Context binding. | Context reference, freshness, expiry. |
| `ReasoningStrategySelected` | Explain strategy class selection. | Mode, provider class, policy justification. |
| `EvidenceEvaluated` | Record evidence use without leaking content. | Evidence refs, trust labels, freshness. |
| `AssumptionApplied` | Record a material assumption. | Assumption ref and qualification. |
| `ContradictionDetected` | Record a conflict. | Subject refs and contradiction class. |
| `UncertaintyUpdated` | Record meaningful uncertainty transition. | Prior and new uncertainty states. |
| `ReasoningBudgetConsumed` | Observe bounded resource use. | Consumed and remaining budget dimensions. |
| `ReasoningStopped` | Record lifecycle termination. | Stop reason and partial-result state. |
| `ReasoningEscalationRequested` | Request stronger strategy or review. | Justification, target class, remaining budget. |
| `ReasoningAssessmentProduced` | Record assessment completion. | Assessment ref, confidence, expiry. |
| `ReasoningAssessmentRejected` | Record policy or consumer rejection. | Rejection class and policy refs. |
| `ReasoningAssessmentExposed` | Record authorized exposure. | Recipient class and redaction state. |
| `ReasoningAssessmentConsumed` | Record downstream use. | Consumer domain and purpose reference. |
| `ReasoningAssessmentExpired` | Record invalidation for reuse. | Expiry reason and cache invalidation refs. |

Traceability explains inputs, ownership, constraints, strategies, lifecycle,
budget use, uncertainty, and output handling. It does not require disclosure of
private internal reasoning tokens.

## 17. RFC-0009 Efficiency Requirements

Reasoning MUST apply the Efficient Intelligence direction from RFC-0009:

- use the minimum sufficient reasoning depth;
- enforce finite budgets;
- reduce irrelevant inputs progressively and transparently;
- preserve critical evidence, policy, provenance, and uncertainty during
  compaction;
- support safe interruption and explicit partial results;
- use policy-approved fallback;
- justify escalation to stronger or more expensive providers;
- prefer local-first processing when safe, sufficient, and policy-compatible;
- reuse cached assessments only with valid provenance and freshness;
- expose conceptual metrics for tokens, latency, cost, and energy;
- keep security and policy above efficiency;
- prohibit unlimited reasoning;
- prohibit silent quality reduction;
- prohibit escalation without traceable justification.

Tool-first execution does not authorize Reasoning to call tools. It means a
caller SHOULD use a deterministic capability when that capability can answer
the question more safely and efficiently, then provide the result to Reasoning
through an authorized contract if evaluation is still needed.

## 18. Security and Policy

Reasoning is exposed to adversarial and accidental cognitive risks:

- prompt injection and instruction smuggling;
- Context and evidence poisoning;
- fabricated or misattributed evidence;
- hallucination and unsupported synthesis;
- overconfidence and false precision;
- policy bypass or policy-result suppression;
- reasoning laundering, where an assessment disguises an unauthorized choice;
- inference promoted to fact;
- sensitive data or explanation leakage;
- unauthorized data use;
- malicious agent requests;
- provider manipulation or provider-owned behavior;
- recursive loops and hidden retries;
- denial-of-wallet and excessive resource consumption;
- stale assessment reuse;
- cross-owner or cross-tenant leakage.

Future controls MUST include authorization, minimization, evidence trust labels,
provenance, freshness, expiry, policy gates, redaction, provider isolation,
budget enforcement, loop detection, traceability, safe failure, and explicit
partial-result markers.

Sensitive requests SHOULD fail closed when authorization, policy, provenance,
or audit requirements cannot be established. Efficiency MUST NOT weaken these
controls.

## 19. ASB and Contract Bus

All future Reasoning communication MUST use versioned contracts through ASB or
Contract Bus. Direct service references and service-to-service calls are
prohibited.

Future contracts MUST address:

- idempotency;
- correlation and causation;
- bounded timeout and retry;
- circuit breaking;
- policy-approved fallback;
- observability and AEP-0016 traceability;
- policy enforcement;
- failure isolation;
- additive contract evolution and compatibility;
- payload validation;
- sensitivity and redaction propagation.

Reasoning MUST NOT become a mandatory passage for every cognitive operation.
Callers MAY use simpler deterministic validation, Planning-only degraded mode,
or policy-defined alternatives when advanced assessment is unnecessary.

## 20. Provider Boundary

Providers perform inference; they do not own the domain.

- Provider outputs are untrusted until validated.
- Provider selection is policy-controlled and budget-constrained.
- Local and external providers MAY coexist.
- Provider-specific prompts, response formats, sessions, and hidden state MUST
  NOT become domain contracts.
- Provider failure does not redefine the Reasoning lifecycle.
- Provider escalation requires explicit justification, authorization, and
  remaining budget.
- No provider may decide, execute, persist Memory, mutate Knowledge, assemble
  Context, or own Planning artifacts.
- Replacing a provider MUST NOT require redefining Reasoning ownership.

## 21. Cache and Reuse

Cache is optional. A cached assessment is never a source of truth.

Reuse requires compatible request semantics, `authorization_ref`, actor when
applicable, owner, scope, policy context, sensitivity classification,
tenant/security boundary, contract version, provenance, evidence version,
Context freshness, expiry, evaluated subject, provider constraints, and
requested output constraints.

Cache hits MUST NOT bypass current authorization checks. Matching content or a
matching cache key is insufficient without compatible authorization. An
assessment produced for one tenant, owner, scope, authorization, or sensitivity
boundary MUST NOT be reused across another boundary without an explicit,
policy-authorized transformation. Revoked or expired authorization invalidates
reuse. Partial, stale, revoked, or policy-incompatible assessments MUST NOT be
silently reused.

Invalidation and expiry are mandatory. Every reuse MUST emit or reference the
traceability required by AEP-0016 and expose that reuse occurred. Sensitive
assessments MAY be declared non-cacheable. Cache failure MUST preserve
correctness and MUST NOT become hidden Memory or global cognitive state.

## 22. Failure Isolation and Degraded Mode

- Reasoning failure MUST NOT crash Planning.
- Planning MAY continue without advanced assessment when risk and policy allow.
- Decision MAY reject partial, stale, expired, or insufficient assessments.
- Timeout produces an explicit failure or partial state.
- Providers remain replaceable.
- Contract Bus boundaries isolate service and provider failures.
- Main remains maestro, not executor.
- No Reasoning God Engine is allowed.
- No single global mutable cognitive state is allowed.
- Hidden retry loops and indefinite blocking are prohibited.
- Fallback MUST preserve policy, sensitivity, provenance, and traceability.
- Recovery MUST use bounded retries or a new explicitly correlated request.

## 23. Manager and Service Shape

This section is conceptual only. It does not authorize creation of any
component.

The Reasoning Domain would own assessment semantics, lifecycle invariants,
uncertainty vocabulary, and boundaries.

A future Reasoning Manager might coordinate request lifecycle, policy
orchestration, provider selection boundaries, budgets, health, expiry, and
traceability. It MUST NOT execute user-facing reasoning capabilities or store
hidden assessment state.

A future Reasoning Service might execute declared assessment capabilities and
handle versioned contracts. It MUST NOT decide, execute actions, call services
directly, or become a God Service.

Future provider adapters might translate approved contracts into
provider-specific requests and validate provider responses. Policy gates would
govern data exposure, strategy eligibility, escalation, explanation, reuse, and
redaction. Contract handlers would validate version, scope, idempotency,
sensitivity, and lifecycle transitions.

Main and Kernel only orchestrate lifecycle. Future files and modules MUST be
small, cohesive, independently testable, and free from circular dependencies.
AI-Maintainable Architecture is mandatory.

## 24. Non-Goals

RFC-0005 does not:

- implement Reasoning;
- create a Reasoning Engine;
- create a Reasoning Service;
- create a Reasoning Manager;
- create runtime behavior;
- create APIs or concrete contracts;
- create storage or migrations;
- create agents;
- execute tools;
- authorize actions;
- make final operational decisions;
- persist Memory;
- mutate Knowledge;
- assemble Context;
- train models;
- define a universal intelligence algorithm;
- require or expose private chain-of-thought;
- claim AGI, consciousness, infallibility, or human-perfect reasoning;
- bypass Policies, permissions, audit, or traceability;
- require a specific provider;
- replace Planning, Decision, or Action;
- change Kernel, Manager, Service, Driver, Domain, Policy, ASB, or Contract Bus
  runtime behavior;
- add dependencies or functional code.

## 25. Risks and Mitigations

| Risk | Severity | Impact | Mitigation | Owner | Future validation |
| --- | --- | --- | --- | --- | --- |
| God Reasoning Engine | Critical | Central coupling and opaque failure | Split ownership, contracts, budgets, and provider adapters | Architecture Guardian | Boundary and size reviews |
| Decision laundering | Critical | Recommendation becomes hidden choice | Required non-binding invariant and Decision ownership | Decision/Policy | Negative authority tests |
| Planning/Reasoning loops | High | Unbounded cost and no termination | Iteration, depth, repetition, expiry, and stop limits | Planning/Reasoning | Loop-prevention tests |
| Direct service coupling | Critical | Violates Engineering Rule #002 | ASB/Contract Bus only | Service architecture | No-direct-call tests |
| Unauthorized Context | Critical | Sensitive or out-of-scope evaluation | Authorization, minimization, policy gates | Context/Policy | Authorization tests |
| Evidence without provenance | High | Unsupported assessment | Required provenance and trust labels | Knowledge/Reasoning | Provenance tests |
| Inference promoted to fact | Critical | Polluted structured truth | Explicit artifact classes and Knowledge promotion contracts | Knowledge/Policy | Boundary tests |
| Overconfidence | High | Unsafe downstream reliance | Qualified confidence and visible uncertainty | Reasoning | Calibration tests |
| Unsafe explanation | High | Sensitive or private-state leakage | Safe structured explanation and redaction | Policy/Audit | Leakage tests |
| Provider ownership leakage | High | Model-specific architecture | Provider-independent contracts and replaceability | Inference architecture | Provider swap tests |
| Unlimited budgets | Critical | Denial-of-wallet and indefinite work | Mandatory finite limits | Resource/Policy | Budget exhaustion tests |
| Hidden cache | High | Untraceable cognitive state | Explicit cache events, expiry, and invalidation | Cache/Policy | Reuse trace tests |
| Stale reuse | High | Invalid conclusions | Freshness and compatibility checks | Reasoning/Context | Expiry tests |
| Partial result treated complete | High | Decision based on missing analysis | Required partial marker and stop reason | Reasoning/Decision | Partial-result tests |
| Generic contracts | Medium | Weak validation and ownership ambiguity | Versioned domain contracts and explicit fields | Contract owners | Contract tests |
| Low testability | High | Hidden regressions | Isolated interfaces and deterministic fixtures | Future implementation | Test architecture review |
| Giant files | Medium | Human and AI maintenance degradation | Cohesive modules and file-size review | Architecture Guardian | Maintainability review |
| AI maintainability degradation | High | Agents misplace responsibilities | Explicit invariants, ownership, and docs-first review | Architecture Guardian | Agent-assisted review tests |
| Autonomous-agent Reasoning | Critical | Assessment silently executes | Agent boundary, non-binding output, Action authorization | Agent/Policy | End-to-end negative tests |

## 26. Future Testing Strategy

Future implementation requires:

- contract and schema-evolution tests;
- policy and boundary tests;
- authorization tests;
- provenance tests;
- freshness and expiry tests;
- uncertainty calibration tests;
- invariance tests where a formal interpretation exists;
- timeout, bounded-retry, and circuit-breaker tests;
- loop-prevention and deduplication tests;
- budget-exhaustion tests;
- provider-failure and provider-replacement tests;
- prompt-injection and evidence-poisoning resistance tests;
- redaction and sensitive-data leakage tests;
- traceability and event-correlation tests;
- failure-isolation and degraded-mode tests;
- stale-cache and incompatible-reuse tests.

Required negative tests include:

- Reasoning does not decide.
- Reasoning does not execute.
- Reasoning does not call tools.
- Reasoning does not persist Memory.
- Reasoning does not mutate Knowledge.
- Reasoning does not assemble Context.
- Reasoning does not bypass Policies.
- Reasoning does not expose private chain-of-thought.
- Reasoning does not require a specific provider.
- Reasoning failure does not crash Planning.

No tests are implemented by this RFC.

## 27. Open Questions

- Will Reasoning remain a logical domain, become a future service, or both?
- Should contracts be general or specialized by reasoning mode and risk class?
- How should confidence be represented without false precision?
- What formal distinction will separate inference, hypothesis, recommendation,
  observation, and fact?
- Which reasoning modes are normative capabilities versus advisory hints?
- What cache and reuse policy is acceptable for each sensitivity class?
- How will assessments and evidence dependencies be versioned?
- How should partial and streaming assessments be represented?
- When may local processing escalate to an external provider?
- Which Planning classes may omit Reasoning entirely?
- What explanation content is allowed for each audience and policy boundary?
- Which layer owns provider adapters?
- What uncertainty taxonomy is sufficiently precise and maintainable?
- How much authority may Policy have over strategy selection?
- May assessments be streamed, and how would partial ordering be represented?
- What retention applies to trace metadata and safe explanations?
- How should conflicting assessments be represented and resolved downstream?

These questions require future architectural review, contracts, or ADRs. This
RFC does not close them prematurely.

## 28. Acceptance Criteria

RFC-0005 is ready for architectural review only when:

- it remains documentation-only;
- no code or implementation is added;
- Reasoning ownership is explicit;
- Planning, Reasoning, Decision, and Action remain separated;
- every recommendation is non-binding;
- Policies do not choose among policy-valid alternatives;
- explicit authorization references govern requests, assessments, exposure,
  and reuse without carrying secrets;
- direct service calls are prohibited;
- Contract Bus mediation is required;
- authorized Context is required;
- provenance, sensitivity, and freshness propagate;
- inference, hypothesis, recommendation, and fact remain distinct;
- budgets and stop conditions are explicit;
- failure isolation and degraded mode are defined;
- AEP-0016 events are defined;
- RFC-0009 requirements are incorporated;
- security and policy override efficiency;
- no provider owns the domain;
- no private chain-of-thought is required or exposed;
- human and AI maintainability are preserved;
- RFC-0001 through RFC-0004 boundaries are not violated;
- open questions remain explicit;
- no implementation claim exists.

Acceptance of this document would approve architecture governance only. It
would not authorize runtime implementation.

## 29. Compatibility and Migration

This RFC is compatible with:

- RFC-0001 Memory Domain: Memory supplies governed experiential candidates and
  retains persistence ownership;
- RFC-0002 Knowledge Domain: Knowledge supplies structured claims and retains
  truth lifecycle ownership;
- RFC-0003 Context Domain: Context supplies the authorized present-state view;
- RFC-0004 Planning Domain: Planning owns candidate futures and receives only
  explicit assessment findings;
- RFC-0009 Efficient Intelligence: Reasoning remains bounded, measurable, and
  policy-safe;
- AEP-0016 Cognitive Traceability: lifecycle, provenance, policy, budget, and
  output use remain traceable;
- Architecture Constitution v2: Kernel, Manager, Service, Domain, Policy, ASB,
  and Contract Bus boundaries remain intact.

### 29.1 Known Governance Follow-Up: Service Map

The current Aether Official Service Map is a Phase 4 baseline that predates
RFC-0005. Its Reasoning Service description includes `response synthesis`,
which may broaden Reasoning ownership beyond structured assessment, and lists
`memory.retrieve` as a required capability even though this RFC prefers
authorized Memory references through Context or explicit contracts.

The Service Map MUST be reconciled in a separate documentation task before
RFC-0005 can become Accepted or Published. The current Service Map MUST NOT be
read as implemented runtime, and this RFC neither changes that document nor
authorizes implementation.

There is no runtime migration because Reasoning is not implemented. Future
implementation requires separately approved ADRs, contracts, policies,
provider boundaries, storage/non-storage decisions, security review, and test
strategy. No existing API, database, storage, or service migration is
authorized.

## 30. Decision Summary

- Reasoning evaluates implications and uncertainty.
- Assessments are structured and non-binding.
- Decision owns choice.
- Action owns execution.
- Policies constrain every stage.
- Authorized Context and provenance are mandatory.
- Confidence is not certainty, and evidence is not automatically truth.
- Reasoning is bounded, traceable, replaceable, and failure-isolated.
- Providers perform inference; they do not own cognition.
- Private chain-of-thought is not a required domain artifact.
- This RFC does not implement Reasoning.
