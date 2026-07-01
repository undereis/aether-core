# Project Structure

```text
Aether/
  core/              Rust core runtime and kernel crates
  backend/           FastAPI backend service
  frontend/          Reserved for Next.js
  desktop/           Reserved for Tauri
  infrastructure/    Operational architecture notes
  docker/            Docker support files
  docs/              Engineering documentation
  scripts/           Local automation scripts
  tools/             Developer tooling notes
  tests/             Cross-system tests, introduced after Phase 0
  .github/           Future CI/CD automation
```

## Ownership Rules

- Each top-level module must be able to evolve independently.
- Cross-module contracts must be documented before implementation.
- Shared code must live in an intentional package, not in incidental utility
  folders.
- Future business domains must not be placed directly at repository root.

## Core Crates

- `aether-events`: base events and internal event bus.
- `aether-config`: local runtime configuration.
- `aether-ids`: typed identifier strategy.
- `aether-logging`: structured logging primitives.
- `aether-telemetry`: telemetry abstraction.
- `aether-core`: module lifecycle contracts.
- `aether-runtime`: runtime bootstrap.
- `aether-kernel`: kernel orchestration, registry, lifecycle, health, and discovery.
- `aether-cli`: validation CLI.
