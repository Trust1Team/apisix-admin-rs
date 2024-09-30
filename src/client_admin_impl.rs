use tracing::instrument;
use anyhow::Result;
use crate::client::AdminConnector;
use crate::config::ApisixConfig;
use crate::models::admin_api_responses::{ListResponse, TypedItem, Upstream};

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
