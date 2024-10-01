//! In Apache APISIX, the admin API is used to:
//!
//! The Admin API lets users control their deployed Apache APISIX instance.
//! The architecture design gives an idea about how everything fits together.

use anyhow::Result;
use reqwest::Response;
use serde_json::Value;
use tracing::{debug, info, instrument};
use crate::admin_service_requests::ServiceRequest;
use crate::admin_service_responses::ApisixService;
use crate::client::admin::{path_check_version, path_service_with_id, path_services, path_upstream_with_id, path_upstreams, ADMIN_PATH};
use crate::client::reqwest_generic::{delete, get, head, post, put};
use crate::common_responses::{ListResponse, TypedItem};
use crate::config::ApisixConfig;
use crate::error::ApisixClientError;
use crate::models::admin_upstream_responses::ApisixUpstream;
use crate::models::UpstreamRequest;

#[derive(Debug)]
pub struct AdminConnector {
    pub cfg: ApisixConfig,
}

impl Default for AdminConnector {
    fn default() -> Self {
        AdminConnector {
            cfg: ApisixConfig::default(),
        }
    }
}

impl AdminConnector {

    #[instrument]
    pub async fn new(cfg: &ApisixConfig) -> Self {
        AdminConnector {
            cfg: cfg.clone(),
            ..Default::default()
        }
    }

    #[instrument(skip(self))]
    pub async fn check_version(&self) -> Result<()> {
        let path = format!("{}{}", self.cfg.admin_url, path_check_version());
        debug!("admin_api::check_version: {}", path);
        debug!("admin_api::get_certificate_by_document_number::body {:#?}", "None");
        head(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    // region: upstream api
    #[instrument(skip(self))]
    pub async fn get_upstreams(&self) -> Result<ListResponse<TypedItem<ApisixUpstream>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstreams());
        debug!("admin_api::get_upstreams: {}", path);
        get::<ListResponse<TypedItem<ApisixUpstream>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub async fn get_upstream(&self, id: &str) -> Result<TypedItem<ApisixUpstream>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::get_upstream: {}", path);
        get::<TypedItem<ApisixUpstream>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub async fn create_upstream_with_id(&self, id: &str, req: &UpstreamRequest) -> Result<TypedItem<ApisixUpstream>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::create_upstream_with_id: {}", path);
        put::<UpstreamRequest, TypedItem<ApisixUpstream>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub async fn delete_upstream(&self, id: &str) -> Result<Response> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::delete_upstream: {}", path);
        delete(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
    // endregion: upstream api

    // region: service api
    #[instrument(skip(self))]
    pub async fn get_services(&self) -> Result<ListResponse<TypedItem<ApisixService>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_services());
        debug!("admin_api::get_services: {}", path);
        get::<ListResponse<TypedItem<ApisixService>>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument(skip(self))]
    pub async fn create_service_with_id(&self, id: &str, req: &ServiceRequest) -> Result<TypedItem<ApisixService>> {
        let path = format!("{}{}", self.cfg.admin_url, path_service_with_id(id));
        debug!("admin_api::create_service_with_id: {}", path);
        put::<ServiceRequest, TypedItem<ApisixService>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }
    // endregion: service api

    // create route

    // create consumer

    // create plugin


}