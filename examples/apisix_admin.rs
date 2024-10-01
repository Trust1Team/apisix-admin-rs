use std::ops::Index;
#[allow(dead_code)]
use tracing::{error, info, warn, instrument, debug};
use serde_json::Value;
use apisix_admin_client::{admin_check, admin_create_upstream_with_id, admin_delete_upstream, admin_get_services, admin_get_upstream, admin_get_upstreams, UpstreamBuilder, UpstreamSchema, UpstreamTimeout, UpstreamType};
use apisix_admin_client::config::{ApisixConfig, ApisixConfigBuilder};
use apisix_admin_client::error::ApisixClientError;
type Result<T> = std::result::Result<T, ApisixClientError>;

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
    info!("=== Example::Apisix Admin Client ===");

    // Get default Config (from environment variables)
    // let cfg = get_config_from_env().await;
    // or use builder pattern to construct the config
    //let cfg = ApisixConfigBuilder::new().admin_url("http://localhost:9180").build().expect("Error building config");
    let cfg = ApisixConfigBuilder::new().admin_url("http://mbwgm22d.ngrok.app").build().expect("Error building config");

    debug!("Apisix Config: {:?}", cfg);

    let _ = admin_check(&cfg).await.map(|_| info!("OK::Admin API is up and running"))?;

    // Upstream
    let upstream_id = "test_upstream";
    let _ = admin_get_upstreams(&cfg).await.map(|_| info!("OK::Upstream API get upstreams"))?;
    let _ = upstream_use_cases_create_upstream_with_id(&cfg, upstream_id).await?;
    let _ = admin_get_upstream(&cfg, upstream_id).await.map(|_| info!("OK::admin_get_upstream"))?;
    let _ = admin_delete_upstream(&cfg, &upstream_id).await.map(|_| info!("OK::upstream_delete"))?;

    // Service
    let service_id = "test_service";
    let _ = admin_get_services(&cfg).await.map(|_| info!("OK::admin_get_services"))?;

    info!("=== Example::Apisix Admin Client END ===");
    Ok(())
}

// region: helpers

async fn upstream_use_cases_create_upstream_with_id(cfg: &ApisixConfig, id: impl Into<String>) -> Result<String> {
    let upstream_id: String = id.into();
    // Create Upstream with custom id
    let nodes = r#"
        {
            "localhost:9000": 1
        }"#;
    let node_defs: Value = serde_json::from_str(nodes).unwrap();

    let upstream_req = UpstreamBuilder::new()
        .id(upstream_id.clone())
        .name("Test Upstream".to_string())
        .desc("Test Upstream Description".to_string())
        .schema(UpstreamSchema::http)
        .u_type(UpstreamType::roundrobin)
        .nodes(node_defs)
        .retries(3)
        .retry_timeout(5)
        .timeout(UpstreamTimeout { connect: 0.5, send: 0.5, read: 0.5 })
        .build().unwrap();

    debug!("==> Creating Upstream with custom id: {:?}", serde_json::to_string(&upstream_req));

    match admin_create_upstream_with_id(cfg, upstream_id.as_str(), &upstream_req).await {
        Ok(res) => {
            debug!("Upstream response: {:?}", res);
            info!(r#"OK::Upstream API create upstream by id: {:?}"#, "test_upstream");
            Ok(String::from("test_upstream"))
        },
        Err(e) => {
            error!("Error creating upstream by id: {:?}", e);
            Err(e)
        }
    }
}
// endregion: helpers