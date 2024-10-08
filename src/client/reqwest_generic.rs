use anyhow::{bail, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::time::Duration;
use reqwest::Response;
use tracing::debug;

const HEADER_CONTENT_TYPE: &str = "content-type";
const HEADER_CONTENT_TYPE_DEFAULT: &str = "application/json";
const HEADER_USER_AGENT: &str = "User-Agent";
const HEADER_API_KEY: &str = "X-API-KEY";
const HEADER_USER_AGENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const HEADER_USER_AGENT_RUST_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

/// Generic get JWT based on APIKEY
/// Not used for Apisix client
#[allow(dead_code)]
pub async fn get_token<R>(url: &str, apikey: &str, timeout_millis: Option<u64>) -> Result<R>
where
    R: DeserializeOwned,
{
    let client = reqwest::Client::builder()
        //.danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_millis(
            timeout_millis.unwrap_or(30000),
        ))
        .build()?;
    client
        .get(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .send()
        .await?
        .json::<R>()
        .await
        .map_err(|e| e.into())
}

/// Generic HEAD request
pub async fn head(url: &str, apikey: &str, timeout_millis: u64, ) -> Result<()>
{
    let client = reqwest::Client::builder()
        //.danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()?;
    let send_response = client
        .head(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .send()
        .await?;
    let status = send_response.status().as_u16();
    match status {
        200..=299 => Ok(()),
        _ => {
            let response = send_response.bytes().await?;
            let text = String::from_utf8(response.to_vec()).unwrap();
            debug!("{:?}", text);
            bail!(text)
        }
    }
}

/// Generic GET request
/// Connection pooling is provided in `reqwest`
pub async fn get<R>(url: &str, apikey: &str, timeout_millis: u64, ) -> Result<R>
where
    R: DeserializeOwned,
{
    let client = reqwest::Client::builder()
        //.danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()?;
    let send_response = client
        .get(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .send()
        .await?;
    let status = send_response.status().as_u16();
    match status {
        200..=299 => send_response.json::<R>().await.map_err(|e| e.into()),
        _ => {
            let response = send_response.bytes().await?;
            let text = String::from_utf8(response.to_vec()).unwrap();
            debug!("{:?}", text);
            bail!(text)
        }
    }
}

/// Generic DELETE request
/// Connection pooling is provided in `reqwest`
pub async fn delete(
    url: &str,
    apikey: &str,
    timeout_millis: u64,
) -> Result<Response> {
    let client = reqwest::Client::builder()
        //.danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()?;
    client
        .delete(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .send()
        .await.map_err(|e| e.into())
}

/// Generic POST request
/// Connection pooling is provided in `reqwest`
#[allow(dead_code)]
pub async fn post<T, R>(
    url: &str,
    apikey: &str,
    req: &T,
    timeout_millis: u64,
) -> Result<R>
where
    T: Serialize + Debug,
    R: DeserializeOwned,
{
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()
        .unwrap();
    let send_response = client
        .post(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .json(req)
        .send()
        .await?;
    let status = send_response.status().as_u16();
    match status {
        200..=299 => send_response.json::<R>().await.map_err(|e| e.into()),
        _ => {
            let response = send_response.bytes().await?;
            let text = String::from_utf8(response.to_vec()).unwrap();
            debug!("{:?}", text);
            bail!(text)
        }
    }
}

/// Generic POST request
/// Connection pooling is provided in `reqwest`
pub async fn post_empty_body(
    url: &str,
    apikey: &str,
    timeout_millis: u64,
) -> Result<()>
{
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()
        .unwrap();
    let send_response = client
        .post(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .send()
        .await?;
    let status = send_response.status().as_u16();
    match status {
        200..=299 => Ok(()),
        _ => {
            let response = send_response.bytes().await?;
            let text = String::from_utf8(response.to_vec()).unwrap();
            debug!("{:?}", text);
            bail!(text)
        }
    }
}

// Generic POST request
/// Connection pooling is provided in `reqwest`
#[allow(dead_code)]
pub async fn post_json_value<T>(
    url: &str,
    apikey: &str,
    req: &T,
    timeout_millis: u64,
) -> Result<serde_json::Value>
where
    T: Serialize + Debug,
{
    let client = reqwest::Client::builder()
        //.danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()
        .unwrap();
    let send_response = client
        .post(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .json(req)
        .send()
        .await?;
    let status = send_response.status().as_u16();
    match status {
        200..=299 => send_response
            .json::<serde_json::Value>()
            .await
            .map_err(|e| e.into()),
        _ => {
            let response = send_response.bytes().await?;
            let text = String::from_utf8(response.to_vec()).unwrap();
            debug!("{:?}", text);
            bail!(text)
        }
    }
}

/// Generic PUT request
/// Connection pooling is provided in `reqwest`
pub async fn put<T, R>(
    url: &str,
    apikey: &str,
    req: &T,
    timeout_millis: u64,
) -> Result<R>
where
    T: Serialize + Debug,
    R: DeserializeOwned,
{
    let client = reqwest::Client::builder()
        //.danger_accept_invalid_certs(true)
        .timeout(Duration::from_millis(timeout_millis))
        .build()
        .unwrap();
    client
        .put(url)
        .header(HEADER_CONTENT_TYPE, HEADER_CONTENT_TYPE_DEFAULT)
        .header(HEADER_API_KEY, apikey)
        .header(HEADER_USER_AGENT, format!("apisix-admin-client/{:?}/rust/{:?}",HEADER_USER_AGENT_VERSION, HEADER_USER_AGENT_RUST_VERSION))
        .json(req)
        .send()
        .await?
        .json::<R>()
        .await
        .map_err(|e| e.into())
}


