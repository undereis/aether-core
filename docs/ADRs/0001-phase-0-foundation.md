# ADR 0001: Phase 0 Foundation Boundaries

## Status

Accepted.

## Context

Aether is intended to evolve for decades. Phase 0 must establish engineering
governance, validation, and operational boundaries before feature work begins.

## Decision

Phase 0 contains infrastructure, documentation, development tooling, and a
minimal backend surface only. It explicitly excludes memory, agents, AI,
embeddings, chat, authentication, business APIs, and product UI.

## Consequences

- Future features must enter through documented architecture decisions.
- The repository can be validated before product complexity is introduced.
- The platform starts with clear module ownership and extension points.

