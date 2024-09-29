use crate::models::common::{SemanticsIdentifier};
use crate::models::session::SessionStatus;
use anyhow::Result;
use tracing::{debug, info, instrument};
use crate::client::reqwest_generic::{get, post};
use crate::models::requests::{AuthenticationSessionRequest, CertificateRequest, SignatureSessionRequest};
use crate::models::responses::{AuthenticationSessionResponse, CertificateChoiceResponse, SignatureSessionResponse};
use crate::config::ApisixConfig;
use crate::error::ApisixClientError;

// region: Path definitions
const PATH_SESSION_STATUS_URI: &'static str = "/session";

fn path_session_status_uri(session_id: String) -> String {
    format!("{}/{}", PATH_SESSION_STATUS_URI, session_id)
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
    pub async fn new_with_time_interval(cfg: ApisixConfig) -> Self {
        AdminConnector {
            cfg,
            ..Default::default()
        }
    }

    #[instrument]
    pub async fn new(cfg: &ApisixConfig) -> Self {
        AdminConnector {
            cfg: cfg.clone(),
            ..Default::default()
        }
    }

    /// Request long poll timeout value. If not provided, a default is used.
    /// This parameter is used for a long poll method, meaning the request method might not return until a timeout expires
    #[instrument]
    pub async fn get_session_status(&self, session_id: &str) -> Result<SessionStatus> {
        let path = format!("{}{}", self.cfg.url, path_session_status_uri(session_id.into()));
        debug!("smart_id_client::get_session_status: {}", path);
        match get::<SessionStatus>(path.as_str(), self.cfg.client_request_timeout).await {
            Ok(res) => {
                info!("smart_id_client::get_session_status::SESSION_STATUS: {:#?}", res.state);
                if res.state == "COMPLETE" {
                    Ok(res)
                } else {
                    Err(ApisixClientError::SessionRetryException.into())
                }
            }
            Err(e) => {
                info!("smart_id_client::get_session_status::ERROR: {:#?}", e);
                Err(ApisixClientError::SessionTimeoutException.into())
            }
        }
    }

    #[instrument]
    pub async fn get_certificate_by_document_number(&self, document_number: String, req: &CertificateRequest) -> Result<CertificateChoiceResponse> {
        let path = format!("{}{}", self.cfg.url, path_certificate_choice_by_document_number(document_number));
        debug!("smart_id_client::get_certificate_by_document_number: {}", path);
        debug!("smart_id_client::get_certificate_by_document_number::body {:#?}", serde_json::to_string(req));
        post::<CertificateRequest, CertificateChoiceResponse>(path.as_str(), req, self.cfg.client_request_timeout).await
    }
}