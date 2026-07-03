# Domain Architecture

Domains define permanent responsibility boundaries inside Aether.

They are not implementations. They prevent future services, managers, drivers,
and policies from collapsing into one undifferentiated platform area.

## Domains Introduced

- `SystemDomain`: Kernel, Managers, and core system coordination.
- `MemoryDomain`: future memory contracts only.
- `KnowledgeDomain`: future knowledge graph contracts only.
- `TelemetryDomain`: operational signals, logs, metrics, traces, and health.
- `IdentityDomain`: future identity and access boundaries.
- `AutomationDomain`: future controlled automation boundaries.

## Current Implementation

Phase 4.5 defines:

- `DomainKind`;
- `DomainDescriptor`;
- `DomainRegistry`;
- standard domain descriptors.

No Memory Engine, Knowledge Engine, AI, agents, automation runtime, identity
system, or new storage model is implemented.

## Usage

Future phases should place new capabilities inside an explicit Domain before
adding runtime behavior. A Domain answers where a responsibility belongs; it
does not execute that responsibility.
