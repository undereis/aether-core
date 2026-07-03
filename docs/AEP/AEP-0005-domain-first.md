# AEP-0005: Domain First

## Objective

Require every future capability to belong to an explicit Domain before
implementation.

## Motivation

Domains prevent cognitive, platform, automation, identity, and telemetry
responsibilities from blending into vague services.

## Context

Phase 4.5 introduced `SystemDomain`, `MemoryDomain`, `KnowledgeDomain`,
`TelemetryDomain`, `IdentityDomain`, and `AutomationDomain` as boundaries, not
implementations.

## Rules

- Identify the Domain before adding a capability.
- Do not put domain behavior in the Kernel.
- Managers may coordinate within Domains; Services execute capabilities inside
  Domains.
- New Domains require architecture review.

## Mandatory Flow

1. Name the target Domain.
2. Verify the capability belongs there.
3. Define the Manager, Service, Policy, and Driver relationships.
4. Document unresolved domain risks.
5. Implement only after the boundary is clear.

## Correct Examples

- Assigning future memory retention to `MemoryDomain`.
- Assigning operational metrics to `TelemetryDomain`.

## Incorrect Examples

- Adding planning logic under `SystemDomain`.
- Creating a service without deciding whether it is cognitive, AI, device, or
  automation.

## Violation Detection

- Capabilities with no domain owner.
- Services that mix memory, inference, and automation responsibilities.
- Managers named after features instead of operational responsibility.

## Violation Correction

Split the capability into domain-owned contracts and move behavior to the
correct service or manager.

## Relationship With Other AEPs

Precedes AEP-0008 and AEP-0009. Supports the Architecture Evolution Roadmap.
