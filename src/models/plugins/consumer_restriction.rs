use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;
use crate::{Result};
use crate::models::Plugin;

/// Builder pattern to create a ConsumerRestriction
#[serde_with::skip_serializing_none]
#[derive(Validate, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerRestrictionBuilder {
    pub type_field: Option<ConsumerRestrictionType>,
    pub whitelist: Option<Vec<String>>,
    pub blacklist: Option<Vec<String>>,
    pub rejected_code: Option<i32>,
    pub rejected_msg: Option<String>,
    pub allowed_by_methods: Option<Vec<AllowedByMethods>>,
}

impl ConsumerRestrictionBuilder {
    pub fn new() -> Self {
        ConsumerRestriction::default().into()
    }

    /// Type of object to base the restriction on.
    pub fn with_type(mut self, type_field: ConsumerRestrictionType) -> Self {
        self.type_field = Some(type_field);
        self
    }

    /// List of objects to whitelist. Has a higher priority than allowed_by_methods.
    pub fn with_whitelist(mut self, whitelist: Vec<String>) -> Self {
        self.whitelist = Some(whitelist);
        self
    }

    /// List of objects to blacklist. Has a higher priority than whitelist.
    pub fn with_blacklist(mut self, blacklist: Vec<String>) -> Self {
        self.blacklist = Some(blacklist);
        self
    }

    /// HTTP status code returned when the request is rejected.
    pub fn with_rejected_code(mut self, rejected_code: i32) -> Self {
        self.rejected_code = Some(rejected_code);
        self
    }

    /// Message returned when the request is rejected.
    pub fn with_rejected_msg(mut self, rejected_msg: impl Into<String>) -> Self {
        self.rejected_msg = Some(rejected_msg.into());
        self
    }

    /// List of allowed configurations for Consumer settings, including a username of the Consumer and a list of allowed HTTP methods.
    pub fn with_allowed_by_methods(mut self, allowed_by_methods: Vec<AllowedByMethods>) -> Self {
        self.allowed_by_methods = Some(allowed_by_methods);
        self
    }

    pub fn build(self) -> Result<ConsumerRestriction> {
        Ok(ConsumerRestriction {
            type_field: self.type_field,
            whitelist: self.whitelist,
            blacklist: self.blacklist,
            rejected_code: self.rejected_code,
            rejected_msg: self.rejected_msg,
            allowed_by_methods: self.allowed_by_methods,
        })
    }
}



/// The consumer-restriction Plugin allows users to configure access restrictions on
/// Consumer, Route, Service, or Consumer Group.
#[serde_with::skip_serializing_none]
#[derive(Validate, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerRestriction {
    #[serde(rename = "type")]
    pub type_field: Option<ConsumerRestrictionType>,
    pub whitelist: Option<Vec<String>>,
    pub blacklist: Option<Vec<String>>,
    #[validate(range(min = 200, max = 999))]
    pub rejected_code: Option<i32>,
    pub rejected_msg: Option<String>,
    pub allowed_by_methods: Option<Vec<AllowedByMethods>>,
}

impl Plugin for ConsumerRestriction {}

impl From<ConsumerRestriction> for ConsumerRestrictionBuilder {
    fn from(consumer_restriction: ConsumerRestriction) -> Self {
        ConsumerRestrictionBuilder {
            type_field: consumer_restriction.type_field,
            whitelist: consumer_restriction.whitelist,
            blacklist: consumer_restriction.blacklist,
            rejected_code: consumer_restriction.rejected_code,
            rejected_msg: consumer_restriction.rejected_msg,
            allowed_by_methods: consumer_restriction.allowed_by_methods,
        }
    }
}

/// List of allowed configurations for Consumer settings, including a username of the Consumer and a list of allowed HTTP methods
/// `user` - A username for a Consumer.
/// `methods` - A list of allowed HTTP methods for a Consumer.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AllowedByMethods {
    pub user: Option<String>,
    pub methods: Option<Vec<AllowedMethodsType>>,
}

/// Type of user specified key to use
/// `consumer_name` - Username of the Consumer to restrict access to a Route or a Service
/// `consumer_group_id` - ID of the Consumer Group to restrict access to a Route or a Service
/// `service_id` - ID of the Service to restrict access from a Consumer. Need to be used with an Authentication Plugin
/// `route_id` - ID of the Route to restrict access from a Consumer
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum ConsumerRestrictionType {
    consumer_name,
    consumer_group_id,
    service_id,
    route_id,
}

/// List of allowed HTTP methods for a Consumer
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum AllowedMethodsType {
    GET,
    POST,
    PUT,
    HEAD,
    DELETE,
    PATH,
    OPTIONS,
    CONNECT,
    PURGE,
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
    async fn test_parse_consumer_restriction_empty_response() {
        let nodes = r#"{}"#;
        let nodes: ConsumerRestriction = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.type_field, None);
        assert_eq!(nodes.whitelist, None);
        assert_eq!(nodes.blacklist, None);
        assert_eq!(nodes.rejected_code, None);
        assert_eq!(nodes.rejected_msg, None);
        assert_eq!(nodes.allowed_by_methods, None);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_parse_consumer_restriction_response() {
        let nodes = r#"
        {
            "whitelist": [
                "jack1"
            ]
        }"#;
        let nodes: ConsumerRestriction = serde_json::from_str(nodes).unwrap();
        assert_eq!(nodes.whitelist.unwrap(), vec!["jack1"]);
    }
}
// endregion: tests