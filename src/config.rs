use crate::{Result};
use std::env;
use std::sync::OnceLock;
use crate::error::ApisixClientError;

/// Loads the Smart ID configuration from the environment variables
pub fn config() -> &'static ApisixConfig {
    static INSTANCE: OnceLock<ApisixConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        ApisixConfig::load_from_env().unwrap_or_else(|e| {
            panic!("FATAL - WHILE LOADING CONF - Cause: {e:?}");
        })
    })
}

/// Apisix Client Configuration
#[derive(Debug, Clone)]
pub struct ApisixConfig {
    pub url: String,
    pub admin_url: String,
    pub control_url: String,
    pub admin_path: String,
}

impl From<ApisixConfig> for ApisixConfigBuilder {
    fn from(config: ApisixConfig) -> Self {
        ApisixConfigBuilder {
            url: Some(config.url),
            admin_url: Some(config.admin_url),
            control_url: Some(config.control_url),
            admin_path: Some(config.admin_path),
        }
    }
}

// region: Config Builder
/// Apisix Configuration Builder
///
/// Use this builder to create a Apisix Admin Configuration using a building pattern
/// # Example
/// ```
/// # use anyhow::Result;
/// # use apisi_admin_client::config::{ApisixConfig, ApisixConfigBuilder};
/// let cfg: Result<ApisixConfig> = ApisixConfigBuilder::new()
///     .url("http://localhost:9080") // DEV environment
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ApisixConfigBuilder {
    pub url: Option<String>,
    pub admin_url: Option<String>,
    pub control_url: Option<String>,
    pub admin_path: Option<String>,
}

impl ApisixConfigBuilder {
    pub fn new() -> Self {
        ApisixConfig::default().into() //from env
    }

    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        let _ = self.url.insert(url.into());
        self
    }

    pub fn admin_url(&mut self, admin_url: impl Into<String>) -> &mut Self {
        let _ = self.admin_url.insert(admin_url.into());
        self
    }

    pub fn control_url(&mut self, control_url: impl Into<String>) -> &mut Self {
        let _ = self.control_url.insert(control_url.into());
        self
    }

    pub fn admin_path(&mut self, admin_path:  impl Into<String>) -> &mut Self {
        let _ = self.admin_path.insert(admin_path);
        self
    }

    pub fn build(&self) -> Result<ApisixConfig> {
        Ok(ApisixConfig {
            url: self.url.clone().ok_or(ApisixClientError::ConfigMissingException("url"))?,
            admin_url: self.admin_url.clone().ok_or(ApisixClientError::ConfigMissingException("admin_url"))?,
            control_url: self.control_url.clone().ok_or(ApisixClientError::ConfigMissingException("control_url"))?,
            admin_path: self.admin_path.clone().ok_or(ApisixClientError::ConfigMissingException("admin_path"))?,
        })
    }

}
// endregion: Config Builder


/// Use this config builder for testing purposes
impl Default for ApisixConfig {
    fn default() -> Self {
        Self {
            url: String::from("http://localhost:9080"),
            admin_url: "http://localhost:9180".to_string(),
            control_url: "http://localhost:9090".to_string(),
            admin_path: "/apisix/admin".to_string(),
        }
    }
}

/// Use this configuration builder
impl ApisixConfig {
    pub fn from_env() -> Self {
        Self::load_from_env().expect("Failed to initialize Apisix Client or load SmartIDConfig from env")
    }
}

impl ApisixConfig {
    pub fn load_from_env() -> Result<ApisixConfig> {
        Ok(ApisixConfig {
            url: get_env("APISIX_URL").unwrap(),
            admin_url: get_env("APISIX_ADMIN_URL").unwrap(),
            control_url: get_env("APISIX_CONTROL_URL").unwrap(),
            admin_path: get_env("APISIX_ADMIN_PATH").unwrap(),
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| ApisixClientError::ConfigMissingException(name).into())
}