# Decision Log

This log records lightweight decisions that do not yet require a full ADR.

## 2026-06-30

- Established Phase 0 as a foundation-only milestone.
- Chose `uv` for Python workspace and virtual environment management.
- Chose local pre-commit hooks backed by workspace commands instead of remote
  hook repositories.
- Chose Docker Compose as the local infrastructure contract.
- Chose Apache-2.0 for explicit patent protection.
- Added ADR-0002 to require LTS runtimes for production usage.
- Added ADR-0003 to establish the Phase 1 Rust Core Runtime.

## 2026-07-01

- Added ADR-0004 to establish the Aether Kernel above the Core Runtime.
- Added ADR-0005 to define the typed ID prefix strategy.
- Kept the Runtime as the concrete execution boundary and placed registry,
  lifecycle, health, and discovery responsibilities in the Kernel.
- Introduced telemetry as a forward-compatible abstraction above structured
  logging without implementing OpenTelemetry in Phase 2.
- Added ADR-0006 to establish the Aether Service Platform.
- Added ADR-0007 to define the Aether Service Bus as the only service
  communication boundary.
- Added Engineering Rule #002: services must communicate only through ASB.
