# Aether Architecture Constitution v2

Status: Phase 4.6 constitutional baseline.

This constitution formalizes the architecture established by the Foundation Era
and governed by the Aether Engineering Protocols.

## Architecture Guardian

The Architecture Guardian role is mandatory from Phase 4.6 onward.

If a request violates an AEP, the Architecture Guardian must stop the
implementation, identify the violated protocol, explain the technical risk, and
propose a compliant alternative.

## Read Before Modify

No architectural component may be changed before reading its owning docs, ADRs,
AEPs, tests, and public contracts.

This constitution treats uninformed changes as architecture risk, even when the
code compiles.

## Kernel

The Kernel is sacred.

The Kernel may:

- start the platform;
- stop the platform safely;
- coordinate Runtime and Managers;
- expose compatibility facades;
- supervise lifecycle and health;
- emit foundational telemetry.

The Kernel must not:

- execute domain behavior;
- own service internals directly;
- touch external IO;
- evaluate policies directly;
- become a convenience container for future features.

## Managers

Managers own operational governance inside platform boundaries.

Managers coordinate registration, discovery, lifecycle, health, resources,
permissions, telemetry, drivers, plugins, and configuration. They do not execute
user-facing capabilities.

## Services

Services execute declared capabilities.

Every service must have a manifest, descriptor, capabilities, permissions,
resources, dependencies, lifecycle status, health status, and ASB or Contract
Bus communication path.

No service may know another service directly.

## Drivers

Drivers touch the outside world.

External IO belongs behind Driver contracts and future DriverManager
supervision. Drivers must declare capabilities, resources, provider, version,
and health.

## Domains

Domains define permanent responsibility boundaries.

Domains do not execute behavior. They answer where a capability belongs before
Managers, Services, Drivers, or Policies are selected.

## Policies

Policies govern behavior.

Policies define constraints for security, privacy, filesystem access,
telemetry, memory, automation, and future sensitive operations. The Kernel must
not become the policy engine.

## Aether Service Bus

The ASB is the official service communication boundary.

Allowed communication forms:

- events;
- subscriptions;
- request/reply;
- service commands;
- service notifications;
- typed Contract Bus requests.

Direct service references are prohibited.

## Contract Bus

The Contract Bus base is the future typed communication layer above the ASB.

It currently provides local typed request routing. Future phases may extend it
for schema negotiation, distributed transport, or IPC, but must preserve ASB
compatibility.

## Architectural Gravity

The closer a component is to the Kernel, the more stable it must be.

High-gravity layers require stronger review, smaller diffs, compatibility
facades, and explicit migration plans.

## Compatibility

Compatibility is the default.

Public contracts, manifests, CLI commands, typed IDs, permissions, capabilities,
and routes must be preserved unless an ADR and migration plan approve a breaking
change.

## Constitutional Rule

Every future implementation must pass three checks:

1. Does it obey the AEPs?
2. Does it preserve the architectural gravity of the affected layer?
3. Does it keep the Kernel, Managers, Services, Drivers, Domains, Policies, ASB,
   and Contract Bus boundaries intact?
