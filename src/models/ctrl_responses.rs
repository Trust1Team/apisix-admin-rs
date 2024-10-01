use serde::{Deserialize, Serialize};
use serde_json::Value;

// region: ctrl_schema
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CtrlSchemaResponse {
    pub main: Value,
    pub plugins: Value,
    #[serde(rename = "stream_plugins")]
    pub stream_plugins: Value,
}
// endregion: ctrl_schema

// region: ctrl_health_check
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct CtrlHealthCheckResponse {
    pub nodes: Option<Vec<Node>>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub ip: Option<String>,
    pub counter: Option<Counter>,
    pub port: Option<i64>,
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde_with::skip_serializing_none]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    #[serde(rename = "http_failure")]
    pub http_failure: Option<i64>,
    pub success: Option<i64>,
    #[serde(rename = "timeout_failure")]
    pub timeout_failure: Option<i64>,
    #[serde(rename = "tcp_failure")]
    pub tcp_failure: Option<i64>,
}
// endregion: ctrl_health_check
