use tracing::{error, info, warn, instrument, debug};
use anyhow::Result;
use serde_json::Value;
use apisix_admin_client::{admin_check, admin_create_upstream, admin_create_upstream_with_id, admin_delete_upstream, admin_get_upstream, admin_get_upstreams, UpstreamBuilder, UpstreamSchema, UpstreamTimeout, UpstreamType};
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

    // Get default Config (from environment variables)
    // let cfg = get_config_from_env().await;
    // or use builder pattern to construct the config
    let cfg = ApisixConfigBuilder::new().admin_url("http://localhost:9180").build().expect("Error building config");
    debug!("Apisix Config: {:?}", cfg);

    // Check if Admin API is up and running
    match admin_check(&cfg).await {
        Ok(_) => info!("OK::Admin API is up and running"),
        Err(e) => error!("Error checking Admin API: {:?}", e)
    }

    // Upstream Use Cases
    let upstream_id = upstream_use_cases(&cfg).await?;

    // Delete Upstream
    match admin_delete_upstream(&cfg, &upstream_id).await {
        Ok(_) => info!("OK::Upstream API delete upstream by id"),
        Err(e) => error!("Error deleting upstream: {:?}", e)
    }

    info!("===Example::Apisix Admin Client END===");
    Ok(())
}

async fn upstream_use_cases(cfg: &ApisixConfig) -> Result<String> {
    // Get Upstreams
    match admin_get_upstreams(cfg).await {
        Ok(upstreams) => {
            debug!("Upstreams: {:?}", upstreams);
            info!("OK::Upstream API get upstreams")
        },
        Err(e) => error!("Error getting upstreams: {:?}", e)
    }

    // Get Upstream by id
    match admin_get_upstream(cfg, "1").await {
        Ok(upstream) => {
            info!("OK::Upstream API get upstream by id")
        },
        Err(e) => error!("Error getting upstream by id: {:?}", e)
    }

    upstream_use_cases_create_upstream(cfg).await
}

async fn upstream_use_cases_create_upstream(cfg: &ApisixConfig) -> Result<String> {
    // Create Upstream with custom id
    let nodes = r#"
        {
            "localhost:9000": 1
        }"#;
    let node_defs: Value = serde_json::from_str(nodes).unwrap();

    let upstream_req = UpstreamBuilder::new()
        .name("Test Upstream".to_string())
        .desc("Test Upstream Description".to_string())
        .schema(UpstreamSchema::https)
        .u_type(UpstreamType::roundrobin)
        .nodes(node_defs)
        .retries(3)
        .retry_timeout(5)
        .timeout(UpstreamTimeout { connect: 0.5, send: 0.5, read: 0.5 })
        .build()?;

    info!("==> Creating Upstream with custom id: {:?}", serde_json::to_string(&upstream_req));

    match admin_create_upstream(cfg, &upstream_req).await {
        Ok(res) => {
            debug!("Upstream response: {:?}", res);
            info!("OK::Upstream API create upstream by id");
            Ok(res.value.unwrap().id.unwrap())
        },
        Err(e) => {
            error!("Error creating upstream by id: {:?}", e);
            Err(e)
        }
    }
}

async fn upstream_use_cases_create_upstream_with_id(cfg: &ApisixConfig) -> Result<String> {
    // Create Upstream with custom id
    let nodes = r#"
        {
            "localhost:9000": 1
        }"#;
    let node_defs: Value = serde_json::from_str(nodes).unwrap();

    let upstream_req = UpstreamBuilder::new()
        .id("test_upstream".to_string())
        .name("Test Upstream".to_string())
        .desc("Test Upstream Description".to_string())
        .schema(UpstreamSchema::https)
        .u_type(UpstreamType::roundrobin)
        .nodes(node_defs)
        .retries(3)
        .retry_timeout(5)
        .timeout(UpstreamTimeout { connect: 0.5, send: 0.5, read: 0.5 })
        .build().unwrap();

    info!("==> Creating Upstream with custom id: {:?}", serde_json::to_string(&upstream_req));

    match admin_create_upstream_with_id(cfg, "test_upstream", &upstream_req).await {
        Ok(res) => {
            debug!("Upstream response: {:?}", res);
            info!("OK::Upstream API create upstream by id")
        },
        Err(e) => error!("Error creating upstream by id: {:?}", e)
    }

    Ok(String::from("test_upstream"))
}
