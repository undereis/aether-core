# AEP-0011: Policies Govern Behavior

## Objective

Define Policies as the official mechanism for constraining sensitive platform
behavior.

## Motivation

Permissions declare intent, but Policies govern whether behavior should be
allowed under context. This distinction keeps authorization, privacy, and safety
from becoming scattered conditionals.

## Context

Phase 4.5 introduced policy contracts for security, filesystem, telemetry,
memory, and privacy. Enforcement is not implemented yet.

## Rules

- Policies express constraints; they do not execute product behavior.
- Managers or future enforcement components evaluate policies.
- The Kernel must not become the policy engine.
- Sensitive service, driver, memory, telemetry, and automation behavior must
  eventually pass through policy evaluation.
- Policy changes require tests and architecture review.

## Mandatory Flow

1. Identify sensitive behavior.
2. Determine the relevant Policy kind.
3. Define evaluation context and expected effect.
4. Bind enforcement outside the Kernel.
5. Test allow, deny, and not-applicable cases.

## Correct Examples

- Future FilesystemPolicy evaluating filesystem access.
- Future PrivacyPolicy constraining memory retention.

## Incorrect Examples

- Hard-coded privacy checks inside a service.
- Kernel directly deciding whether a user action is allowed.

## Violation Detection

- Scattered `if allowed` logic without a policy contract.
- Sensitive behavior with permissions but no policy path.
- Policy decisions coupled to service internals.

## Violation Correction

Extract the constraint into a Policy contract and route evaluation through the
appropriate manager or enforcement component.

## Relationship With Other AEPs

Required by AEP-0010. Supports AEP-0009 and AEP-0012.
