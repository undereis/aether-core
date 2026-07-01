#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Resource declaration model for Aether services.

use serde::{Deserialize, Serialize};

/// CPU class requested by a service.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum CpuClass {
    /// Low CPU allocation.
    #[default]
    Low,
    /// Medium CPU allocation.
    Medium,
    /// High CPU allocation.
    High,
}

/// Memory class requested by a service.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum MemoryClass {
    /// Low memory allocation.
    #[default]
    Low,
    /// Medium memory allocation.
    Medium,
    /// High memory allocation.
    High,
}

/// Storage class requested by a service.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum StorageClass {
    /// No persistent storage.
    #[default]
    None,
    /// Low storage allocation.
    Low,
    /// Medium storage allocation.
    Medium,
    /// High storage allocation.
    High,
}

/// Filesystem access requested by a service.
#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FilesystemAccess {
    /// No filesystem access.
    #[default]
    None,
    /// Read-only filesystem access.
    ReadOnly,
    /// Read-write filesystem access.
    ReadWrite,
}

/// Resource profile declared by a service manifest.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResourceProfile {
    /// CPU class.
    #[serde(default)]
    pub cpu_class: CpuClass,
    /// Memory class.
    #[serde(default)]
    pub memory_class: MemoryClass,
    /// Storage class.
    #[serde(default)]
    pub storage_class: StorageClass,
    /// Whether network access is requested.
    #[serde(default, alias = "network")]
    pub network_access: bool,
    /// Filesystem access level.
    #[serde(default)]
    pub filesystem_access: FilesystemAccess,
}

impl Default for ResourceProfile {
    fn default() -> Self {
        Self {
            cpu_class: CpuClass::Low,
            memory_class: MemoryClass::Low,
            storage_class: StorageClass::None,
            network_access: false,
            filesystem_access: FilesystemAccess::None,
        }
    }
}

impl ResourceProfile {
    /// Return whether the service requests network access.
    #[must_use]
    pub const fn requests_network(&self) -> bool {
        self.network_access
    }

    /// Return whether the service requests filesystem access.
    #[must_use]
    pub const fn requests_filesystem(&self) -> bool {
        !matches!(self.filesystem_access, FilesystemAccess::None)
    }
}

#[cfg(test)]
mod tests {
    use super::{CpuClass, FilesystemAccess, ResourceProfile, StorageClass};

    #[test]
    fn default_resource_profile_is_restricted() {
        let profile = ResourceProfile::default();

        assert_eq!(profile.cpu_class, CpuClass::Low);
        assert_eq!(profile.storage_class, StorageClass::None);
        assert!(!profile.requests_network());
        assert!(!profile.requests_filesystem());
    }

    #[test]
    fn resource_profile_supports_filesystem_access() {
        let profile = ResourceProfile {
            filesystem_access: FilesystemAccess::ReadOnly,
            ..ResourceProfile::default()
        };

        assert!(profile.requests_filesystem());
    }
}
