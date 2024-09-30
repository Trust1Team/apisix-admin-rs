use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    pub created_index: i64,
    pub key: String,
    pub value: T,
    pub modified_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upstream {
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(rename = "hash_on")]
    pub hash_on: Option<String>,
    pub desc: Option<String>,
    pub scheme: Option<String>,
    #[serde(rename = "update_time")]
    pub update_time: Option<i64>,
    pub id: Option<i64>,
    pub nodes: Value, //untyped
    #[serde(rename = "create_time")]
    pub create_time: Option<i64>,
    #[serde(rename = "pass_host")]
    pub pass_host: Option<String>,
}