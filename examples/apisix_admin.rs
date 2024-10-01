use std::ops::Index;
#[allow(dead_code)]
use tracing::{error, info, warn, instrument, debug};
use serde_json::Value;
use apisix_admin_client::{admin_check, admin_create_route_with_id, admin_create_service_with_id, admin_create_upstream_with_id, admin_delete_route, admin_delete_service, admin_delete_upstream, admin_get_routes, admin_get_service, admin_get_services, admin_get_upstream, admin_get_upstreams, UpstreamBuilder, UpstreamSchema, UpstreamType};
use apisix_admin_client::admin_route_requests::{RouteBuilder, RouteRequest};
use apisix_admin_client::admin_service_requests::{ServiceBuilder, ServiceRequest};
use apisix_admin_client::common::ApisixTimeout;
use apisix_admin_client::config::{ApisixConfig, ApisixConfigBuilder};
use apisix_admin_client::error::ApisixClientError;
use apisix_admin_client::plugins::{Plugin, Plugins};

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
    let _ = use_cases_create_upstream_with_id(&cfg, upstream_id).await?;
    let _ = admin_get_upstream(&cfg, upstream_id).await.map(|_| info!("OK::admin_get_upstream"))?;

    // Service
    let service_id = "test_service";
    let _ = admin_get_services(&cfg).await.map(|_| info!("OK::admin_get_services"))?;
    let _ = use_case_create_service_with_id(&cfg, service_id, upstream_id.to_string()).await?;
    let _ = admin_get_service(&cfg, service_id).await.map(|_| info!("OK::admin_get_service"))?;

    // Routes
    let route_id = "test_route";
    let _ = admin_get_routes(&cfg).await.map(|_| info!("OK::admin_get_routes"))?;
    let _ = use_case_create_route_with_id(&cfg, route_id, service_id.to_string()).await?;


    // Clean up (reverse)
    let _ = admin_delete_route(&cfg, &route_id).await.map(|_| info!("OK::route_delete"))?;
    let _ = admin_delete_service(&cfg, &service_id).await.map(|_| info!("OK::service_delete"))?;
    let _ = admin_delete_upstream(&cfg, &upstream_id).await.map(|_| info!("OK::upstream_delete"))?;

    info!("=== Example::Apisix Admin Client END ===");
    Ok(())
}

// region: helpers
async fn use_case_create_route_with_id(cfg: &ApisixConfig, id: impl Into<String>, service_id: String) -> Result<String> {
    let route_id: String = id.into();

    let req: RouteRequest = RouteBuilder::new()
        .id(route_id.clone())
        .name("Test Route".to_string())
        .desc("Test Route Description".to_string())
        .uri("/test".to_string())
        .methods(vec!["GET".to_string()])
        .service_id(service_id)
        .build()?;

    debug!("==> Creating Route with custom id: {:?}", serde_json::to_string(&req));

    match admin_create_route_with_id(cfg, route_id.as_str(), &req).await {
        Ok(res) => {
            debug!("Route response: {:?}", res);
            info!(r#"OK::Route API create route by id: {:?}"#, route_id);
            Ok(route_id)
        },
        Err(e) => {
            error!("Error creating route by id: {:?}", e);
            Err(e)
        }
    }
}
async fn use_case_create_service_with_id(cfg: &ApisixConfig, id: impl Into<String>, upstream_id: String) -> Result<String> {
    let service_id: String = id.into();
    let plugins: Plugins = Plugins::default();

    let req = ServiceBuilder::new()
        .id(service_id.clone())
        .name("Test Service".to_string())
        .desc("Test Service Description".to_string())
        .enable_websocket(false)
        .upstream_id(upstream_id)
        .plugins(plugins)
        .build()?;

    debug!("==> Creating Service with custom id: {:?}", serde_json::to_string(&req));

    match admin_create_service_with_id(cfg, service_id.as_str(), &req).await {
        Ok(res) => {
            debug!("Service response: {:?}", res);
            info!(r#"OK::Service API create service by id: {:?}"#, service_id);
            Ok(service_id)
        },
        Err(e) => {
            error!("Error creating service by id: {:?}", e);
            Err(e)
        }
    }
}

async fn use_cases_create_upstream_with_id(cfg: &ApisixConfig, id: impl Into<String>) -> Result<String> {
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
        .timeout(ApisixTimeout { connect: Some(0.5), send: Some(0.5), read: Some(0.5) })
        .build()?;

    debug!("==> Creating Upstream with custom id: {:?}", serde_json::to_string(&upstream_req));

    match admin_create_upstream_with_id(cfg, upstream_id.as_str(), &upstream_req).await {
        Ok(res) => {
            debug!("Upstream response: {:?}", res);
            info!(r#"OK::Upstream API create upstream by id: {:?}"#, upstream_id);
            Ok(upstream_id)
        },
        Err(e) => {
            error!("Error creating upstream by id: {:?}", e);
            Err(e)
        }
    }
}
// endregion: helpers