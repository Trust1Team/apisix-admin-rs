use serde::{Deserialize, Serialize};
use crate::plugins::Plugins;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApisixConsumerGroup {
    pub plugins: Option<Plugins>,
    pub id: Option<String>,
    pub desc: Option<String>,
    pub update_time: Option<i64>,
    pub create_time: Option<i64>,
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
    async fn test_parse_consumer_group_response() {
        let nodes = r#"
        {
            "createdIndex": 173,
            "key": "/apisix/consumer_groups/trust1team",
            "value": {
                "update_time": 1727786609,
                "plugins": {
                    "limit-count": {
                        "time_window": 1,
                        "policy": "local",
                        "show_limit_quota_header": true,
                        "allow_degradation": false,
                        "rejected_code": 429,
                        "rejected_msg": "Request count limit reached, try again later",
                        "count": 10,
                        "key": "remote_addr",
                        "key_type": "var"
                    }
                },
                "id": "trust1team",
                "desc": "Trust1Team",
                "create_time": 1727786609
            },
            "modifiedIndex": 173
        }"#;
        let nodes: TypedItem<ApisixConsumerGroup> = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.key.unwrap(), "/apisix/consumer_groups/trust1team");
        assert_eq!(nodes.value.clone().unwrap().id.unwrap(), "trust1team");
        assert_eq!(nodes.value.clone().unwrap().desc.unwrap(), "Trust1Team");
        assert_eq!(nodes.value.clone().unwrap().create_time.unwrap(), 1727786609);
        assert_eq!(nodes.value.clone().unwrap().update_time.unwrap(), 1727786609);
    }
}
// endregion: tests