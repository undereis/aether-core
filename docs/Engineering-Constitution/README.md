# Engineering Constitution

Aether is not optimized for short-term demos. Engineering decisions must
preserve long-term changeability, safety, and operational clarity.

Phase 4.6 establishes the Aether Engineering Protocols as active governance for
all future work. The Architecture Constitution v2 is the authoritative
architecture baseline after the Foundation Era.

## Principles

- Architecture before features.
- Clear boundaries before scale.
- Observability before production complexity.
- Typed contracts before implicit coupling.
- Tests before broad refactors.
- Boring, mature technology before novelty.
- AEP compliance before implementation.

## Non-Negotiables

- No business logic in infrastructure code.
- No feature work without an owning module.
- No global mutable runtime state.
- No hidden service dependencies.
- No unmanaged secrets in source control.
- No broad dependency additions without justification.
- No implementation that violates an Aether Engineering Protocol.
- No architecture change without reading the owning contracts first.

## Engineering Rules

### Rule #001: Kernel-Controlled Platform Resources

Internal platform resources must be registered, inspected, and supervised
through Kernel-owned contracts.

### Rule #002: Service Communication Through ASB

No service may know directly about another service.

All service communication must pass through the Aether Service Bus. Direct
service references, direct service method calls, and direct service-owned
channels are architectural violations.

### Rule #003: AEP Compliance

All future implementation must comply with the Aether Engineering Protocols in
`docs/AEP`.

If a request violates an AEP, implementation must stop until the violation is
resolved or the protocol is changed through architecture review.

### Rule #004: Architecture Guardian Authority

The Architecture Guardian is responsible for blocking changes that violate the
Architecture Constitution, AEPs, or approved architectural boundaries.

The Guardian must explain the violated protocol, technical risk, and a compliant
alternative before implementation resumes.
