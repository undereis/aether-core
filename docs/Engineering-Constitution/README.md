# Engineering Constitution

Aether is not optimized for short-term demos. Engineering decisions must
preserve long-term changeability, safety, and operational clarity.

## Principles

- Architecture before features.
- Clear boundaries before scale.
- Observability before production complexity.
- Typed contracts before implicit coupling.
- Tests before broad refactors.
- Boring, mature technology before novelty.

## Non-Negotiables

- No business logic in infrastructure code.
- No feature work without an owning module.
- No global mutable runtime state.
- No hidden service dependencies.
- No unmanaged secrets in source control.
- No broad dependency additions without justification.

## Engineering Rules

### Rule #001: Kernel-Controlled Platform Resources

Internal platform resources must be registered, inspected, and supervised
through Kernel-owned contracts.

### Rule #002: Service Communication Through ASB

No service may know directly about another service.

All service communication must pass through the Aether Service Bus. Direct
service references, direct service method calls, and direct service-owned
channels are architectural violations.
