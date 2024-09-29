use tracing::instrument;
use anyhow::Result;
use crate::client::admin_api_client::AdminConnector;
use crate::config::ApisixConfig;

/// Check if Admin API is up and running
#[instrument(skip_all)]
pub async fn ctrl_admin_check_version(cfg: &ApisixConfig) -> Result<()> {
    let sc: AdminConnector =  AdminConnector::new(cfg).await;
    sc.check_version().await
}
