use serde::{Deserialize, Serialize};

mod key_auth;
mod proxy_rewrite;
mod limit_count;
mod consumer_restriction;
//mod authz_keycloak;

pub use key_auth::*;
pub use proxy_rewrite::*;
pub use limit_count::*;
pub use consumer_restriction::*;

// region: common
/// Plugins that are executed during the request/response cycle.
#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugins {
    #[serde(rename = "key-auth")]
    pub key_auth: Option<KeyAuth>,
    #[serde(rename = "proxy-rewrite")]
    pub proxy_rewrite: Option<ProxyRewrite>,
    #[serde(rename = "limit-count")]
    pub limit_count: Option<LimitCount>,
    #[serde(rename = "consumer-restriction")]
    pub consumer_restriction: Option<ConsumerRestriction>,
}

pub trait Plugin {}
// endregion: common