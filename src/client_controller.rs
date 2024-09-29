use tracing::instrument;
use anyhow::Result;
use serde_json::Value;
use crate::client::admin_api_client::AdminConnector;
use crate::client::control_api_client::ControllerConnector;
use crate::config::ApisixConfig;

/// Check if Admin API is up and running
#[instrument(skip_all)]
pub async fn api_admin_check_version(cfg: &ApisixConfig) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.check_version().await
}

pub async fn api_ctrl_schema(cfg: &ApisixConfig) -> Result<Value> {
    let cc: ControllerConnector =  ControllerConnector::new(cfg).await;
    cc.schema().await
}
