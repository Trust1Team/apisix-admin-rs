use serde::{Deserialize, Serialize};
use crate::models::{generate_identifier, Plugins};
use crate::UpstreamRequest;
use crate::{Result};
use crate::models::common::ApisixTimeout;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteBuilder {
    pub id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub status: Option<i64>,
    pub plugins: Option<Plugins>,
    pub uri: Option<String>,
    pub uris: Option<Vec<String>>,
    pub host: Option<String>,
    pub hosts: Option<Vec<String>>,
    pub remote_addr: Option<String>,
    pub remote_addrs: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub upstream: Option<UpstreamRequest>,
    pub upstream_id: Option<String>,
    pub service_id: Option<String>,
    pub timeout: Option<ApisixTimeout>,
    pub enable_websocket: Option<bool>,
    pub priority: Option<i64>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        RouteRequest::default().into()
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Identifier for the Route
    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Description of usage scenarios
    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// Enables the current Route. Set to 1 (enabled) by default
    /// 1 to enable, 0 to disable
    pub fn with_status(mut self, status: i64) -> Self {
        self.status = Some(status);
        self
    }

    /// Plugins that are executed during the request/response cycle. See [@Plugins] for more
    pub fn with_plugins(mut self, plugins: Plugins) -> Self {
        self.plugins = Some(plugins);
        self
    }

    /// Matches the uri. For more advanced matching see Router
    /// Example: "/hello"
    pub fn with_uri(mut self, uri: String) -> Self {
        self.uri = Some(uri);
        self
    }

    /// Matches with any one of the multiple uris specified in the form of a non-empty list
    /// Example: ["/hello", "/word"]
    pub fn with_uris(mut self, uris: Vec<String>) -> Self {
        self.uris = Some(uris);
        self
    }

    /// Matches with domain names such as foo.com or PAN domain names like *.foo.com.
    /// Example: "foo.com"
    pub fn with_host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    /// Matches with any one of the multiple hosts specified in the form of a non-empty list
    /// Example: ["foo.com", "bar.com"]
    pub fn with_hosts(mut self, hosts: Vec<String>) -> Self {
        self.hosts = Some(hosts);
        self
    }

    /// Matches with the specified IP address in standard IPv4 format (192.168.1.101),
    /// CIDR format (192.168.1.0/24), or in IPv6 format (::1, fe80::1, fe80::1/64)
    /// Example: "192.168.1.0/24"
    pub fn with_remote_addr(mut self, remote_addr: String) -> Self {
        self.remote_addr = Some(remote_addr);
        self
    }

    /// Matches with any one of the multiple remote_addrs specified in the form of a non-empty list
    /// Example: ["127.0.0.1", "192.0.0.0/8", "::1"]
    pub fn with_remote_addrs(mut self, remote_addrs: Vec<String>) -> Self {
        self.remote_addrs = Some(remote_addrs);
        self
    }

    /// Matches with the specified methods. Matches all methods if empty or unspecified
    /// Example: ["GET", "POST"]
    pub fn with_methods(mut self, methods: Vec<String>) -> Self {
        self.methods = Some(methods);
        self
    }

    /// Configuration of the Upstream [@UpstreamRequest]
    pub fn with_upstream(mut self, upstream: UpstreamRequest) -> Self {
        self.upstream = Some(upstream);
        self
    }

    /// Id of the Upstream service
    pub fn with_upstream_id(mut self, upstream_id: String) -> Self {
        self.upstream_id = Some(upstream_id);
        self
    }

    /// Configuration of the bound Service
    pub fn with_service_id(mut self, service_id: String) -> Self {
        self.service_id = Some(service_id);
        self
    }

    /// Sets the timeout (in seconds) for connecting to, and sending and receiving messages
    /// between the Upstream and the Route.
    /// This will overwrite the timeout value configured in your Upstream
    /// Example: {"connect": 0.5,"send": 0.5,"read": 0.5}
    pub fn with_timeout(mut self, timeout: ApisixTimeout) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Enables a websocket. Set to false by default
    pub fn with_enable_websocket(mut self, enable_websocket: bool) -> Self {
        self.enable_websocket = Some(enable_websocket);
        self
    }

    /// If different Routes matches to the same uri,
    /// then the Route is matched based on its priority.
    /// A higher value corresponds to higher priority. It is set to 0 by default.
    pub fn priority(mut self, priority: i64) -> Self {
        self.priority = Some(priority);
        self
    }

    pub fn build(self) -> Result<RouteRequest> {
        Ok(RouteRequest {
            id: self.id,
            name: self.name,
            desc: self.desc,
            status: self.status,
            plugins: self.plugins,
            uri: self.uri,
            uris: self.uris,
            host: self.host,
            hosts: self.hosts,
            remote_addr: self.remote_addr,
            remote_addrs: self.remote_addrs,
            methods: self.methods,
            upstream: self.upstream,
            upstream_id: self.upstream_id,
            service_id: self.service_id,
            timeout: self.timeout,
            enable_websocket: self.enable_websocket,
            priority: self.priority,
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub status: Option<i64>,
    pub plugins: Option<Plugins>,
    pub uri: Option<String>,
    pub uris: Option<Vec<String>>,
    pub host: Option<String>,
    pub hosts: Option<Vec<String>>,
    pub remote_addr: Option<String>,
    pub remote_addrs: Option<Vec<String>>,
    pub methods: Option<Vec<String>>,
    pub upstream: Option<UpstreamRequest>,
    pub upstream_id: Option<String>,
    pub service_id: Option<String>,
    pub timeout: Option<ApisixTimeout>,
    pub enable_websocket: Option<bool>,
    pub priority: Option<i64>,
}

impl Default for RouteRequest {
    fn default() -> Self {
        RouteRequest {
            id: Some(generate_identifier()),
            name: None,
            desc: None,
            status: None,
            plugins: None,
            uri: None,
            uris: None,
            host: None,
            hosts: None,
            remote_addr: None,
            remote_addrs: None,
            methods: None,
            upstream: None,
            upstream_id: None,
            service_id: None,
            timeout: None,
            enable_websocket: None,
            priority: None,
        }
    }
}

impl From<RouteRequest> for RouteBuilder {
    fn from(route: RouteRequest) -> Self {
        RouteBuilder {
            id: route.id,
            name: route.name,
            desc: route.desc,
            status: route.status,
            plugins: route.plugins,
            uri: route.uri,
            uris: route.uris,
            host: route.host,
            hosts: route.hosts,
            remote_addr: route.remote_addr,
            remote_addrs: route.remote_addrs,
            methods: route.methods,
            upstream: route.upstream,
            upstream_id: route.upstream_id,
            service_id: route.service_id,
            timeout: route.timeout,
            enable_websocket: route.enable_websocket,
            priority: route.priority,
        }
    }
}