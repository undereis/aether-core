#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Manager layer infrastructure for Aether Kernel decomposition.

use std::collections::BTreeMap;

use aether_core::{
    AetherModule, Capability, LifecycleStatus, ModuleDescriptor, ModuleHealth, ModuleId,
};
use aether_ids::{ManagerId, ServiceId};
use aether_permissions::PermissionSet;
use aether_service::{ServiceDescriptor, ServiceError, ServiceHealthAggregation, ServiceRegistry};
use aether_service_bus::{
    InMemoryAetherServiceBus, RegisterServiceError, ServiceBusError, register_service_with_bus,
};
use thiserror::Error;

/// Manager layer target name.
pub const MANAGER_LAYER_TARGET: &str = "aether-managers";

/// Health status reported by an Aether manager.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ManagerHealth {
    /// Manager is available and healthy.
    Healthy,
    /// Manager is partially available.
    Degraded,
    /// Manager is unavailable.
    Unhealthy,
    /// Manager has not been checked yet.
    Unknown,
}

impl ManagerHealth {
    /// Return the canonical manager health value.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
            Self::Unknown => "unknown",
        }
    }
}

/// Declarative manager manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerManifest {
    /// Manager name.
    pub name: String,
    /// Manager version.
    pub version: String,
    /// Permanent responsibilities owned by this manager.
    pub responsibilities: Vec<String>,
}

impl ManagerManifest {
    /// Create a manager manifest.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when required fields are empty.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        responsibilities: Vec<String>,
    ) -> Result<Self, ManagerError> {
        let manifest = Self {
            name: name.into(),
            version: version.into(),
            responsibilities,
        };
        validate_required("manager manifest name", &manifest.name)?;
        validate_required("manager manifest version", &manifest.version)?;
        Ok(manifest)
    }
}

/// Manager descriptor registered with the Kernel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerDescriptor {
    /// Typed manager identifier.
    pub id: ManagerId,
    /// Human-readable manager name.
    pub name: String,
    /// Manager implementation version.
    pub version: String,
    /// Current manager health.
    pub health: ManagerHealth,
    /// Capabilities provided by the manager.
    pub capabilities: Vec<Capability>,
    /// Source manager manifest.
    pub manifest: ManagerManifest,
}

impl ManagerDescriptor {
    /// Create a manager descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when required fields are invalid.
    pub fn new(
        id: ManagerId,
        name: impl Into<String>,
        version: impl Into<String>,
        capabilities: Vec<Capability>,
        manifest: ManagerManifest,
    ) -> Result<Self, ManagerError> {
        let descriptor = Self {
            id,
            name: name.into(),
            version: version.into(),
            health: ManagerHealth::Healthy,
            capabilities: unique_capabilities(capabilities),
            manifest,
        };
        descriptor.validate()?;
        Ok(descriptor)
    }

    /// Return a copy with updated health.
    #[must_use]
    pub const fn with_health(mut self, health: ManagerHealth) -> Self {
        self.health = health;
        self
    }

    /// Validate the descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when required fields are empty.
    pub fn validate(&self) -> Result<(), ManagerError> {
        validate_required("manager name", &self.name)?;
        validate_required("manager version", &self.version)?;
        validate_required("manager manifest name", &self.manifest.name)?;
        Ok(())
    }
}

/// Contract implemented by Aether managers.
pub trait Manager {
    /// Return the manager descriptor.
    fn descriptor(&self) -> &ManagerDescriptor;

    /// Return current manager health.
    fn health(&self) -> ManagerHealth {
        self.descriptor().health
    }
}

/// Registry for manager descriptors.
#[derive(Clone, Debug, Default)]
pub struct ManagerRegistry {
    managers: BTreeMap<ManagerId, ManagerDescriptor>,
}

impl ManagerRegistry {
    /// Create an empty manager registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            managers: BTreeMap::new(),
        }
    }

    /// Create a registry with the built-in platform managers.
    #[must_use]
    pub fn with_default_managers() -> Self {
        let mut registry = Self::new();
        for descriptor in default_manager_descriptors() {
            let _ = registry.register(descriptor);
        }
        registry
    }

    /// Register a manager descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when the manager is invalid or duplicated.
    pub fn register(&mut self, descriptor: ManagerDescriptor) -> Result<(), ManagerError> {
        descriptor.validate()?;
        if self.managers.contains_key(&descriptor.id) {
            return Err(ManagerError::DuplicateManager(descriptor.id.to_string()));
        }
        self.managers.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    /// Return a manager by identifier.
    #[must_use]
    pub fn get(&self, manager_id: &ManagerId) -> Option<&ManagerDescriptor> {
        self.managers.get(manager_id)
    }

    /// Return registered manager descriptors.
    #[must_use]
    pub fn managers(&self) -> Vec<ManagerDescriptor> {
        self.managers.values().cloned().collect()
    }

    /// Return registered manager count.
    #[must_use]
    pub fn len(&self) -> usize {
        self.managers.len()
    }

    /// Return whether no managers are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.managers.is_empty()
    }
}

/// Manager responsible for service registration, discovery, inspection, and health.
#[derive(Clone, Debug)]
pub struct ServiceManager {
    descriptor: ManagerDescriptor,
    registry: ServiceRegistry,
    service_bus: InMemoryAetherServiceBus,
}

impl ServiceManager {
    /// Create an empty service manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            descriptor: manager_descriptor(
                "service-manager",
                "ServiceManager",
                &[
                    "service.register",
                    "service.discovery",
                    "service.inspect",
                    "service.health",
                    "service.version",
                ],
                &[
                    "Register service descriptors",
                    "Discover services by capability",
                    "Inspect service descriptors",
                    "Aggregate service health",
                    "Expose service manager version",
                ],
            ),
            registry: ServiceRegistry::new(),
            service_bus: InMemoryAetherServiceBus::new(),
        }
    }

    /// Register a service descriptor and index its permissions with the ASB.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when registration or permission indexing fails.
    pub fn register_service(&mut self, descriptor: ServiceDescriptor) -> Result<(), ManagerError> {
        register_service_with_bus(&mut self.registry, &self.service_bus, descriptor)?;
        Ok(())
    }

    /// Return registered services.
    #[must_use]
    pub fn services(&self) -> Vec<ServiceDescriptor> {
        self.registry.services()
    }

    /// Return a service descriptor by identifier.
    #[must_use]
    pub fn inspect(&self, service_id: &ServiceId) -> Option<&ServiceDescriptor> {
        self.registry.get(service_id)
    }

    /// Return services that provide a capability.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when the capability name is invalid.
    pub fn providers_for_capability(
        &self,
        capability_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, ManagerError> {
        Ok(self
            .registry
            .providers_for_capability_name(capability_name)?)
    }

    /// Return services that require a capability.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when the capability name is invalid.
    pub fn dependents_on_capability(
        &self,
        capability_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, ManagerError> {
        Ok(self
            .registry
            .dependents_on_capability_name(capability_name)?)
    }

    /// Return requested permissions for a service.
    #[must_use]
    pub fn permissions_for_service(&self, service_id: &ServiceId) -> Option<&PermissionSet> {
        self.registry.permissions_for_service(service_id)
    }

    /// Return service platform health aggregation.
    #[must_use]
    pub fn health_aggregation(&self) -> ServiceHealthAggregation {
        self.registry.health_aggregation()
    }

    /// Return the service manager version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.descriptor.version
    }

    /// Validate that a service could be reloaded by the manager.
    ///
    /// This phase only establishes the contract; future phases can bind this to concrete reload
    /// behavior without changing Kernel ownership.
    ///
    /// # Errors
    ///
    /// Returns [`ManagerError`] when the service is unknown.
    pub fn reload(&self, service_id: &ServiceId) -> Result<(), ManagerError> {
        if self.registry.get(service_id).is_some() {
            Ok(())
        } else {
            Err(ManagerError::UnknownService(service_id.to_string()))
        }
    }

    /// Return the internal service registry.
    #[must_use]
    pub const fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// Return the Aether Service Bus.
    #[must_use]
    pub const fn service_bus(&self) -> &InMemoryAetherServiceBus {
        &self.service_bus
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager for ServiceManager {
    fn descriptor(&self) -> &ManagerDescriptor {
        &self.descriptor
    }
}

/// Manager responsible for module lifecycle state and module capability indexes.
#[derive(Clone, Debug)]
pub struct LifecycleManager {
    descriptor: ManagerDescriptor,
    registry: ModuleRegistry,
}

impl LifecycleManager {
    /// Create an empty lifecycle manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            descriptor: manager_descriptor(
                "lifecycle-manager",
                "LifecycleManager",
                &[
                    "lifecycle.register",
                    "lifecycle.transition",
                    "lifecycle.health",
                    "module.capabilities",
                ],
                &[
                    "Register module descriptors",
                    "Control module lifecycle transitions",
                    "Expose module health snapshots",
                    "Index module capabilities",
                ],
            ),
            registry: ModuleRegistry::new(),
        }
    }

    /// Register a module descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the module is invalid, duplicated, or missing dependencies.
    pub fn register(&mut self, descriptor: ModuleDescriptor) -> Result<(), RegistryError> {
        self.registry.register(descriptor)
    }

    /// Return whether a module is registered.
    #[must_use]
    pub fn contains(&self, module_id: &ModuleId) -> bool {
        self.registry.contains(module_id)
    }

    /// Transition a module lifecycle state.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the module is unknown or transition is invalid.
    pub fn transition(
        &mut self,
        module_id: &ModuleId,
        next: LifecycleStatus,
    ) -> Result<(), RegistryError> {
        self.registry.transition(module_id, next)
    }

    /// Force a module lifecycle state.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when the module is unknown.
    pub fn force_status(
        &mut self,
        module_id: &ModuleId,
        status: LifecycleStatus,
    ) -> Result<(), RegistryError> {
        self.registry.force_status(module_id, status)
    }

    /// Return module registration snapshots.
    #[must_use]
    pub fn registrations(&self) -> Vec<ModuleRegistration> {
        self.registry.registrations()
    }

    /// Return module capability snapshots.
    #[must_use]
    pub fn capabilities(&self) -> Vec<CapabilityRegistration> {
        self.registry.capabilities()
    }

    /// Return module health reports.
    #[must_use]
    pub fn health_reports(&self) -> Vec<ModuleHealthReport> {
        self.registry.health_reports()
    }

    /// Transition all active modules to stopping.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when any lifecycle transition is invalid.
    pub fn transition_active_modules_to_stopping(&mut self) -> Result<(), RegistryError> {
        self.registry.transition_active_modules_to_stopping()
    }

    /// Transition all stopping modules to stopped.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when any lifecycle transition is invalid.
    pub fn transition_stopping_modules_to_stopped(&mut self) -> Result<(), RegistryError> {
        self.registry.transition_stopping_modules_to_stopped()
    }

    /// Load a concrete module through a caller-provided runtime operation.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when lifecycle state cannot be updated or the loader fails.
    pub fn load_with<F, E>(&mut self, module: Box<dyn AetherModule>, mut load: F) -> Result<(), E>
    where
        F: FnMut(Box<dyn AetherModule>) -> Result<(), E>,
        E: From<RegistryError>,
    {
        let descriptor = module.descriptor().clone();
        let module_id = descriptor.id().clone();
        if !self.contains(&module_id) {
            self.register(descriptor)?;
        }

        self.transition(&module_id, LifecycleStatus::Initializing)?;
        match load(module) {
            Ok(()) => {
                self.transition(&module_id, LifecycleStatus::Running)?;
                Ok(())
            }
            Err(error) => {
                self.force_status(&module_id, LifecycleStatus::Failed)?;
                Err(error)
            }
        }
    }
}

impl Default for LifecycleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Manager for LifecycleManager {
    fn descriptor(&self) -> &ManagerDescriptor {
        &self.descriptor
    }
}

macro_rules! define_basic_manager {
    (
        $(#[$doc:meta]
        $name:ident, $suffix:literal, $display:literal, [$($capability:literal),+], [$($responsibility:literal),+];)+
    ) => {
        $(
            #[$doc]
            #[derive(Clone, Debug)]
            pub struct $name {
                descriptor: ManagerDescriptor,
            }

            impl Default for $name {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl $name {
                /// Create the manager infrastructure descriptor.
                #[must_use]
                pub fn new() -> Self {
                    Self {
                        descriptor: manager_descriptor(
                            $suffix,
                            $display,
                            &[$($capability),+],
                            &[$($responsibility),+],
                        ),
                    }
                }

                /// Return the manager descriptor.
                #[must_use]
                pub const fn descriptor(&self) -> &ManagerDescriptor {
                    &self.descriptor
                }
            }

            impl Manager for $name {
                fn descriptor(&self) -> &ManagerDescriptor {
                    &self.descriptor
                }
            }
        )+
    };
}

define_basic_manager! {
    /// Manager responsible for resource declaration and future enforcement orchestration.
    ResourceManager, "resource-manager", "ResourceManager",
    ["resource.declare", "resource.inspect", "resource.enforce"],
    ["Inspect resource declarations", "Prepare resource enforcement", "Expose resource manager health"];

    /// Manager responsible for telemetry coordination.
    TelemetryManager, "telemetry-manager", "TelemetryManager",
    ["telemetry.emit", "telemetry.route", "telemetry.health"],
    ["Coordinate telemetry emission", "Prepare metrics and tracing", "Expose telemetry health"];

    /// Manager responsible for configuration provider coordination.
    ConfigurationManager, "configuration-manager", "ConfigurationManager",
    ["config.read", "config.validate", "config.provider"],
    ["Coordinate configuration providers", "Validate configuration contracts", "Expose configuration health"];

    /// Manager responsible for permission declaration and future enforcement.
    PermissionManager, "permission-manager", "PermissionManager",
    ["permission.declare", "permission.check", "permission.audit"],
    ["Inspect permission declarations", "Prepare permission checks", "Expose permission health"];

    /// Manager responsible for health aggregation coordination.
    HealthManager, "health-manager", "HealthManager",
    ["health.report", "health.aggregate", "health.degrade"],
    ["Aggregate health reports", "Prepare controlled degradation", "Expose health manager status"];

    /// Manager responsible for future driver registration and supervision.
    DriverManager, "driver-manager", "DriverManager",
    ["driver.register", "driver.inspect", "driver.health"],
    ["Register driver descriptors", "Inspect driver capabilities", "Expose driver health"];

    /// Manager responsible for future plugin registration and supervision.
    PluginManager, "plugin-manager", "PluginManager",
    ["plugin.register", "plugin.inspect", "plugin.health"],
    ["Register plugin descriptors", "Inspect plugin capabilities", "Expose plugin health"];
}

/// Return built-in manager descriptors.
#[must_use]
pub fn default_manager_descriptors() -> Vec<ManagerDescriptor> {
    vec![
        ServiceManager::new().descriptor().clone(),
        LifecycleManager::new().descriptor().clone(),
        ResourceManager::new().descriptor().clone(),
        TelemetryManager::new().descriptor().clone(),
        ConfigurationManager::new().descriptor().clone(),
        PermissionManager::new().descriptor().clone(),
        HealthManager::new().descriptor().clone(),
        DriverManager::new().descriptor().clone(),
        PluginManager::new().descriptor().clone(),
    ]
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

    /// Return module health reports.
    #[must_use]
    pub fn health_reports(&self) -> Vec<ModuleHealthReport> {
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

    /// Transition all active modules to stopping.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when any transition is invalid.
    pub fn transition_active_modules_to_stopping(&mut self) -> Result<(), RegistryError> {
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

    /// Transition all stopping modules to stopped.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] when any transition is invalid.
    pub fn transition_stopping_modules_to_stopped(&mut self) -> Result<(), RegistryError> {
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

/// Module health report returned by lifecycle management.
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

/// Manager layer errors.
#[derive(Debug, Error)]
pub enum ManagerError {
    /// Manager descriptor or manifest is invalid.
    #[error("invalid manager field: {0}")]
    InvalidField(String),
    /// Manager identifier is invalid.
    #[error("invalid manager id: {0}")]
    Id(#[from] aether_ids::IdError),
    /// Manager capability declaration is invalid.
    #[error("invalid manager capability: {0}")]
    Capability(#[from] aether_core::ModuleError),
    /// Manager is already registered.
    #[error("manager already registered: {0}")]
    DuplicateManager(String),
    /// Manager is not registered.
    #[error("unknown manager: {0}")]
    UnknownManager(String),
    /// Module lifecycle registry failed.
    #[error("module lifecycle failed: {0}")]
    Registry(#[from] RegistryError),
    /// Service operation failed.
    #[error("service manager operation failed: {0}")]
    Service(#[from] ServiceError),
    /// Service Bus operation failed.
    #[error("service bus operation failed: {0}")]
    ServiceBus(#[from] ServiceBusError),
    /// Service registration with bus failed.
    #[error("service registration failed: {0}")]
    RegisterService(#[from] RegisterServiceError),
    /// Service is unknown.
    #[error("unknown service: {0}")]
    UnknownService(String),
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

fn manager_descriptor(
    suffix: &str,
    name: &str,
    capability_names: &[&str],
    responsibilities: &[&str],
) -> ManagerDescriptor {
    let name = name.to_owned();
    let version = env!("CARGO_PKG_VERSION").to_owned();
    let manifest = ManagerManifest {
        name: name.clone(),
        version: version.clone(),
        responsibilities: responsibilities
            .iter()
            .map(|responsibility| (*responsibility).to_owned())
            .collect(),
    };
    ManagerDescriptor {
        id: static_manager_id(suffix),
        name,
        version,
        health: ManagerHealth::Healthy,
        capabilities: capabilities_from_names(capability_names),
        manifest,
    }
}

fn capabilities_from_names(names: &[&str]) -> Vec<Capability> {
    let mut capabilities = Vec::with_capacity(names.len());
    for name in names {
        if let Ok(capability) = Capability::new(*name) {
            capabilities.push(capability);
        }
    }
    unique_capabilities(capabilities)
}

fn static_manager_id(suffix: &str) -> ManagerId {
    match ManagerId::new(suffix) {
        Ok(id) => id,
        Err(_) => ManagerId::generate(),
    }
}

fn unique_capabilities(capabilities: Vec<Capability>) -> Vec<Capability> {
    let mut unique = Vec::new();
    for capability in capabilities {
        if !unique.contains(&capability) {
            unique.push(capability);
        }
    }
    unique
}

fn validate_required(field: &str, value: &str) -> Result<(), ManagerError> {
    if value.trim().is_empty() {
        Err(ManagerError::InvalidField(format!(
            "{field} cannot be empty"
        )))
    } else {
        Ok(())
    }
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
    use aether_core::{Capability, LifecycleStatus, ModuleDescriptor, ModuleId};
    use aether_resources::{CpuClass, StorageClass};
    use aether_service::{ServiceDescriptor, ServiceHealthStatus, ServiceManifest};

    use super::{
        LifecycleManager, ManagerHealth, ManagerRegistry, ModuleRegistry, RegistryError,
        ServiceManager, default_manager_descriptors,
    };

    fn service_descriptor() -> ServiceDescriptor {
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
            requested = ["event.publish", "event.subscribe", "service.command"]

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

    fn module_descriptor() -> ModuleDescriptor {
        ModuleDescriptor::new("test-module", "Test Module", "0.1.0")
            .expect("module descriptor")
            .with_capability(Capability::new("events.publish").expect("capability"))
    }

    #[test]
    fn managers_have_required_identity_and_capabilities() {
        let descriptors = default_manager_descriptors();

        assert_eq!(descriptors.len(), 9);
        assert!(
            descriptors
                .iter()
                .all(|descriptor| descriptor.id.as_str().starts_with("mgr_"))
        );
        assert!(
            descriptors
                .iter()
                .all(|descriptor| !descriptor.capabilities.is_empty())
        );
    }

    #[test]
    fn manager_registry_registers_default_managers() {
        let registry = ManagerRegistry::with_default_managers();

        assert_eq!(registry.len(), 9);
        assert!(
            registry
                .managers()
                .iter()
                .all(|manager| manager.health == ManagerHealth::Healthy)
        );
    }

    #[test]
    fn service_manager_registers_and_discovers_services() {
        let mut manager = ServiceManager::new();
        let descriptor = service_descriptor();
        let service_id = descriptor.id.clone();

        manager.register_service(descriptor).expect("register");

        assert_eq!(manager.services().len(), 1);
        assert!(manager.inspect(&service_id).is_some());
        assert_eq!(
            manager
                .providers_for_capability("telemetry.emit")
                .expect("providers")
                .len(),
            1
        );
        assert!(
            manager
                .permissions_for_service(&service_id)
                .expect("permissions")
                .contains_name("event.publish")
                .expect("permission")
        );
    }

    #[test]
    fn service_manager_aggregates_service_health() {
        let mut manager = ServiceManager::new();
        manager
            .register_service(service_descriptor())
            .expect("register");

        let health = manager.health_aggregation();

        assert_eq!(health.total_services, 1);
        assert_eq!(health.running_services, 1);
        assert_eq!(health.failed_services, 0);
    }

    #[test]
    fn lifecycle_manager_controls_module_registry() {
        let mut manager = LifecycleManager::new();
        let descriptor = module_descriptor();
        let module_id = descriptor.id().clone();

        manager.register(descriptor).expect("register");
        manager
            .transition(&module_id, LifecycleStatus::Initializing)
            .expect("initializing");
        manager
            .transition(&module_id, LifecycleStatus::Running)
            .expect("running");

        assert_eq!(
            manager.registrations()[0].status(),
            LifecycleStatus::Running
        );
        assert_eq!(
            manager.health_reports()[0].health,
            aether_core::ModuleHealth::Healthy
        );
    }

    #[test]
    fn module_registry_validates_dependencies() {
        let mut registry = ModuleRegistry::new();
        let missing = ModuleId::new("missing").expect("module id");
        let descriptor = ModuleDescriptor::new("dependent", "Dependent", "0.1.0")
            .expect("descriptor")
            .with_dependency(missing);

        let result = registry.register(descriptor);

        assert!(matches!(
            result,
            Err(RegistryError::MissingDependency { .. })
        ));
    }

    #[test]
    fn service_manager_preserves_resource_declarations() {
        let mut manager = ServiceManager::new();
        manager
            .register_service(service_descriptor())
            .expect("register");
        let service = manager.services().remove(0);

        assert_eq!(service.resources.cpu_class, CpuClass::Low);
        assert_eq!(service.resources.storage_class, StorageClass::None);
    }
}
