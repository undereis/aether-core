#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Typed identifier strategy for Aether core components.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Stable identifier prefix used by a typed Aether identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum IdPrefix {
    /// Event identifiers use the `evt_` prefix.
    Event,
    /// Module identifiers use the `mod_` prefix.
    Module,
    /// Kernel identifiers use the `ker_` prefix.
    Kernel,
    /// Capability identifiers use the `cap_` prefix.
    Capability,
    /// Service identifiers use the `svc_` prefix.
    Service,
}

impl IdPrefix {
    /// Return the canonical textual prefix without the separator.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Event => "evt",
            Self::Module => "mod",
            Self::Kernel => "ker",
            Self::Capability => "cap",
            Self::Service => "svc",
        }
    }
}

impl fmt::Display for IdPrefix {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Prefix-safe identifier value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TypedId(String);

impl TypedId {
    /// Create an identifier with the provided prefix and suffix.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the suffix is empty or contains unsupported characters.
    pub fn new(prefix: IdPrefix, suffix: impl Into<String>) -> Result<Self, IdError> {
        let suffix = suffix.into();
        let suffix = suffix.trim();
        validate_suffix(suffix)?;
        Ok(Self(format!("{}_{suffix}", prefix.as_str())))
    }

    /// Generate a time-sortable `UUIDv7` identifier using the provided prefix.
    #[must_use]
    pub fn generate(prefix: IdPrefix) -> Self {
        Self(format!(
            "{}_{}",
            prefix.as_str(),
            Uuid::now_v7().as_simple()
        ))
    }

    /// Parse and validate a complete typed identifier.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the value does not match the expected prefix or suffix policy.
    pub fn parse(prefix: IdPrefix, value: impl Into<String>) -> Result<Self, IdError> {
        let value = value.into();
        let expected_prefix = format!("{}_", prefix.as_str());
        if !value.starts_with(&expected_prefix) {
            return Err(IdError::InvalidPrefix {
                expected: prefix.as_str(),
                value,
            });
        }

        let suffix = &value[expected_prefix.len()..];
        validate_suffix(suffix)?;
        Ok(Self(value))
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Return whether this identifier starts with the expected prefix.
    #[must_use]
    pub fn has_prefix(&self, prefix: IdPrefix) -> bool {
        self.0.starts_with(&format!("{}_", prefix.as_str()))
    }
}

impl fmt::Display for TypedId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Typed event identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EventId(TypedId);

impl EventId {
    /// Generate a typed event identifier.
    #[must_use]
    pub fn generate() -> Self {
        Self(TypedId::generate(IdPrefix::Event))
    }

    /// Create a typed event identifier from a stable suffix.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the suffix is invalid.
    pub fn new(suffix: impl Into<String>) -> Result<Self, IdError> {
        Ok(Self(TypedId::new(IdPrefix::Event, suffix)?))
    }

    /// Return the raw typed identifier.
    #[must_use]
    pub const fn typed(&self) -> &TypedId {
        &self.0
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Typed module identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModuleId(TypedId);

impl ModuleId {
    /// Generate a typed module identifier.
    #[must_use]
    pub fn generate() -> Self {
        Self(TypedId::generate(IdPrefix::Module))
    }

    /// Create a typed module identifier from a stable suffix.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the suffix is invalid.
    pub fn new(suffix: impl Into<String>) -> Result<Self, IdError> {
        Ok(Self(TypedId::new(IdPrefix::Module, suffix)?))
    }

    /// Return the raw typed identifier.
    #[must_use]
    pub const fn typed(&self) -> &TypedId {
        &self.0
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ModuleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Typed kernel identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct KernelId(TypedId);

impl KernelId {
    /// Generate a typed kernel identifier.
    #[must_use]
    pub fn generate() -> Self {
        Self(TypedId::generate(IdPrefix::Kernel))
    }

    /// Create a typed kernel identifier from a stable suffix.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the suffix is invalid.
    pub fn new(suffix: impl Into<String>) -> Result<Self, IdError> {
        Ok(Self(TypedId::new(IdPrefix::Kernel, suffix)?))
    }

    /// Return the raw typed identifier.
    #[must_use]
    pub const fn typed(&self) -> &TypedId {
        &self.0
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for KernelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Typed capability identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityId(TypedId);

impl CapabilityId {
    /// Generate a typed capability identifier.
    #[must_use]
    pub fn generate() -> Self {
        Self(TypedId::generate(IdPrefix::Capability))
    }

    /// Create a typed capability identifier from a stable suffix.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the suffix is invalid.
    pub fn new(suffix: impl Into<String>) -> Result<Self, IdError> {
        Ok(Self(TypedId::new(IdPrefix::Capability, suffix)?))
    }

    /// Return the raw typed identifier.
    #[must_use]
    pub const fn typed(&self) -> &TypedId {
        &self.0
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Typed service identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServiceId(TypedId);

impl ServiceId {
    /// Generate a typed service identifier.
    #[must_use]
    pub fn generate() -> Self {
        Self(TypedId::generate(IdPrefix::Service))
    }

    /// Create a typed service identifier from a stable suffix.
    ///
    /// # Errors
    ///
    /// Returns [`IdError`] when the suffix is invalid.
    pub fn new(suffix: impl Into<String>) -> Result<Self, IdError> {
        Ok(Self(TypedId::new(IdPrefix::Service, suffix)?))
    }

    /// Return the raw typed identifier.
    #[must_use]
    pub const fn typed(&self) -> &TypedId {
        &self.0
    }

    /// Return the identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for ServiceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Identifier validation errors.
#[derive(Debug, Error)]
pub enum IdError {
    /// Identifier suffix was empty.
    #[error("typed id suffix cannot be empty")]
    EmptySuffix,
    /// Identifier suffix contains unsupported characters.
    #[error("typed id suffix contains unsupported characters: {0}")]
    InvalidCharacters(String),
    /// Identifier did not start with the expected prefix.
    #[error("typed id expected prefix {expected}_ but got {value}")]
    InvalidPrefix {
        /// Expected prefix without separator.
        expected: &'static str,
        /// Provided identifier value.
        value: String,
    },
}

fn validate_suffix(suffix: &str) -> Result<(), IdError> {
    if suffix.is_empty() {
        return Err(IdError::EmptySuffix);
    }

    let is_valid = suffix
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-');

    if is_valid {
        Ok(())
    } else {
        Err(IdError::InvalidCharacters(suffix.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{CapabilityId, IdError, IdPrefix, KernelId, ServiceId, TypedId};

    #[test]
    fn generated_kernel_id_uses_kernel_prefix() {
        let id = KernelId::generate();

        assert!(id.as_str().starts_with("ker_"));
    }

    #[test]
    fn typed_id_rejects_invalid_suffix() {
        let result = TypedId::new(IdPrefix::Module, "bad suffix");

        assert!(matches!(result, Err(IdError::InvalidCharacters(_))));
    }

    #[test]
    fn typed_id_parses_expected_prefix() {
        let id = TypedId::parse(IdPrefix::Event, "evt_01HZX").expect("typed id");

        assert!(id.has_prefix(IdPrefix::Event));
    }

    #[test]
    fn capability_id_uses_capability_prefix() {
        let id = CapabilityId::new("events_publish").expect("capability id");

        assert_eq!(id.as_str(), "cap_events_publish");
    }

    #[test]
    fn service_id_uses_service_prefix() {
        let id = ServiceId::new("telemetry-service").expect("service id");

        assert_eq!(id.as_str(), "svc_telemetry-service");
    }
}
