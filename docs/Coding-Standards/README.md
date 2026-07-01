# Coding Standards

## General

- Keep modules cohesive and small.
- Prefer explicit types over implicit behavior.
- Avoid global mutable state.
- Do not introduce dependencies without a clear long-term reason.
- Write tests at the boundary where behavior is owned.

## Python

- Python target: 3.12.
- Formatting: Black and Ruff format.
- Linting: Ruff.
- Typing: MyPy in strict mode.
- Tests: Pytest.
- Configuration: Pydantic Settings.

## Rust

- Edition: 2024.
- Unsafe code is forbidden by default.
- Formatting: rustfmt.
- Linting: Clippy with warnings denied in validation.

## TypeScript

TypeScript will be introduced with the Next.js and Tauri phases. Frontend and
desktop code must follow the same module-boundary discipline as backend code.

