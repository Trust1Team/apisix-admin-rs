use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};
use crate::{Result};
use crate::models::Plugin;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyRewriteBuilder {
    pub uri: Option<String>,
    pub method: Option<ProxyRewriteMethod>,
    pub regex_uri: Option<Vec<String>>,
    pub host: Option<String>,
    pub use_real_request_uri_unsafe: Option<bool>,
    pub headers: Option<ProxyRewriteHeaders>,
}

impl ProxyRewriteBuilder {
    pub fn new() -> Self {
        ProxyRewrite::default().into()
    }

    /// New Upstream forwarding address. Value supports Nginx variables.
    ///
    /// Example, $arg_name
    pub fn with_uri(mut self, uri: impl Into<String>) -> Self {
        self.uri = Some(uri.into());
        self
    }

    /// Rewrites the HTTP method
    pub fn with_method(mut self, method: ProxyRewriteMethod) -> Self {
        self.method = Some(method);
        self
    }

    /// Regular expressions can be used to match the URL from client.
    /// If it matches, the URL template is forwarded to the upstream.
    /// Otherwise, the URL from the client is forwarded.
    /// When both uri and regex_uri are configured, uri has a higher priority.
    /// Multiple regular expressions are currently supported for pattern matching,
    /// and the plugin will try to match them one by one until they succeed or all fail.
    ///
    /// For example:
    /// ["^/iresty/(. *)/(. *)/(. *)", "/$1-$2-$3", ^/theothers/(. *)/(. *)", "/theothers/$1-$2"],
    /// the element with the odd index represents the uri regular expression that matches
    /// the request from the client, and the element with the even index represents
    /// the uri template that is forwarded upstream upon a successful match.
    /// Please note that the length of this value must be an even number.
    pub fn with_regex_uri(mut self, regex_uri: Vec<String>) -> Self {
        self.regex_uri = Some(regex_uri);
        self
    }

    /// New Upstream host address
    pub fn with_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Use real_request_uri (original $request_uri in nginx) to bypass URI normalization.
    /// Enabling this is considered unsafe as it bypasses all URI normalization steps.
    pub fn with_use_real_request_uri_unsafe(mut self, use_real_request_uri_unsafe: bool) -> Self {
        self.use_real_request_uri_unsafe = Some(use_real_request_uri_unsafe);
        self
    }

    pub fn with_headers(mut self, headers: ProxyRewriteHeaders) -> Self {
        self.headers = Some(headers);
        self
    }

    /// Header manipulator
    pub fn build(self) -> Result<ProxyRewrite> {
        Ok(ProxyRewrite {
            uri: self.uri,
            method: self.method,
            regex_uri: self.regex_uri,
            host: self.host,
            use_real_request_uri_unsafe: self.use_real_request_uri_unsafe,
            headers: self.headers,
        })
    }
}
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

impl From<ProxyRewrite> for ProxyRewriteBuilder {
    fn from(item: ProxyRewrite) -> Self {
        ProxyRewriteBuilder {
            uri: item.uri,
            method: item.method,
            regex_uri: item.regex_uri,
            host: item.host,
            use_real_request_uri_unsafe: item.use_real_request_uri_unsafe,
            headers: item.headers,
        }
    }
}
impl Plugin for ProxyRewrite {}

/// [add]: Append the new headers.
/// The format is {"name": "value",...}.
/// The values in the header can contain Nginx variables like $remote_addr and $balancer_ip.
/// It also supports referencing the match result of regex_uri as a variable like $1-$2-$3
///
/// [set]: Overwrite the headers. If the header does not exist, it will be added.
/// The format is {"name": "value", ...}.
/// The values in the header can contain Nginx variables like $remote_addr and $balancer_ip.
/// It also supports referencing the match result of regex_uri as a variable like $1-$2-$3.
/// Note that if you would like to set the Host header, use the host attribute instead
///
/// [remove]: Remove the headers. The format is ["name", ...]
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyRewriteHeaders {
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