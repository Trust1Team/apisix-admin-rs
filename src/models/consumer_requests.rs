use serde::{Deserialize, Serialize};
use crate::{Result};
use crate::models::Plugins;

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerBuilder {
    pub plugins: Option<Plugins>,
    pub username: Option<String>,
    pub group_id: Option<String>,
    pub desc: Option<String>,
}

impl ConsumerBuilder {

    pub fn new() -> Self {
        ConsumerBuilder::default().into()
    }

    /// Name of the Consumer
    pub fn with_username(mut self, username: String) -> Self {
        self.username = Some(username);
        self
    }

    /// Group of the Consumer
    pub fn with_group_id(mut self, group_id: String) -> Self {
        self.group_id = Some(group_id);
        self
    }

    /// Description of usage scenarios
    pub fn with_desc(mut self, desc: String) -> Self {
        self.desc = Some(desc);
        self
    }

    /// Plugins that are executed during the request/response cycle. See [@Plugins]] for more
    pub fn with_plugins(mut self, plugins: Plugins) -> Self {
        self.plugins = Some(plugins);
        self
    }
    pub fn build(self) -> Result<ConsumerRequest> {
        Ok(ConsumerRequest {
            plugins: self.plugins,
            username: self.username,
            group_id: self.group_id,
            desc: self.desc,
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerRequest {
    pub plugins: Option<Plugins>,
    pub username: Option<String>,
    pub group_id: Option<String>,
    pub desc: Option<String>,
}

impl Default for ConsumerRequest {
    fn default() -> Self {
        ConsumerRequest {
            plugins: None,
            username: None,
            group_id: None,
            desc: None,
        }
    }
}

impl From<ConsumerRequest> for ConsumerBuilder {
    fn from(consumer: ConsumerRequest) -> Self {
        ConsumerBuilder {
            plugins: consumer.plugins,
            username: consumer.username,
            group_id: consumer.group_id,
            desc: consumer.desc,
        }
    }
}