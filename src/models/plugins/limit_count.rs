use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use crate::plugins::Plugin;
use validator::Validate;
use crate::error::ApisixClientError;
use crate::{Result};

#[serde_with::skip_serializing_none]
#[derive(Validate, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitCountBuilder {
    #[validate(range(min = 1))]
    pub count: Option<i64>,
    #[validate(range(min = 1))]
    pub time_window: Option<i64>,
    pub key_type: Option<LimitCountKeyType>,
    pub key: Option<String>,
    #[validate(range(min = 200, max = 599))]
    pub rejected_code: Option<i64>,
    #[validate(length(min = 1))]
    pub rejected_msg: Option<String>,
    pub policy: Option<String>,
    pub allow_degradation: Option<bool>,
    pub show_limit_quota_headers: Option<bool>,
    pub group: Option<String>,
    pub redis_host: Option<String>,
    pub redis_port: Option<i64>,
    pub redis_username: Option<String>,
    pub redis_password: Option<String>,
    pub redis_ssl: Option<bool>,
    pub redis_ssl_verify: Option<bool>,
    pub redis_database: Option<i64>,
    pub redis_timeout: Option<i64>,
    pub redis_cluster_nodes: Option<Vec<String>>,
    pub redis_cluster_name: Option<String>,
    pub redis_cluster_ssl: Option<bool>,
    pub redis_cluster_ssl_verify: Option<bool>,
}

impl LimitCountBuilder {
    pub fn new() -> Self {
        LimitCount::default().into()
    }

    /// Maximum number of requests to allow.
    pub fn with_count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn with_time_window(mut self, time_window: i64) -> Self {
        self.time_window = Some(time_window);
        self
    }

    pub fn with_key_type(mut self, key_type: LimitCountKeyType) -> Self {
        self.key_type = Some(key_type);
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn with_rejected_code(mut self, rejected_code: i64) -> Self {
        self.rejected_code = Some(rejected_code);
        self
    }

    pub fn with_rejected_msg(mut self, rejected_msg: impl Into<String>) -> Self {
        self.rejected_msg = Some(rejected_msg.into());
        self
    }

    pub fn with_policy(mut self, policy: impl Into<String>) -> Self {
        self.policy = Some(policy.into());
        self
    }

    pub fn with_allow_degradation(mut self, allow_degradation: bool) -> Self {
        self.allow_degradation = Some(allow_degradation);
        self
    }

    pub fn with_show_limit_quota_headers(mut self, show_limit_quota_headers: bool) -> Self {
        self.show_limit_quota_headers = Some(show_limit_quota_headers);
        self
    }

    pub fn with_group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    pub fn with_redis_host(mut self, redis_host: impl Into<String>) -> Self {
        self.redis_host = Some(redis_host.into());
        self
    }

    pub fn with_redis_port(mut self, redis_port: i64) -> Self {
        self.redis_port = Some(redis_port);
        self
    }

    pub fn with_redis_username(mut self, redis_username: impl Into<String>) -> Self {
        self.redis_username = Some(redis_username.into());
        self
    }

    pub fn with_redis_password(mut self, redis_password: impl Into<String>) -> Self {
        self.redis_password = Some(redis_password.into());
        self
    }

    pub fn with_redis_ssl(mut self, redis_ssl: bool) -> Self {
        self.redis_ssl = Some(redis_ssl);
        self
    }

    pub fn with_redis_ssl_verify(mut self, redis_ssl_verify: bool) -> Self {
        self.redis_ssl_verify = Some(redis_ssl_verify);
        self
    }

    pub fn with_redis_database(mut self, redis_database: i64) -> Self {
        self.redis_database = Some(redis_database);
        self
    }

    pub fn with_redis_timeout(mut self, redis_timeout: i64) -> Self {
        self.redis_timeout = Some(redis_timeout);
        self
    }

    pub fn with_redis_cluster_nodes(mut self, redis_cluster_nodes: Vec<String>) -> Self {
        self.redis_cluster_nodes = Some(redis_cluster_nodes);
        self
    }

    pub fn with_redis_cluster_name(mut self, redis_cluster_name: impl Into<String>) -> Self {
        self.redis_cluster_name = Some(redis_cluster_name.into());
        self
    }

    pub fn with_redis_cluster_ssl(mut self, redis_cluster_ssl: bool) -> Self {
        self.redis_cluster_ssl = Some(redis_cluster_ssl);
        self
    }

    pub fn with_redis_cluster_ssl_verify(mut self, redis_cluster_ssl_verify: bool) -> Self {
        self.redis_cluster_ssl_verify = Some(redis_cluster_ssl_verify);
        self
    }

    pub fn build(self) -> Result<LimitCount> {
        self.validate().map_err(|v| ApisixClientError::PluginConfigException(v.to_string()))?;
        Ok(LimitCount {
            count: self.count,
            time_window: self.time_window,
            key_type: self.key_type,
            key: self.key,
            rejected_code: self.rejected_code,
            rejected_msg: self.rejected_msg,
            policy: self.policy,
            allow_degradation: self.allow_degradation,
            show_limit_quota_headers: self.show_limit_quota_headers,
            group: self.group,
            redis_host: self.redis_host,
            redis_port: self.redis_port,
            redis_username: self.redis_username,
            redis_password: self.redis_password,
            redis_ssl: self.redis_ssl,
            redis_ssl_verify: self.redis_ssl_verify,
            redis_database: self.redis_database,
            redis_timeout: self.redis_timeout,
            redis_cluster_nodes: self.redis_cluster_nodes,
            redis_cluster_name: self.redis_cluster_name,
            redis_cluster_ssl: self.redis_cluster_ssl,
            redis_cluster_ssl_verify: self.redis_cluster_ssl_verify,
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LimitCount {
    pub count: Option<i64>,
    pub time_window: Option<i64>,
    pub key_type: Option<LimitCountKeyType>,
    pub key: Option<String>,
    pub rejected_code: Option<i64>,
    pub rejected_msg: Option<String>,
    pub policy: Option<String>,
    pub allow_degradation: Option<bool>,
    pub show_limit_quota_headers: Option<bool>,
    pub group: Option<String>,
    pub redis_host: Option<String>,
    pub redis_port: Option<i64>,
    pub redis_username: Option<String>,
    pub redis_password: Option<String>,
    pub redis_ssl: Option<bool>,
    pub redis_ssl_verify: Option<bool>,
    pub redis_database: Option<i64>,
    pub redis_timeout: Option<i64>,
    pub redis_cluster_nodes: Option<Vec<String>>,
    pub redis_cluster_name: Option<String>,
    pub redis_cluster_ssl: Option<bool>,
    pub redis_cluster_ssl_verify: Option<bool>,
}

impl Default for LimitCount {
    fn default() -> Self {
        LimitCount {
            count: None,
            time_window: None,
            key_type: None,
            key: None,
            rejected_code: None,
            rejected_msg: None,
            policy: None,
            allow_degradation: None,
            show_limit_quota_headers: None,
            group: None,
            redis_host: None,
            redis_port: None,
            redis_username: None,
            redis_password: None,
            redis_ssl: None,
            redis_ssl_verify: None,
            redis_database: None,
            redis_timeout: None,
            redis_cluster_nodes: None,
            redis_cluster_name: None,
            redis_cluster_ssl: None,
            redis_cluster_ssl_verify: None,
        }
    }
}

impl From<LimitCount> for LimitCountBuilder {
    fn from(item: LimitCount) -> Self {
        LimitCountBuilder {
            count: item.count,
            time_window: item.time_window,
            key_type: item.key_type,
            key: item.key,
            rejected_code: item.rejected_code,
            rejected_msg: item.rejected_msg,
            policy: item.policy,
            allow_degradation: item.allow_degradation,
            show_limit_quota_headers: item.show_limit_quota_headers,
            group: item.group,
            redis_host: item.redis_host,
            redis_port: item.redis_port,
            redis_username: item.redis_username,
            redis_password: item.redis_password,
            redis_ssl: item.redis_ssl,
            redis_ssl_verify: item.redis_ssl_verify,
            redis_database: item.redis_database,
            redis_timeout: item.redis_timeout,
            redis_cluster_nodes: item.redis_cluster_nodes,
            redis_cluster_name: item.redis_cluster_name,
            redis_cluster_ssl: item.redis_cluster_ssl,
            redis_cluster_ssl_verify: item.redis_cluster_ssl_verify,
        }
    }
}

impl Plugin for LimitCount {}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum LimitCountKeyType {
    var,
    var_combination,
    constant,
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
    async fn test_parse_limit_count_empty_response() {
        let nodes = r#"{}"#;
        let nodes: LimitCount = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.count, None);
        assert_eq!(nodes.time_window, None);
        assert_eq!(nodes.key_type, None);
        assert_eq!(nodes.key, None);
        assert_eq!(nodes.rejected_code, None);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_validate() {
        let mut test = LimitCountBuilder::new()
            .with_count(0) // count min > 0
            .build();
        assert!(test.is_err());
        test = LimitCountBuilder::new()
            .with_rejected_code(199) // rejected_code min > 200
            .build();
        assert!(test.is_err());
        test = LimitCountBuilder::new()
            .with_rejected_code(600) // rejected_code min > 200
            .build();
        assert!(test.is_err());
        test = LimitCountBuilder::new()
            .with_rejected_code(200) // rejected_code min > 200
            .build();
        assert!(test.is_ok());
    }

}
// endregion: tests