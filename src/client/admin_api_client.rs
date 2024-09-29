//! In Apache APISIX, the admin API is used to:
//!
//! The Admin API lets users control their deployed Apache APISIX instance.
//! The architecture design gives an idea about how everything fits together.

use crate::models::common::{SemanticsIdentifier};
use crate::models::session::SessionStatus;
use anyhow::Result;
use tracing::{debug, info, instrument};
use crate::client::reqwest_generic::{get, head, post};
use crate::models::requests::{AuthenticationSessionRequest, CertificateRequest, SignatureSessionRequest};
use crate::models::responses::{AuthenticationSessionResponse, CertificateChoiceResponse, SignatureSessionResponse};
use crate::config::ApisixConfig;
use crate::error::ApisixClientError;

// region: Path definitions
const ADMIN_PATH: &'static str = "/apisix/admin";

fn patch_check_version() -> String {
    format!("{}", ADMIN_PATH)
}
// endregion: Path definitions

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
        let path = format!("{}{}", self.cfg.admin_url, patch_check_version());
        debug!("admin_api::check_version: {}", path);
        debug!("admin_api::get_certificate_by_document_number::body {:#?}", "None");
        head(path.as_str(), self.cfg.admin_apikey.as_str(), self.cfg.client_request_timeout).await
    }
}