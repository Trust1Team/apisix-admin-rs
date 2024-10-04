use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;
use crate::error::ApisixClientError;
use crate::{Result};
use crate::models::Plugin;


#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthzKeycloak {
    pub discovery_url: Option<String>,
    pub token_endpoint: Option<String>,
    pub resource_registration_endpoint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub grant_type: Option<String>,
    pub policy_enforcement_mode: Option<PolicyEnforcementMode>,
    pub permissions: Option<Vec<String>>,
    pub lazy_load_paths: Option<bool>,
    pub http_method_as_scope: Option<bool>,
    pub timeout: Option<i32>,
    pub access_token_expires_in: Option<i32>,
    pub access_token_expires_leeway: Option<i32>,
    pub refresh_token_expires_in: Option<i32>,
    pub refresh_token_expires_leeway: Option<i32>,
    pub ssl_verify: Option<bool>,
    pub cache_ttl_seconds: Option<i32>,
    pub keepalive: Option<bool>,
    pub keepalive_timeout: Option<i32>,
    pub keepalive_pool: Option<i32>,
    pub access_denied_redirect_uri: Option<String>,
    pub password_grant_token_generation_incoming_uri: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
enum PolicyEnforcementMode {
    ENFORCING,
    PERMISSIVE,
}