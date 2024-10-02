use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::models::Plugins;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApisixService {
    pub update_time: Option<i64>,
    pub create_time: Option<i64>,
    pub plugins: Plugins,
    pub id: Option<Value>,
    pub upstream_id: Option<Value>,
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
    use crate::models::common::TypedItem;

    #[traced_test]
    #[tokio::test]
    async fn test_parse_service_response() {
        let nodes = r#"
        {
            "createdIndex": 17,
            "key": "/apisix/services/100",
            "value": {
                "update_time": 1727660558,
                "create_time": 1727660558,
                "plugins": {
                    "key-auth": {
                        "hide_credentials": false,
                        "query": "apikey",
                        "header": "apikey"
                    }
                },
                "id": "100",
                "upstream_id": "1",
                "name": "t1c-auth-api-service",
                "desc": "Authentication API Service",
                "enable_websocket": false
            },
            "modifiedIndex": 17
        }"#;
        let nodes: TypedItem<ApisixService> = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.key.unwrap(), "/apisix/services/100");
        assert_eq!(nodes.value.clone().unwrap().id.unwrap(), "100");
        assert_eq!(nodes.value.clone().unwrap().upstream_id.unwrap(), "1");
        assert_eq!(nodes.value.clone().unwrap().name.unwrap(), "t1c-auth-api-service");
        assert_eq!(nodes.value.clone().unwrap().desc.unwrap(), "Authentication API Service");
        assert_eq!(nodes.value.clone().unwrap().enable_websocket.unwrap(), false);
        assert_eq!(nodes.value.clone().unwrap().plugins.key_auth.clone().unwrap().hide_credentials.unwrap(), false);
        assert_eq!(nodes.value.clone().unwrap().plugins.key_auth.clone().unwrap().query.unwrap(), "apikey");
        assert_eq!(nodes.value.clone().unwrap().plugins.key_auth.clone().unwrap().header.unwrap(), "apikey");
    }
}
// endregion: tests