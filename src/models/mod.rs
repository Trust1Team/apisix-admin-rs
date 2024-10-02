pub mod common;
pub mod ctrl_responses;
pub mod admin_upstream_responses;
pub mod admin_upstream_requests;
pub mod admin_service_requests;
pub mod admin_service_responses;
pub mod plugins;
pub mod admin_route_requests;
pub mod admin_route_responses;
pub mod consumer_group_requests;
pub mod consumer_group_responses;
pub mod consumer_requests;
pub mod consumer_responses;

pub use admin_upstream_requests::*;

use rand::distr::Alphanumeric;
use serde::de::Visitor;

/// ID's as a text string must be of a length between 1 and 64 characters
/// and they should only contain uppercase, lowercase, numbers
/// and no special characters apart from dashes ( - ), periods ( . ) and underscores ( _ ).
/// For integer values they simply must have a minimum character count of 1.
pub (super) fn generate_identifier() -> String {
    use rand::Rng;
    format!("gen-{}", rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::{error, info};
    use tracing_test::traced_test;
    use crate::models::admin_upstream_requests::UpstreamType;

    #[traced_test]
    #[tokio::test]
    async fn test_generate_id() {
        let id = generate_identifier();
        info!("Generated id: {}", id);
        assert_eq!(id.len(), 16);
    }

    #[traced_test]
    #[tokio::test]
    async fn test_enum_case() {
        let mut t = UpstreamType::roundrobin;
        info!("Upstream type [roundrobin]: {}", t);
        assert_eq!(t.to_string(), "roundrobin");
        t = UpstreamType::least_conn;
        info!("Upstream type [least_conn]: {}", t);
        assert_eq!(t.to_string(), "least_conn");
    }
}
