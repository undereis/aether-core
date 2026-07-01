# ADR-0003

## Core Runtime in Rust

## Status

Accepted.

## Context

Aether needs a native foundation for local runtime execution, module lifecycle
contracts, internal events, configuration, and structured logs. This foundation
must later support desktop integration and local event capture without forcing
those capabilities into Phase 1.

## Decision

Implement the Core Runtime in Rust as a modular workspace under `core/crates`.

Rust is selected because it provides strong typing, memory safety without a
garbage collector, predictable performance, native portability, and a mature
toolchain for long-lived systems software.

## Scope

Phase 1 includes:

- base event types
- in-memory event bus
- local configuration
- structured logging
- module lifecycle trait
- runtime bootstrap
- minimal validation CLI
- unit tests

Phase 1 explicitly excludes AI, agents, UI, OS capture, persistence, and
business behavior.

## Consequences

- Future modules can depend on stable Rust contracts instead of ad hoc runtime
  conventions.
- The Tauri desktop shell can integrate with the native runtime when the
  desktop phase begins.
- Event contracts are established early without committing to a production
  message broker in Phase 1.
- Additional runtime capabilities must be introduced through later ADRs.

