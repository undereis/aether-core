#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Structured logging primitives for Aether.

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

/// Log metadata map.
pub type LogMetadata = BTreeMap<String, String>;

/// Structured log severity level.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    /// Diagnostic information.
    Debug,
    /// General runtime information.
    Info,
    /// Recoverable concern.
    Warning,
    /// Runtime error.
    Error,
}

impl LogLevel {
    /// Return the canonical level name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for LogLevel {
    type Err = LoggingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warning" | "warn" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            _ => Err(LoggingError::InvalidLevel(value.to_owned())),
        }
    }
}

/// Structured log record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogRecord {
    timestamp: OffsetDateTime,
    level: LogLevel,
    target: String,
    message: String,
    metadata: LogMetadata,
}

impl LogRecord {
    /// Create a log record.
    #[must_use]
    pub fn new(level: LogLevel, target: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            timestamp: OffsetDateTime::now_utc(),
            level,
            target: target.into(),
            message: message.into(),
            metadata: LogMetadata::new(),
        }
    }

    /// Attach structured metadata.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
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

    /// Return metadata.
    #[must_use]
    pub const fn metadata(&self) -> &LogMetadata {
        &self.metadata
    }
}

/// Sink used by structured loggers.
pub trait LogSink: Send + Sync {
    /// Write a structured log record.
    ///
    /// # Errors
    ///
    /// Implementations return [`LoggingError`] when the record cannot be stored or emitted.
    fn write(&self, record: &LogRecord) -> Result<(), LoggingError>;
}

/// Structured logger with level filtering.
#[derive(Clone)]
pub struct StructuredLogger {
    minimum_level: LogLevel,
    sink: Arc<dyn LogSink>,
}

impl StructuredLogger {
    /// Create a structured logger.
    #[must_use]
    pub fn new(minimum_level: LogLevel, sink: Arc<dyn LogSink>) -> Self {
        Self {
            minimum_level,
            sink,
        }
    }

    /// Log a structured record when it passes the configured level.
    ///
    /// # Errors
    ///
    /// Returns [`LoggingError`] when the sink cannot emit the record.
    pub fn log(&self, record: &LogRecord) -> Result<(), LoggingError> {
        if record.level() >= self.minimum_level {
            self.sink.write(record)?;
        }
        Ok(())
    }

    /// Log an informational message.
    ///
    /// # Errors
    ///
    /// Returns [`LoggingError`] when the sink cannot emit the record.
    pub fn info(&self, target: &str, message: &str) -> Result<(), LoggingError> {
        self.log(&LogRecord::new(LogLevel::Info, target, message))
    }

    /// Return the minimum enabled level.
    #[must_use]
    pub const fn minimum_level(&self) -> LogLevel {
        self.minimum_level
    }
}

impl fmt::Debug for StructuredLogger {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("StructuredLogger")
            .field("minimum_level", &self.minimum_level)
            .finish_non_exhaustive()
    }
}

/// JSON-lines log sink.
#[derive(Debug)]
pub struct JsonLogSink<W>
where
    W: Write + Send,
{
    writer: Mutex<W>,
}

impl<W> JsonLogSink<W>
where
    W: Write + Send,
{
    /// Create a JSON-lines sink.
    #[must_use]
    pub const fn new(writer: W) -> Self {
        Self {
            writer: Mutex::new(writer),
        }
    }
}

impl<W> LogSink for JsonLogSink<W>
where
    W: Write + Send,
{
    fn write(&self, record: &LogRecord) -> Result<(), LoggingError> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| LoggingError::SinkUnavailable)?;
        serde_json::to_writer(&mut *writer, record)?;
        writeln!(&mut *writer)?;
        Ok(())
    }
}

/// In-memory log sink for tests and local validation.
#[derive(Debug, Default)]
pub struct MemoryLogSink {
    records: Mutex<Vec<LogRecord>>,
}

impl MemoryLogSink {
    /// Create an empty in-memory sink.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return a snapshot of stored records.
    ///
    /// # Errors
    ///
    /// Returns [`LoggingError::SinkUnavailable`] when the sink lock is poisoned.
    pub fn records(&self) -> Result<Vec<LogRecord>, LoggingError> {
        self.records
            .lock()
            .map_err(|_| LoggingError::SinkUnavailable)
            .map(|records| records.clone())
    }
}

impl LogSink for MemoryLogSink {
    fn write(&self, record: &LogRecord) -> Result<(), LoggingError> {
        self.records
            .lock()
            .map_err(|_| LoggingError::SinkUnavailable)?
            .push(record.clone());
        Ok(())
    }
}

/// Create a logger that writes JSON lines to standard output.
#[must_use]
pub fn stdout_logger(minimum_level: LogLevel) -> StructuredLogger {
    StructuredLogger::new(minimum_level, Arc::new(JsonLogSink::new(io::stdout())))
}

/// Logging subsystem errors.
#[derive(Debug, Error)]
pub enum LoggingError {
    /// Log level could not be parsed.
    #[error("invalid log level: {0}")]
    InvalidLevel(String),
    /// Sink lock or writer is unavailable.
    #[error("log sink is unavailable")]
    SinkUnavailable,
    /// JSON serialization failed.
    #[error("failed to serialize log record: {0}")]
    Serialize(#[from] serde_json::Error),
    /// Writer failed.
    #[error("failed to write log record: {0}")]
    Io(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::{LogLevel, LogRecord, MemoryLogSink, StructuredLogger};

    #[test]
    fn logger_records_structured_log() {
        let sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(LogLevel::Debug, sink.clone());

        logger
            .log(&LogRecord::new(LogLevel::Info, "test", "hello").with_metadata("phase", "1"))
            .expect("log record");

        let records = sink.records().expect("records");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].level(), LogLevel::Info);
        assert_eq!(records[0].target(), "test");
        assert_eq!(
            records[0].metadata().get("phase").map(String::as_str),
            Some("1")
        );
    }

    #[test]
    fn logger_filters_below_minimum_level() {
        let sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(LogLevel::Warning, sink.clone());

        logger
            .log(&LogRecord::new(LogLevel::Info, "test", "filtered"))
            .expect("filtered log");

        assert!(sink.records().expect("records").is_empty());
    }
}
