# ARR-0001: Architecture Readiness Review

## 1. Status

Draft.

## 2. Review Context

ARR-0001 reviews whether the current Aether architecture is ready to proceed
to RFC-0002 Knowledge Domain.

This review occurs after:

- Foundation Era completion;
- AEP-0001 through AEP-0015;
- Architecture Constitution v2;
- CDR-0001 Cognitive Design Review;
- RFC-0001 Memory Domain;
- ADR-0010 Memory Domain Implementation Architecture.

Memory has been architecturally defined, but it has not been implemented. No
Memory Engine, Memory Service, Memory Manager, database schema, vector index,
Knowledge implementation, AI behavior, or agent behavior exists yet.

ARR-0001 is documentary review only. It does not alter CDR-0001, RFC-0001, or
ADR-0010.

## 3. Current Architecture Snapshot

The current architecture is coherent and layered:

- Kernel remains coordination-only.
- Managers govern operational domains.
- Services execute declared capabilities.
- Drivers touch the external world.
- Policies govern sensitive behavior.
- ASB and Contract Bus mediate service communication.
- Memory stores experience.
- Knowledge stores structured truth.
- Context stores the present.
- Models only provide inference.
- The Cognitive Core owns intelligence.

The strongest current architectural guarantees are the Kernel boundary, ASB
communication boundary, service manifest model, domain-first governance, and
the explicit separation between Memory, Knowledge, and Context.

## 4. Readiness Assessment

| Area | Status | Assessment |
| --- | --- | --- |
| Kernel boundary | Ready | Kernel is documented and implemented as orchestration and compatibility facade only. |
| Manager boundary | Ready | Managers coordinate registries, lifecycle, health, and platform governance. Future cognitive managers must follow this pattern. |
| Service boundary | Ready | Services execute capabilities through manifests, permissions, resources, lifecycle, health, ASB, and Contract Bus. |
| Policy boundary | Mostly Ready | Policy contracts exist, but future cognitive enforcement strategy is not implemented yet. RFC-0002 must not bypass this gap. |
| Driver boundary | Ready | Drivers are correctly reserved for external IO and are not used for cognitive behavior yet. |
| Domain boundary | Ready | Domains define responsibility boundaries and support the Cognitive Era roadmap. |
| Memory boundary | Ready | RFC-0001 and ADR-0010 sufficiently constrain Memory as experience, not database, Knowledge, Context, or LLM-owned state. |
| Knowledge boundary | Mostly Ready | CDR-0001 defines Knowledge conceptually, but RFC-0002 must formalize truth, evidence, graph, confidence, contradiction, and provenance. |
| Context boundary | Mostly Ready | CDR-0001 defines Context as present state, but RFC-0003 must later prevent Context from becoming persistence, prompt, chat history, or generic state store. |
| ASB/Contract Bus boundary | Ready | Service-to-service direct calls are prohibited and typed Contract Bus exists as the future communication layer. |
| Traceability | Needs Clarification | AEP-0016 is still Proposed. It is strong enough as guidance, but its status must be controlled before implementation. |
| Security/privacy | Mostly Ready | Memory risks are identified; enforcement awaits policies, contracts, and implementation tests. |
| Testing strategy | Mostly Ready | ADR-0010 defines the needed Memory tests. Knowledge must get an equivalent strategy in RFC-0002. |
| Storage strategy | Mostly Ready | Memory storage is correctly behind abstractions. Knowledge must avoid premature graph or database commitment. |
| LLM independence | Ready | CDR-0001 and ADR-0010 keep models as replaceable inference providers. |
| Implementation sequencing | Ready | RFC-0002 before RFC-0003 still makes sense; implementation remains blocked until domain RFCs, ADRs, contracts, policies, and tests are approved. |

## 5. Memory Readiness

RFC-0001 and ADR-0010 are sufficient to prevent the main Memory failure modes
if future implementation follows them.

Memory is protected from becoming a generic database because storage is defined
as an implementation detail behind contracts, store ports, policies, and
resource boundaries.

Memory is protected from becoming Knowledge because it stores experience and
only emits candidates, evidence, or provenance-aware signals to future
Knowledge contracts.

Memory is protected from becoming Context because Context owns the present
while Memory owns historical experience and retrieval candidates.

Memory is protected from becoming LLM-owned memory because all model assistance
must flow through a future Inference Provider and cannot own durable records.

Memory is protected from living in the Kernel because ADR-0010 places
governance in a future Memory Manager and execution in a future Memory Service.

Memory is protected from direct service calls because all future communication
must pass through ASB or Contract Bus.

Memory is protected from hidden state because AEP-0016, RFC-0001, and ADR-0010
require provenance, traceability, explainability, lifecycle, confidence,
retention, forgetting, and audit-friendly metadata.

Memory is not yet protected by runtime enforcement. That is acceptable because
Memory has not been implemented, but implementation must not begin until
contracts, policies, and tests exist.

## 6. Knowledge Readiness

There is enough architectural foundation to start RFC-0002 Knowledge Domain.

Knowledge should be structured truth or structured truth candidates. It should
own entities, relationships, facts, rules, evidence, sources, provenance,
confidence, contradiction handling, and knowledge lifecycle.

Knowledge should not be memory, chat history, vector search, an LLM context
window, raw documents, unvalidated graph edges, or a generic database.

The boundary with Memory must preserve that Memory stores experience and
Knowledge stores structured claims. A memory can support a Knowledge candidate,
but it does not become Knowledge automatically.

The boundary with Context must preserve that Context assembles the present.
Knowledge may inform Context, but it does not own current task/session state.

RFC-0002 must address:

- whether Knowledge can contain uncertain or conflicting claims;
- how confidence and provenance work for Knowledge;
- how evidence is attached and invalidated;
- whether graph storage is required or deferred;
- how Knowledge avoids becoming LLM-owned state;
- how Knowledge receives memory-derived candidates only through explicit
  contracts;
- how Knowledge invalidation affects Context, Reasoning, Decision, and Memory
  evidence links.

## 7. Context Readiness

RFC-0003 Context Domain should come after RFC-0002 Knowledge Domain because
Context needs clear inputs from Memory and Knowledge before its assembly rules
can be safely defined.

Current Context risks:

- Context may be persisted improperly if it is treated as long-term state.
- Context may be confused with Memory if current-state snapshots are stored as
  durable experience without policy.
- Context may be confused with prompt construction if prompt text is treated as
  the full cognitive state.
- Context may be confused with chat history if conversation is treated as the
  only active context.
- Context may be confused with Knowledge if current task state is elevated to
  structured truth.

RFC-0003 should explicitly define Context as scoped, current, compositional,
policy-filtered, and explainable.

## 8. Cognitive Pipeline Readiness

The current CDR-0001 pipeline remains conceptually valid:

```text
User Input
  -> Perception
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

The sequence still makes sense as a conceptual flow: perception structures raw
input, working memory holds temporary cognitive state, Memory retrieves
experience, Knowledge contributes structured truth, Context assembles the
present, Planning creates futures, Reasoning evaluates, Decision chooses,
Action executes, Learning suggests, and Memory Update persists only approved
changes.

No change to CDR-0001 is recommended now. RFC-0002 may add notes about
Knowledge participating both before Context assembly and later during
Reasoning, but that should be recorded as interpretation, not a pipeline
change.

## 9. AEP-0016 Readiness

AEP-0016 should be accepted before Memory implementation begins.

AEP-0016 should also be accepted before Knowledge implementation begins, or
RFC-0002 must explicitly bind itself to AEP-0016 as Proposed and define a
controlled traceability requirement.

AEP-0016 may remain Proposed during RFC-0002 drafting if the RFC treats
traceability as mandatory design input rather than optional future work.

Risks of keeping traceability as Proposed too long:

- cognitive records may be designed without durable provenance;
- Knowledge claims may lack explainability;
- policy decisions may leave no audit surface;
- inference outputs may become hidden state;
- future implementation may under-test traceability;
- later adoption may require expensive contract revisions.

Recommendation: keep AEP-0016 Proposed during RFC-0002 only if the Architecture
Guardian explicitly requires RFC-0002 to satisfy its intent. Accept AEP-0016
before any Memory or Knowledge implementation.

## 10. Risk Review

Current risks:

- Architecture can become too heavy if every cognitive concept requires too
  many crates before the first small implementation.
- Architecture can become too loose if Knowledge is drafted without contracts,
  policies, and traceability.
- Excess documentation before implementation can delay feedback.
- Implementation could start without contracts or policies if phase gates are
  weakened.
- Knowledge could be designed as a database instead of structured truth.
- Knowledge could become LLM memory if model outputs are trusted without
  provenance and evidence.
- Context could become persistent history instead of present state.
- Memory and Knowledge could mix responsibilities through relationship graphs.
- A graph layer could become a premature dependency.
- Vector search could be treated as source of truth.
- Traceability could be delayed too long.
- Policy layer could remain formal if no enforcement path is designed.
- Confidence scores could become false certainty.
- Evidence links could leak sensitive memory if policy visibility is weak.

None of these risks currently block RFC-0002, but they must shape RFC-0002.

## 11. Required Clarifications Before RFC-0002

RFC-0002 must answer:

- What is the difference between Memory and Knowledge?
- When does a memory become a Knowledge candidate?
- Who promotes Knowledge?
- Does Knowledge store facts, relationships, rules, ontologies, evidence, or
  all of them?
- Can Knowledge contain uncertainty?
- Does Knowledge have confidence?
- Does Knowledge have provenance?
- Can Knowledge contradict other Knowledge?
- Does Knowledge depend on a graph layer?
- Does Knowledge depend on LLMs?
- How is Knowledge updated?
- How is Knowledge invalidated?
- How is Knowledge audited?
- How does Knowledge relate to Context?
- How does Knowledge expose evidence without leaking sensitive Memory?
- How are Knowledge claims versioned?
- How are deprecated claims handled?
- How are rules separated from Policies?
- How are ontology decisions governed?
- Which contracts are required before implementation?
- Which tests prove Knowledge does not become Memory, Context, or LLM-owned
  state?

## 12. Decision

B. Ready with minor cautions.

The architecture is ready to proceed to RFC-0002 Knowledge Domain as a
document-only phase. The current foundation, CDR-0001, RFC-0001, and ADR-0010
provide enough boundary clarity to start Knowledge design.

Minor cautions:

- AEP-0016 should be treated as mandatory design input even while Proposed.
- RFC-0002 must not choose graph storage prematurely.
- RFC-0002 must define Knowledge confidence, provenance, evidence, conflict,
  invalidation, and audit boundaries before implementation.
- No Memory or Knowledge implementation should begin from ARR-0001 alone.

## 13. Recommendations

- Proceed to RFC-0002 Knowledge Domain as the next document-only phase.
- Do not begin Memory implementation, Knowledge implementation, AI, agents,
  database schemas, migrations, crates, or APIs.
- Keep the order RFC-0002 Knowledge Domain, then RFC-0003 Context Domain.
- Accept AEP-0016 before implementation of Memory or Knowledge.
- During RFC-0002 drafting, treat AEP-0016 as a mandatory proposed protocol.
- Keep graph storage as a possible future implementation detail, not the
  definition of Knowledge.
- Require RFC-0002 to define no-direct-service-call, traceability,
  provenance, evidence, policy, and sensitive-data leakage test categories.
- Keep LLMs outside Knowledge ownership; models may assist inference but must
  not own claims.

## 14. Acceptance Criteria For Moving To RFC-0002

RFC-0002 may begin when:

- ARR-0001 is reviewed;
- no critical ambiguity remains between Memory, Knowledge, and Context;
- AEP-0016 status is decided or explicitly controlled as Proposed but
  mandatory design input;
- Architecture Guardian approves advancement;
- repository is clean;
- no implementation has started;
- the next prompt is limited to RFC-0002 documentation;
- CDR-0001, RFC-0001, and ADR-0010 remain unchanged unless a critical factual
  error is reported first.

## 15. Non-Goals

ARR-0001 does not:

- implement Memory;
- implement Knowledge;
- create crates;
- create real contracts;
- create structs;
- create traits;
- create APIs;
- alter runtime behavior;
- alter the Kernel;
- alter Services;
- create databases;
- create migrations;
- create AI;
- create agents;
- alter CDR-0001;
- alter RFC-0001;
- alter ADR-0010.
