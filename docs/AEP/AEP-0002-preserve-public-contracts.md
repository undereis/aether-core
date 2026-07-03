# AEP-0002: Preserve Public Contracts

## Objective

Protect public APIs, manifests, CLI commands, typed IDs, and documented
contracts from accidental breakage.

## Motivation

Aether will evolve through many phases. Stable contracts allow future modules to
build safely on earlier foundation work.

## Context

The Kernel exposes a compatibility facade. Services use manifests and typed
descriptors. The ASB exposes routing contracts. These are platform contracts,
not incidental implementation details.

## Rules

- Do not remove public methods without a documented migration.
- Do not rename IDs, capabilities, permissions, routes, or manifest fields
  casually.
- Do not change CLI output semantics without tests and documentation.
- Breaking changes require an ADR and compatibility plan.

## Mandatory Flow

1. Identify all public contracts touched by the change.
2. Check downstream crates and CLI usage.
3. Preserve old contracts or provide an adapter.
4. Add tests for compatibility.
5. Document any intentional migration.

## Correct Examples

- Keeping `AetherKernel::services()` while internally delegating to
  `ServiceManager`.
- Adding Contract Bus base without removing existing ASB operations.

## Incorrect Examples

- Replacing service manifest fields without a migration.
- Renaming `service.command` permissions because a new name looks cleaner.

## Violation Detection

- Removed public symbols.
- Changed serialized names.
- Tests from earlier phases fail after a refactor.
- CLI commands disappear or change meaning.

## Violation Correction

Restore the old contract, add a facade, or create a documented compatibility
layer before proceeding.

## Relationship With Other AEPs

Supports AEP-0014. Constrains AEP-0003 by requiring refactors to preserve
externally visible behavior.
