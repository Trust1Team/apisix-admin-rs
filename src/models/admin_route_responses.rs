use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::common::ApisixTimeout;
use crate::plugins::Plugins;
use crate::UpstreamRequest;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApisixRoute {
    pub id: Option<Value>, //preferred String
    pub name: Option<String>,
    pub desc: Option<String>,
    pub status: Option<i64>,
    pub update_time: Option<i64>,
    pub plugins: Option<Plugins>,
    pub uri: Option<String>,
    pub uris: Option<Vec<String>>,
    pub hosts: Option<Vec<String>>,
    pub remote_addr: Option<String>,
    pub remote_addrs: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub upstream: Option<UpstreamRequest>,
    pub upstream_id: Option<Value>, //preferred String
    pub service_id: Option<Value>, //preferred String
    pub timeout: Option<ApisixTimeout>,
    pub enable_websocket: Option<bool>,
    pub create_time: Option<i64>,
    pub priority: Option<i64>,
}

// region: tests
#[cfg(test)]
mod tests {
    use serde_json::{to_string, to_string_pretty};
    use super::*;
    use tracing::{error, info};
    use tracing_test::traced_test;
    use crate::models::common::TypedItem;

    #[traced_test]
    #[tokio::test]
    async fn test_parse_route_response() {
        let nodes = r#"
        {
            "createdIndex": 139,
            "key": "/apisix/routes/route-t1c-authentication-api",
            "value": {
                "status": 1,
                "update_time": 1727776498,
                "plugins": {
                    "proxy-rewrite": {
                        "regex_uri": [
                            "^/auth/v1/(. *)",
                            "/v1/$1"
                        ],
                        "use_real_request_uri_unsafe": false
                    }
                },
                "uri": "/auth/v1/*",
                "upstream_id": "test_upstream",
                "service_id": "test_service",
                "create_time": 1727776498,
                "priority": 0,
                "id": "route-t1c-authentication-api"
            },
            "modifiedIndex": 139
        }
        "#;
        let nodes: TypedItem<ApisixRoute> = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.value.clone().unwrap().id.unwrap(), "route-t1c-authentication-api");
        assert_eq!(nodes.value.clone().unwrap().status.unwrap(), 1);
        assert_eq!(nodes.value.clone().unwrap().update_time.unwrap(), 1727776498);
        assert_eq!(nodes.value.clone().unwrap().uri.unwrap(), "/auth/v1/*");
        assert_eq!(nodes.value.clone().unwrap().upstream_id.unwrap(), "test_upstream");
        assert_eq!(nodes.value.clone().unwrap().create_time.unwrap(), 1727776498);
        assert_eq!(nodes.value.clone().unwrap().priority.unwrap(), 0);
    }
}
// endregion: tests