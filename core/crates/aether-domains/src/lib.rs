#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! System domain model for Aether architecture boundaries.

use std::collections::BTreeMap;

use aether_ids::DomainId;
use thiserror::Error;

/// Permanent Aether system domain kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DomainKind {
    /// Core system orchestration domain.
    System,
    /// Future memory domain boundary.
    Memory,
    /// Future knowledge domain boundary.
    Knowledge,
    /// Telemetry and observability domain boundary.
    Telemetry,
    /// Identity and access domain boundary.
    Identity,
    /// Automation domain boundary.
    Automation,
}

impl DomainKind {
    /// Return the canonical domain name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => "SystemDomain",
            Self::Memory => "MemoryDomain",
            Self::Knowledge => "KnowledgeDomain",
            Self::Telemetry => "TelemetryDomain",
            Self::Identity => "IdentityDomain",
            Self::Automation => "AutomationDomain",
        }
    }

    /// Return the stable id suffix for this domain.
    #[must_use]
    pub const fn id_suffix(self) -> &'static str {
        match self {
            Self::System => "system",
            Self::Memory => "memory",
            Self::Knowledge => "knowledge",
            Self::Telemetry => "telemetry",
            Self::Identity => "identity",
            Self::Automation => "automation",
        }
    }
}

/// Domain descriptor used to organize long-lived platform boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DomainDescriptor {
    /// Typed domain identifier.
    pub id: DomainId,
    /// Domain kind.
    pub kind: DomainKind,
    /// Human-readable domain name.
    pub name: String,
    /// Responsibilities owned by the domain boundary.
    pub responsibilities: Vec<String>,
    /// Managers allowed to operate inside the domain.
    pub allowed_managers: Vec<String>,
}

impl DomainDescriptor {
    /// Create a domain descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when required fields are invalid.
    pub fn new(
        kind: DomainKind,
        responsibilities: Vec<String>,
        allowed_managers: Vec<String>,
    ) -> Result<Self, DomainError> {
        let name = kind.as_str().to_owned();
        Ok(Self {
            id: DomainId::new(kind.id_suffix())?,
            kind,
            name,
            responsibilities,
            allowed_managers,
        })
    }
}

/// Registry for domain descriptors.
#[derive(Clone, Debug, Default)]
pub struct DomainRegistry {
    domains: BTreeMap<DomainId, DomainDescriptor>,
}

impl DomainRegistry {
    /// Create an empty domain registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            domains: BTreeMap::new(),
        }
    }

    /// Create a registry with standard system domains.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when a descriptor cannot be constructed.
    pub fn with_standard_domains() -> Result<Self, DomainError> {
        let mut registry = Self::new();
        for descriptor in standard_domain_descriptors()? {
            registry.register(descriptor)?;
        }
        Ok(registry)
    }

    /// Register a domain descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the domain is duplicated.
    pub fn register(&mut self, descriptor: DomainDescriptor) -> Result<(), DomainError> {
        if self.domains.contains_key(&descriptor.id) {
            return Err(DomainError::DuplicateDomain(descriptor.id.to_string()));
        }
        self.domains.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    /// Return a domain descriptor by id.
    #[must_use]
    pub fn get(&self, domain_id: &DomainId) -> Option<&DomainDescriptor> {
        self.domains.get(domain_id)
    }

    /// Return all registered domains.
    #[must_use]
    pub fn domains(&self) -> Vec<DomainDescriptor> {
        self.domains.values().cloned().collect()
    }
}

/// System domain marker.
pub struct SystemDomain;

impl SystemDomain {
    /// Return the `SystemDomain` descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the descriptor cannot be constructed.
    pub fn descriptor() -> Result<DomainDescriptor, DomainError> {
        DomainDescriptor::new(
            DomainKind::System,
            vec![
                "Kernel coordination".to_owned(),
                "Manager supervision".to_owned(),
                "Service platform foundation".to_owned(),
            ],
            vec![
                "ServiceManager".to_owned(),
                "LifecycleManager".to_owned(),
                "HealthManager".to_owned(),
            ],
        )
    }
}

/// Memory domain marker for future memory boundaries.
pub struct MemoryDomain;

impl MemoryDomain {
    /// Return the `MemoryDomain` descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the descriptor cannot be constructed.
    pub fn descriptor() -> Result<DomainDescriptor, DomainError> {
        DomainDescriptor::new(
            DomainKind::Memory,
            vec!["Future memory contracts only".to_owned()],
            vec!["ResourceManager".to_owned(), "PermissionManager".to_owned()],
        )
    }
}

/// Knowledge domain marker for future knowledge boundaries.
pub struct KnowledgeDomain;

impl KnowledgeDomain {
    /// Return the `KnowledgeDomain` descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the descriptor cannot be constructed.
    pub fn descriptor() -> Result<DomainDescriptor, DomainError> {
        DomainDescriptor::new(
            DomainKind::Knowledge,
            vec!["Future knowledge graph contracts only".to_owned()],
            vec!["ResourceManager".to_owned(), "PermissionManager".to_owned()],
        )
    }
}

/// Telemetry domain marker.
pub struct TelemetryDomain;

impl TelemetryDomain {
    /// Return the `TelemetryDomain` descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the descriptor cannot be constructed.
    pub fn descriptor() -> Result<DomainDescriptor, DomainError> {
        DomainDescriptor::new(
            DomainKind::Telemetry,
            vec![
                "Telemetry contracts".to_owned(),
                "Operational signal boundaries".to_owned(),
            ],
            vec!["TelemetryManager".to_owned(), "HealthManager".to_owned()],
        )
    }
}

/// Identity domain marker.
pub struct IdentityDomain;

impl IdentityDomain {
    /// Return the `IdentityDomain` descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the descriptor cannot be constructed.
    pub fn descriptor() -> Result<DomainDescriptor, DomainError> {
        DomainDescriptor::new(
            DomainKind::Identity,
            vec!["Future identity contracts only".to_owned()],
            vec![
                "PermissionManager".to_owned(),
                "ConfigurationManager".to_owned(),
            ],
        )
    }
}

/// Automation domain marker.
pub struct AutomationDomain;

impl AutomationDomain {
    /// Return the `AutomationDomain` descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`DomainError`] when the descriptor cannot be constructed.
    pub fn descriptor() -> Result<DomainDescriptor, DomainError> {
        DomainDescriptor::new(
            DomainKind::Automation,
            vec!["Future automation contracts only".to_owned()],
            vec!["DriverManager".to_owned(), "PermissionManager".to_owned()],
        )
    }
}

/// Return the standard domain descriptors.
///
/// # Errors
///
/// Returns [`DomainError`] when any descriptor cannot be constructed.
pub fn standard_domain_descriptors() -> Result<Vec<DomainDescriptor>, DomainError> {
    Ok(vec![
        SystemDomain::descriptor()?,
        MemoryDomain::descriptor()?,
        KnowledgeDomain::descriptor()?,
        TelemetryDomain::descriptor()?,
        IdentityDomain::descriptor()?,
        AutomationDomain::descriptor()?,
    ])
}

/// Domain model errors.
#[derive(Debug, Error)]
pub enum DomainError {
    /// Domain identifier is invalid.
    #[error("invalid domain id: {0}")]
    Id(#[from] aether_ids::IdError),
    /// Domain is already registered.
    #[error("domain already registered: {0}")]
    DuplicateDomain(String),
}

#[cfg(test)]
mod tests {
    use super::{DomainKind, DomainRegistry, SystemDomain, standard_domain_descriptors};

    #[test]
    fn standard_domains_define_architecture_boundaries() {
        let descriptors = standard_domain_descriptors().expect("domains");

        assert_eq!(descriptors.len(), 6);
        assert!(
            descriptors
                .iter()
                .any(|descriptor| descriptor.kind == DomainKind::Memory)
        );
        assert!(
            descriptors
                .iter()
                .all(|descriptor| descriptor.id.as_str().starts_with("dom_"))
        );
    }

    #[test]
    fn domain_registry_registers_domain_descriptors() {
        let mut registry = DomainRegistry::new();
        let descriptor = SystemDomain::descriptor().expect("descriptor");
        let domain_id = descriptor.id.clone();

        registry.register(descriptor).expect("register");

        assert!(registry.get(&domain_id).is_some());
        assert_eq!(registry.domains().len(), 1);
    }

    #[test]
    fn domain_registry_loads_standard_domains() {
        let registry = DomainRegistry::with_standard_domains().expect("registry");

        assert_eq!(registry.domains().len(), 6);
    }
}
