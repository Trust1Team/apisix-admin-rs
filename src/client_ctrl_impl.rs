use serde_json::Value;
use crate::client::ControllerConnector;
use crate::config::ApisixConfig;
use crate::models::ctrl_responses::CtrlHealthCheckResponse;
type Result<T> = std::result::Result<T, crate::error::ApisixClientError>;
pub async fn api_ctrl_schema(cfg: &ApisixConfig) -> Result<Value> {
    let cc: ControllerConnector =  ControllerConnector::new(cfg).await;
    cc.schema().await.map_err(|e| crate::error::ApisixClientError::InvalidRequest(e.to_string()))
}

pub async fn api_ctrl_health_check(cfg: &ApisixConfig) -> Result<CtrlHealthCheckResponse> {
    let cc: ControllerConnector =  ControllerConnector::new(cfg).await;
    cc.health_check().await.map_err(|e| crate::error::ApisixClientError::InvalidRequest(e.to_string()))
}

pub async fn api_ctrl_garbage_collect(cfg: &ApisixConfig) -> Result<()> {
    let cc: ControllerConnector =  ControllerConnector::new(cfg).await;
    cc.gc().await.map_err(|e| crate::error::ApisixClientError::InvalidRequest(e.to_string()))
}
