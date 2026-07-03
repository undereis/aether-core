# Architecture Bible

Status: Phase 4.6 governance baseline.

This document answers when to create major architectural elements in Aether.

## Manager

Create a Manager when the platform needs operational governance:
registration, discovery, lifecycle, inspection, health, resource coordination,
permission coordination, driver supervision, plugin supervision, configuration
coordination, or telemetry coordination.

Do not create a Manager for user-facing product behavior.

## Service

Create a Service when Aether needs to execute a declared platform capability.

A Service must have a manifest, descriptor, capabilities, permissions,
resources, dependencies, lifecycle, health, and ASB or Contract Bus
communication path.

## Driver

Create a Driver when Aether must touch external surfaces such as filesystem,
clipboard, screen, camera, microphone, browser, network, local IPC, OS APIs, or
external devices.

Drivers require policy and resource constraints.

## Domain

Create or extend a Domain when a permanent responsibility boundary is missing.

Domains are organizational boundaries, not runtime features.

## Policy

Create a Policy when sensitive behavior needs governance across services,
drivers, memory, telemetry, privacy, security, automation, or filesystem access.

Policies constrain behavior; they do not execute features.

## Capability

Create a Capability when a service, manager, or driver needs to declare what it
can provide or require.

Capabilities should be stable, dotted names such as `memory.retrieve` or
`telemetry.emit`.

## Contract

Create a Contract when two platform areas need an explicit communication,
schema, command, request, event, or evaluation boundary.

Use typed structs, errors, and tests. Avoid implicit string conventions.

## Plugin

Create a Plugin only when Aether needs externally installed or independently
evolvable behavior.

Plugins must eventually be registered, inspected, permissioned, policy-gated,
and supervised by the platform.

## ADR

Create an ADR when the project makes a significant technical decision.

ADRs explain context, decision, and consequences. They are historical records.

## AEP

Create an AEP when the project needs a permanent engineering protocol that
governs all future work.

AEPs are active law. They override future implementation convenience.
