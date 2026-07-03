# AEP-0012: Communication Is Explicit

## Objective

Require all cross-service communication to use explicit platform communication
mechanisms.

## Motivation

Hidden communication creates fragile coupling. Explicit routing enables
permissions, telemetry, discovery, future distribution, and contract evolution.

## Context

The ASB supports events, subscriptions, request/reply, commands, notifications,
and status. The Contract Bus base introduces typed internal contract routing.

## Rules

- Services must communicate through ASB or Contract Bus.
- Do not pass direct service references between services.
- Routes, subjects, events, and contracts must be named and documented.
- Commands must be routed by bus route, not concrete service identity.
- Permissions must be declared before bus actions.

## Mandatory Flow

1. Choose event, request/reply, command, notification, or typed contract.
2. Define route, subject, event type, or contract name/version.
3. Declare permissions and capabilities.
4. Add tests for routing and permission failure.
5. Document the communication path.

## Correct Examples

- `system.health` command route handled through ASB.
- `BusContract::new("system.inspect", "v1")` for typed local routing.

## Incorrect Examples

- Calling another service method directly.
- Sharing an internal channel owned by a service.

## Violation Detection

- Service-to-service imports.
- Routes containing concrete service IDs.
- Permissionless bus calls.
- Communication paths missing tests.

## Violation Correction

Replace direct references with ASB or Contract Bus contracts and add manifest
permissions.

## Relationship With Other AEPs

Directly enforces Engineering Rule #002 and supports AEP-0009 and AEP-0013.
