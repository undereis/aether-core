# AEP-0015: Git Checkpoint Discipline

## Objective

Define official discipline for validation, staging, commits, tags, and phase
closure.

## Motivation

Architecture checkpoints are historical anchors. Dirty, unvalidated, or secret
containing checkpoints weaken trust in the project timeline.

## Context

The Foundation Era closed with tagged checkpoints through `v0.4.5`. Future eras
must preserve this review and validation discipline.

## Rules

- Do not commit without explicit authorization.
- Do not tag without explicit authorization.
- Run required validation before official checkpoint.
- Audit secrets and temporary files before staging.
- Confirm `git status --short` before and after checkpoint.
- Commit messages and tags must match the approved phase scope.

## Mandatory Flow

1. Run `git status --short`.
2. Run required validation.
3. Audit secrets, temporary files, generated artifacts, and lockfiles.
4. Confirm files to be versioned with `git diff --stat`.
5. Stage, commit, tag only when authorized.
6. Confirm final clean status.

## Correct Examples

- Creating `v0.4.5-kernel-decomposition` only after validation and approval.
- Leaving a completed phase uncommitted when the user requested no commit.

## Incorrect Examples

- Auto-committing at the end of a phase without authorization.
- Tagging a checkpoint after partial validation.

## Violation Detection

- Commit exists with failed or missing validation.
- Generated cache or real secrets are staged.
- Git status is dirty after official checkpoint.

## Violation Correction

Stop, unstage unsafe files, rerun validation, fix issues, and recreate the
checkpoint only with explicit approval.

## Relationship With Other AEPs

Operationalizes AEP-0001, AEP-0002, and AEP-0014 at phase boundaries.
