#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Local runtime configuration for Aether.

use std::fs;
use std::path::{Path, PathBuf};

use aether_logging::LogLevel;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Aether runtime configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AetherConfig {
    /// Runtime section.
    pub runtime: RuntimeConfig,
    /// Module loading section.
    pub modules: ModuleConfig,
}

impl AetherConfig {
    /// Load configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when the file cannot be read or parsed.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        Self::from_toml_str(&contents)
    }

    /// Load configuration from a TOML string.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError::Parse`] when TOML parsing fails.
    pub fn from_toml_str(contents: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(contents)?)
    }
}

/// Configuration source abstraction for runtime and kernel bootstrapping.
pub trait ConfigProvider: Send + Sync {
    /// Load an [`AetherConfig`] from the provider.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] when the provider cannot produce a valid configuration.
    fn load(&self) -> Result<AetherConfig, ConfigError>;
}

/// Static in-memory configuration provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticConfigProvider {
    config: AetherConfig,
}

impl StaticConfigProvider {
    /// Create a provider that always returns the supplied configuration.
    #[must_use]
    pub const fn new(config: AetherConfig) -> Self {
        Self { config }
    }
}

impl ConfigProvider for StaticConfigProvider {
    fn load(&self) -> Result<AetherConfig, ConfigError> {
        Ok(self.config.clone())
    }
}

/// TOML file configuration provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TomlConfigProvider {
    path: PathBuf,
}

impl TomlConfigProvider {
    /// Create a provider backed by a TOML file path.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Return the configured TOML file path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl ConfigProvider for TomlConfigProvider {
    fn load(&self) -> Result<AetherConfig, ConfigError> {
        AetherConfig::load_from_path(&self.path)
    }
}

/// Runtime-level configuration values.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Runtime instance name.
    pub name: String,
    /// Runtime environment label.
    pub environment: String,
    /// Minimum structured log level.
    pub log_level: LogLevel,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            name: "aether-core".to_owned(),
            environment: "local".to_owned(),
            log_level: LogLevel::Info,
        }
    }
}

/// Module loading configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleConfig {
    /// Whether module loading is enabled.
    pub enabled: bool,
    /// Module identifiers that should be loaded at runtime bootstrap.
    pub load: Vec<String>,
}

/// Configuration loading errors.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Reading the configuration file failed.
    #[error("failed to read configuration: {0}")]
    Read(#[from] std::io::Error),
    /// Parsing the configuration file failed.
    #[error("failed to parse configuration: {0}")]
    Parse(#[from] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use std::fs;

    use aether_logging::LogLevel;

    use super::{AetherConfig, ConfigProvider, StaticConfigProvider, TomlConfigProvider};

    #[test]
    fn default_config_is_local() {
        let config = AetherConfig::default();

        assert_eq!(config.runtime.name, "aether-core");
        assert_eq!(config.runtime.environment, "local");
        assert_eq!(config.runtime.log_level, LogLevel::Info);
        assert!(!config.modules.enabled);
    }

    #[test]
    fn load_config_from_toml() {
        let config = AetherConfig::from_toml_str(
            r#"
            [runtime]
            name = "aether-test"
            environment = "test"
            log_level = "debug"

            [modules]
            enabled = true
            load = ["diagnostics"]
            "#,
        )
        .expect("config");

        assert_eq!(config.runtime.name, "aether-test");
        assert_eq!(config.runtime.environment, "test");
        assert_eq!(config.runtime.log_level, LogLevel::Debug);
        assert_eq!(config.modules.load, ["diagnostics"]);
    }

    #[test]
    fn static_provider_returns_config() {
        let provider = StaticConfigProvider::new(AetherConfig::default());
        let config = provider.load().expect("config");

        assert_eq!(config.runtime.name, "aether-core");
    }

    #[test]
    fn toml_provider_loads_from_path() {
        let path = std::env::temp_dir().join(format!(
            "aether-config-provider-{}.toml",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"
            [runtime]
            name = "provider-test"
            environment = "test"
            log_level = "info"

            [modules]
            enabled = false
            load = []
            "#,
        )
        .expect("write config");

        let provider = TomlConfigProvider::new(&path);
        let config = provider.load().expect("config");
        fs::remove_file(path).expect("remove config");

        assert_eq!(config.runtime.name, "provider-test");
    }
}
