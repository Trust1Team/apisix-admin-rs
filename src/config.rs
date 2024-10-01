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
    pub admin_apikey: String,
    pub client_request_timeout: u64,
}

impl From<ApisixConfig> for ApisixConfigBuilder {
    fn from(config: ApisixConfig) -> Self {
        ApisixConfigBuilder {
            url: Some(config.url),
            admin_url: Some(config.admin_url),
            control_url: Some(config.control_url),
            admin_apikey: Some(config.admin_apikey),
            client_request_timeout: Some(config.client_request_timeout),
        }
    }
}

// region: Config Builder
/// Apisix Configuration Builder
///
/// Use this builder to create an Apisix Admin Configuration using a building pattern
/// # Example
/// ```
/// # use apisix_admin_client::error::ApisixClientError;
/// # type Result<T> = std::result::Result<T, ApisixClientError>;
/// # use apisix_admin_client::config::{ApisixConfig, ApisixConfigBuilder};
/// let cfg: Result<ApisixConfig> = ApisixConfigBuilder::new()
///     .url("http://localhost:9080") // DEV environment
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ApisixConfigBuilder {
    url: Option<String>,
    admin_url: Option<String>,
    control_url: Option<String>,
    admin_apikey: Option<String>,
    client_request_timeout: Option<u64>,
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
        let _ = self.admin_apikey.insert(admin_path.into());
        self
    }

    pub fn client_request_timeout(&mut self, client_request_timeout: u64) -> &mut Self {
        let _ = self.client_request_timeout.insert(client_request_timeout);
        self
    }

    pub fn build(&self) -> Result<ApisixConfig> {
        Ok(ApisixConfig {
            url: self.url.clone().ok_or(ApisixClientError::ConfigMissingException("url"))?,
            admin_url: self.admin_url.clone().ok_or(ApisixClientError::ConfigMissingException("admin_url"))?,
            control_url: self.control_url.clone().ok_or(ApisixClientError::ConfigMissingException("control_url"))?,
            admin_apikey: self.admin_apikey.clone().ok_or(ApisixClientError::ConfigMissingException("admin_path"))?,
            client_request_timeout: self.client_request_timeout.clone().ok_or(ApisixClientError::ConfigMissingException("admin_path"))?,
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
            admin_apikey: "edd1c9f034335f136f87ad84b625c8f1".to_string(),
            client_request_timeout: 30000,
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
            url: get_env("APISIX_URL")?,
            admin_url: get_env("APISIX_ADMIN_URL")?,
            control_url: get_env("APISIX_CONTROL_URL")?,
            admin_apikey: get_env("APISIX_ADMIN_API_KEY")?,
            client_request_timeout: get_env_u64("CLIENT_REQ_NETWORK_TIMEOUT_MILLIS")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| ApisixClientError::ConfigMissingException(name).into())
}

fn get_env_u64(name: &'static str) -> Result<u64> {
    env::var(name).unwrap().parse().map_err(|_| ApisixClientError::ConfigMissingException(name).into())
}