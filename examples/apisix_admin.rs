use tracing::{error, info, warn, instrument, debug};
use anyhow::Result;
use base64::Engine;
use apisix_admin_client::admin_check;
use apisix_admin_client::config::ApisixConfigBuilder;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    admin_ucs().await?;

    Ok(())
}

#[instrument]
async fn admin_ucs() -> Result<()> {
    info!("===Example::Apisix Admin Client===");

    /// Get default Config (from environment variables)
    /// let cfg = get_config_from_env().await;
    /// or use builder pattern to construct the config
    let cfg = ApisixConfigBuilder::new().admin_url("http://localhost:9180").build().expect("Error building config");
    debug!("Apisix Config: {:?}", cfg);

    /// Check if Admin API is up and running
    match admin_check(cfg).await {
        Ok(_) => info!("Admin API is up and running"),
        Err(e) => error!("Error checking Admin API: {:?}", e)
    }

    info!("===Example::Apisix Admin Client END===");
    Ok(())
}
