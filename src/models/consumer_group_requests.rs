use serde::{Deserialize, Serialize};
use crate::models::{generate_identifier, Plugins};
use crate::{Result};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroupBuilder {
    pub plugins: Plugins,
    pub id: Option<String>,
    pub desc: Option<String>,
}

impl ConsumerGroupBuilder {
    pub fn new() -> Self {
        ConsumerGroupRequest::default().into()
    }

    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    /// Description of usage scenarios
    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// Plugins that are executed during the request/response cycle. See [Plugins] for more
    pub fn with_plugins(mut self, plugins: Plugins) -> Self {
        self.plugins = plugins;
        self
    }

    pub fn build(&self) -> Result<ConsumerGroupRequest> {
        Ok(ConsumerGroupRequest {
            plugins: self.plugins.clone(),
            id: self.id.clone(),
            desc: self.desc.clone(),
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerGroupRequest {
    pub plugins: Plugins,
    pub id: Option<String>,
    pub desc: Option<String>,
}

impl Default for ConsumerGroupRequest {
    fn default() -> Self {
        ConsumerGroupRequest {
            plugins: Plugins::default(),
            id: Some(generate_identifier()),
            desc: None,
        }
    }
}

impl From<ConsumerGroupRequest> for ConsumerGroupBuilder {
    fn from(consumer_group: ConsumerGroupRequest) -> Self {
        ConsumerGroupBuilder {
            plugins: consumer_group.plugins,
            id: consumer_group.id,
            desc: consumer_group.desc,
        }
    }
}