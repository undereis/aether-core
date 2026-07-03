#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Policy layer contracts for Aether behavior control.

use std::collections::BTreeMap;

use aether_ids::PolicyId;
use thiserror::Error;

/// Policy kind supported by the initial policy model.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PolicyKind {
    /// Security policy contract.
    Security,
    /// Filesystem policy contract.
    Filesystem,
    /// Telemetry policy contract.
    Telemetry,
    /// Memory policy contract.
    Memory,
    /// Privacy policy contract.
    Privacy,
}

impl PolicyKind {
    /// Return the canonical policy name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Security => "SecurityPolicy",
            Self::Filesystem => "FilesystemPolicy",
            Self::Telemetry => "TelemetryPolicy",
            Self::Memory => "MemoryPolicy",
            Self::Privacy => "PrivacyPolicy",
        }
    }

    /// Return the stable id suffix.
    #[must_use]
    pub const fn id_suffix(self) -> &'static str {
        match self {
            Self::Security => "security",
            Self::Filesystem => "filesystem",
            Self::Telemetry => "telemetry",
            Self::Memory => "memory",
            Self::Privacy => "privacy",
        }
    }
}

/// Policy evaluation effect.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PolicyEffect {
    /// Policy explicitly allows the action.
    Allow,
    /// Policy explicitly denies the action.
    Deny,
    /// Policy does not apply to the action.
    NotApplicable,
}

/// Minimal policy decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDecision {
    /// Evaluation effect.
    pub effect: PolicyEffect,
    /// Human-readable decision reason.
    pub reason: String,
}

impl PolicyDecision {
    /// Create a policy decision.
    #[must_use]
    pub fn new(effect: PolicyEffect, reason: impl Into<String>) -> Self {
        Self {
            effect,
            reason: reason.into(),
        }
    }
}

/// Declarative policy manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyManifest {
    /// Policy name.
    pub name: String,
    /// Policy version.
    pub version: String,
    /// Policy purpose.
    pub purpose: String,
}

impl PolicyManifest {
    /// Create a policy manifest.
    ///
    /// # Errors
    ///
    /// Returns [`PolicyError`] when a required field is empty.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        purpose: impl Into<String>,
    ) -> Result<Self, PolicyError> {
        let manifest = Self {
            name: name.into(),
            version: version.into(),
            purpose: purpose.into(),
        };
        validate_required("policy name", &manifest.name)?;
        validate_required("policy version", &manifest.version)?;
        validate_required("policy purpose", &manifest.purpose)?;
        Ok(manifest)
    }
}

/// Policy descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyDescriptor {
    /// Typed policy identifier.
    pub id: PolicyId,
    /// Policy kind.
    pub kind: PolicyKind,
    /// Policy name.
    pub name: String,
    /// Policy version.
    pub version: String,
    /// Policy manifest.
    pub manifest: PolicyManifest,
}

impl PolicyDescriptor {
    /// Create a policy descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`PolicyError`] when a required field is invalid.
    pub fn new(kind: PolicyKind, purpose: impl Into<String>) -> Result<Self, PolicyError> {
        let name = kind.as_str().to_owned();
        let version = env!("CARGO_PKG_VERSION").to_owned();
        let manifest = PolicyManifest::new(name.clone(), version.clone(), purpose)?;
        Ok(Self {
            id: PolicyId::new(kind.id_suffix())?,
            kind,
            name,
            version,
            manifest,
        })
    }
}

/// Policy evaluation contract.
pub trait Policy {
    /// Return the policy descriptor.
    fn descriptor(&self) -> &PolicyDescriptor;

    /// Evaluate a policy context.
    fn evaluate(&self, _context: &PolicyEvaluationContext) -> PolicyDecision {
        PolicyDecision::new(
            PolicyEffect::NotApplicable,
            "policy contract is not bound to enforcement in this phase",
        )
    }
}

/// Minimal policy evaluation context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyEvaluationContext {
    /// Action being evaluated.
    pub action: String,
    /// Subject requesting the action.
    pub subject: String,
    /// Resource targeted by the action.
    pub resource: String,
}

impl PolicyEvaluationContext {
    /// Create an evaluation context.
    ///
    /// # Errors
    ///
    /// Returns [`PolicyError`] when required fields are empty.
    pub fn new(
        action: impl Into<String>,
        subject: impl Into<String>,
        resource: impl Into<String>,
    ) -> Result<Self, PolicyError> {
        let context = Self {
            action: action.into(),
            subject: subject.into(),
            resource: resource.into(),
        };
        validate_required("policy action", &context.action)?;
        validate_required("policy subject", &context.subject)?;
        validate_required("policy resource", &context.resource)?;
        Ok(context)
    }
}

macro_rules! define_policy {
    ($(#[$doc:meta] $name:ident, $kind:expr, $purpose:literal;)+) => {
        $(
            #[$doc]
            #[derive(Clone, Debug)]
            pub struct $name {
                descriptor: PolicyDescriptor,
            }

            impl $name {
                /// Create the policy contract.
                ///
                /// # Errors
                ///
                /// Returns [`PolicyError`] when the descriptor cannot be built.
                pub fn new() -> Result<Self, PolicyError> {
                    Ok(Self {
                        descriptor: PolicyDescriptor::new($kind, $purpose)?,
                    })
                }

                /// Return the policy descriptor.
                #[must_use]
                pub const fn descriptor(&self) -> &PolicyDescriptor {
                    &self.descriptor
                }
            }

            impl Policy for $name {
                fn descriptor(&self) -> &PolicyDescriptor {
                    &self.descriptor
                }
            }
        )+
    };
}

define_policy! {
    /// Security policy contract.
    SecurityPolicy, PolicyKind::Security, "Prepare security behavior constraints";
    /// Filesystem policy contract.
    FilesystemPolicy, PolicyKind::Filesystem, "Prepare filesystem access constraints";
    /// Telemetry policy contract.
    TelemetryPolicy, PolicyKind::Telemetry, "Prepare telemetry retention and emission constraints";
    /// Memory policy contract.
    MemoryPolicy, PolicyKind::Memory, "Prepare future memory retention and access constraints";
    /// Privacy policy contract.
    PrivacyPolicy, PolicyKind::Privacy, "Prepare privacy boundaries and user data constraints";
}

/// Registry for policy descriptors.
#[derive(Clone, Debug, Default)]
pub struct PolicyRegistry {
    policies: BTreeMap<PolicyId, PolicyDescriptor>,
}

impl PolicyRegistry {
    /// Create an empty policy registry.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            policies: BTreeMap::new(),
        }
    }

    /// Create a registry with standard policy contracts.
    ///
    /// # Errors
    ///
    /// Returns [`PolicyError`] when a policy descriptor cannot be built.
    pub fn with_standard_policies() -> Result<Self, PolicyError> {
        let mut registry = Self::new();
        for descriptor in standard_policy_descriptors()? {
            registry.register(descriptor)?;
        }
        Ok(registry)
    }

    /// Register a policy descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`PolicyError`] when the policy is duplicated.
    pub fn register(&mut self, descriptor: PolicyDescriptor) -> Result<(), PolicyError> {
        if self.policies.contains_key(&descriptor.id) {
            return Err(PolicyError::DuplicatePolicy(descriptor.id.to_string()));
        }
        self.policies.insert(descriptor.id.clone(), descriptor);
        Ok(())
    }

    /// Return a policy by identifier.
    #[must_use]
    pub fn get(&self, policy_id: &PolicyId) -> Option<&PolicyDescriptor> {
        self.policies.get(policy_id)
    }

    /// Return all policy descriptors.
    #[must_use]
    pub fn policies(&self) -> Vec<PolicyDescriptor> {
        self.policies.values().cloned().collect()
    }
}

/// Return standard policy descriptors.
///
/// # Errors
///
/// Returns [`PolicyError`] when any descriptor cannot be built.
pub fn standard_policy_descriptors() -> Result<Vec<PolicyDescriptor>, PolicyError> {
    Ok(vec![
        SecurityPolicy::new()?.descriptor().clone(),
        FilesystemPolicy::new()?.descriptor().clone(),
        TelemetryPolicy::new()?.descriptor().clone(),
        MemoryPolicy::new()?.descriptor().clone(),
        PrivacyPolicy::new()?.descriptor().clone(),
    ])
}

/// Policy layer errors.
#[derive(Debug, Error)]
pub enum PolicyError {
    /// Policy field is invalid.
    #[error("invalid policy field: {0}")]
    InvalidField(String),
    /// Policy identifier is invalid.
    #[error("invalid policy id: {0}")]
    Id(#[from] aether_ids::IdError),
    /// Policy is already registered.
    #[error("policy already registered: {0}")]
    DuplicatePolicy(String),
}

fn validate_required(field: &str, value: &str) -> Result<(), PolicyError> {
    if value.trim().is_empty() {
        Err(PolicyError::InvalidField(format!(
            "{field} cannot be empty"
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Policy, PolicyEffect, PolicyEvaluationContext, PolicyKind, PolicyRegistry, SecurityPolicy,
        standard_policy_descriptors,
    };

    #[test]
    fn standard_policies_define_behavior_contracts() {
        let descriptors = standard_policy_descriptors().expect("policies");

        assert_eq!(descriptors.len(), 5);
        assert!(
            descriptors
                .iter()
                .any(|descriptor| descriptor.kind == PolicyKind::Privacy)
        );
        assert!(
            descriptors
                .iter()
                .all(|descriptor| descriptor.id.as_str().starts_with("pol_"))
        );
    }

    #[test]
    fn policy_registry_registers_policy_descriptors() {
        let mut registry = PolicyRegistry::new();
        let descriptor = SecurityPolicy::new().expect("policy").descriptor().clone();
        let policy_id = descriptor.id.clone();

        registry.register(descriptor).expect("register");

        assert!(registry.get(&policy_id).is_some());
        assert_eq!(registry.policies().len(), 1);
    }

    #[test]
    fn policy_contract_does_not_enforce_behavior_yet() {
        let policy = SecurityPolicy::new().expect("policy");
        let context = PolicyEvaluationContext::new(
            "service.command",
            "svc_telemetry-service",
            "system.inspect",
        )
        .expect("context");

        let decision = policy.evaluate(&context);

        assert_eq!(decision.effect, PolicyEffect::NotApplicable);
    }

    #[test]
    fn policy_registry_loads_standard_policies() {
        let registry = PolicyRegistry::with_standard_policies().expect("registry");

        assert_eq!(registry.policies().len(), 5);
    }
}
