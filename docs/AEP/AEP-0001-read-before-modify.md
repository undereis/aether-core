# AEP-0001: Read Before Modify

## Objective

Require every change to begin with reading the relevant code, documentation, and
contracts before editing.

## Motivation

Aether is a long-lived architecture. Changes made from memory can break subtle
boundaries, duplicate responsibility, or weaken compatibility.

## Context

The Foundation Era established Kernel, Managers, Services, Drivers, Domains,
Policies, ASB, and Contract Bus base. Future work must be grounded in those
actual artifacts.

## Rules

- Read the owning documentation before changing a component.
- Read the public contracts before changing a crate.
- Read existing tests before adding or changing behavior.
- Never rely only on phase summaries when implementation files exist.

## Mandatory Flow

1. Run `git status --short`.
2. Identify affected architecture areas.
3. Read owning docs, ADRs, AEPs, and source contracts.
4. Map dependencies and public API impact.
5. Edit only after the context is understood.

## Correct Examples

- Reading `aether-service-bus` and ASB docs before changing service routing.
- Reading `manager-architecture.md` before moving registry responsibility.

## Incorrect Examples

- Adding a new service because the name sounds right.
- Changing a public method without checking CLI and tests that call it.

## Violation Detection

- Diffs that introduce duplicate concepts already present elsewhere.
- Changes that contradict current architecture docs.
- New APIs that do not match existing naming, errors, or lifecycle style.

## Violation Correction

Pause the implementation, read the missed artifacts, update the design, and
reduce the diff to the smallest compatible change.

## Relationship With Other AEPs

This protocol precedes all others. AEP-0002, AEP-0004, and AEP-0014 cannot be
enforced without reading first.
