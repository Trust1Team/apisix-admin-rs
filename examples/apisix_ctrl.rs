use tracing::{error, info, warn, instrument, debug};
use anyhow::Result;
use base64::Engine;
use apisix_admin_client::{admin_check, ctrl_schema};
use apisix_admin_client::config::ApisixConfigBuilder;

#[tokio::main]
#[instrument]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    ctrl_ucs().await?;

    Ok(())
}

#[instrument]
async fn ctrl_ucs() -> Result<()> {
    info!("===Example::Apisix Controller Client ===");

    // Get default Config (from environment variables)
    // let cfg = get_config_from_env().await;
    // or use builder pattern to construct the config
    let cfg = ApisixConfigBuilder::new().control_url("http://localhost:9090").build().expect("Error building config");
    debug!("Apisix Config: {:?}", cfg);

    // Check Apisix schema - not diplayed
    match ctrl_schema(cfg).await {
        Ok(_) => info!("Schema: OK"),
        Err(e) => error!("Error checking schema: {:?}", e)
    }

    info!("===Example::Apisix Controller Client END===");
    Ok(())
}
