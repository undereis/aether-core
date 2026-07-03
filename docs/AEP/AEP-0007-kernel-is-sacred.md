# AEP-0007: Kernel Is Sacred

## Objective

Protect the Kernel from business logic, domain logic, external IO, and
unbounded responsibility.

## Motivation

The Kernel is the highest-gravity component in Aether. If it becomes a God
Object, every future phase becomes harder and riskier.

## Context

Phase 4.5 reduced Kernel responsibility. The Kernel now coordinates Runtime,
Managers, lifecycle, service registration facade, telemetry, and shutdown.

## Rules

- The Kernel starts, stops, supervises, registers, discovers, coordinates, and
  recovers.
- The Kernel does not execute domain behavior.
- The Kernel does not call drivers directly.
- The Kernel does not know service internals.
- The Kernel may expose compatibility facades that delegate to Managers.

## Mandatory Flow

1. Ask whether the change is coordination or behavior.
2. If behavior, move it out of the Kernel.
3. If coordination, delegate to a Manager where possible.
4. Preserve public Kernel compatibility methods.
5. Add tests proving the Kernel remains a facade.

## Correct Examples

- `AetherKernel::register_service` delegating to `ServiceManager`.
- Kernel shutdown coordinating runtime and lifecycle transitions.

## Incorrect Examples

- Adding memory retrieval to `AetherKernel`.
- Adding policy enforcement directly inside Kernel methods.

## Violation Detection

- Kernel imports cognitive, AI, device, automation, or business crates.
- Kernel methods contain domain decisions.
- Kernel file grows to own new registries directly.

## Violation Correction

Extract the behavior to the correct Manager, Service, Driver, Domain, or Policy
contract. Keep a compatibility facade only when necessary.

## Relationship With Other AEPs

Enforced with AEP-0003 and AEP-0014. Supported by AEP-0008.
