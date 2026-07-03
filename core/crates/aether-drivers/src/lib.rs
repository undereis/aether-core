#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Driver layer contracts for future Aether native integrations.

use std::collections::BTreeMap;

use aether_core::Capability;
use aether_ids::DriverId;
use aether_resources::ResourceProfile;
use thiserror::Error;

/// Health status reported by a driver.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DriverHealth {
    /// Driver contract is available.
    Healthy,
    /// Driver contract is partially available.
    Degraded,
    /// Driver contract is unavailable.
    Unhealthy,
    /// Driver health has not been evaluated.
    Unknown,
}

/// Capability declared by a driver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverCapability {
    /// Capability exposed by the driver.
    pub capability: Capability,
}

impl DriverCapability {
    /// Create a driver capability from a dotted capability name.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when the capability name is invalid.
    pub fn new(name: impl Into<String>) -> Result<Self, DriverError> {
        Ok(Self {
            capability: Capability::new(name)?,
        })
    }
}

/// Declarative driver manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverManifest {
    /// Driver name.
    pub name: String,
    /// Driver version.
    pub version: String,
    /// Driver provider or owner.
    pub provider: String,
    /// Driver capability names.
    pub capabilities: Vec<String>,
    /// Driver resource declaration.
    pub resources: ResourceProfile,
}

impl DriverManifest {
    /// Create a driver manifest.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when required fields are empty.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        provider: impl Into<String>,
        capabilities: Vec<String>,
        resources: ResourceProfile,
    ) -> Result<Self, DriverError> {
        let manifest = Self {
            name: name.into(),
            version: version.into(),
            provider: provider.into(),
            capabilities,
            resources,
        };
        validate_required("driver name", &manifest.name)?;
        validate_required("driver version", &manifest.version)?;
        validate_required("driver provider", &manifest.provider)?;
        Ok(manifest)
    }
}

/// Driver descriptor registered with the `DriverManager` in a future phase.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DriverDescriptor {
    /// Typed driver identifier.
    pub id: DriverId,
    /// Driver name.
    pub name: String,
    /// Driver version.
    pub version: String,
    /// Declared capabilities.
    pub capabilities: Vec<DriverCapability>,
    /// Current health status.
    pub health: DriverHealth,
    /// Driver manifest.
    pub manifest: DriverManifest,
}

impl DriverDescriptor {
    /// Build a descriptor from a manifest.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when fields or capabilities are invalid.
    pub fn from_manifest(manifest: DriverManifest) -> Result<Self, DriverError> {
        let id = DriverId::new(slug_suffix(&manifest.name)?)?;
        let mut capabilities = Vec::with_capacity(manifest.capabilities.len());
        for capability in &manifest.capabilities {
            capabilities.push(DriverCapability::new(capability.clone())?);
        }
        Ok(Self {
            id,
            name: manifest.name.trim().to_owned(),
            version: manifest.version.trim().to_owned(),
            capabilities: unique_driver_capabilities(capabilities),
            health: DriverHealth::Unknown,
            manifest,
        })
    }
}

/// Contract implemented by future concrete drivers.
pub trait Driver {
    /// Return the driver descriptor.
    fn descriptor(&self) -> &DriverDescriptor;

    /// Return current driver health.
    fn health(&self) -> DriverHealth {
        self.descriptor().health
    }
}

/// Registry for driver descriptors.
#[derive(Clone, Debug, Default)]
pub struct DriverRegistry {
    drivers: BTreeMap<DriverId, DriverDescriptor>,
}

impl DriverRegistry {
    /// Create an empty driver registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
        }
    }

    /// Register a driver descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when the driver is duplicated.
    pub fn register(&mut self, descriptor: DriverDescriptor) -> Result<(), DriverError> {
        if self.drivers.contains_key(&descriptor.id) {
            return Err(DriverError::DuplicateDriver(descriptor.id.to_string()));
        }
        self.drivers.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    /// Return a driver descriptor by id.
    #[must_use]
    pub fn get(&self, driver_id: &DriverId) -> Option<&DriverDescriptor> {
        self.drivers.get(driver_id)
    }

    /// Return all registered driver descriptors.
    #[must_use]
    pub fn drivers(&self) -> Vec<DriverDescriptor> {
        self.drivers.values().cloned().collect()
    }
}

/// Driver layer errors.
#[derive(Debug, Error)]
pub enum DriverError {
    /// Driver field is invalid.
    #[error("invalid driver field: {0}")]
    InvalidField(String),
    /// Driver id is invalid.
    #[error("invalid driver id: {0}")]
    Id(#[from] aether_ids::IdError),
    /// Driver capability is invalid.
    #[error("invalid driver capability: {0}")]
    Capability(#[from] aether_core::ModuleError),
    /// Driver is already registered.
    #[error("driver already registered: {0}")]
    DuplicateDriver(String),
}

fn unique_driver_capabilities(capabilities: Vec<DriverCapability>) -> Vec<DriverCapability> {
    let mut unique = Vec::new();
    for capability in capabilities {
        if !unique.contains(&capability) {
            unique.push(capability);
        }
    }
    unique
}

fn validate_required(field: &str, value: &str) -> Result<(), DriverError> {
    if value.trim().is_empty() {
        Err(DriverError::InvalidField(format!(
            "{field} cannot be empty"
        )))
    } else {
        Ok(())
    }
}

fn slug_suffix(value: &str) -> Result<String, DriverError> {
    let suffix = value
        .trim()
        .to_ascii_lowercase()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                character
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_owned();
    if suffix.is_empty() {
        Err(DriverError::InvalidField(
            "driver name cannot produce an id".to_owned(),
        ))
    } else {
        Ok(suffix)
    }
}

#[cfg(test)]
mod tests {
    use aether_resources::{FilesystemAccess, ResourceProfile};

    use super::{DriverDescriptor, DriverHealth, DriverManifest, DriverRegistry};

    fn manifest() -> DriverManifest {
        DriverManifest::new(
            "Filesystem Driver",
            "0.1.0",
            "neuroforge-labs",
            vec!["filesystem.read".to_owned(), "filesystem.read".to_owned()],
            ResourceProfile {
                filesystem_access: FilesystemAccess::ReadOnly,
                ..ResourceProfile::default()
            },
        )
        .expect("manifest")
    }

    #[test]
    fn driver_manifest_builds_descriptor_contract() {
        let descriptor = DriverDescriptor::from_manifest(manifest()).expect("descriptor");

        assert_eq!(descriptor.id.as_str(), "drv_filesystem-driver");
        assert_eq!(descriptor.capabilities.len(), 1);
        assert_eq!(descriptor.health, DriverHealth::Unknown);
        assert!(descriptor.manifest.resources.requests_filesystem());
    }

    #[test]
    fn driver_registry_registers_driver_descriptor() {
        let descriptor = DriverDescriptor::from_manifest(manifest()).expect("descriptor");
        let driver_id = descriptor.id.clone();
        let mut registry = DriverRegistry::new();

        registry.register(descriptor).expect("register");

        assert!(registry.get(&driver_id).is_some());
        assert_eq!(registry.drivers().len(), 1);
    }
}
