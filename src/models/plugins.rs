use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Plugins {
    key_auth: Option<KeyAuth>,
}

pub trait Plugin {}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyAuth {
    pub hide_credentials: Option<bool>,
    pub query: Option<String>,
    pub header: Option<String>,
}

impl Plugin for KeyAuth {}