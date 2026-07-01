#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Core contracts shared by Aether runtime modules.

use std::fmt;

use aether_ids::{CapabilityId, IdError, IdPrefix, TypedId};
use thiserror::Error;

/// Current core crate version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Stable identifier for an Aether module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModuleId(String);

impl ModuleId {
    /// Create a module identifier.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError::InvalidDescriptor`] when the identifier is empty.
    pub fn new(value: impl Into<String>) -> Result<Self, ModuleError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(ModuleError::InvalidDescriptor(
                "module id cannot be empty".to_owned(),
            ));
        }
        Ok(Self(value))
    }

    /// Create a module identifier using the Phase 2 typed ID strategy.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError::InvalidDescriptor`] when the typed suffix is invalid.
    pub fn typed(suffix: impl Into<String>) -> Result<Self, ModuleError> {
        let typed_id =
            TypedId::new(IdPrefix::Module, suffix).map_err(|error| invalid_id(&error))?;
        Ok(Self(typed_id.to_string()))
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Capability declared by an Aether module.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Capability {
    id: CapabilityId,
    name: String,
}

impl Capability {
    /// Create a capability from its canonical dotted name.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError::InvalidDescriptor`] when the capability name is invalid.
    pub fn new(name: impl Into<String>) -> Result<Self, ModuleError> {
        let name = name.into();
        validate_capability_name(&name)?;
        let name = name.trim().to_owned();
        let id_suffix = name.replace(['.', '-'], "_");
        let id = CapabilityId::new(id_suffix).map_err(|error| invalid_id(&error))?;
        Ok(Self { id, name })
    }

    /// Return the typed capability identifier.
    #[must_use]
    pub const fn id(&self) -> &CapabilityId {
        &self.id
    }

    /// Return the canonical capability name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// Lifecycle state for a registered module.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LifecycleStatus {
    /// Module metadata exists but it has not been registered by the kernel yet.
    Created,
    /// Module has been registered by the kernel.
    Registered,
    /// Module initialization is in progress.
    Initializing,
    /// Module is running normally.
    Running,
    /// Module is partially available and should be watched.
    Degraded,
    /// Module shutdown is in progress.
    Stopping,
    /// Module has stopped cleanly.
    Stopped,
    /// Module has failed.
    Failed,
}

impl LifecycleStatus {
    /// Return the canonical lifecycle state name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Registered => "registered",
            Self::Initializing => "initializing",
            Self::Running => "running",
            Self::Degraded => "degraded",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        }
    }

    /// Return whether a transition between states is allowed.
    #[must_use]
    pub const fn can_transition_to(self, next: Self) -> bool {
        match self {
            Self::Created => matches!(next, Self::Registered | Self::Initializing),
            Self::Registered => matches!(next, Self::Initializing | Self::Stopped),
            Self::Initializing => matches!(next, Self::Running | Self::Degraded | Self::Failed),
            Self::Running => matches!(next, Self::Degraded | Self::Stopping | Self::Failed),
            Self::Degraded => matches!(next, Self::Running | Self::Stopping | Self::Failed),
            Self::Stopping => matches!(next, Self::Stopped | Self::Failed),
            Self::Stopped | Self::Failed => matches!(next, Self::Initializing),
        }
    }
}

impl fmt::Display for LifecycleStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Human-readable description of a runtime module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModuleDescriptor {
    id: ModuleId,
    name: String,
    version: String,
    capabilities: Vec<Capability>,
    dependencies: Vec<ModuleId>,
}

impl ModuleDescriptor {
    /// Create a module descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError::InvalidDescriptor`] when `name` or `version` is empty,
    /// or when `id` is invalid.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Result<Self, ModuleError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(ModuleError::InvalidDescriptor(
                "module name cannot be empty".to_owned(),
            ));
        }

        let version = version.into();
        if version.trim().is_empty() {
            return Err(ModuleError::InvalidDescriptor(
                "module version cannot be empty".to_owned(),
            ));
        }

        Ok(Self {
            id: ModuleId::new(id)?,
            name,
            version,
            capabilities: Vec::new(),
            dependencies: Vec::new(),
        })
    }

    /// Add a capability declaration to the descriptor.
    #[must_use]
    pub fn with_capability(mut self, capability: Capability) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self
    }

    /// Add a module dependency declaration to the descriptor.
    #[must_use]
    pub fn with_dependency(mut self, dependency: ModuleId) -> Self {
        if !self.dependencies.contains(&dependency) {
            self.dependencies.push(dependency);
        }
        self
    }

    /// Return the module identifier.
    #[must_use]
    pub const fn id(&self) -> &ModuleId {
        &self.id
    }

    /// Return the module name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the module version.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Return declared module capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    /// Return declared module dependencies.
    #[must_use]
    pub fn dependencies(&self) -> &[ModuleId] {
        &self.dependencies
    }
}

/// Health status reported by a module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModuleHealth {
    /// Module is ready for runtime work.
    Healthy,
    /// Module is degraded but still able to participate.
    Degraded,
    /// Module cannot participate safely.
    Unhealthy,
}

/// Standard lifecycle contract for runtime modules.
pub trait AetherModule: Send {
    /// Return immutable module metadata.
    fn descriptor(&self) -> &ModuleDescriptor;

    /// Start the module.
    ///
    /// # Errors
    ///
    /// Implementations return [`ModuleError`] when startup fails.
    fn start(&mut self) -> Result<(), ModuleError>;

    /// Stop the module.
    ///
    /// # Errors
    ///
    /// Implementations return [`ModuleError`] when shutdown fails.
    fn stop(&mut self) -> Result<(), ModuleError>;

    /// Return current module health.
    ///
    /// # Errors
    ///
    /// Implementations return [`ModuleError`] when health cannot be determined.
    fn health(&self) -> Result<ModuleHealth, ModuleError>;
}

/// Errors raised by module contracts.
#[derive(Debug, Error)]
pub enum ModuleError {
    /// Module metadata is invalid.
    #[error("invalid module descriptor: {0}")]
    InvalidDescriptor(String),
    /// Module startup failed.
    #[error("module start failed for {module_id}: {reason}")]
    StartFailed {
        /// Module identifier.
        module_id: String,
        /// Failure reason.
        reason: String,
    },
    /// Module shutdown failed.
    #[error("module stop failed for {module_id}: {reason}")]
    StopFailed {
        /// Module identifier.
        module_id: String,
        /// Failure reason.
        reason: String,
    },
    /// Module health check failed.
    #[error("module health check failed for {module_id}: {reason}")]
    HealthCheckFailed {
        /// Module identifier.
        module_id: String,
        /// Failure reason.
        reason: String,
    },
}

fn invalid_id(error: &IdError) -> ModuleError {
    ModuleError::InvalidDescriptor(format!("invalid typed id: {error}"))
}

fn validate_capability_name(name: &str) -> Result<(), ModuleError> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(ModuleError::InvalidDescriptor(
            "capability name cannot be empty".to_owned(),
        ));
    }

    let valid = trimmed.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || character == '.'
            || character == '-'
    });
    if valid {
        Ok(())
    } else {
        Err(ModuleError::InvalidDescriptor(format!(
            "invalid capability name: {name}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::{Capability, LifecycleStatus, ModuleDescriptor, ModuleError, ModuleId};

    #[test]
    fn descriptor_rejects_empty_id() {
        let result = ModuleDescriptor::new("", "Core", "0.1.0");

        assert!(matches!(result, Err(ModuleError::InvalidDescriptor(_))));
    }

    #[test]
    fn descriptor_exposes_metadata() {
        let descriptor = ModuleDescriptor::new("core", "Core", "0.1.0").expect("valid descriptor");

        assert_eq!(descriptor.id().as_str(), "core");
        assert_eq!(descriptor.name(), "Core");
        assert_eq!(descriptor.version(), "0.1.0");
    }

    #[test]
    fn descriptor_declares_capabilities_and_dependencies() {
        let dependency = ModuleId::typed("events").expect("module id");
        let capability = Capability::new("events.publish").expect("capability");
        let descriptor = ModuleDescriptor::new("mod_kernel", "Kernel", "0.1.0")
            .expect("descriptor")
            .with_dependency(dependency.clone())
            .with_capability(capability.clone());

        assert_eq!(descriptor.dependencies(), [dependency]);
        assert_eq!(descriptor.capabilities(), [capability]);
        assert_eq!(
            descriptor.capabilities()[0].id().as_str(),
            "cap_events_publish"
        );
    }

    #[test]
    fn lifecycle_allows_expected_transitions() {
        assert!(LifecycleStatus::Registered.can_transition_to(LifecycleStatus::Initializing));
        assert!(LifecycleStatus::Initializing.can_transition_to(LifecycleStatus::Running));
        assert!(LifecycleStatus::Running.can_transition_to(LifecycleStatus::Stopping));
        assert!(LifecycleStatus::Stopping.can_transition_to(LifecycleStatus::Stopped));
        assert!(!LifecycleStatus::Stopped.can_transition_to(LifecycleStatus::Running));
    }
}
