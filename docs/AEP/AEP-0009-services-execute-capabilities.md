# AEP-0009: Services Execute Capabilities

## Objective

Define Services as the units that execute declared platform capabilities.

## Motivation

Aether services must be stable platform responsibilities, not arbitrary feature
classes. Capability declaration makes service behavior inspectable and
governable.

## Context

The Service Model defines manifests, descriptors, capabilities, permissions,
dependencies, resources, lifecycle, health, and ASB communication.

## Rules

- A Service must declare provided and required capabilities.
- A Service must declare permissions before using platform actions.
- A Service must declare resources before consuming them.
- A Service must communicate through ASB or Contract Bus, never direct service
  references.
- A Service must not perform unmanaged external IO; that belongs to Drivers.

## Mandatory Flow

1. Define service manifest.
2. Define capabilities, permissions, resources, dependencies, and health.
3. Register through Kernel/ServiceManager.
4. Expose communication through ASB or Contract Bus route.
5. Add tests for registration, discovery, permissions, resources, and health.

## Correct Examples

- Core System Services exposing `system.*` command routes through ASB.
- Service dependency expressed as required capability, not service object.

## Incorrect Examples

- Service storing a reference to another service.
- Service accessing filesystem directly without a driver and policy path.

## Violation Detection

- Direct imports of peer services.
- Dependencies expressed as concrete service IDs instead of capabilities.
- Bus actions without declared permissions.

## Violation Correction

Replace direct coupling with ASB route, request/reply, event, notification, or
typed contract. Add missing manifest declarations.

## Relationship With Other AEPs

Enforces AEP-0012 and depends on AEP-0006 and AEP-0013.
