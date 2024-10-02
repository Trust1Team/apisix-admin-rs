use std::ops::Index;
#[allow(dead_code)]
use tracing::{error, info, warn, instrument, debug};
use apisix_admin_client::error::ApisixClientError;

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

#[instrument]
async fn admin_plugin_config() -> Result<()> {
    let _ = KeyAuthBuilder::new();
}