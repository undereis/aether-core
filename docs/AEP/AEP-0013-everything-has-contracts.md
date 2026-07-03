# AEP-0013: Everything Has Contracts

## Objective

Require every significant platform element to expose typed, documented
contracts.

## Motivation

Contracts keep a long-lived system understandable and testable. They also allow
future implementation changes without breaking callers.

## Context

Aether already uses typed IDs, manifests, descriptors, registries, traits,
errors, lifecycle states, health models, permissions, resources, ASB routes, and
Contract Bus contracts.

## Rules

- Prefer typed structs, enums, traits, and result errors over ad hoc strings.
- Use manifests for declarative service intent.
- Use typed IDs for stable identities.
- Define explicit errors for failure modes.
- Add tests that prove contract behavior.

## Mandatory Flow

1. Identify the contract surface.
2. Define types and errors.
3. Define serialization or manifest shape if applicable.
4. Add compatibility tests.
5. Document the contract.

## Correct Examples

- `ServiceDescriptor` derived from `ServiceManifest`.
- `PolicyDecision` and `PolicyEffect` for future policy evaluation.

## Incorrect Examples

- Passing loosely typed JSON maps across internal boundaries without a contract.
- Returning raw strings for structured platform health.

## Violation Detection

- Untyped maps used as primary internal model.
- Missing error enum for recoverable failure.
- Behavior depends on undocumented string conventions.

## Violation Correction

Introduce typed models, update tests, and document the contract before behavior
expands.

## Relationship With Other AEPs

Supports AEP-0002, AEP-0006, AEP-0012, and AEP-0014.
