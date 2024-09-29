//! Apisix Client Library
//!
//! Provides a REST models for service consumers of Apisix.
//!
//! Maintained by [Trust1Team](https://trust1team.com) for [Apisix](https://apisix.apache.org/)

use anyhow::Result;
use serde_json::Value;
use crate::config::ApisixConfig;
pub mod client;
mod error;
pub mod utils;
pub mod config;
mod models;
pub mod client_admin_impl;
pub mod client_ctrl_impl;

/// Common models are exposed
pub use models::common;
use crate::client_admin_impl::{api_admin_check_version};
use crate::client_ctrl_impl::api_ctrl_schema;
use crate::models::ctrl_api_responses::CtrlHealthCheckResponse;

/// Get configuration based on the environment variables (default config override)
/// Function will panic when the environment variables are not set
/// The following variables must be set:
///| ENV_VAR                                   | DESCRIPTION                                                                    | REQUIRED       |
///|-------------------------------------------|--------------------------------------------------------------------------------|----------------|
///| APISIX_URL                                | The Apisix gateway url for operational traffic                                 | N - default () |
///| APISIX_ADMIN_URL                          | The Apisix admin api url                                                       | N - default () |
///| APISIX_CONTROL_URL                        | The Apisix control api url                                                     | N - default () |
///| APISIX_ADMIN_API_KEY                      | The Apisix admin api key                                                       | N - default () |
pub async fn get_config_from_env() -> ApisixConfig {
    ApisixConfig::from_env()
}

/// Get configuration based on the Apisix development environment variables
/// Function is used during development, and in combination with a Smart ID demo client application
pub async fn get_config_default() -> ApisixConfig {
    ApisixConfig::default()
}

/// Check if the Admin API is up and running
pub async fn admin_check(cfg: &ApisixConfig) -> Result<()> {
    api_admin_check_version(cfg).await
}

/// Returns the JSON schema used by the APISIX instance (untyped JSON)
pub async  fn ctrl_schema(cfg: &ApisixConfig) -> Result<Value> {
    api_ctrl_schema(cfg).await
}

/// Returns a health check of the APISIX instance
pub async fn ctrl_health_check(cfg: &ApisixConfig) -> Result<CtrlHealthCheckResponse> {
    client_ctrl_impl::api_ctrl_health_check(cfg).await
}



