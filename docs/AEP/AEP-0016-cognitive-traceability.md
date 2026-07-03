# AEP-0016 Proposal: Cognitive Traceability

Status: Proposed.

## Objective

Require every cognitive component, record, decision, event, and lifecycle
transition to be traceable, explainable, and auditable across its full
lifecycle.

## Motivation

Aether is a cognitive operating system. Cognitive behavior can affect memory,
knowledge, context, reasoning, decisions, actions, learning, and user trust.
Without traceability, the system may produce useful-looking output that cannot
be explained, audited, corrected, or safely evolved.

## Context

CDR-0001 established that the Cognitive Core owns intelligence and models only
provide inference. RFC-0001 extends that foundation by defining Memory as a
living cognitive domain with provenance, confidence, retention, forgetting,
relationships, events, explainability, and evolution.

Future cognitive components must not become hidden state or opaque model-driven
behavior. They must expose their origins, inputs, outputs, policies, contracts,
events, confidence, and lifecycle transitions.

## Rules

- Every cognitive record must expose identity, source, provenance, owner,
  policy boundary, lifecycle state, and confidence where applicable.
- Every cognitive lifecycle transition must be attributable to a request,
  event, policy, manager, service, user, or system trigger.
- Every cognitive decision or promotion must be explainable after the fact.
- Every cognitive component must emit or expose audit-friendly trace metadata.
- Inference output must be traceable to the request, provider, model boundary,
  input context, policy constraints, and consuming cognitive domain.
- Traceability metadata must avoid leaking sensitive content by default.
- Hidden cognitive state is an architectural violation.

## Mandatory Flow

1. Identify the cognitive artifact or component.
2. Define its identity and lifecycle.
3. Define provenance and ownership metadata.
4. Define policy and permission boundaries.
5. Define explainability output.
6. Define audit or trace events.
7. Validate that sensitive content is not exposed unnecessarily.

## Correct Examples

- A memory record can explain its source, confidence, retention policy,
  relationships, version lineage, and reason for retrieval.
- A reasoning result can explain which memory candidates, knowledge claims,
  context signals, policies, constraints, and inference provider were used.
- A learning suggestion can explain the observations and patterns that produced
  it without mutating the system directly.

## Incorrect Examples

- A service stores cognitive state that cannot be inspected or explained.
- A memory is retrieved with no reason, score dimensions, source, or policy
  visibility.
- An LLM output is persisted as memory without provenance, confidence,
  evidence, or retention.
- A decision is recorded without alternatives, policy result, or confidence.

## Violation Detection

- Cognitive records have no source or provenance.
- Lifecycle transitions happen without an attributable trigger.
- Retrieval results cannot explain why they were selected.
- Policies affect behavior but leave no trace.
- Model output is stored without a cognitive domain owner.
- Audit trails contain raw sensitive content when metadata would be sufficient.

## Violation Correction

Stop the implementation, add traceable identity and provenance metadata, define
explainability and audit surfaces, bind policy context, and update tests before
the cognitive behavior is allowed to proceed.

## Relationship With Other AEPs

AEP-0016 extends AEP-0006, AEP-0011, AEP-0012, and AEP-0013 into the Cognitive
Era. It strengthens CDR-0001 and RFC-0001 by making cognitive lifecycle
visibility mandatory.
