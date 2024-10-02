use serde::{Deserialize, Serialize};
use crate::{Result};
use crate::models::Plugin;

/// Builder pattern to create a KeyAuth
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAuthBuilder {
    pub hide_credentials: Option<bool>,
    pub query: Option<String>,
    pub header: Option<String>,
}

impl KeyAuthBuilder {
    pub fn new() -> Self {
        KeyAuth::default().into()
    }

    /// Apache APISIX will pass the request header or query string that contains the
    /// authentication information to the Upstream if hide_credentials is false
    /// Otherwise the authentication information will be removed before proxying
    pub fn with_hide_credentials(mut self, hide_credentials: bool) -> Self {
        self.hide_credentials = Some(hide_credentials);
        self
    }
    /// The query string to get the key from
    /// Lower priority than header
    pub fn with_query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }

    /// The header to get the key from
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    pub fn build(self) -> Result<KeyAuth> {
        Ok(KeyAuth {
            hide_credentials: self.hide_credentials,
            query: self.query,
            header: self.header,
        })
    }
}

/// The key-auth Plugin is used to add an authentication key (API key) to a Route or a Service.
/// This works well with a Consumer. Consumers of the API can then add their key to
/// the query string or the header to authenticate their requests.
/// [Documentation](https://apisix.apache.org/docs/apisix/plugins/key-auth/)
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAuth {
    pub hide_credentials: Option<bool>,
    pub query: Option<String>,
    pub header: Option<String>,
}

impl From<KeyAuth> for KeyAuthBuilder {
    fn from(item: KeyAuth) -> Self {
        KeyAuthBuilder {
            hide_credentials: item.hide_credentials,
            query: item.query,
            header: item.header,
        }
    }
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
    async fn test_parse_key_auth_empty_response() {
        let nodes = r#"{}"#;
        let nodes: KeyAuth = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.query, None);
        assert_eq!(nodes.header, None);
        assert_eq!(nodes.hide_credentials, None);
    }

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
