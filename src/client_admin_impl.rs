use tracing::instrument;
use crate::admin_route_requests::RouteRequest;
use crate::admin_route_responses::ApisixRoute;
use crate::admin_service_requests::ServiceRequest;
use crate::admin_service_responses::ApisixService;
use crate::client::AdminConnector;
use crate::common::{ListResponse, TypedItem};
use crate::config::ApisixConfig;
use crate::consumer_group_requests::ConsumerGroupRequest;
use crate::consumer_group_responses::ApisixConsumerGroup;
use crate::error::ApisixClientError::InvalidRequest;
use crate::models::admin_upstream_responses::ApisixUpstream;
use crate::models::UpstreamRequest;

type Result<T> = std::result::Result<T, crate::error::ApisixClientError>;

#[instrument(skip_all)]
pub (crate) async fn api_admin_check_version(cfg: &ApisixConfig) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.check_version().await.map_err(|e| InvalidRequest(e.to_string()))
}

// region: upstream
#[instrument(skip_all)]
pub (crate) async fn api_admin_get_upstreams(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixUpstream>>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_upstreams().await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_get_upstream(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixUpstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_upstream(id).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_create_upstream_with_id(cfg: &ApisixConfig, id: &str, req: &UpstreamRequest) -> Result<TypedItem<ApisixUpstream>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_upstream_with_id(id, req).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_delete_upstream(cfg: &ApisixConfig, id: &str) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.delete_upstream(id).await.map(|_| ()).map_err(|e| InvalidRequest(e.to_string()))
}
// endregion: upstream

// region: service
#[instrument(skip_all)]
pub (crate) async fn api_admin_get_services(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixService>>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_services().await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_get_service(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixService>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_service(id).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_create_service_with_id(cfg: &ApisixConfig, id: &str, req: &ServiceRequest) -> Result<TypedItem<ApisixService>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_service_with_id(id, req).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_delete_service(cfg: &ApisixConfig, id: &str) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.delete_service(id).await.map(|_| ()).map_err(|e| InvalidRequest(e.to_string()))
}
// endregion: service

// region: route
#[instrument(skip_all)]
pub (crate) async fn api_admin_get_routes(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixRoute>>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_routes().await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_get_route(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixRoute>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_route(id).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_create_route_with_id(cfg: &ApisixConfig, id: &str, req: &RouteRequest) -> Result<TypedItem<ApisixRoute>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_route_with_id(id, req).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_delete_route(cfg: &ApisixConfig, id: &str) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.delete_route(id).await.map(|_| ()).map_err(|e| InvalidRequest(e.to_string()))
}
// endregion: route

// region: consumer group
#[instrument(skip_all)]
pub (crate) async fn api_admin_get_consumer_groups(cfg: &ApisixConfig) -> Result<ListResponse<TypedItem<ApisixConsumerGroup>>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_consumer_groups().await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_get_consumer_group(cfg: &ApisixConfig, id: &str) -> Result<TypedItem<ApisixConsumerGroup>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.get_consumer_group(id).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_create_consumer_group_with_id(cfg: &ApisixConfig, id: &str, req: &ConsumerGroupRequest) -> Result<TypedItem<ApisixConsumerGroup>> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.create_consumer_group_with_id(id, req).await.map_err(|e| InvalidRequest(e.to_string()))
}

#[instrument(skip_all)]
pub (crate) async fn api_admin_delete_consumer_group(cfg: &ApisixConfig, id: &str) -> Result<()> {
    let ac: AdminConnector =  AdminConnector::new(cfg).await;
    ac.delete_consumer_group(id).await.map(|_| ()).map_err(|e| InvalidRequest(e.to_string()))
}
// endregion: consumer group