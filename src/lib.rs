//! Apisix Client Library
//!
//! Provides a REST models for service consumers of Apisix.
//!
//! Maintained by [Trust1Team](https://trust1team.com) for [Apisix](https://apisix.apache.org/)

use serde_json::Value;
use crate::config::ApisixConfig;
pub mod client;
pub mod error;
pub mod models;
pub mod config;

pub mod client_admin_impl;
pub mod client_ctrl_impl;

type Result<T> = std::result::Result<T, crate::error::ApisixClientError>;

/// Common models are exposed
use crate::client_admin_impl::{api_admin_check_version, api_admin_get_services, api_admin_get_upstreams};
use crate::client_ctrl_impl::api_ctrl_schema;
use crate::models::{ApisixConsumer, ApisixConsumerGroup, ApisixRoute, ApisixService, ApisixUpstream, ConsumerGroupRequest, ConsumerRequest, RouteRequest, ServiceRequest};
use crate::models::common::{ListResponse, TypedItem};
use crate::models::ctrl_responses::CtrlHealthCheckResponse;
use crate::models::UpstreamRequest;

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
pub async fn admin_get_upstreams(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixUpstream>>> {
    api_admin_get_upstreams(cfg).await
}

/// Fetches specified Upstream by id
pub async fn admin_get_upstream(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixUpstream>> {
    client_admin_impl::api_admin_get_upstream(cfg, id).await
}

/// Creates or updates a Route with the specified id
pub async fn admin_create_upstream_with_id(cfg: &ApisixConfig, id: &str, req: &UpstreamRequest) -> Result<TypedItem<ApisixUpstream>> {
    client_admin_impl::api_admin_create_upstream_with_id(cfg, id, req).await
}

/*/// Creates an Upstream and assigns a random id
/// The default behaviour is that Apisix generated an ID for you, but that causes problems as Apisix
/// allows the use of integers or strings for id's.
/// We force a random generated string, fallback is the default Apisix behaviour
pub async fn admin_create_upstream(cfg: &ApisixConfig, req: &UpstreamRequest) -> Result<TypedItem<ApisixUpstream>> {
    client_admin_impl::api_admin_create_upstream(cfg, req).await
}*/

/// Removes the Upstream with the specified id
pub async fn admin_delete_upstream(cfg: &ApisixConfig, id: &str) -> Result<()> {
    client_admin_impl::api_admin_delete_upstream(cfg, id).await
}

/// Fetches a list of available Services
pub async fn admin_get_services(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixService>>> {
    api_admin_get_services(cfg).await
}

/// Fetches specified Service by id
pub async fn admin_get_service(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixService>> {
    client_admin_impl::api_admin_get_service(cfg, id).await
}

/// Creates or updates a Service with the specified id
pub async fn admin_create_service_with_id(cfg: &ApisixConfig, id: &str, req: &ServiceRequest) -> Result<TypedItem<ApisixService>> {
    client_admin_impl::api_admin_create_service_with_id(cfg, id, req).await
}

/// Removes the Service with the specified id
pub async fn admin_delete_service(cfg: &ApisixConfig, id: &str) -> Result<()> {
    client_admin_impl::api_admin_delete_service(cfg, id).await
}

/// Fetches a list of all configured Routes
pub async fn admin_get_routes(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixRoute>>>
{
    client_admin_impl::api_admin_get_routes(cfg).await
}

/// Fetches specified Route by id
pub async fn admin_get_route(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixRoute>> {
    client_admin_impl::api_admin_get_route(cfg, id).await
}

/// Creates or updates a Route with the specified id
pub async fn admin_create_route_with_id(cfg: &ApisixConfig, id: &str, req: &RouteRequest) -> Result<TypedItem<ApisixRoute>> {
    client_admin_impl::api_admin_create_route_with_id(cfg, id, req).await
}

/// Removes the Route with the specified id
pub async fn admin_delete_route(cfg: &ApisixConfig, id: &str) -> Result<()> {
    client_admin_impl::api_admin_delete_route(cfg, id).await
}

/// Fetches a list of all configured Consumer Groups
pub async fn admin_get_consumer_groups(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixConsumerGroup>>> {
    client_admin_impl::api_admin_get_consumer_groups(cfg).await
}

/// Fetches specified Consumer group by id
pub async fn admin_get_consumer_group(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixConsumerGroup>> {
    client_admin_impl::api_admin_get_consumer_group(cfg, id).await
}

/// Creates or updates a new Consumer group with the specified id
pub async fn admin_create_consumer_group_with_id(cfg: &ApisixConfig, id: &str, req: &ConsumerGroupRequest) -> Result<TypedItem<ApisixConsumerGroup>> {
    client_admin_impl::api_admin_create_consumer_group_with_id(cfg, id, req).await
}

/// Removes the Consumer group with the specified id
pub async fn admin_delete_consumer_group(cfg: &ApisixConfig, id: &str) -> Result<()> {
    client_admin_impl::api_admin_delete_consumer_group(cfg, id).await
}

/// Fetches a list of all Consumers
pub async fn admin_get_consumers(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixConsumer>>> {
    client_admin_impl::api_admin_get_consumers(cfg).await
}

/// Fetches specified Consumer by username
pub async fn admin_get_consumer(cfg: &ApisixConfig, username: &str) -> Result<TypedItem<ApisixConsumer>> {
    client_admin_impl::api_admin_get_consumer(cfg, username).await
}

/// Create new Consumer
pub async fn admin_create_consumer_with_name(cfg: &ApisixConfig, id: &str, req: &ConsumerRequest) -> Result<TypedItem<ApisixConsumer>> {
    client_admin_impl::api_admin_create_consumer(cfg, id, req).await
}

/// Removes the Consumer with the specified username
pub async fn admin_delete_consumer(cfg: &ApisixConfig, username: &str) -> Result<()> {
    client_admin_impl::api_admin_delete_consumer(cfg, username).await
}
// region: controller
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
// endregion: controller

