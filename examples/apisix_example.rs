use tracing::{error, info, warn};
use anyhow::Result;
use base64::Engine;
use apisi_admin_client::config::ApisixConfigBuilder;
use apisi_admin_client::get_config_from_env;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("===Example::Apisix Client===");

    /// Get default Config (from environment variables)
    /// let cfg = get_config_from_env().await;
    /// or use builder pattern to construct the config
    let cfg = ApisixConfigBuilder::new().admin_url("http://localhost:9180").build().expect("Error building config");
    info!("Apisix Config: {:?}", cfg);

    info!("===Example::Apisix Client END===");
    Ok(())
}
