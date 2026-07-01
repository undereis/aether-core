#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Runtime bootstrap and module orchestration for Aether.

use std::fmt;

use aether_config::AetherConfig;
use aether_core::{AetherModule, ModuleHealth};
use aether_events::{Event, EventBus, EventError, EventSource, EventType};
use aether_logging::{LogLevel, LogRecord, LoggingError, StructuredLogger};
use serde_json::json;
use thiserror::Error;

/// Runtime source name used for internal events.
pub const RUNTIME_SOURCE: &str = "aether-runtime";

/// Aether runtime bootstrap container.
pub struct AetherRuntime {
    config: AetherConfig,
    event_bus: EventBus,
    logger: StructuredLogger,
    modules: Vec<Box<dyn AetherModule>>,
    running: bool,
}

impl fmt::Debug for AetherRuntime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AetherRuntime")
            .field("config", &self.config)
            .field("event_bus", &self.event_bus)
            .field("logger", &self.logger)
            .field("module_count", &self.modules.len())
            .field("running", &self.running)
            .finish()
    }
}

impl AetherRuntime {
    /// Create a runtime from explicit dependencies.
    #[must_use]
    pub fn new(config: AetherConfig, event_bus: EventBus, logger: StructuredLogger) -> Self {
        Self {
            config,
            event_bus,
            logger,
            modules: Vec::new(),
            running: false,
        }
    }

    /// Start the runtime and emit foundational lifecycle events.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError`] when logging or event publication fails.
    pub fn start(&mut self) -> Result<(), RuntimeError> {
        self.emit(EventType::ConfigLoaded)?;
        self.emit(EventType::SystemStarted)?;
        self.logger
            .log(&LogRecord::new(
                LogLevel::Info,
                RUNTIME_SOURCE,
                "runtime started",
            ))
            .map_err(RuntimeError::Logging)?;
        self.running = true;
        Ok(())
    }

    /// Stop the runtime.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError`] when a module stop operation, logging, or event publication fails.
    pub fn stop(&mut self) -> Result<(), RuntimeError> {
        for module in &mut self.modules {
            module.stop()?;
        }
        self.emit(EventType::SystemStopped)?;
        self.logger
            .info(RUNTIME_SOURCE, "runtime stopped")
            .map_err(RuntimeError::Logging)?;
        self.running = false;
        Ok(())
    }

    /// Load and start a module.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError`] when module startup, logging, or event publication fails.
    pub fn load_module(&mut self, mut module: Box<dyn AetherModule>) -> Result<(), RuntimeError> {
        let module_id = module.descriptor().id().to_string();
        if let Err(error) = module.start() {
            self.emit(
                Event::new(runtime_source()?, EventType::ModuleFailed)
                    .with_metadata("module_id", module_id)
                    .with_metadata("reason", error.to_string()),
            )?;
            return Err(RuntimeError::Module(error));
        }
        self.emit(
            Event::new(runtime_source()?, EventType::ModuleLoaded)
                .with_metadata("module_id", module_id.clone()),
        )?;
        self.logger
            .log(
                &LogRecord::new(LogLevel::Info, RUNTIME_SOURCE, "module loaded")
                    .with_metadata("module_id", module_id),
            )
            .map_err(RuntimeError::Logging)?;
        self.modules.push(module);
        Ok(())
    }

    /// Execute a runtime health check.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError`] when event publication or module health checks fail.
    pub fn health_check(&self) -> Result<RuntimeHealth, RuntimeError> {
        self.emit(EventType::HealthCheckRequested)?;
        let module_count = self.modules.len();

        for module in &self.modules {
            if module.health()? == ModuleHealth::Unhealthy {
                self.emit(
                    Event::new(runtime_source()?, EventType::HealthCheckCompleted)
                        .with_payload_value("healthy", json!(false))
                        .with_payload_value("modules", json!(module_count)),
                )?;
                return Ok(RuntimeHealth {
                    running: self.running,
                    module_count,
                    healthy: false,
                });
            }
        }

        self.emit(
            Event::new(runtime_source()?, EventType::HealthCheckCompleted)
                .with_payload_value("healthy", json!(true))
                .with_payload_value("modules", json!(module_count)),
        )?;
        Ok(RuntimeHealth {
            running: self.running,
            module_count,
            healthy: true,
        })
    }

    /// Return the runtime event bus.
    #[must_use]
    pub const fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Return whether the runtime is running.
    #[must_use]
    pub const fn is_running(&self) -> bool {
        self.running
    }

    fn emit(&self, event: impl IntoRuntimeEvent) -> Result<(), RuntimeError> {
        self.event_bus.publish(&event.into_event()?)?;
        Ok(())
    }
}

/// Runtime health response.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeHealth {
    /// Whether the runtime has started.
    pub running: bool,
    /// Number of loaded modules.
    pub module_count: usize,
    /// Whether all modules are healthy.
    pub healthy: bool,
}

trait IntoRuntimeEvent {
    fn into_event(self) -> Result<Event, RuntimeError>;
}

impl IntoRuntimeEvent for Event {
    fn into_event(self) -> Result<Event, RuntimeError> {
        Ok(self)
    }
}

impl IntoRuntimeEvent for EventType {
    fn into_event(self) -> Result<Event, RuntimeError> {
        Ok(Event::new(runtime_source()?, self))
    }
}

fn runtime_source() -> Result<EventSource, RuntimeError> {
    EventSource::new(RUNTIME_SOURCE).map_err(RuntimeError::Events)
}

/// Runtime errors.
#[derive(Debug, Error)]
pub enum RuntimeError {
    /// Event subsystem failed.
    #[error("event subsystem failed: {0}")]
    Events(#[from] EventError),
    /// Logging subsystem failed.
    #[error("logging subsystem failed: {0}")]
    Logging(#[from] LoggingError),
    /// Module contract failed.
    #[error("module failed: {0}")]
    Module(#[from] aether_core::ModuleError),
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use aether_config::AetherConfig;
    use aether_core::{AetherModule, ModuleDescriptor, ModuleError, ModuleHealth};
    use aether_events::{EventBus, EventType};
    use aether_logging::{MemoryLogSink, StructuredLogger};

    use super::AetherRuntime;

    struct TestModule {
        descriptor: ModuleDescriptor,
        started: bool,
    }

    impl TestModule {
        fn new() -> Self {
            Self {
                descriptor: ModuleDescriptor::new("test-module", "Test Module", "0.1.0")
                    .expect("descriptor"),
                started: false,
            }
        }
    }

    impl AetherModule for TestModule {
        fn descriptor(&self) -> &ModuleDescriptor {
            &self.descriptor
        }

        fn start(&mut self) -> Result<(), ModuleError> {
            self.started = true;
            Ok(())
        }

        fn stop(&mut self) -> Result<(), ModuleError> {
            self.started = false;
            Ok(())
        }

        fn health(&self) -> Result<ModuleHealth, ModuleError> {
            Ok(if self.started {
                ModuleHealth::Healthy
            } else {
                ModuleHealth::Unhealthy
            })
        }
    }

    fn test_runtime() -> (AetherRuntime, Arc<MemoryLogSink>) {
        let config = AetherConfig::default();
        let bus = EventBus::new();
        let sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(config.runtime.log_level, sink.clone());

        (AetherRuntime::new(config, bus, logger), sink)
    }

    #[test]
    fn runtime_starts_and_emits_events() {
        let (mut runtime, sink) = test_runtime();
        let receiver = runtime.event_bus().subscribe().expect("subscriber");

        runtime.start().expect("start runtime");

        let first = receiver
            .recv_timeout(Duration::from_millis(100))
            .expect("config event");
        let second = receiver
            .recv_timeout(Duration::from_millis(100))
            .expect("start event");

        assert!(runtime.is_running());
        assert_eq!(first.event_type(), EventType::ConfigLoaded);
        assert_eq!(second.event_type(), EventType::SystemStarted);
        assert_eq!(sink.records().expect("records").len(), 1);
    }

    #[test]
    fn runtime_loads_module_and_reports_health() {
        let (mut runtime, _sink) = test_runtime();
        let receiver = runtime.event_bus().subscribe().expect("subscriber");

        runtime.start().expect("start runtime");
        runtime
            .load_module(Box::new(TestModule::new()))
            .expect("load module");
        let health = runtime.health_check().expect("health");

        assert_eq!(health.module_count, 1);
        assert!(health.healthy);
        assert!(receiver.recv_timeout(Duration::from_millis(100)).is_ok());
    }
}
