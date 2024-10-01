use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "key-auth")]
    pub key_auth: Option<KeyAuth>,
}

impl Default for Plugins {
    fn default() -> Self {
        Plugins {
            key_auth: None,
        }
    }
}

pub trait Plugin {}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAuth {
    pub hide_credentials: Option<bool>,
    pub query: Option<String>,
    pub header: Option<String>,
}

impl Plugin for KeyAuth {}

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
    async fn test_parse_key_auth_response() {
        let nodes = r#"
        {
            "hide_credentials": false,
            "query": "apikey",
            "header": "apikey"
        }"#;
        let nodes: KeyAuth = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.hide_credentials.unwrap(), false);
        assert_eq!(nodes.query.unwrap(), "apikey");
        assert_eq!(nodes.header.unwrap(), "apikey");
    }
}
// endregion: tests