use anyhow::Result;
use serde_json::Value;
use crate::client::control_api_client::ControllerConnector;
use crate::config::ApisixConfig;
use crate::models::ctrl_api_responses::CtrlHealthCheckResponse;

/// Retrieve Apisix Schema
pub async fn api_ctrl_schema(cfg: &ApisixConfig) -> Result<Value> {
    let cc: ControllerConnector =  ControllerConnector::new(cfg).await;
    cc.schema().await
}

/// Check if the Control API is up and running
pub async fn api_ctrl_health_check(cfg: &ApisixConfig) -> Result<CtrlHealthCheckResponse> {
    let cc: ControllerConnector =  ControllerConnector::new(cfg).await;
    cc.health_check().await
}
