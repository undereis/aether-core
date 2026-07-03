# Architectural Gravity

Status: Phase 4.6 governance baseline.

Architectural gravity measures how costly it is to change a layer.

The closer a layer is to the Kernel, the more stable, reviewed, tested, and
compatible it must be.

## Classification

| Layer | Gravity | Stability Expectation |
| --- | --- | --- |
| Kernel | ★★★★★ | Highest stability. Coordination only. Breaking changes require exceptional review. |
| Managers | ★★★★☆ | High stability. Own operational governance and registries. |
| Policies | ★★★★☆ | High stability. Govern sensitive behavior and future enforcement. |
| Domains | ★★★★☆ | High stability. Define permanent responsibility boundaries. |
| Drivers | ★★★☆☆ | Medium stability. Touch external world through constrained contracts. |
| Services | ★★☆☆☆ | Evolvable. Execute capabilities behind manifests and bus contracts. |
| Plugins | ★☆☆☆☆ | Highly evolvable. Must remain isolated and policy-gated. |
| Frontend/Desktop | ★☆☆☆☆ | Highly evolvable. Must not redefine platform contracts. |

## Rules

- High-gravity changes require small diffs and explicit review.
- High-gravity public contracts should be additive by default.
- Low-gravity layers may evolve faster, but cannot bypass higher-gravity
  contracts.
- Moving behavior toward the Kernel increases risk and must be justified.
- Moving behavior away from the Kernel is preferred when compatibility is
  preserved.

## Review Intensity

Kernel changes require the strongest review.

Manager, Policy, and Domain changes require architecture review.

Driver changes require architecture, security, and resource review.

Service changes require capability, permission, resource, health, and bus
contract review.

Frontend, Desktop, and Plugin changes require boundary review to ensure they do
not redefine platform behavior.
