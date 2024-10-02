use std::ops::Index;
#[allow(dead_code)]
use tracing::{error, info, warn, instrument, debug};
use apisix_admin_client::error::ApisixClientError;
use apisix_admin_client::models::{KeyAuthBuilder, LimitCountBuilder, LimitCountPolicy, Plugin, Plugins, ProxyRewriteBuilder, ProxyRewriteHeaders, ProxyRewriteMethod};

type Result<T> = std::result::Result<T, ApisixClientError>;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    admin_plugin_config().await?;

    Ok(())
}

/// Plugin examples
#[instrument]
async fn admin_plugin_config() -> Result<()> {
    // KeyAuth Plugin
    let pka = KeyAuthBuilder::new()
        .with_header("X-API-KEY")
        .with_query("apikey")
        .with_hide_credentials(true)
        .build()?;
    info!("KeyAuthPlugin::OK::{:?}", pka);

    // ProxyRewrite Plugin
    let ppr_headers: ProxyRewriteHeaders = ProxyRewriteHeaders {
        set: Some(serde_json::json!({
            "X-Server-id": "3",
            "X-Server-status": "on",
            "X-Server-balancer-addr": "$balancer_ip:$balancer_port"
        })),
        remove: Some(serde_json::json!({
            "X-TOBE-REMOVED": "112233"
        })),
        add: Some(serde_json::json!({
            "X-Request-ID": "111111"
        })),
    };
    let ppr = ProxyRewriteBuilder::new()
        .with_uri("http://localhost:8080")
        .with_host("localhost")
        .with_headers(ppr_headers)
        .build()?;
    info!("ProxyRewritePlugin::OK::{:?}", ppr);

    // LimitCount Plugin
    let plc = LimitCountBuilder::new()
        .with_count(100)
        .with_time_window(60)
        .with_rejected_code(429)
        .with_key("remote_addr")
        .with_policy(LimitCountPolicy::local)
        .build()?;
    info!("LimitCountPlugin::OK::{:?}", plc);

    // Plugin collection
    let plugins: Plugins = Plugins {
        key_auth: Some(pka),
        proxy_rewrite: Some(ppr),
        limit_count: Some(plc),
        ..Default::default()
    };

    Ok(())
}