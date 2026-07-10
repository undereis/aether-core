# RFCs

RFCs define major architectural direction before implementation. They are
review artifacts, not runtime behavior.

## RFC Lifecycle

The minimum document lifecycle is:

```text
Draft -> Accepted -> Superseded or Rejected
```

Document status, Git publication, and runtime implementation are independent:

- `Accepted` means the architecture document is approved as governance.
- `Published` means its approved checkpoint tag exists on the remote.
- `Implemented` means separately authorized runtime behavior exists.

An Accepted and Published RFC remains documentation-only until an implementation
ADR, contracts, policies, storage strategy where applicable, test strategy, and
Architecture Guardian approval explicitly authorize implementation.

## RFC Status

| RFC | Domain or scope | Document status | Publication | Runtime |
| --- | --- | --- | --- | --- |
| [RFC-0001](RFC-0001-Memory-Domain.md) | Memory Domain | Accepted | `v0.5.1-memory-domain-rfc` | Not implemented |
| [RFC-0002](RFC-0002-Knowledge-Domain.md) | Knowledge Domain | Accepted | `v0.5.4-knowledge-domain-rfc` | Not implemented |
| [RFC-0003](RFC-0003-Context-Domain.md) | Context Domain | Accepted | `v0.5.7-context-domain-rfc` | Not implemented |
| [RFC-0004](RFC-0004-Planning-Domain.md) | Planning Domain | Accepted | `v0.5.8-planning-domain-rfc` | Not implemented |
| RFC-0005 | Reasoning Domain | Reserved, next | Not published | Not implemented |
| RFC-0006 | Decision Domain | Reserved | Not published | Not implemented |
| RFC-0007 | Learning Domain | Reserved | Not published | Not implemented |
| RFC-0008 | Perception Domain | Reserved | Not published | Not implemented |
| [RFC-0009](RFC-0009-Efficient-Intelligence-Energy-Aware-Architecture.md) | Efficient Intelligence, transversal | Accepted | `v0.5.6-efficient-intelligence-rfc` | Not implemented |

RFC-0004 through RFC-0008 retain their assigned positions in the cognitive
domain sequence approved in CDR-0001. RFC-0004 is accepted and published as
documentation-only architecture in `v0.5.8-planning-domain-rfc`; this does not
mean Planning is implemented. RFC-0005 is the next reserved domain, and
RFC-0006 through RFC-0008 remain reserved. RFC-0009 is a transversal
architecture RFC and does not replace or reorder those domains.
