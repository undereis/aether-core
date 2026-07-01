# ADR-0002

## Runtime Version Strategy

## Status

Accepted.

## Decisão

O projeto Aether utilizará sempre versões LTS para runtimes utilizados em
produção.

Inclui:

- Node.js
- Python
- Rust (Stable)
- Docker
- PostgreSQL
- Redis

Versões bleeding-edge somente poderão ser utilizadas quando houver justificativa
técnica documentada em um novo ADR.

## Objetivo

Garantir estabilidade, compatibilidade, manutenibilidade e facilidade de
atualização.
