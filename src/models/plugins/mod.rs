use serde::{Deserialize, Serialize};

mod key_auth;
pub use key_auth::KeyAuth;
mod proxy_rewrite;
mod limit_count;

pub use proxy_rewrite::ProxyRewrite;

// region: common
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
// endregion: common