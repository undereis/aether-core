# Tech Stack

## Official Stack

- Core runtime: Rust.
- Backend: Python 3.12.
- Framework: FastAPI.
- Desktop: Tauri.
- Frontend: Next.js with TypeScript.
- Relational database: PostgreSQL.
- Vector database: Qdrant.
- Cache: Redis.
- Containerization: Docker Compose.
- Future messaging: NATS.
- Future observability: OpenTelemetry.
- Future CI/CD: GitHub Actions.

## Runtime Version Policy

Production runtimes must follow the LTS strategy defined in
`docs/ADRs/ADR-0002-runtime-version-strategy.md`.

## Phase 0 Tooling

- Python workspace: `uv`.
- Python linting and formatting: Ruff, Black.
- Python typing: MyPy.
- Python testing: Pytest.
- Rust validation: Cargo, rustfmt, Clippy.
- Git hooks: Pre-commit with local hooks.

## Phase 1 Core Runtime

- Rust workspace crates under `core/crates`.
- Serialization: Serde and JSON.
- Local configuration format: TOML.
- Internal event IDs: UUID v7.
- Timestamp representation: UTC offsets from the `time` crate.

## Phase 2 Kernel

- Kernel orchestration crate: `aether-kernel`.
- Typed ID crate: `aether-ids`.
- Telemetry abstraction crate: `aether-telemetry`.
- Event bus abstraction: `EventBusPort`.
- Configuration abstraction: `ConfigProvider`.
- Current event bus implementation: in-memory.
- Current configuration implementation: TOML and static in-memory providers.
