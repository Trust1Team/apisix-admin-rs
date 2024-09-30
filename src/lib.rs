//! Apisix Client Library
//!
//! Provides a REST models for service consumers of Apisix.
//!
//! Maintained by [Trust1Team](https://trust1team.com) for [Apisix](https://apisix.apache.org/)

use anyhow::Result;
use serde_json::Value;
mod models;
use crate::config::ApisixConfig;
pub mod client;
mod error;
pub mod config;
pub use models::*;
pub mod client_admin_impl;
pub mod client_ctrl_impl;

/// Common models are exposed
pub use models::common;
use crate::client_admin_impl::{api_admin_check_version, api_admin_get_upstreams};
use crate::client_ctrl_impl::api_ctrl_schema;
use crate::models::admin_api_responses::{ListResponse, TypedItem, Upstream};
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

/// Fetch a list of all configured Upstreams
pub async fn admin_get_upstreams(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<Upstream>>> {
    api_admin_get_upstreams(cfg).await
}

/// Fetches specified Upstream by id
pub async fn admin_get_upstream(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<Upstream>> {
    client_admin_impl::api_admin_get_upstream(cfg, id).await
}

/// Creates a Route with the specified id
pub async fn admin_create_upstream_with_id(cfg: &ApisixConfig, id: &str, req: &UpstreamRequest) -> Result<TypedItem<Upstream>> {
    client_admin_impl::api_admin_create_upstream_with_id(cfg, id, req).await
}

/// Creates an Upstream and assigns a random id
pub async fn admin_create_upstream(cfg: &ApisixConfig, req: &UpstreamRequest) -> Result<TypedItem<Upstream>> {
    client_admin_impl::api_admin_create_upstream(cfg, req).await
}

/// Removes the Upstream with the specified id
pub async fn admin_delete_upstream(cfg: &ApisixConfig, id: &str) -> Result<()> {
    client_admin_impl::api_admin_delete_upstream(cfg, id).await
}

/// Returns the JSON schema used by the APISIX instance (untyped JSON)
pub async  fn ctrl_schema(cfg: &ApisixConfig) -> Result<Value> {
    api_ctrl_schema(cfg).await
}

/// Returns a health check of the APISIX instance
pub async fn ctrl_health_check(cfg: &ApisixConfig) -> Result<CtrlHealthCheckResponse> {
    client_ctrl_impl::api_ctrl_health_check(cfg).await
}

/// Triggers a full garbage collection in the HTTP subsystem.
/// Note: When stream proxy is enabled, APISIX runs another Lua VM for the stream subsystem.
/// Full garbage collection is not triggered in this VM.
pub async fn ctrl_garbage_collect(cfg: &ApisixConfig) -> Result<()> {
    client_ctrl_impl::api_ctrl_garbage_collect(cfg).await
}


