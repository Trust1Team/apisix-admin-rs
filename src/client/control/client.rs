//! In Apache APISIX, the control API is used to:
//!
//! Expose the internal state of APISIX.
//! Control the behavior of a single, isolated APISIX data plane.

use crate::models::common::{SemanticsIdentifier};
use crate::models::session::SessionStatus;
use anyhow::Result;
use serde_json::Value;
use tracing::{debug, info, instrument};
use crate::client::reqwest_generic::{get, head, post, post_empty_body};
use crate::models::requests::{AuthenticationSessionRequest, CertificateRequest, SignatureSessionRequest};
use crate::models::responses::{AuthenticationSessionResponse, CertificateChoiceResponse, SignatureSessionResponse};
use crate::config::ApisixConfig;
use crate::error::ApisixClientError;
use crate::models::ctrl_api_responses::CtrlHealthCheckResponse;
// region: Path definitions

fn get_schema() -> String { format!("{}", "/v1/schema") }
fn get_health_check() -> String { format!("{}", "/v1/healthcheck") }
fn get_garbage_collect() -> String { format!("{}", "/v1/gc") }
// endregion: Path definitions

#[derive(Debug)]
pub struct ControllerConnector {
    pub cfg: ApisixConfig,
}

impl Default for ControllerConnector {
    fn default() -> Self {
        ControllerConnector {
            cfg: ApisixConfig::default(),
        }
    }
}

impl ControllerConnector {

    #[instrument]
    pub async fn new(cfg: &ApisixConfig) -> Self {
        ControllerConnector {
            cfg: cfg.clone(),
            ..Default::default()
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