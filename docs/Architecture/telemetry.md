# Telemetry

Telemetry is the forward-compatible observability boundary for the Core.

Phase 1 introduced structured logging. Phase 2 keeps that logging intact and
adds a telemetry abstraction that can later carry logs, metrics, and tracing
without changing Kernel call sites.

## Current Model

`aether-telemetry` defines:

- telemetry signals: `log`, `metric`, and `trace`
- telemetry records with UTC timestamps
- attribute maps for structured context
- `TelemetrySink`
- `TelemetryEmitter`
- in-memory sink for tests
- logging-backed sink for compatibility with `aether-logging`

Only log signals are actively emitted in Phase 2. Metric and trace signals are
part of the contract so future OpenTelemetry integration can be added through a
new ADR without rewriting Kernel orchestration.

## Relationship With Logging

Logging remains the concrete local output mechanism.

Telemetry is the abstraction above it:

- Kernel code emits telemetry records.
- The initial sink can forward those records to the structured logger.
- Tests can use the in-memory telemetry sink.
- Future sinks can export metrics or traces.

## Current Limits

Phase 2 does not implement OpenTelemetry, distributed tracing, metrics
aggregation, dashboards, alerting, or production log shipping.
