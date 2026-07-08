# Aether Engineering Protocols

Aether Engineering Protocols, or AEPs, are mandatory engineering laws for the
Aether project.

ADRs explain why a technical decision was made. AEPs define how future work must
be performed so the architecture remains coherent across years of change.

## When To Create A New AEP

Create a new AEP when a recurring architectural or engineering rule must govern
all future work, not only one decision. AEPs are appropriate for workflow
discipline, compatibility policy, contract rules, boundary ownership, and
platform-wide safety constraints.

Do not create an AEP for a one-time implementation choice. Use an ADR for that.

## Who Can Change An AEP

AEPs can be changed only through explicit architecture review. Any change must:

- preserve existing public contracts unless a migration path exists;
- document the reason for the change;
- update related AEPs, ADRs, and architecture documents;
- pass the full validation suite;
- be accepted by the Architecture Guardian role.

## Relationship With ADRs

ADRs are historical decisions. AEPs are active governance.

An ADR may introduce a component such as the Kernel or ASB. An AEP defines the
ongoing protocol for modifying, extending, or protecting that component.

If an ADR and an AEP appear to conflict, the current AEP governs future changes
and the ADR should receive an addendum.

## Project Governance

Every future phase must be checked against the AEP set before implementation.
If a request violates an AEP, implementation must stop until the violation is
resolved or an explicit architecture review changes the relevant protocol.

## Protocol Index

- [AEP-0001: Read Before Modify](AEP-0001-read-before-modify.md)
- [AEP-0002: Preserve Public Contracts](AEP-0002-preserve-public-contracts.md)
- [AEP-0003: Refactor Before Expand](AEP-0003-refactor-before-expand.md)
- [AEP-0004: Architecture First](AEP-0004-architecture-first.md)
- [AEP-0005: Domain First](AEP-0005-domain-first.md)
- [AEP-0006: Everything Is Discoverable](AEP-0006-everything-is-discoverable.md)
- [AEP-0007: Kernel Is Sacred](AEP-0007-kernel-is-sacred.md)
- [AEP-0008: Managers Own Domains](AEP-0008-managers-own-domains.md)
- [AEP-0009: Services Execute Capabilities](AEP-0009-services-execute-capabilities.md)
- [AEP-0010: Drivers Touch The Outside World](AEP-0010-drivers-touch-the-outside-world.md)
- [AEP-0011: Policies Govern Behavior](AEP-0011-policies-govern-behavior.md)
- [AEP-0012: Communication Is Explicit](AEP-0012-communication-is-explicit.md)
- [AEP-0013: Everything Has Contracts](AEP-0013-everything-has-contracts.md)
- [AEP-0014: Compatibility First](AEP-0014-compatibility-first.md)
- [AEP-0015: Git Checkpoint Discipline](AEP-0015-git-checkpoint-discipline.md)
- [AEP-0016: Cognitive Traceability](AEP-0016-cognitive-traceability.md)
