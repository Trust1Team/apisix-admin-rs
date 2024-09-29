use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Retrurn the Apisix schema, untyped
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CtrlSchemaResponse {
    pub main: Value,
    pub plugins: Value,
    #[serde(rename = "stream_plugins")]
    pub stream_plugins: Value,
}
