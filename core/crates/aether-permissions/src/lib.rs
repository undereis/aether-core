#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Internal permission model for Aether services.

use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Permission required by a service action.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Permission(String);

impl Permission {
    /// Create a permission from a canonical dotted name.
    ///
    /// # Errors
    ///
    /// Returns [`PermissionError::InvalidPermission`] when the name is invalid.
    pub fn new(name: impl Into<String>) -> Result<Self, PermissionError> {
        let name = name.into();
        let name = name.trim();
        validate_permission_name(name)?;
        Ok(Self(name.to_owned()))
    }

    /// Permission required to publish events.
    #[must_use]
    pub fn event_publish() -> Self {
        Self("event.publish".to_owned())
    }

    /// Permission required to subscribe to events.
    #[must_use]
    pub fn event_subscribe() -> Self {
        Self("event.subscribe".to_owned())
    }

    /// Permission required to read configuration.
    #[must_use]
    pub fn config_read() -> Self {
        Self("config.read".to_owned())
    }

    /// Permission required to emit telemetry.
    #[must_use]
    pub fn telemetry_emit() -> Self {
        Self("telemetry.emit".to_owned())
    }

    /// Permission required to send service commands.
    #[must_use]
    pub fn service_command() -> Self {
        Self("service.command".to_owned())
    }

    /// Permission required to inspect service metadata.
    #[must_use]
    pub fn service_inspect() -> Self {
        Self("service.inspect".to_owned())
    }

    /// Return the permission name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Set of permissions requested by a service.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PermissionSet {
    requested: Vec<Permission>,
}

impl PermissionSet {
    /// Create a permission set from validated permissions.
    #[must_use]
    pub fn new(requested: Vec<Permission>) -> Self {
        let mut unique = Vec::new();
        for permission in requested {
            if !unique.contains(&permission) {
                unique.push(permission);
            }
        }
        Self { requested: unique }
    }

    /// Create a permission set from canonical names.
    ///
    /// # Errors
    ///
    /// Returns [`PermissionError`] when any permission name is invalid.
    pub fn from_names(names: &[String]) -> Result<Self, PermissionError> {
        let mut permissions = Vec::with_capacity(names.len());
        for name in names {
            permissions.push(Permission::new(name)?);
        }
        Ok(Self::new(permissions))
    }

    /// Return requested permissions.
    #[must_use]
    pub fn requested(&self) -> &[Permission] {
        &self.requested
    }

    /// Return whether this set contains the provided permission.
    #[must_use]
    pub fn contains(&self, permission: &Permission) -> bool {
        self.requested.contains(permission)
    }

    /// Return whether this set contains a permission by name.
    ///
    /// # Errors
    ///
    /// Returns [`PermissionError`] when the provided permission name is invalid.
    pub fn contains_name(&self, name: impl Into<String>) -> Result<bool, PermissionError> {
        Ok(self.contains(&Permission::new(name)?))
    }
}

/// Permission validation errors.
#[derive(Debug, Error)]
pub enum PermissionError {
    /// Permission name is invalid.
    #[error("invalid permission name: {0}")]
    InvalidPermission(String),
}

fn validate_permission_name(name: &str) -> Result<(), PermissionError> {
    let valid = !name.is_empty()
        && name.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || character == '.'
                || character == '-'
        });

    if valid {
        Ok(())
    } else {
        Err(PermissionError::InvalidPermission(name.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::{Permission, PermissionError, PermissionSet};

    #[test]
    fn permission_rejects_invalid_name() {
        let result = Permission::new("Event.Publish");

        assert!(matches!(result, Err(PermissionError::InvalidPermission(_))));
    }

    #[test]
    fn permission_set_verifies_declared_permission() {
        let permissions = PermissionSet::new(vec![
            Permission::event_publish(),
            Permission::config_read(),
            Permission::event_publish(),
        ]);

        assert_eq!(permissions.requested().len(), 2);
        assert!(permissions.contains(&Permission::event_publish()));
        assert!(!permissions.contains(&Permission::service_command()));
    }
}
