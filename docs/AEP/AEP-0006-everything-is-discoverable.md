# AEP-0006: Everything Is Discoverable

## Objective

Ensure every platform resource can be registered, inspected, and understood
through explicit metadata.

## Motivation

A cognitive operating system cannot rely on hidden components. Discovery enables
health, supervision, telemetry, permissions, and future plugins.

## Context

Aether already models managers, services, domains, drivers, and policies through
descriptors, manifests, registries, and health surfaces.

## Rules

- New resources must expose identity, name, version, health, capabilities, and
  ownership where applicable.
- Services must declare manifests.
- Managers, Drivers, Domains, and Policies must have descriptors.
- CLI or inspection APIs should expose platform metadata when a resource is
  introduced.

## Mandatory Flow

1. Define the descriptor.
2. Define registration and inspection path.
3. Define health or status representation.
4. Add tests for discovery.
5. Document how future modules find the resource.

## Correct Examples

- `ServiceDescriptor` created from a TOML manifest.
- `ManagerRegistry` exposing built-in manager descriptors.

## Incorrect Examples

- Creating a singleton service that is reachable only by direct import.
- Adding a driver without a descriptor or registry.

## Violation Detection

- Resources cannot be listed.
- Health aggregation cannot see a component.
- Capabilities are only present in comments or code paths.

## Violation Correction

Add descriptor, registry, inspection path, and tests before using the resource.

## Relationship With Other AEPs

Supports AEP-0013. Required by AEP-0008, AEP-0009, AEP-0010, and AEP-0011.
