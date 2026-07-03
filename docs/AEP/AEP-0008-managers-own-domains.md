# AEP-0008: Managers Own Domains

## Objective

Define Managers as the owners of operational governance inside architectural
domains.

## Motivation

Managers keep the Kernel small while giving the platform a place for
coordination, validation, discovery, lifecycle, and future enforcement.

## Context

Phase 4.5 introduced `ServiceManager`, `LifecycleManager`, `ResourceManager`,
`TelemetryManager`, `ConfigurationManager`, `PermissionManager`,
`HealthManager`, `DriverManager`, and `PluginManager`.

## Rules

- Managers coordinate; they do not implement user-facing features.
- Managers may own registries and lifecycle indexes.
- Managers must expose descriptors, capabilities, health, and manifests.
- Managers must not bypass ASB for service-to-service communication.
- New Managers require architecture review.

## Mandatory Flow

1. Identify the operational responsibility.
2. Confirm no existing Manager owns it.
3. Define manager descriptor and capabilities.
4. Define registry or coordination boundary.
5. Add tests for manager health and discovery.

## Correct Examples

- `LifecycleManager` owning module lifecycle transitions.
- `ServiceManager` owning service discovery and ASB permission registration.

## Incorrect Examples

- Manager executing reasoning or memory retrieval.
- Manager calling one service directly from another service.

## Violation Detection

- Manager names describe product features instead of governance.
- Manager contains domain algorithm behavior.
- Manager has hidden dependencies on concrete services.

## Violation Correction

Move product behavior to Services, external behavior to Drivers, and constraints
to Policies. Keep Managers focused on coordination.

## Relationship With Other AEPs

Implements the boundary required by AEP-0007. Precedes AEP-0009 and AEP-0011.
