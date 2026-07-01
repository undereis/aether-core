#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Kernel orchestration layer for Aether.

use std::collections::BTreeMap;
use std::fmt;

use aether_config::{AetherConfig, ConfigError, ConfigProvider};
use aether_core::{
    AetherModule, Capability, LifecycleStatus, ModuleDescriptor, ModuleHealth, ModuleId,
};
use aether_events::EventBus;
use aether_ids::KernelId;
use aether_logging::{LogLevel, StructuredLogger};
use aether_runtime::{AetherRuntime, RuntimeError, RuntimeHealth};
use aether_service::{ServiceDescriptor, ServiceError, ServiceHealthAggregation, ServiceRegistry};
use aether_service_bus::{AetherServiceBus, InMemoryAetherServiceBus, ServiceBusError};
use aether_telemetry::{TelemetryEmitter, TelemetryError, TelemetryRecord};
use thiserror::Error;

/// Kernel telemetry target name.
pub const KERNEL_TARGET: &str = "aether-kernel";

/// Aether Kernel responsible for controlled orchestration above the runtime.
pub struct AetherKernel {
    id: KernelId,
    runtime: AetherRuntime,
    registry: ModuleRegistry,
    services: ServiceRegistry,
    service_bus: InMemoryAetherServiceBus,
    telemetry: TelemetryEmitter,
    status: LifecycleStatus,
}

impl fmt::Debug for AetherKernel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AetherKernel")
            .field("id", &self.id)
            .field("runtime", &self.runtime)
            .field("registry", &self.registry)
            .field("services", &self.services)
            .field("service_bus", &self.service_bus)
            .field("telemetry", &self.telemetry)
            .field("status", &self.status)
            .finish()
    }
}

impl AetherKernel {
    /// Create a kernel from explicit dependencies.
    #[must_use]
    pub fn new(
        id: KernelId,
        config: AetherConfig,
        event_bus: EventBus,
        logger: StructuredLogger,
        telemetry: TelemetryEmitter,
    ) -> Self {
        Self {
            id,
            runtime: AetherRuntime::new(config, event_bus, logger),
            registry: ModuleRegistry::new(),
            services: ServiceRegistry::new(),
            service_bus: InMemoryAetherServiceBus::new(),
            telemetry,
            status: LifecycleStatus::Created,
        }
    }

    /// Create a kernel by loading configuration through a provider.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when configuration loading fails.
    pub fn from_config_provider(
        provider: &dyn ConfigProvider,
        id: KernelId,
        event_bus: EventBus,
        logger: StructuredLogger,
        telemetry: TelemetryEmitter,
    ) -> Result<Self, KernelError> {
        let config = provider.load()?;
        Ok(Self::new(id, config, event_bus, logger, telemetry))
    }

    /// Start the kernel and underlying runtime.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when lifecycle transition, telemetry, or runtime startup fails.
    pub fn start(&mut self) -> Result<(), KernelError> {
        if self.status == LifecycleStatus::Running {
            return Ok(());
        }

        self.transition_kernel(LifecycleStatus::Initializing)?;
        self.emit_kernel_telemetry(LogLevel::Info, "kernel starting")?;
        self.runtime.start()?;
        self.status = LifecycleStatus::Running;
        self.emit_kernel_telemetry(LogLevel::Info, "kernel started")?;
        Ok(())
    }

    /// Shutdown the kernel and underlying runtime safely.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when lifecycle transition, telemetry, or runtime shutdown fails.
    pub fn shutdown(&mut self) -> Result<(), KernelError> {
        if self.status == LifecycleStatus::Stopped {
            return Ok(());
        }

        self.transition_kernel(LifecycleStatus::Stopping)?;
        self.registry.transition_active_modules_to_stopping()?;
        self.emit_kernel_telemetry(LogLevel::Info, "kernel stopping")?;
        self.runtime.stop()?;
        self.registry.transition_stopping_modules_to_stopped()?;
        self.status = LifecycleStatus::Stopped;
        self.emit_kernel_telemetry(LogLevel::Info, "kernel stopped")?;
        Ok(())
    }

    /// Register a module descriptor with the kernel registry.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when registry validation or telemetry emission fails.
    pub fn register_module(&mut self, descriptor: ModuleDescriptor) -> Result<(), KernelError> {
        let module_id = descriptor.id().clone();
        self.registry.register(descriptor)?;
        self.telemetry.emit(
            &TelemetryRecord::log(LogLevel::Info, KERNEL_TARGET, "module registered")
                .with_attribute("kernel_id", self.id.as_str())
                .with_attribute("module_id", module_id.to_string()),
        )?;
        Ok(())
    }

    /// Load a concrete module through the runtime and track its lifecycle in the registry.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when the kernel is not running, registry validation fails,
    /// or runtime module loading fails.
    pub fn load_module(&mut self, module: Box<dyn AetherModule>) -> Result<(), KernelError> {
        if self.status != LifecycleStatus::Running {
            return Err(KernelError::KernelNotRunning);
        }

        let descriptor = module.descriptor().clone();
        let module_id = descriptor.id().clone();
        if !self.registry.contains(&module_id) {
            self.register_module(descriptor)?;
        }

        self.registry
            .transition(&module_id, LifecycleStatus::Initializing)?;
        match self.runtime.load_module(module) {
            Ok(()) => {
                self.registry
                    .transition(&module_id, LifecycleStatus::Running)?;
                Ok(())
            }
            Err(error) => {
                self.registry
                    .force_status(&module_id, LifecycleStatus::Failed)?;
                Err(KernelError::Runtime(error))
            }
        }
    }

    /// Execute a kernel health check.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when the runtime health check fails.
    pub fn health(&self) -> Result<KernelHealth, KernelError> {
        let runtime = self.runtime.health_check()?;
        let modules = self.registry.health_reports();
        let healthy = runtime.healthy
            && modules
                .iter()
                .all(|report| report.health != ModuleHealth::Unhealthy);

        Ok(KernelHealth {
            kernel_id: self.id.clone(),
            status: self.status,
            runtime,
            modules,
            healthy,
        })
    }

    /// Return a snapshot of registered modules.
    #[must_use]
    pub fn modules(&self) -> Vec<ModuleRegistration> {
        self.registry.registrations()
    }

    /// Return a snapshot of registered module capabilities.
    #[must_use]
    pub fn capabilities(&self) -> Vec<CapabilityRegistration> {
        self.registry.capabilities()
    }

    /// Register a service descriptor with the Kernel-controlled service registry.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when service registration, bus permission registration,
    /// or telemetry emission fails.
    pub fn register_service(&mut self, descriptor: ServiceDescriptor) -> Result<(), KernelError> {
        let service_id = descriptor.id.clone();
        let permissions = descriptor.permissions.clone();
        self.services.register(descriptor)?;
        self.service_bus
            .register_permissions(&service_id, permissions)?;
        self.telemetry.emit(
            &TelemetryRecord::log(LogLevel::Info, KERNEL_TARGET, "service registered")
                .with_attribute("kernel_id", self.id.as_str())
                .with_attribute("service_id", service_id.to_string()),
        )?;
        Ok(())
    }

    /// Return registered services.
    #[must_use]
    pub fn services(&self) -> Vec<ServiceDescriptor> {
        self.services.services()
    }

    /// Return service platform health aggregation.
    #[must_use]
    pub fn service_health(&self) -> ServiceHealthAggregation {
        self.services.health_aggregation()
    }

    /// Return the service registry.
    #[must_use]
    pub const fn service_registry(&self) -> &ServiceRegistry {
        &self.services
    }

    /// Return the Aether Service Bus.
    #[must_use]
    pub const fn service_bus(&self) -> &InMemoryAetherServiceBus {
        &self.service_bus
    }

    /// Return the kernel identifier.
    #[must_use]
    pub const fn id(&self) -> &KernelId {
        &self.id
    }

    /// Return the current kernel lifecycle status.
    #[must_use]
    pub const fn status(&self) -> LifecycleStatus {
        self.status
    }

    /// Return whether the underlying runtime is running.
    #[must_use]
    pub const fn runtime_is_running(&self) -> bool {
        self.runtime.is_running()
    }

    fn transition_kernel(&mut self, next: LifecycleStatus) -> Result<(), KernelError> {
        if self.status == next {
            return Ok(());
        }
        if !self.status.can_transition_to(next) {
            return Err(KernelError::InvalidKernelTransition {
                from: self.status,
                to: next,
            });
        }
        self.status = next;
        Ok(())
    }

    fn emit_kernel_telemetry(&self, level: LogLevel, message: &str) -> Result<(), KernelError> {
        self.telemetry.emit(
            &TelemetryRecord::log(level, KERNEL_TARGET, message)
                .with_attribute("kernel_id", self.id.as_str())
                .with_attribute("kernel_status", self.status.as_str()),
        )?;
        Ok(())
    }
}

/// Registry for module descriptors and lifecycle state.
#[derive(Clone, Debug, Default)]
pub struct ModuleRegistry {
    modules: BTreeMap<ModuleId, ModuleRegistration>,
}

impl ModuleRegistry {
    /// Create an empty module registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            modules: BTreeMap::new(),
        }
    }

    /// Register a module descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the module already exists or dependencies are missing.
    pub fn register(&mut self, descriptor: ModuleDescriptor) -> Result<(), RegistryError> {
        let module_id = descriptor.id().clone();
        if self.modules.contains_key(&module_id) {
            return Err(RegistryError::DuplicateModule {
                module_id: module_id.to_string(),
            });
        }

        for dependency in descriptor.dependencies() {
            if !self.modules.contains_key(dependency) {
                return Err(RegistryError::MissingDependency {
                    module_id: module_id.to_string(),
                    dependency_id: dependency.to_string(),
                });
            }
        }

        let mut registration = ModuleRegistration::new(descriptor);
        registration.transition(LifecycleStatus::Registered)?;
        self.modules.insert(module_id, registration);
        Ok(())
    }

    /// Return whether a module is registered.
    #[must_use]
    pub fn contains(&self, module_id: &ModuleId) -> bool {
        self.modules.contains_key(module_id)
    }

    /// Transition a registered module to a new lifecycle state.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the module is unknown or the transition is invalid.
    pub fn transition(
        &mut self,
        module_id: &ModuleId,
        next: LifecycleStatus,
    ) -> Result<(), RegistryError> {
        let registration =
            self.modules
                .get_mut(module_id)
                .ok_or_else(|| RegistryError::UnknownModule {
                    module_id: module_id.to_string(),
                })?;
        registration.transition(next)
    }

    /// Force a lifecycle state after an external failure path.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the module is unknown.
    pub fn force_status(
        &mut self,
        module_id: &ModuleId,
        status: LifecycleStatus,
    ) -> Result<(), RegistryError> {
        let registration =
            self.modules
                .get_mut(module_id)
                .ok_or_else(|| RegistryError::UnknownModule {
                    module_id: module_id.to_string(),
                })?;
        registration.status = status;
        Ok(())
    }

    /// Return a snapshot of registrations.
    #[must_use]
    pub fn registrations(&self) -> Vec<ModuleRegistration> {
        self.modules.values().cloned().collect()
    }

    /// Return a snapshot of capability registrations.
    #[must_use]
    pub fn capabilities(&self) -> Vec<CapabilityRegistration> {
        self.modules
            .values()
            .flat_map(|registration| {
                registration
                    .descriptor()
                    .capabilities()
                    .iter()
                    .cloned()
                    .map(|capability| CapabilityRegistration {
                        module_id: registration.descriptor().id().clone(),
                        capability,
                    })
            })
            .collect()
    }

    fn health_reports(&self) -> Vec<ModuleHealthReport> {
        self.modules
            .values()
            .map(|registration| ModuleHealthReport {
                module_id: registration.descriptor().id().clone(),
                name: registration.descriptor().name().to_owned(),
                status: registration.status(),
                health: health_from_status(registration.status()),
            })
            .collect()
    }

    fn transition_active_modules_to_stopping(&mut self) -> Result<(), RegistryError> {
        let module_ids = self
            .modules
            .values()
            .filter(|registration| {
                matches!(
                    registration.status(),
                    LifecycleStatus::Running | LifecycleStatus::Degraded
                )
            })
            .map(|registration| registration.descriptor().id().clone())
            .collect::<Vec<_>>();

        for module_id in module_ids {
            self.transition(&module_id, LifecycleStatus::Stopping)?;
        }

        Ok(())
    }

    fn transition_stopping_modules_to_stopped(&mut self) -> Result<(), RegistryError> {
        let module_ids = self
            .modules
            .values()
            .filter(|registration| registration.status() == LifecycleStatus::Stopping)
            .map(|registration| registration.descriptor().id().clone())
            .collect::<Vec<_>>();

        for module_id in module_ids {
            self.transition(&module_id, LifecycleStatus::Stopped)?;
        }

        Ok(())
    }
}

/// Registered module metadata and lifecycle state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleRegistration {
    descriptor: ModuleDescriptor,
    status: LifecycleStatus,
}

impl ModuleRegistration {
    /// Create an unregistered module registration record.
    #[must_use]
    pub const fn new(descriptor: ModuleDescriptor) -> Self {
        Self {
            descriptor,
            status: LifecycleStatus::Created,
        }
    }

    /// Return the module descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &ModuleDescriptor {
        &self.descriptor
    }

    /// Return the lifecycle status.
    #[must_use]
    pub const fn status(&self) -> LifecycleStatus {
        self.status
    }

    /// Transition to the next lifecycle status.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the transition is invalid.
    pub fn transition(&mut self, next: LifecycleStatus) -> Result<(), RegistryError> {
        if self.status == next {
            return Ok(());
        }

        if !self.status.can_transition_to(next) {
            return Err(RegistryError::InvalidLifecycleTransition {
                module_id: self.descriptor.id().to_string(),
                from: self.status,
                to: next,
            });
        }

        self.status = next;
        Ok(())
    }
}

/// Capability exposed by a registered module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRegistration {
    /// Module that declares the capability.
    pub module_id: ModuleId,
    /// Declared capability.
    pub capability: Capability,
}

/// Module health report returned by the kernel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleHealthReport {
    /// Module identifier.
    pub module_id: ModuleId,
    /// Human-readable module name.
    pub name: String,
    /// Current lifecycle status.
    pub status: LifecycleStatus,
    /// Health derived from lifecycle status.
    pub health: ModuleHealth,
}

/// Kernel health report.
#[derive(Clone, Debug, PartialEq)]
pub struct KernelHealth {
    /// Kernel identifier.
    pub kernel_id: KernelId,
    /// Kernel lifecycle status.
    pub status: LifecycleStatus,
    /// Underlying runtime health.
    pub runtime: RuntimeHealth,
    /// Registered module health reports.
    pub modules: Vec<ModuleHealthReport>,
    /// Overall kernel health.
    pub healthy: bool,
}

/// Kernel errors.
#[derive(Debug, Error)]
pub enum KernelError {
    /// Configuration loading failed.
    #[error("kernel configuration failed: {0}")]
    Config(#[from] ConfigError),
    /// Module registry failed.
    #[error("module registry failed: {0}")]
    Registry(#[from] RegistryError),
    /// Runtime orchestration failed.
    #[error("runtime orchestration failed: {0}")]
    Runtime(#[from] RuntimeError),
    /// Telemetry emission failed.
    #[error("telemetry emission failed: {0}")]
    Telemetry(#[from] TelemetryError),
    /// Service registry failed.
    #[error("service registry failed: {0}")]
    Service(#[from] ServiceError),
    /// Service bus failed.
    #[error("service bus failed: {0}")]
    ServiceBus(#[from] ServiceBusError),
    /// Kernel cannot load modules until it is running.
    #[error("kernel is not running")]
    KernelNotRunning,
    /// Kernel lifecycle transition is invalid.
    #[error("invalid kernel lifecycle transition from {from} to {to}")]
    InvalidKernelTransition {
        /// Current lifecycle status.
        from: LifecycleStatus,
        /// Requested lifecycle status.
        to: LifecycleStatus,
    },
}

/// Module registry errors.
#[derive(Debug, Error)]
pub enum RegistryError {
    /// Module is already registered.
    #[error("module is already registered: {module_id}")]
    DuplicateModule {
        /// Module identifier.
        module_id: String,
    },
    /// Module dependency is not registered.
    #[error("module {module_id} depends on missing module {dependency_id}")]
    MissingDependency {
        /// Module identifier.
        module_id: String,
        /// Missing dependency identifier.
        dependency_id: String,
    },
    /// Module is not registered.
    #[error("module is not registered: {module_id}")]
    UnknownModule {
        /// Module identifier.
        module_id: String,
    },
    /// Lifecycle transition is invalid.
    #[error("invalid lifecycle transition for {module_id}: {from} to {to}")]
    InvalidLifecycleTransition {
        /// Module identifier.
        module_id: String,
        /// Current lifecycle status.
        from: LifecycleStatus,
        /// Requested lifecycle status.
        to: LifecycleStatus,
    },
}

fn health_from_status(status: LifecycleStatus) -> ModuleHealth {
    match status {
        LifecycleStatus::Running => ModuleHealth::Healthy,
        LifecycleStatus::Degraded | LifecycleStatus::Registered | LifecycleStatus::Initializing => {
            ModuleHealth::Degraded
        }
        LifecycleStatus::Created
        | LifecycleStatus::Stopping
        | LifecycleStatus::Stopped
        | LifecycleStatus::Failed => ModuleHealth::Unhealthy,
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use aether_config::{AetherConfig, StaticConfigProvider};
    use aether_core::{AetherModule, ModuleError};
    use aether_events::EventBus;
    use aether_ids::KernelId;
    use aether_logging::{MemoryLogSink, StructuredLogger};
    use aether_service::{ServiceDescriptor, ServiceHealthStatus, ServiceManifest};
    use aether_service_bus::AetherServiceBus;
    use aether_telemetry::{MemoryTelemetrySink, TelemetryEmitter};

    use super::{
        AetherKernel, Capability, LifecycleStatus, ModuleDescriptor, ModuleHealth, ModuleId,
        ModuleRegistry, RegistryError,
    };

    struct TestModule {
        descriptor: ModuleDescriptor,
        started: bool,
    }

    impl TestModule {
        fn new() -> Self {
            Self {
                descriptor: ModuleDescriptor::new("mod_test", "Test Module", "0.1.0")
                    .expect("descriptor")
                    .with_capability(Capability::new("health.report").expect("capability")),
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

    fn test_kernel() -> (AetherKernel, Arc<MemoryTelemetrySink>) {
        let config = AetherConfig::default();
        let event_bus = EventBus::new();
        let log_sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(config.runtime.log_level, log_sink);
        let telemetry_sink = Arc::new(MemoryTelemetrySink::new());
        let telemetry = TelemetryEmitter::new(config.runtime.log_level, telemetry_sink.clone());
        let kernel = AetherKernel::new(KernelId::generate(), config, event_bus, logger, telemetry);

        (kernel, telemetry_sink)
    }

    fn test_service_descriptor() -> ServiceDescriptor {
        let manifest = ServiceManifest::from_toml_str(
            r#"
            [service]
            name = "telemetry-service"
            version = "0.1.0"
            description = "Base telemetry service"
            owner = "neuroforge-labs"

            [capabilities]
            provides = ["telemetry.emit"]
            requires = ["events.publish"]

            [permissions]
            requested = ["event.publish", "event.subscribe"]

            [resources]
            cpu_class = "low"
            memory_class = "low"
            storage_class = "none"
            network = false
            "#,
        )
        .expect("manifest");

        ServiceDescriptor::from_manifest(manifest)
            .expect("descriptor")
            .with_lifecycle_status(LifecycleStatus::Running)
            .with_health_status(ServiceHealthStatus::Healthy)
    }

    #[test]
    fn registry_registers_module() {
        let mut registry = ModuleRegistry::new();
        let descriptor =
            ModuleDescriptor::new("mod_events", "Events", "0.1.0").expect("descriptor");

        registry.register(descriptor).expect("register");

        assert_eq!(registry.registrations().len(), 1);
        assert_eq!(
            registry.registrations()[0].status(),
            LifecycleStatus::Registered
        );
    }

    #[test]
    fn registry_validates_dependencies() {
        let mut registry = ModuleRegistry::new();
        let dependency = ModuleId::new("mod_missing").expect("module id");
        let descriptor = ModuleDescriptor::new("mod_worker", "Worker", "0.1.0")
            .expect("descriptor")
            .with_dependency(dependency);

        let result = registry.register(descriptor);

        assert!(matches!(
            result,
            Err(RegistryError::MissingDependency { .. })
        ));
    }

    #[test]
    fn registry_discovers_capabilities() {
        let mut registry = ModuleRegistry::new();
        let descriptor = ModuleDescriptor::new("mod_events", "Events", "0.1.0")
            .expect("descriptor")
            .with_capability(Capability::new("events.publish").expect("capability"))
            .with_capability(Capability::new("events.subscribe").expect("capability"));

        registry.register(descriptor).expect("register");

        let capabilities = registry.capabilities();
        assert_eq!(capabilities.len(), 2);
        assert_eq!(capabilities[0].module_id.as_str(), "mod_events");
    }

    #[test]
    fn registry_transitions_lifecycle() {
        let mut registry = ModuleRegistry::new();
        let module_id = ModuleId::new("mod_events").expect("module id");
        let descriptor =
            ModuleDescriptor::new("mod_events", "Events", "0.1.0").expect("descriptor");

        registry.register(descriptor).expect("register");
        registry
            .transition(&module_id, LifecycleStatus::Initializing)
            .expect("initializing");
        registry
            .transition(&module_id, LifecycleStatus::Running)
            .expect("running");

        assert_eq!(
            registry.registrations()[0].status(),
            LifecycleStatus::Running
        );
    }

    #[test]
    fn kernel_bootstraps_from_config_provider() {
        let config = AetherConfig::default();
        let provider = StaticConfigProvider::new(config.clone());
        let event_bus = EventBus::new();
        let log_sink = Arc::new(MemoryLogSink::new());
        let logger = StructuredLogger::new(config.runtime.log_level, log_sink);
        let telemetry_sink = Arc::new(MemoryTelemetrySink::new());
        let telemetry = TelemetryEmitter::new(config.runtime.log_level, telemetry_sink);

        let kernel = AetherKernel::from_config_provider(
            &provider,
            KernelId::generate(),
            event_bus,
            logger,
            telemetry,
        )
        .expect("kernel");

        assert_eq!(kernel.status(), LifecycleStatus::Created);
    }

    #[test]
    fn kernel_loads_module_and_reports_health() {
        let (mut kernel, _telemetry_sink) = test_kernel();

        kernel.start().expect("start kernel");
        kernel
            .load_module(Box::new(TestModule::new()))
            .expect("load module");
        let health = kernel.health().expect("health");

        assert!(health.healthy);
        assert_eq!(health.modules.len(), 1);
        assert_eq!(health.modules[0].status, LifecycleStatus::Running);
        assert_eq!(kernel.capabilities().len(), 1);
    }

    #[test]
    fn kernel_shutdown_stops_runtime_and_modules() {
        let (mut kernel, telemetry_sink) = test_kernel();

        kernel.start().expect("start kernel");
        kernel
            .load_module(Box::new(TestModule::new()))
            .expect("load module");
        kernel.shutdown().expect("shutdown kernel");

        assert_eq!(kernel.status(), LifecycleStatus::Stopped);
        assert!(!kernel.runtime_is_running());
        assert_eq!(kernel.modules()[0].status(), LifecycleStatus::Stopped);
        assert!(!telemetry_sink.records().expect("records").is_empty());
    }

    #[test]
    fn kernel_registers_service_platform_descriptor() {
        let (mut kernel, _telemetry_sink) = test_kernel();

        kernel
            .register_service(test_service_descriptor())
            .expect("register service");

        assert_eq!(kernel.services().len(), 1);
        assert_eq!(
            kernel
                .service_registry()
                .providers_for_capability_name("telemetry.emit")
                .expect("providers")
                .len(),
            1
        );
        assert_eq!(
            kernel
                .service_bus()
                .status()
                .expect("bus status")
                .permission_entries,
            1
        );
    }

    #[test]
    fn kernel_exposes_service_health_aggregation() {
        let (mut kernel, _telemetry_sink) = test_kernel();

        kernel
            .register_service(test_service_descriptor())
            .expect("register service");

        let health = kernel.service_health();

        assert_eq!(health.total_services, 1);
        assert_eq!(health.running_services, 1);
        assert_eq!(health.capabilities_available, ["telemetry.emit"]);
    }
}
