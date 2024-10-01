use serde::{Deserialize, Serialize};
use crate::plugins::Plugins;
use crate::{UpstreamRequest, UpstreamSchema, UpstreamTimeout, UpstreamType};
use crate::models::generate_identifier;
use crate::{Result};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceBuilder {
    pub id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub enable_websocket: Option<bool>,
    pub upstream: Option<UpstreamRequest>,
    pub upstream_id: Option<String>,
    pub plugins: Plugins,
    pub hosts: Option<Vec<String>>,
}

impl ServiceBuilder {
    pub fn new() -> Self {
        ServiceRequest::default().into()
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Identifier for the Service
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// Description of usage scenarios
    pub fn desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// Enables a websocket. Set to false by default
    pub fn enable_websocket(mut self, enable_websocket: bool) -> Self {
        self.enable_websocket = Some(enable_websocket);
        self
    }

    /// Configuration of the Upstream [@UpstreamRequest]
    pub fn upstream(mut self, upstream: UpstreamRequest) -> Self {
        self.upstream = Some(upstream);
        self
    }

    /// Id of the Upstream
    pub fn upstream_id(mut self, upstream_id: String) -> Self {
        self.upstream_id = Some(upstream_id);
        self
    }

    /// Plugins that are executed during the request/response cycle
    pub fn plugins(mut self, plugins: Plugins) -> Self {
        self.plugins = plugins;
        self
    }

    /// Matches with any one of the multiple hosts specified in the form of a non-empty list
    pub fn hosts(mut self, hosts: Vec<String>) -> Self {
        self.hosts = Some(hosts);
        self
    }

    pub fn build(self) -> Result<ServiceRequest> {
        Ok(ServiceRequest {
            id: self.id,
            name: self.name,
            desc: self.desc,
            enable_websocket: self.enable_websocket,
            upstream: self.upstream,
            upstream_id: self.upstream_id,
            plugins: self.plugins,
            hosts: self.hosts,
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: Option<String>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub enable_websocket: Option<bool>,
    pub upstream: Option<UpstreamRequest>,
    pub upstream_id: Option<String>,
    pub plugins: Plugins,
    pub hosts: Option<Vec<String>>,
}

impl Default for ServiceRequest {
    fn default() -> Self {
        ServiceRequest {
            id: Some(generate_identifier()),
            name: None,
            desc: None,
            enable_websocket: None,
            upstream: None,
            upstream_id: None,
            plugins: Plugins::default(),
            hosts: None,
        }
    }
}

impl From<ServiceRequest> for ServiceBuilder {
    fn from(service: ServiceRequest) -> Self {
        ServiceBuilder {
            id: service.id,
            name: service.name,
            desc: service.desc,
            enable_websocket: service.enable_websocket,
            upstream: service.upstream,
            upstream_id: service.upstream_id,
            plugins: service.plugins,
            hosts: service.hosts,
        }
    }
}