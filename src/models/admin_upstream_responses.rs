use serde::{Deserialize, Serialize};
use serde_json::Value;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApisixUpstream {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub desc: Option<String>,
    pub scheme: Option<String>,
    pub nodes: Option<Value>, //untyped
    pub create_time: Option<i64>,
    pub update_time: Option<i64>,
    pub name: Option<String>,
    pub id: Option<Value>, //can be both string or integer
}

// region: tests
#[cfg(test)]
mod tests {
    use serde_json::{to_string, to_string_pretty};
    use super::*;
    use tracing::{error, info};
    use tracing_test::traced_test;
    use crate::models::admin_upstream_requests::UpstreamType;
    use crate::models::common::TypedItem;

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
        assert_eq!(nodes.key.unwrap(), "/apisix/upstreams/gen-5NzR8BYUwwQX");
        assert_eq!(nodes.value.clone().unwrap().id.unwrap(), "gen-5NzR8BYUwwQX");
        assert_eq!(nodes.value.clone().unwrap().name.unwrap(), "Test Upstream");
        assert_eq!(nodes.value.clone().unwrap().desc.unwrap(), "Test Upstream Description");
        assert_eq!(nodes.value.clone().unwrap().scheme.unwrap(), "http");
        assert_eq!(nodes.value.clone().unwrap().type_field.unwrap(), "roundrobin");
        assert_eq!(nodes.value.clone().unwrap().nodes.clone().unwrap()["localhost:9000"], 1);
        assert_eq!(nodes.value.clone().unwrap().update_time.unwrap(), 1727703719);
    }
}
// endregion: tests