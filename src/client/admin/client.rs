//! In Apache APISIX, the admin API is used to:
//!
//! The Admin API lets users control their deployed Apache APISIX instance.
//! The architecture design gives an idea about how everything fits together.

use anyhow::Result;
use serde_json::Value;
use tracing::{debug, info, instrument};
use crate::client::admin::{path_check_version, path_upstream_with_id, path_upstreams, ADMIN_PATH};
use crate::client::reqwest_generic::{get, head, post, put};
use crate::config::ApisixConfig;
use crate::error::ApisixClientError;
use crate::models::admin_api_responses::{GenericJsonResponse, ListResponse, TypedItem, Upstream};
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

    #[instrument]
    pub async fn check_version(&self) -> Result<()> {
        let path = format!("{}{}", self.cfg.admin_url, path_check_version());
        debug!("admin_api::check_version: {}", path);
        debug!("admin_api::get_certificate_by_document_number::body {:#?}", "None");
        head(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    // region: upstream api
    pub async fn get_upstreams(&self) -> Result<ListResponse<TypedItem<Upstream>>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstreams());
        debug!("admin_api::get_upstreams: {}", path);
        get::<ListResponse<TypedItem<Upstream>>>(path.as_str(), self.cfg.admin_apikey.as_str() , self.cfg.client_request_timeout).await
    }

    pub async fn get_upstream(&self, id: &str) -> Result<TypedItem<Upstream>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::get_upstream: {}", path);
        get::<TypedItem<Upstream>>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    pub async fn create_upstream_with_id(&self, id: &str, req: &UpstreamRequest) -> Result<TypedItem<Upstream>> {
        let path = format!("{}{}", self.cfg.admin_url, path_upstream_with_id(id));
        debug!("admin_api::create_upstream_with_id: {}", path);
        put::<UpstreamRequest, TypedItem<Upstream>>(path.as_str(), self.cfg.admin_apikey.as_str(), req, self.cfg.client_request_timeout).await
    }
    // endregion: upstream api

    // create service

    // create route

    // create consumer

    // create plugin


}