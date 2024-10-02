use serde::{Deserialize, Serialize, Serializer};
use serde::ser::SerializeSeq;
use serde_json::Value;
use strum_macros::{Display, EnumString};
use crate::models::generate_identifier;
use crate::{Result};
use crate::common::ApisixTimeout;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpstreamBuilder {
    pub id: Option<String>,
    pub retries: Option<i32>,
    pub retry_timeout: Option<i32>,
    pub timeout: Option<ApisixTimeout>,
    pub nodes: Option<Value>,
    pub service_name: Option<String>,
    pub discovery_type: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<UpstreamType>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub scheme: Option<UpstreamSchema>,
}

impl UpstreamBuilder {
    pub fn new() -> Self {
        UpstreamRequest::default().into()
    }

    /// Upstream ID
    /// TODO validate id See [generate_identifier]
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Load balancing algorithm to be used, and the default value is roundrobin.
    /// See [UpstreamType]
    pub fn with_u_type(mut self, u_type: UpstreamType) -> Self {
        self.type_field = Some(u_type);
        self
    }

    /// IP addresses (with optional ports) of the Upstream nodes represented as a hash table or an array.
    /// In the hash table, the key is the IP address and the value is the weight of the node for the load balancing algorithm.
    /// For hash table case, if the key is IPv6 address with port, then the IPv6 address must be quoted with square brackets.
    /// In the array, each item is a hash table with keys host, weight, and the optional port and priority (defaults to 0).
    /// Nodes with lower priority are used only when all nodes with a higher priority are tried and are unavailable.
    /// Empty nodes are treated as placeholders and clients trying to access this Upstream will receive a 502 response.
    ///
    /// Restrictions: can not be used with `service_name`
    ///
    /// Example: `192.168.1.100:80`, `[::1]:80`
    pub fn with_nodes(mut self, nodes: Value) -> Self {
        self.nodes = Some(nodes);
        self
    }

    /// Service name used for service discovery
    ///
    /// Restrictions: can not be used with `nodes`
    pub fn with_service_name(mut self, service_name: String) -> Self {
        self.service_name = Some(service_name);
        self.discovery_type = Some("eureka".to_string()); //default
        self.nodes = None; //reset nodes when service name is used
        self
    }

    /// The type of service discovery to be used. The default value is eureka.
    /// Required when `service_name` is defined
    pub fn with_discovery_type(mut self, discovery_type: String) -> Self {
        self.discovery_type = Some(discovery_type);
        self
    }

    /// Sets the number of retries while passing the request to Upstream using the underlying Nginx mechanism.
    /// Set according to the number of available backend nodes by default.
    /// Setting this to 0 disables retry.
    pub fn with_retries(mut self, retries: i32) -> Self {
        self.retries = Some(retries);
        self
    }

    /// Timeout to continue with retries. Setting this to 0 disables the retry timeout.
    pub fn with_retry_timeout(mut self, retry_timeout: i32) -> Self {
        self.retry_timeout = Some(retry_timeout);
        self
    }

    /// Sets the timeout (in seconds) for connecting to,
    /// and sending and receiving messages to and from the Upstream.
    ///
    /// Example: {"connect": 0.5,"send": 0.5,"read": 0.5}
    pub fn with_timeout(mut self, timeout: ApisixTimeout) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Identifier for the Upstream
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Description of usage scenarios
    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// The scheme used when communicating with the Upstream.
    /// For an L7 proxy, this value can be one of http, https, grpc, grpcs.
    /// For an L4 proxy, this value could be one of tcp, udp, tls.
    /// Defaults to http.
    pub fn with_schema(mut self, scheme: UpstreamSchema) -> Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn build(&self) -> Result<UpstreamRequest> {
        Ok(UpstreamRequest {
            id: self.id.clone(),
            retries: self.retries.clone(),
            retry_timeout: self.retry_timeout.clone(),
            timeout: self.timeout.clone(),
            nodes: self.nodes.clone(),
            service_name: self.service_name.clone(),
            discovery_type: self.discovery_type.clone(),
            type_field: self.type_field.clone(),
            name: self.name.clone(),
            desc: self.desc.clone(),
            scheme: self.scheme.clone(),
        })
    }

}

// TODO: health checks => Configures the parameters for the health check.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpstreamRequest {
    pub id: Option<String>,
    pub retries: Option<i32>,
    pub retry_timeout: Option<i32>,
    pub timeout: Option<ApisixTimeout>,
    pub nodes: Option<Value>,
    pub service_name: Option<String>,
    pub discovery_type: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<UpstreamType>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub scheme: Option<UpstreamSchema>,
}

impl Default for UpstreamRequest {
    fn default() -> Self {
        let nodes = r#"
        {
            "localhost:9000": 1
        }"#;
        let nodes = serde_json::from_str(nodes).unwrap();
        UpstreamRequest {
            id: Some(generate_identifier()),
            retries: Some(0_i32), //disabled by default
            retry_timeout: Some(0_i32),
            timeout: None,
            nodes: Some(nodes),
            service_name: None,
            discovery_type: None,
            type_field: None,
            name: None,
            desc: None,
            scheme: Some(UpstreamSchema::http),
        }
    }
}

impl From<UpstreamRequest> for UpstreamBuilder {
    fn from(upstream: UpstreamRequest) -> Self {
        UpstreamBuilder {
            id: upstream.id,
            retries: upstream.retries,
            retry_timeout: upstream.retry_timeout,
            timeout: upstream.timeout,
            nodes: upstream.nodes,
            service_name: upstream.service_name,
            discovery_type: upstream.discovery_type,
            type_field: upstream.type_field,
            name: upstream.name,
            desc: upstream.desc,
            scheme: upstream.scheme,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum UpstreamType {
    roundrobin,
    chash,
    ewma,
    least_conn,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum UpstreamTypeChashAuxiliary {
    vars,
    header,
    cookie,
    consumer,
}

impl From<String> for UpstreamTypeChashAuxiliary {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "vars" => UpstreamTypeChashAuxiliary::vars,
            "header" => UpstreamTypeChashAuxiliary::header,
            "cookie" => UpstreamTypeChashAuxiliary::cookie,
            "consumer" => UpstreamTypeChashAuxiliary::consumer,
            _ => UpstreamTypeChashAuxiliary::vars
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Display, EnumString)]
#[allow(non_camel_case_types)]
#[strum(ascii_case_insensitive)]
#[non_exhaustive]
pub enum UpstreamSchema {
    http,
    https,
    grpc,
    grpcs,
    tcp,
    udp,
    tls
}

// region: tests
#[cfg(test)]
mod tests {
    use serde_json::{to_string, to_string_pretty};
    use super::*;
    use tracing::{error, info};
    use tracing_test::traced_test;
    use crate::models::admin_upstream_requests::UpstreamType;

    #[traced_test]
    #[tokio::test]
    async fn test_generate_upstream_request() {
        let nodes = r#"
        {
            "localhost:9000": 1
        }"#;
        let nodes = serde_json::from_str(nodes).unwrap();

        let upstream_req = UpstreamBuilder::new()
            .with_id("test_upstream".to_string())
            .with_name("Test Upstream".to_string())
            .with_desc("Test Upstream Description".to_string())
            .with_schema(UpstreamSchema::https)
            .with_u_type(UpstreamType::roundrobin)
            .with_nodes(nodes)
            .with_retries(3)
            .with_retry_timeout(5)
            .with_timeout(ApisixTimeout { connect: Some(0.5), send: Some(0.5), read: Some(0.5) })
            .build().unwrap();
        info!("Upstream Request: {:?}", to_string(&upstream_req));
        assert!(true)
    }
}
// endregion: tests