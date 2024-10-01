use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::plugins::Plugins;
use crate::{UpstreamBuilder, UpstreamSchema, UpstreamTimeout};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    #[serde(rename = "list")]
    pub list: Vec<T>,
    pub total: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericJsonResponse {
    pub created_index: i64,
    pub key: String,
    pub value: Value,
    pub modified_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedItem<T> {
    pub created_index: Option<i64>,
    pub key: Option<String>,
    pub value: Option<T>,
    pub modified_index: Option<i64>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApisixUpstream {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub desc: Option<String>,
    pub scheme: Option<String>,
    pub update_time: Option<i64>,
    pub nodes: Option<Value>, //untyped
    pub create_time: Option<i64>,
    pub name: Option<String>,
    pub id: Option<Value>, //can be both string or integer
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApisixService {
    pub update_time: Option<i64>,
    pub create_time: Option<i64>,
    pub plugins: Plugins,
    pub id: Option<String>,
    pub upstream_id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub enable_websocket: Option<bool>,
}

// region: tests
#[cfg(test)]
mod tests {
    use serde_json::{to_string, to_string_pretty};
    use super::*;
    use tracing::{error, info};
    use tracing_test::traced_test;
    use crate::models::admin_api_upstream_requests::UpstreamType;

    #[traced_test]
    #[tokio::test]
    async fn test_parse_upstream_response() {
        let nodes = r#"
        {
            "key": "/apisix/upstreams/gen-5NzR8BYUwwQX",
            "value": {
                "timeout": {
                    "connect": 0.5,
                    "send": 0.5,
                    "read": 0.5
                },
                "type": "roundrobin",
                "hash_on": "vars",
                "desc": "Test Upstream Description",
                "scheme": "http",
                "update_time": 1727703719,
                "retries": 3,
                "id": "gen-5NzR8BYUwwQX",
                "name": "Test Upstream",
                "create_time": 1727703719,
                "nodes": {
                    "localhost:9000": 1
                },
                "retry_timeout": 5,
                "pass_host": "pass"
            }
        }"#;
        let nodes: TypedItem<ApisixUpstream> = serde_json::from_str(nodes).unwrap();

        info!("Upstream response: {:?}", to_string(&nodes));
        assert!(true)
    }
}
// endregion: tests