# AEP-0014: Compatibility First

## Objective

Make backward compatibility the default posture for all platform evolution.

## Motivation

Aether will span many eras. Each era must build on previous checkpoints without
forcing needless rewrites.

## Context

Phase 4.5 preserved Kernel public methods while moving internals into Managers.
This is the compatibility model for future refactors.

## Rules

- Preserve public APIs unless a migration is approved.
- Add adapters or facades when moving responsibility.
- Keep manifests and CLI commands stable unless an ADR documents migration.
- Breaking changes require clear justification, tests, and release notes.
- Compatibility must be validated before checkpoint.

## Mandatory Flow

1. Identify callers and serialized surfaces.
2. Decide whether the change can be additive.
3. Add compatibility facade when moving ownership.
4. Test old and new paths.
5. Document migration only if breakage is unavoidable.

## Correct Examples

- Kernel methods delegating to Managers after decomposition.
- Adding `contract_handlers` to bus status without removing existing fields.

## Incorrect Examples

- Removing `service list` because `system services` exists.
- Replacing existing permission names without alias or migration.

## Violation Detection

- Earlier phase tests fail.
- Existing commands or methods disappear.
- Serialized manifests become unreadable.

## Violation Correction

Restore compatibility, add adapters, or create an approved migration ADR before
continuing.

## Relationship With Other AEPs

Strengthens AEP-0002. Constrains AEP-0003 and AEP-0015.
