use tracing::instrument;
use crate::client::AdminConnector;
use crate::common_responses::{ListResponse, TypedItem};
use crate::config::ApisixConfig;
use crate::error::ApisixClientError::InvalidRequest;
use crate::models::admin_upstream_responses::ApisixUpstream;
use crate::models::UpstreamRequest;

type Result<T> = std::result::Result<T, crate::error::ApisixClientError>;

#[instrument(skip_all)]
pub async fn api_admin_check_version(cfg: &ApisixConfig) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.check_version().await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub async fn api_admin_get_upstreams(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixUpstream>>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_upstreams().await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub async fn api_admin_get_upstream(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixUpstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_upstream(id).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub async fn api_admin_create_upstream_with_id(cfg: &ApisixConfig, id: &str, req: &UpstreamRequest) -> Result<TypedItem<ApisixUpstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_upstream_with_id(id, req).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub async fn api_admin_delete_upstream(cfg: &ApisixConfig, id: &str) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.delete_upstream(id).await.map(|_| ()).map_err(|e| InvalidRequest(e.to_string()))
}
