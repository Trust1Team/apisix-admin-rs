use tracing::instrument;
use anyhow::Result;
use serde_json::Value;
use crate::client::AdminConnector;
use crate::config::ApisixConfig;
use crate::models::admin_api_responses::{ListResponse, TypedItem, Upstream};
use crate::models::UpstreamRequest;

#[instrument(skip_all)]
pub async fn api_admin_check_version(cfg: &ApisixConfig) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.check_version().await
}

#[instrument(skip_all)]
pub async fn api_admin_get_upstreams(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<Upstream>>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_upstreams().await
}

#[instrument(skip_all)]
pub async fn api_admin_get_upstream(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<Upstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_upstream(id).await
}

#[instrument(skip_all)]
pub async fn api_admin_create_upstream_with_id(cfg: &ApisixConfig, id: &str, req: &UpstreamRequest) -> Result<TypedItem<Upstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_upstream_with_id(id, req).await
}

#[instrument(skip_all)]
pub async fn api_admin_create_upstream(cfg: &ApisixConfig, req: &UpstreamRequest) -> Result<TypedItem<Upstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_upstream(req).await
}

#[instrument(skip_all)]
pub async fn api_admin_delete_upstream(cfg: &ApisixConfig, id: &str) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.delete_upstream(id).await
}
