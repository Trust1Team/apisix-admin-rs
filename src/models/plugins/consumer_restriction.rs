use serde::{Deserialize, Serialize};
use crate::{Result};
use crate::models::Plugin;

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAuth {
    pub hide_credentials: Option<bool>,
    pub query: Option<String>,
    pub header: Option<String>,
}