use serde::{Deserialize, Serialize};

mod key_auth;
mod proxy_rewrite;
mod limit_count;
mod consumer_restriction;

pub use key_auth::*;
pub use proxy_rewrite::*;
pub use limit_count::*;

// region: common
/// Plugins that are executed during the request/response cycle.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "key-auth")]
    pub key_auth: Option<KeyAuth>,
    #[serde(rename = "proxy-rewrite")]
    pub proxy_rewrite: Option<ProxyRewrite>,
    #[serde(rename = "limit-count")]
    pub limit_count: Option<LimitCount>,
}

impl Default for Plugins {
    fn default() -> Self {
        Plugins {
            key_auth: None,
            proxy_rewrite: None,
            limit_count: None,
        }
    }
}

pub trait Plugin {}
// endregion: common