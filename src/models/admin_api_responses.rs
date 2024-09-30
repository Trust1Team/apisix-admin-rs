use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::UpstreamTypeChashAuxiliary;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListResponse<T> {
    #[serde(rename = "list")]
    pub list: Vec<T>,
    pub total: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenericJsonResponse {
    pub created_index: i64,
    pub key: String,
    pub value: Value,
    pub modified_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypedItem<T> {
    pub created_index: Option<i64>,
    pub key: Option<String>,
    pub value: Option<T>,
    pub modified_index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Upstream {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub desc: Option<String>,
    pub scheme: Option<String>,
    pub update_time: Option<i64>,
    pub nodes: Value, //untyped
    pub create_time: Option<i64>,
    pub name: Option<String>,
    pub id: Option<String>,
}