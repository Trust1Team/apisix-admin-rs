use tracing::instrument;
use anyhow::Result;
use crate::client::admin_api_client::AdminConnector;
use crate::config::ApisixConfig;
use crate::error::ApisixClientError;
use crate::models::session::SessionStatus;

/// Get certificate by semantic identifier
/// When successful, the session id is used to poll the result
#[instrument(skip_all)]
pub async fn ctrl_get_certificate_by_document_number(cfg: &ApisixConfig, doc_nr: impl Into<String>) -> Result<SessionStatus> {
    let sc: AdminConnector =  AdminConnector::new(cfg).await;
    let req = CertificateRequest::new(cfg).await;
    match sc.get_certificate_by_document_number(doc_nr.into(), &req).await {
        Ok(r) => ctrl_poll_session_status(cfg, r.session_id).await,
        Err(e) => Err(anyhow::anyhow!(ApisixClientError::SessionNotFound(e.to_string())))
    }
}
