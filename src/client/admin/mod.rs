mod client;
pub use client::*;
//pub (crate) mod routes;

// region: mod constants
const ADMIN_PATH: &'static str = "/apisix/admin";
// endregion: mod constants

// region: Path definitions
fn path_check_version() -> String { format!("{}", ADMIN_PATH) }
fn path_upstreams() -> String { format!("{}/upstreams", ADMIN_PATH) }
fn path_upstream_with_id(id: &str) -> String { format!("{}/upstreams/{}", ADMIN_PATH, id) }
fn path_upstreams_with_id_and_path(id: &str, path: &str) -> String { format!("{}/upstreams/{}/{}", ADMIN_PATH, id, path) }
// endregion: Path definitions