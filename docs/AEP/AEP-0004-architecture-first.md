# AEP-0004: Architecture First

## Objective

Require architecture definition before implementation of new platform
capabilities.

## Motivation

Aether is not a feature pile. Every significant capability must enter through a
documented boundary with ownership, contracts, permissions, resources, and tests.

## Context

The project now has formal architecture layers and governance documents. Future
implementation must obey them before code exists.

## Rules

- Define the owning layer before implementation.
- Define contracts before runtime behavior.
- Define capabilities, permissions, resources, and health expectations for
  services.
- Record major decisions as ADRs.
- Create or update AEPs only for ongoing governance rules.

## Mandatory Flow

1. Classify the change by layer and domain.
2. Update architecture docs or ADRs if the boundary changes.
3. Define contracts and validation expectations.
4. Implement within the approved boundary.
5. Validate and record the result.

## Correct Examples

- Writing an ADR before changing service transport strategy.
- Defining a domain boundary before adding memory behavior.

## Incorrect Examples

- Implementing a capability first and documenting later.
- Adding cross-layer dependencies to make a feature work quickly.

## Violation Detection

- Code appears with no owning architecture document.
- New components have no declared capability or contract.
- Review cannot answer where the responsibility belongs.

## Violation Correction

Stop coding, document the boundary, review it, then adapt the implementation to
match.

## Relationship With Other AEPs

Parent protocol for AEP-0005, AEP-0012, and AEP-0013.
