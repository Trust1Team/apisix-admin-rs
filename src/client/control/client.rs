//! In Apache APISIX, the control API is used to:
//!
//! Expose the internal state of APISIX.
//! Control the behavior of a single, isolated APISIX data plane.

use anyhow::Result;
use serde_json::Value;
use tracing::{debug, instrument};
use crate::client::reqwest_generic::{get, post_empty_body};
use crate::config::ApisixConfig;
use crate::models::ctrl_responses::CtrlHealthCheckResponse;
// region: Path definitions

fn get_schema() -> String { "/v1/schema".to_string() }
fn get_health_check() -> String { "/v1/healthcheck".to_string() }
fn get_garbage_collect() -> String { "/v1/gc".to_string() }
// endregion: Path definitions

#[derive(Debug, Default)]
pub struct ControllerConnector {
    pub cfg: ApisixConfig,
}

impl ControllerConnector {

    #[instrument]
    pub async fn new(cfg: &ApisixConfig) -> Self {
        ControllerConnector {
            cfg: cfg.clone(),
        }
    }

    #[instrument]
    pub async fn schema(&self) -> Result<Value> {
        let path = format!("{}{}", self.cfg.control_url, get_schema());
        debug!("controller_api::schema: {}", path);
        get::<Value>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument]
    pub async fn health_check(&self) -> Result<CtrlHealthCheckResponse> {
        let path = format!("{}{}", self.cfg.control_url, get_health_check());
        debug!("controller_api::health_check: {}", path);
        get::<CtrlHealthCheckResponse>(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }

    #[instrument]
    pub async fn gc(&self) -> Result<()> {
        let path = format!("{}{}", self.cfg.control_url, get_garbage_collect());
        debug!("controller_api::gc: {}", path);
        post_empty_body(path.as_str(), self.cfg.admin_apikey.as_str(),self.cfg.client_request_timeout).await
    }
}