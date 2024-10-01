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
    pub created_index: Option<i64>,
    pub key: Option<String>,
    pub value: Option<T>,
    pub modified_index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApisixTimeout {
    pub connect: Option<f32>,
    pub send: Option<f32>,
    pub read: Option<f32>,
}