//! In Apache APISIX, the admin API is used to:
//!
//! The Admin API lets users control their deployed Apache APISIX instance.
//! The architecture design gives an idea about how everything fits together.

use anyhow::Result;
use reqwest::Response;
use tracing::{debug, instrument};
use crate::client::admin::{path_check_version, path_consumer, path_consumer_group_with_id, path_consumer_groups, path_consumer_with_id, path_route_with_id, path_routes, path_service_with_id, path_services, path_upstream_with_id, path_upstreams};
use crate::client::reqwest_generic::{delete, get, head, put};
use crate::config::ApisixConfig;
use crate::models::{ApisixConsumer, ApisixConsumerGroup, ApisixRoute, ApisixService, ApisixUpstream, ConsumerGroupRequest, ConsumerRequest, RouteRequest, ServiceRequest, UpstreamRequest};
use crate::models::common::{ListResponse, TypedItem};

#[derive(Debug, Default)]
pub struct AdminConnector {
    pub cfg: ApisixConfig,
}

impl AdminConnector {

    #[instrument]
    pub (crate) async fn new(cfg: &ApisixConfig) -> Self {
        AdminConnector {
            cfg: cfg.clone(),
        }
    }

    #[instrument(skip(self))]
    pub (crate) async fn check_version(&self) -> Result<()> {
        let path = format!("{}{}", self.cfg.admin_url, path_check_version());
        debug!("admin_api::check_version: {}", path);
        debug!("admin_api::get_certificate_by_document_number::body {:#?}", "None");
        head(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    // region: upstream api
    #[instrument(skip(self))]
    pub (crate) async fn get_upstreams(&self) -> Result<ListResponse<TypedItem<ApisixUpstream>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstreams());
        debug!("admin_api::get_upstreams: {}", path);
        get::<ListResponse<TypedItem<ApisixUpstream>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn get_upstream(&self, id: &str) -> Result<TypedItem<ApisixUpstream>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::get_upstream: {}", path);
        get::<TypedItem<ApisixUpstream>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn create_upstream_with_id(&self, id: &str, req: &UpstreamRequest) -> Result<TypedItem<ApisixUpstream>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::create_upstream_with_id: {}", path);
        put::<UpstreamRequest, TypedItem<ApisixUpstream>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn delete_upstream(&self, id: &str) -> Result<Response> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::delete_upstream: {}", path);
        delete(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
    // endregion: upstream api

    // region: service api
    #[instrument(skip(self))]
    pub (crate) async fn get_services(&self) -> Result<ListResponse<TypedItem<ApisixService>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_services());
        debug!("admin_api::get_services: {}", path);
        get::<ListResponse<TypedItem<ApisixService>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn get_service(&self, id: &str) -> Result<TypedItem<ApisixService>> {
        let path = format!("{}{}", self.cfg.admin_url, path_service_with_id(id));
        debug!("admin_api::get_service: {}", path);
        get::<TypedItem<ApisixService>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn create_service_with_id(&self, id: &str, req: &ServiceRequest) -> Result<TypedItem<ApisixService>> {
        let path = format!("{}{}", self.cfg.admin_url, path_service_with_id(id));
        debug!("admin_api::create_service_with_id: {}", path);
        put::<ServiceRequest, TypedItem<ApisixService>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn delete_service(&self, id: &str) -> Result<Response> {
        let path = format!("{}{}", self.cfg.admin_url, path_service_with_id(id));
        debug!("admin_api::delete_service: {}", path);
        delete(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
    // endregion: service api

    // region: route api
    #[instrument(skip(self))]
    pub (crate) async fn get_routes(&self) -> Result<ListResponse<TypedItem<ApisixRoute>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_routes());
        debug!("admin_api::get_routes: {}", path);
        get::<ListResponse<TypedItem<ApisixRoute>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn get_route(&self, id: &str) -> Result<TypedItem<ApisixRoute>> {
        let path = format!("{}{}", self.cfg.admin_url, path_route_with_id(id));
        debug!("admin_api::get_route: {}", path);
        get::<TypedItem<ApisixRoute>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn create_route_with_id(&self, id: &str, req: &RouteRequest) -> Result<TypedItem<ApisixRoute>> {
        let path = format!("{}{}", self.cfg.admin_url, path_route_with_id(id));
        debug!("admin_api::create_route_with_id: {}", path);
        put::<RouteRequest, TypedItem<ApisixRoute>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn delete_route(&self, id: &str) -> Result<Response> {
        let path = format!("{}{}", self.cfg.admin_url, path_route_with_id(id));
        debug!("admin_api::delete_route: {}", path);
        delete(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
    // endregion: route api

    // region: consumer group api
    #[instrument(skip(self))]
    pub (crate) async fn get_consumer_groups(&self) -> Result<ListResponse<TypedItem<ApisixConsumerGroup>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer_groups());
        debug!("admin_api::get_consumer_groups: {}", path);
        get::<ListResponse<TypedItem<ApisixConsumerGroup>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn get_consumer_group(&self, id: &str) -> Result<TypedItem<ApisixConsumerGroup>> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer_group_with_id(id));
        debug!("admin_api::get_consumer_group: {}", path);
        get::<TypedItem<ApisixConsumerGroup>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn create_consumer_group_with_id(&self, id: &str, req: &ConsumerGroupRequest) -> Result<TypedItem<ApisixConsumerGroup>> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer_group_with_id(id));
        debug!("admin_api::create_consumer_group_with_id: {}", path);
        put::<ConsumerGroupRequest, TypedItem<ApisixConsumerGroup>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn delete_consumer_group(&self, id: &str) -> Result<Response> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer_group_with_id(id));
        debug!("admin_api::delete_consumer_group: {}", path);
        delete(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
    // endregion: consumer group api

    // region: consumer api
    #[instrument(skip(self))]
    pub (crate) async fn get_consumers(&self) -> Result<ListResponse<TypedItem<ApisixConsumer>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer());
        debug!("admin_api::get_consumers: {}", path);
        get::<ListResponse<TypedItem<ApisixConsumer>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn get_consumer(&self, id: &str) -> Result<TypedItem<ApisixConsumer>> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer_with_id(id));
        debug!("admin_api::get_consumer: {}", path);
        get::<TypedItem<ApisixConsumer>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn create_consumer(&self, id: &str, req: &ConsumerRequest) -> Result<TypedItem<ApisixConsumer>> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer());
        debug!("admin_api::create_consumer: {}", path);
        put::<ConsumerRequest, TypedItem<ApisixConsumer>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub (crate) async fn delete_consumer(&self, id: &str) -> Result<Response> {
        let path = format!("{}{}", self.cfg.admin_url, path_consumer_with_id(id));
        debug!("admin_api::delete_consumer: {}", path);
        delete(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
    // endregion: consumer api

}