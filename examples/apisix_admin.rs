use tracing::{error, info, warn, instrument, debug};
use anyhow::Result;
use apisix_admin_client::{admin_check, admin_get_upstreams};
use apisix_admin_client::config::{ApisixConfig, ApisixConfigBuilder};

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
    match admin_check(&cfg).await {
        Ok(_) => info!("OK::Admin API is up and running"),
        Err(e) => error!("Error checking Admin API: {:?}", e)
    }

    /// Upstream Use Cases
    upstream_use_cases(&cfg).await;

    info!("===Example::Apisix Admin Client END===");
    Ok(())
}

async fn upstream_use_cases(cfg: &ApisixConfig) {
    /// Get Upstreams
    match admin_get_upstreams(cfg).await {
        Ok(upstreams) => {
            debug!("Upstreams: {:?}", upstreams);
            info!("OK::Upstream API get upstreams")
        },
        Err(e) => error!("Error getting upstreams: {:?}", e)
    }
}
