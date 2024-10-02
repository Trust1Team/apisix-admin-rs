use serde::{Deserialize, Serialize};
use crate::models::Plugins;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ApisixConsumer {
    pub update_time: Option<i64>,
    pub plugins: Option<Plugins>,
    pub username: Option<String>,
    pub desc: Option<String>,
    pub create_time: Option<i64>,
    pub group_id: Option<String>,
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
    async fn test_parse_consumer_response() {
        let nodes = r#"
        {
            "createdIndex": 223,
            "key": "/apisix/consumers/readmycards",
            "value": {
                "update_time": 1727847549,
                "plugins": {
                    "key-auth": {
                        "key": "7b034e79-2cb9-4e17-af10-ff1aad082307"
                    }
                },
                "username": "readmycards",
                "create_time": 1727847549,
                "group_id": "trust1team"
            },
            "modifiedIndex": 223
        }"#;
        let nodes: TypedItem<ApisixConsumer> = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.key.unwrap(), "/apisix/consumers/readmycards");
        assert_eq!(nodes.value.clone().unwrap().username.unwrap(), "readmycards");
        assert_eq!(nodes.value.clone().unwrap().group_id.unwrap(), "trust1team");
        assert_eq!(nodes.value.clone().unwrap().create_time.unwrap(), 1727847549);
        assert_eq!(nodes.value.clone().unwrap().update_time.unwrap(), 1727847549);
    }
}
// endregion: tests