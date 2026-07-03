# AEP-0003: Refactor Before Expand

## Objective

Require architecture cleanup before adding new responsibility to an overloaded
component.

## Motivation

Long-lived systems fail when growth is added to the easiest nearby object. Phase
4.5 decomposed the Kernel specifically to prevent that pattern.

## Context

The Kernel now delegates registry and lifecycle work to Managers. Future growth
must follow the same discipline.

## Rules

- Do not add new responsibility to a component that already owns a different
  responsibility.
- Refactor ownership boundaries before feature expansion.
- Prefer moving behavior to the correct Manager, Service, Driver, Domain, or
  Policy over enlarging the Kernel.
- Refactors must preserve public contracts.

## Mandatory Flow

1. Identify whether the target component is accumulating responsibility.
2. Compare the change against the Architecture Bible.
3. Move or create the correct boundary only if needed.
4. Preserve the public facade.
5. Validate old and new tests.

## Correct Examples

- Moving service discovery ownership into `ServiceManager`.
- Adding a policy contract before enforcing new behavior.

## Incorrect Examples

- Adding memory retrieval logic to the Kernel.
- Adding direct filesystem access to a service because no driver exists yet.

## Violation Detection

- Files growing with unrelated concepts.
- A component imports layers it should not know.
- New methods mix orchestration, domain behavior, and external IO.

## Violation Correction

Extract the new responsibility into the correct layer, keep adapters where
necessary, and document the boundary.

## Relationship With Other AEPs

Requires AEP-0002 and AEP-0004. Protects AEP-0007 from Kernel growth.
