#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Service model and registry for the Aether Service Platform.

use std::collections::{BTreeMap, BTreeSet};

use aether_core::{Capability, LifecycleStatus};
use aether_ids::ServiceId;
use aether_permissions::{Permission, PermissionError, PermissionSet};
use aether_resources::ResourceProfile;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Declarative service manifest loaded from TOML.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceManifest {
    /// Service metadata section.
    pub service: ServiceManifestMetadata,
    /// Capability declaration section.
    #[serde(default)]
    pub capabilities: ServiceCapabilityManifest,
    /// Permission declaration section.
    #[serde(default)]
    pub permissions: ServicePermissionManifest,
    /// Resource declaration section.
    #[serde(default)]
    pub resources: ResourceProfile,
}

impl ServiceManifest {
    /// Load a service manifest from TOML text.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when TOML parsing fails.
    pub fn from_toml_str(contents: &str) -> Result<Self, ServiceError> {
        Ok(toml::from_str(contents)?)
    }
}

/// Service metadata section in a manifest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceManifestMetadata {
    /// Service name.
    pub name: String,
    /// Service version.
    pub version: String,
    /// Service description.
    pub description: String,
    /// Service owner.
    pub owner: String,
}

/// Capability declaration section in a manifest.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceCapabilityManifest {
    /// Capabilities provided by the service.
    #[serde(default)]
    pub provides: Vec<String>,
    /// Capabilities required by the service.
    #[serde(default)]
    pub requires: Vec<String>,
}

/// Permission declaration section in a manifest.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServicePermissionManifest {
    /// Permissions requested by the service.
    #[serde(default)]
    pub requested: Vec<String>,
}

/// Capability model for a service.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ServiceCapabilities {
    provides: Vec<Capability>,
    requires: Vec<Capability>,
}

impl ServiceCapabilities {
    /// Create service capabilities.
    #[must_use]
    pub fn new(provides: Vec<Capability>, requires: Vec<Capability>) -> Self {
        Self {
            provides: unique_capabilities(provides),
            requires: unique_capabilities(requires),
        }
    }

    /// Return capabilities provided by the service.
    #[must_use]
    pub fn provides(&self) -> &[Capability] {
        &self.provides
    }

    /// Return capabilities required by the service.
    #[must_use]
    pub fn requires(&self) -> &[Capability] {
        &self.requires
    }
}

/// Health status reported for a service descriptor.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ServiceHealthStatus {
    /// Service is healthy.
    Healthy,
    /// Service is degraded.
    Degraded,
    /// Service is unhealthy.
    Unhealthy,
    /// Service health has not been evaluated yet.
    Unknown,
}

/// Service descriptor owned by the Kernel registry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceDescriptor {
    /// Stable service identifier.
    pub id: ServiceId,
    /// Service name.
    pub name: String,
    /// Service version.
    pub version: String,
    /// Service description.
    pub description: String,
    /// Service owner.
    pub owner: String,
    /// Capability model.
    pub capabilities: ServiceCapabilities,
    /// Requested permissions.
    pub permissions: PermissionSet,
    /// Capability dependencies.
    pub dependencies: Vec<Capability>,
    /// Requested resources.
    pub resources: ResourceProfile,
    /// Service lifecycle status.
    pub lifecycle_status: LifecycleStatus,
    /// Service health status.
    pub health_status: ServiceHealthStatus,
    /// Source manifest.
    pub manifest: ServiceManifest,
}

impl ServiceDescriptor {
    /// Build a descriptor from a manifest.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when manifest fields are invalid.
    pub fn from_manifest(manifest: ServiceManifest) -> Result<Self, ServiceError> {
        validate_manifest_metadata(&manifest.service)?;
        let id = ServiceId::new(service_id_suffix(&manifest.service.name)?)
            .map_err(|error| ServiceError::InvalidServiceId(error.to_string()))?;
        let provides = capabilities_from_names(&manifest.capabilities.provides)?;
        let requires = capabilities_from_names(&manifest.capabilities.requires)?;
        let permissions = PermissionSet::from_names(&manifest.permissions.requested)?;
        let capabilities = ServiceCapabilities::new(provides, requires.clone());

        Ok(Self {
            id,
            name: manifest.service.name.trim().to_owned(),
            version: manifest.service.version.trim().to_owned(),
            description: manifest.service.description.trim().to_owned(),
            owner: manifest.service.owner.trim().to_owned(),
            capabilities,
            permissions,
            dependencies: requires,
            resources: manifest.resources.clone(),
            lifecycle_status: LifecycleStatus::Registered,
            health_status: ServiceHealthStatus::Unknown,
            manifest,
        })
    }

    /// Return a copy with updated lifecycle status.
    #[must_use]
    pub const fn with_lifecycle_status(mut self, status: LifecycleStatus) -> Self {
        self.lifecycle_status = status;
        self
    }

    /// Return a copy with updated health status.
    #[must_use]
    pub const fn with_health_status(mut self, status: ServiceHealthStatus) -> Self {
        self.health_status = status;
        self
    }

    /// Validate the descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when required fields are invalid.
    pub fn validate(&self) -> Result<(), ServiceError> {
        validate_required("service name", &self.name)?;
        validate_required("service version", &self.version)?;
        validate_required("service owner", &self.owner)?;
        Ok(())
    }
}

/// Registry for service descriptors.
#[derive(Clone, Debug, Default)]
pub struct ServiceRegistry {
    services: BTreeMap<ServiceId, ServiceDescriptor>,
}

impl ServiceRegistry {
    /// Create an empty service registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            services: BTreeMap::new(),
        }
    }

    /// Register a service descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the descriptor is invalid or duplicated.
    pub fn register(&mut self, descriptor: ServiceDescriptor) -> Result<(), ServiceError> {
        descriptor.validate()?;
        if self.services.contains_key(&descriptor.id) {
            return Err(ServiceError::DuplicateService(descriptor.id.to_string()));
        }
        self.services.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    /// Return a service by identifier.
    #[must_use]
    pub fn get(&self, service_id: &ServiceId) -> Option<&ServiceDescriptor> {
        self.services.get(service_id)
    }

    /// Return registered services.
    #[must_use]
    pub fn services(&self) -> Vec<ServiceDescriptor> {
        self.services.values().cloned().collect()
    }

    /// Return services that provide a capability.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the capability name is invalid.
    pub fn providers_for_capability_name(
        &self,
        capability_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, ServiceError> {
        let capability = Capability::new(capability_name)?;
        Ok(self
            .services
            .values()
            .filter(|descriptor| descriptor.capabilities.provides().contains(&capability))
            .cloned()
            .collect())
    }

    /// Return services that require a capability.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the capability name is invalid.
    pub fn dependents_on_capability_name(
        &self,
        capability_name: &str,
    ) -> Result<Vec<ServiceDescriptor>, ServiceError> {
        let capability = Capability::new(capability_name)?;
        Ok(self
            .services
            .values()
            .filter(|descriptor| descriptor.capabilities.requires().contains(&capability))
            .cloned()
            .collect())
    }

    /// Return requested permissions for a service.
    #[must_use]
    pub fn permissions_for_service(&self, service_id: &ServiceId) -> Option<&PermissionSet> {
        self.services
            .get(service_id)
            .map(|descriptor| &descriptor.permissions)
    }

    /// Ensure a service declared a permission before an action is executed.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the service is unknown or permission is missing.
    pub fn ensure_permission(
        &self,
        service_id: &ServiceId,
        permission: &Permission,
    ) -> Result<(), ServiceError> {
        let descriptor = self
            .services
            .get(service_id)
            .ok_or_else(|| ServiceError::UnknownService(service_id.to_string()))?;
        if descriptor.permissions.contains(permission) {
            Ok(())
        } else {
            Err(ServiceError::PermissionNotDeclared {
                service_id: service_id.to_string(),
                permission: permission.to_string(),
            })
        }
    }

    /// Return consolidated service health.
    #[must_use]
    pub fn health_aggregation(&self) -> ServiceHealthAggregation {
        let mut capabilities_available = BTreeSet::new();
        let mut permissions_requested = BTreeSet::new();
        let mut running_services = 0usize;
        let mut degraded_services = 0usize;
        let mut failed_services = 0usize;

        for descriptor in self.services.values() {
            match descriptor.health_status {
                ServiceHealthStatus::Healthy => running_services += 1,
                ServiceHealthStatus::Degraded | ServiceHealthStatus::Unknown => {
                    degraded_services += 1;
                }
                ServiceHealthStatus::Unhealthy => failed_services += 1,
            }

            for capability in descriptor.capabilities.provides() {
                capabilities_available.insert(capability.name().to_owned());
            }

            for permission in descriptor.permissions.requested() {
                permissions_requested.insert(permission.to_string());
            }
        }

        ServiceHealthAggregation {
            total_services: self.services.len(),
            running_services,
            degraded_services,
            failed_services,
            capabilities_available: capabilities_available.into_iter().collect(),
            permissions_requested: permissions_requested.into_iter().collect(),
            resources_declared: self.services.len(),
        }
    }
}

/// Consolidated service health view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceHealthAggregation {
    /// Total registered services.
    pub total_services: usize,
    /// Services considered running.
    pub running_services: usize,
    /// Services considered degraded.
    pub degraded_services: usize,
    /// Services considered failed.
    pub failed_services: usize,
    /// Unique provided capabilities.
    pub capabilities_available: Vec<String>,
    /// Unique requested permissions.
    pub permissions_requested: Vec<String>,
    /// Number of services that declared resources.
    pub resources_declared: usize,
}

/// Service model errors.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// TOML parsing failed.
    #[error("failed to parse service manifest: {0}")]
    ManifestParse(#[from] toml::de::Error),
    /// Service identifier is invalid.
    #[error("invalid service id: {0}")]
    InvalidServiceId(String),
    /// Service field is invalid.
    #[error("invalid service field: {0}")]
    InvalidField(String),
    /// Capability declaration is invalid.
    #[error("invalid capability declaration: {0}")]
    Capability(#[from] aether_core::ModuleError),
    /// Permission declaration is invalid.
    #[error("invalid permission declaration: {0}")]
    Permission(#[from] PermissionError),
    /// Service is already registered.
    #[error("service already registered: {0}")]
    DuplicateService(String),
    /// Service is not registered.
    #[error("unknown service: {0}")]
    UnknownService(String),
    /// Service did not declare a required permission.
    #[error("service {service_id} did not declare permission {permission}")]
    PermissionNotDeclared {
        /// Service identifier.
        service_id: String,
        /// Missing permission.
        permission: String,
    },
}

fn capabilities_from_names(names: &[String]) -> Result<Vec<Capability>, ServiceError> {
    let mut capabilities = Vec::with_capacity(names.len());
    for name in names {
        capabilities.push(Capability::new(name)?);
    }
    Ok(unique_capabilities(capabilities))
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

fn validate_manifest_metadata(metadata: &ServiceManifestMetadata) -> Result<(), ServiceError> {
    validate_required("service name", &metadata.name)?;
    validate_required("service version", &metadata.version)?;
    validate_required("service description", &metadata.description)?;
    validate_required("service owner", &metadata.owner)?;
    Ok(())
}

fn validate_required(field: &str, value: &str) -> Result<(), ServiceError> {
    if value.trim().is_empty() {
        Err(ServiceError::InvalidField(format!(
            "{field} cannot be empty"
        )))
    } else {
        Ok(())
    }
}

fn service_id_suffix(name: &str) -> Result<String, ServiceError> {
    let mut suffix = String::new();
    for character in name.trim().to_ascii_lowercase().chars() {
        if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
            suffix.push(character);
        } else if character == '.' || character.is_ascii_whitespace() {
            suffix.push('-');
        }
    }

    let suffix = suffix.trim_matches('-').to_owned();
    if suffix.is_empty() {
        Err(ServiceError::InvalidField(
            "service name cannot produce an id".to_owned(),
        ))
    } else {
        Ok(suffix)
    }
}

#[cfg(test)]
mod tests {
    use aether_core::LifecycleStatus;
    use aether_resources::{CpuClass, StorageClass};

    use super::{
        ServiceDescriptor, ServiceError, ServiceHealthStatus, ServiceManifest, ServiceRegistry,
    };

    fn manifest() -> ServiceManifest {
        ServiceManifest::from_toml_str(
            r#"
            [service]
            name = "telemetry-service"
            version = "0.1.0"
            description = "Base telemetry service"
            owner = "neuroforge-labs"

            [capabilities]
            provides = ["telemetry.emit", "telemetry.query"]
            requires = ["events.publish"]

            [permissions]
            requested = ["event.publish", "config.read", "telemetry.emit"]

            [resources]
            cpu_class = "low"
            memory_class = "low"
            storage_class = "none"
            network = false
            "#,
        )
        .expect("manifest")
    }

    fn descriptor() -> ServiceDescriptor {
        ServiceDescriptor::from_manifest(manifest()).expect("descriptor")
    }

    #[test]
    fn loads_service_manifest() {
        let manifest = manifest();

        assert_eq!(manifest.service.name, "telemetry-service");
        assert_eq!(
            manifest.capabilities.provides,
            ["telemetry.emit", "telemetry.query"]
        );
        assert_eq!(manifest.resources.cpu_class, CpuClass::Low);
        assert_eq!(manifest.resources.storage_class, StorageClass::None);
    }

    #[test]
    fn validates_service_descriptor() {
        let descriptor = descriptor();

        assert_eq!(descriptor.id.as_str(), "svc_telemetry-service");
        assert_eq!(descriptor.dependencies.len(), 1);
        assert_eq!(descriptor.capabilities.provides().len(), 2);
        assert!(descriptor.validate().is_ok());
    }

    #[test]
    fn registry_registers_service() {
        let mut registry = ServiceRegistry::new();
        let descriptor = descriptor();

        registry.register(descriptor).expect("register");

        assert_eq!(registry.services().len(), 1);
    }

    #[test]
    fn registry_finds_capability_provider() {
        let mut registry = ServiceRegistry::new();
        registry.register(descriptor()).expect("register");

        let providers = registry
            .providers_for_capability_name("telemetry.emit")
            .expect("providers");

        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].name, "telemetry-service");
    }

    #[test]
    fn registry_finds_dependents_by_required_capability() {
        let mut registry = ServiceRegistry::new();
        registry.register(descriptor()).expect("register");

        let dependents = registry
            .dependents_on_capability_name("events.publish")
            .expect("dependents");

        assert_eq!(dependents.len(), 1);
    }

    #[test]
    fn registry_checks_declared_permission() {
        let mut registry = ServiceRegistry::new();
        let descriptor = descriptor();
        let service_id = descriptor.id.clone();
        registry.register(descriptor).expect("register");

        assert!(
            registry
                .permissions_for_service(&service_id)
                .expect("permissions")
                .contains_name("event.publish")
                .expect("permission")
        );
        assert!(matches!(
            registry.ensure_permission(
                &service_id,
                &aether_permissions::Permission::service_command()
            ),
            Err(ServiceError::PermissionNotDeclared { .. })
        ));
    }

    #[test]
    fn registry_aggregates_health() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(
                descriptor()
                    .with_lifecycle_status(LifecycleStatus::Running)
                    .with_health_status(ServiceHealthStatus::Healthy),
            )
            .expect("register");

        let health = registry.health_aggregation();

        assert_eq!(health.total_services, 1);
        assert_eq!(health.running_services, 1);
        assert_eq!(health.failed_services, 0);
        assert_eq!(
            health.capabilities_available,
            ["telemetry.emit", "telemetry.query"]
        );
        assert!(
            health
                .permissions_requested
                .contains(&"event.publish".to_owned())
        );
    }
}
