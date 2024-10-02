use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "key-auth")]
    pub key_auth: Option<KeyAuth>,
    #[serde(rename = "proxy-rewrite")]
    pub proxy_rewrite: Option<ProxyRewrite>,
}

impl Default for Plugins {
    fn default() -> Self {
        Plugins {
            key_auth: None,
            proxy_rewrite: None,
        }
    }
}

pub trait Plugin {}

// region: key-auth
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAuth {
    pub hide_credentials: Option<bool>,
    pub query: Option<String>,
    pub header: Option<String>,
}
impl Default for KeyAuth {
    fn default() -> Self {
        KeyAuth {
            hide_credentials: None,
            query: None,
            header: None,
        }
    }
}
impl Plugin for KeyAuth {}
// endregion: key-auth

// region: proxy-rewrite
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyRewrite {
    pub uri: Option<String>,
    pub method: Option<ProxyRewriteMethod>,
    pub regex_uri: Option<Vec<String>>,
    pub host: Option<String>,
    pub use_real_request_uri_unsafe: Option<bool>,
    pub headers: Option<ProxyRewriteHeaders>,
}

impl Default for ProxyRewrite {
    fn default() -> Self {
        ProxyRewrite {
            uri: None,
            method: None,
            regex_uri: None,
            host: None,
            use_real_request_uri_unsafe: None,
            headers: None,
        }
    }
}
impl Plugin for ProxyRewrite {}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProxyRewriteHeaders {
    pub set: Option<Value>,
    pub add: Option<Value>,
    pub remove: Option<Value>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum ProxyRewriteMethod {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    OPTIONS,
    MKCOL,
    COPY,
    MOVE,
    PROPFIND,
    LOCK,
    UNLOCK,
    PATH,
    TRACE,
}
// endregion: proxy-rewrite

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

    #[traced_test]
    #[tokio::test]
    async fn test_parse_proxy_rewrite_empty_response() {
        let nodes = r#"{}"#;
        let nodes: ProxyRewrite = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.host, None);
        assert_eq!(nodes.method, None);
        assert_eq!(nodes.uri, None);
        assert_eq!(nodes.regex_uri, None);
        assert_eq!(nodes.use_real_request_uri_unsafe, None);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_parse_proxy_rewrite_response() {
        let nodes = r#"
        {
            "regex_uri": [
                "^/auth/v1/(. *)",
                "/v1/$1"
            ],
            "use_real_request_uri_unsafe": false
        }"#;
        let nodes: ProxyRewrite = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.regex_uri.unwrap(), vec!["^/auth/v1/(. *)", "/v1/$1"]);
        assert_eq!(nodes.use_real_request_uri_unsafe.unwrap(), false);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_parse_proxy_rewrite_headers_response() {
        let nodes = r#"
        {
            "uri": "/test/home.html",
            "host": "iresty.com",
            "headers": {
               "set": {
                    "X-Api-Version": "v1",
                    "X-Api-Engine": "apisix",
                    "X-Api-useless": ""
                },
                "add": {
                    "X-Request-ID": "112233"
                },
                "remove":[
                    "X-test"
                ]
            }
        }"#;
        let nodes: ProxyRewrite = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.uri.unwrap(), "/test/home.html");
        assert_eq!(nodes.host.unwrap(), "iresty.com");
        assert_eq!(nodes.headers.clone().unwrap().set.unwrap()["X-Api-Version"], "v1");
        assert_eq!(nodes.headers.clone().unwrap().set.unwrap()["X-Api-Engine"], "apisix");
        assert_eq!(nodes.headers.clone().unwrap().set.unwrap()["X-Api-useless"], "");
        assert_eq!(nodes.headers.clone().unwrap().add.unwrap()["X-Request-ID"], "112233");
        assert_eq!(nodes.headers.clone().unwrap().remove.unwrap()[0], "X-test");
    }
}
// endregion: tests