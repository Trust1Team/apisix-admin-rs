use serde::{Deserialize, Serialize};
use crate::common::ApisixTimeout;
use crate::models::generate_identifier;
use crate::plugins::Plugins;
use crate::UpstreamRequest;
use crate::{Result};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteBuilder {
    pub id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub status: Option<i64>,
    pub update_time: Option<i64>,
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
    pub create_time: Option<i64>,
    pub priority: Option<i64>,
}

impl RouteBuilder {
    pub fn new() -> Self {
        RouteRequest::default().into()
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Identifier for the Route
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Description of usage scenarios
    pub fn desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// Enables the current Route. Set to 1 (enabled) by default
    /// 1 to enable, 0 to disable
    pub fn status(mut self, status: i64) -> Self {
        self.status = Some(status);
        self
    }

    /// Plugins that are executed during the request/response cycle. See [@Plugins] for more
    pub fn plugins(mut self, plugins: Plugins) -> Self {
        self.plugins = Some(plugins);
        self
    }

    /// Matches the uri. For more advanced matching see Router
    /// Example: "/hello"
    pub fn uri(mut self, uri: String) -> Self {
        self.uri = Some(uri);
        self
    }

    /// Matches with any one of the multiple uris specified in the form of a non-empty list
    /// Example: ["/hello", "/word"]
    pub fn uris(mut self, uris: Vec<String>) -> Self {
        self.uris = Some(uris);
        self
    }

    /// Matches with domain names such as foo.com or PAN domain names like *.foo.com.
    /// Example: "foo.com"
    pub fn host(mut self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    /// Matches with any one of the multiple hosts specified in the form of a non-empty list
    /// Example: ["foo.com", "bar.com"]
    pub fn hosts(mut self, hosts: Vec<String>) -> Self {
        self.hosts = Some(hosts);
        self
    }

    /// Matches with the specified IP address in standard IPv4 format (192.168.1.101),
    /// CIDR format (192.168.1.0/24), or in IPv6 format (::1, fe80::1, fe80::1/64)
    /// Example: "192.168.1.0/24"
    pub fn remote_addr(mut self, remote_addr: String) -> Self {
        self.remote_addr = Some(remote_addr);
        self
    }

    /// Matches with any one of the multiple remote_addrs specified in the form of a non-empty list
    /// Example: ["127.0.0.1", "192.0.0.0/8", "::1"]
    pub fn remote_addrs(mut self, remote_addrs: Vec<String>) -> Self {
        self.remote_addrs = Some(remote_addrs);
        self
    }

    /// Matches with the specified methods. Matches all methods if empty or unspecified
    /// Example: ["GET", "POST"]
    pub fn methods(mut self, methods: Vec<String>) -> Self {
        self.methods = Some(methods);
        self
    }

    /// Configuration of the Upstream [@UpstreamRequest]
    pub fn upstream(mut self, upstream: UpstreamRequest) -> Self {
        self.upstream = Some(upstream);
        self
    }

    /// Id of the Upstream service
    pub fn upstream_id(mut self, upstream_id: String) -> Self {
        self.upstream_id = Some(upstream_id);
        self
    }

    /// Configuration of the bound Service
    pub fn service_id(mut self, service_id: String) -> Self {
        self.service_id = Some(service_id);
        self
    }

    /// Sets the timeout (in seconds) for connecting to, and sending and receiving messages
    /// between the Upstream and the Route.
    /// This will overwrite the timeout value configured in your Upstream
    /// Example: {"connect": 0.5,"send": 0.5,"read": 0.5}
    pub fn timeout(mut self, timeout: ApisixTimeout) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Enables a websocket. Set to false by default
    pub fn enable_websocket(mut self, enable_websocket: bool) -> Self {
        self.enable_websocket = Some(enable_websocket);
        self
    }

    pub fn update_time(mut self, update_time: i64) -> Self {
        self.update_time = Some(update_time);
        self
    }

    pub fn create_time(mut self, create_time: i64) -> Self {
        self.create_time = Some(create_time);
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
            update_time: self.update_time,
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
            create_time: self.create_time,
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
    pub update_time: Option<i64>,
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
    pub create_time: Option<i64>,
    pub priority: Option<i64>,
}

impl Default for RouteRequest {
    fn default() -> Self {
        RouteRequest {
            id: Some(generate_identifier()),
            name: None,
            desc: None,
            status: None,
            update_time: None,
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
            create_time: None,
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
            update_time: route.update_time,
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
            create_time: route.create_time,
            priority: route.priority,
        }
    }
}