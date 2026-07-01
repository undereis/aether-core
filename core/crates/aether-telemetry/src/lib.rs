#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Telemetry abstraction for Aether logs, metrics, and traces.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex};

use aether_logging::{LogLevel, LogRecord, LoggingError, StructuredLogger};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

/// Telemetry attribute map.
pub type TelemetryAttributes = BTreeMap<String, String>;

/// Telemetry signal category.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TelemetrySignal {
    /// Structured log signal.
    Log,
    /// Metric signal placeholder.
    Metric,
    /// Trace signal placeholder.
    Trace,
}

impl TelemetrySignal {
    /// Return the canonical signal name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Metric => "metric",
            Self::Trace => "trace",
        }
    }
}

impl fmt::Display for TelemetrySignal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Telemetry record emitted by core components.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TelemetryRecord {
    timestamp: OffsetDateTime,
    signal: TelemetrySignal,
    level: LogLevel,
    target: String,
    message: String,
    attributes: TelemetryAttributes,
}

impl TelemetryRecord {
    /// Create a telemetry record.
    #[must_use]
    pub fn new(
        signal: TelemetrySignal,
        level: LogLevel,
        target: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc(),
            signal,
            level,
            target: target.into(),
            message: message.into(),
            attributes: TelemetryAttributes::new(),
        }
    }

    /// Create a telemetry log record.
    #[must_use]
    pub fn log(level: LogLevel, target: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new(TelemetrySignal::Log, level, target, message)
    }

    /// Attach a telemetry attribute.
    #[must_use]
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Return the record signal.
    #[must_use]
    pub const fn signal(&self) -> TelemetrySignal {
        self.signal
    }

    /// Return the record level.
    #[must_use]
    pub const fn level(&self) -> LogLevel {
        self.level
    }

    /// Return the record target.
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }

    /// Return the message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Return telemetry attributes.
    #[must_use]
    pub const fn attributes(&self) -> &TelemetryAttributes {
        &self.attributes
    }
}

/// Telemetry sink contract.
pub trait TelemetrySink: Send + Sync {
    /// Emit a telemetry record.
    ///
    /// # Errors
    ///
    /// Implementations return [`TelemetryError`] when the record cannot be emitted.
    fn emit(&self, record: &TelemetryRecord) -> Result<(), TelemetryError>;
}

/// Telemetry emitter with level filtering.
#[derive(Clone)]
pub struct TelemetryEmitter {
    minimum_level: LogLevel,
    sink: Arc<dyn TelemetrySink>,
}

impl TelemetryEmitter {
    /// Create a telemetry emitter.
    #[must_use]
    pub fn new(minimum_level: LogLevel, sink: Arc<dyn TelemetrySink>) -> Self {
        Self {
            minimum_level,
            sink,
        }
    }

    /// Emit a telemetry record when it passes the configured level.
    ///
    /// # Errors
    ///
    /// Returns [`TelemetryError`] when the sink cannot emit the record.
    pub fn emit(&self, record: &TelemetryRecord) -> Result<(), TelemetryError> {
        if record.level() >= self.minimum_level {
            self.sink.emit(record)?;
        }
        Ok(())
    }

    /// Emit a log signal.
    ///
    /// # Errors
    ///
    /// Returns [`TelemetryError`] when the sink cannot emit the record.
    pub fn log(&self, level: LogLevel, target: &str, message: &str) -> Result<(), TelemetryError> {
        self.emit(&TelemetryRecord::log(level, target, message))
    }

    /// Return the minimum enabled level.
    #[must_use]
    pub const fn minimum_level(&self) -> LogLevel {
        self.minimum_level
    }
}

impl fmt::Debug for TelemetryEmitter {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TelemetryEmitter")
            .field("minimum_level", &self.minimum_level)
            .finish_non_exhaustive()
    }
}

/// Telemetry sink that forwards log signals to the structured logger.
#[derive(Clone, Debug)]
pub struct LoggingTelemetrySink {
    logger: StructuredLogger,
}

impl LoggingTelemetrySink {
    /// Create a sink backed by the structured logger.
    #[must_use]
    pub const fn new(logger: StructuredLogger) -> Self {
        Self { logger }
    }
}

impl TelemetrySink for LoggingTelemetrySink {
    fn emit(&self, record: &TelemetryRecord) -> Result<(), TelemetryError> {
        let mut log_record = LogRecord::new(record.level(), record.target(), record.message())
            .with_metadata("telemetry.signal", record.signal().as_str());

        for (key, value) in record.attributes() {
            log_record = log_record.with_metadata(key, value);
        }

        self.logger.log(&log_record)?;
        Ok(())
    }
}

/// In-memory telemetry sink for tests and local validation.
#[derive(Debug, Default)]
pub struct MemoryTelemetrySink {
    records: Mutex<Vec<TelemetryRecord>>,
}

impl MemoryTelemetrySink {
    /// Create an empty in-memory telemetry sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a snapshot of stored telemetry records.
    ///
    /// # Errors
    ///
    /// Returns [`TelemetryError::SinkUnavailable`] when the sink lock is poisoned.
    pub fn records(&self) -> Result<Vec<TelemetryRecord>, TelemetryError> {
        self.records
            .lock()
            .map_err(|_| TelemetryError::SinkUnavailable)
            .map(|records| records.clone())
    }
}

impl TelemetrySink for MemoryTelemetrySink {
    fn emit(&self, record: &TelemetryRecord) -> Result<(), TelemetryError> {
        self.records
            .lock()
            .map_err(|_| TelemetryError::SinkUnavailable)?
            .push(record.clone());
        Ok(())
    }
}

/// Telemetry subsystem errors.
#[derive(Debug, Error)]
pub enum TelemetryError {
    /// Telemetry sink is unavailable.
    #[error("telemetry sink is unavailable")]
    SinkUnavailable,
    /// Structured logging failed while emitting telemetry.
    #[error("structured logging failed while emitting telemetry: {0}")]
    Logging(#[from] LoggingError),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use aether_logging::{LogLevel, MemoryLogSink, StructuredLogger};

    use super::{
        LoggingTelemetrySink, MemoryTelemetrySink, TelemetryEmitter, TelemetryRecord,
        TelemetrySignal, TelemetrySink,
    };

    #[test]
    fn telemetry_emitter_records_log_signal() {
        let sink = Arc::new(MemoryTelemetrySink::new());
        let emitter = TelemetryEmitter::new(LogLevel::Debug, sink.clone());

        emitter
            .emit(
                &TelemetryRecord::log(LogLevel::Info, "test", "kernel started")
                    .with_attribute("kernel_id", "ker_test"),
            )
            .expect("telemetry");

        let records = sink.records().expect("records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].signal(), TelemetrySignal::Log);
        assert_eq!(
            records[0].attributes().get("kernel_id").map(String::as_str),
            Some("ker_test")
        );
    }

    #[test]
    fn logging_sink_forwards_to_structured_logger() {
        let log_sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(LogLevel::Debug, log_sink.clone());
        let telemetry = LoggingTelemetrySink::new(logger);

        telemetry
            .emit(&TelemetryRecord::log(LogLevel::Info, "test", "hello"))
            .expect("telemetry");

        let records = log_sink.records().expect("records");
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0]
                .metadata()
                .get("telemetry.signal")
                .map(String::as_str),
            Some("log")
        );
    }
}
