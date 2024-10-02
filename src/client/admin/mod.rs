mod client;
pub use client::*;
//pub (crate) mod routes;

// region: mod constants
const ADMIN_PATH: &str = "/apisix/admin";
// endregion: mod constants

// region: Path definitions
fn path_check_version() -> String { ADMIN_PATH.to_string() }
fn path_upstreams() -> String { format!("{}/upstreams", ADMIN_PATH) }
fn path_upstream_with_id(id: &str) -> String { format!("{}/upstreams/{}", ADMIN_PATH, id) }
fn path_services() -> String { format!("{}/services", ADMIN_PATH) }
fn path_service_with_id(id: &str) -> String { format!("{}/services/{}", ADMIN_PATH, id) }
fn path_routes() -> String { format!("{}/routes", ADMIN_PATH) }
fn path_route_with_id(id: &str) -> String { format!("{}/routes/{}", ADMIN_PATH, id) }
fn path_consumer_groups() -> String { format!("{}/consumer_groups", ADMIN_PATH) }
fn path_consumer_group_with_id(id: &str) -> String { format!("{}/consumer_groups/{}", ADMIN_PATH, id) }
fn path_consumer() -> String { format!("{}/consumers", ADMIN_PATH) }
fn path_consumer_with_id(id: &str) -> String { format!("{}/consumers/{}", ADMIN_PATH, id) }
// endregion: Path definitions


