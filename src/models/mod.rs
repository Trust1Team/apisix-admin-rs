pub mod requests;
pub mod responses;
pub mod common;
pub mod session;
pub mod ctrl_api_responses;
pub mod admin_api_responses;
mod admin_api_upstream_requests;
pub use admin_api_upstream_requests::*;

use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};

/// ID's as a text string must be of a length between 1 and 64 characters
/// and they should only contain uppercase, lowercase, numbers
/// and no special characters apart from dashes ( - ), periods ( . ) and underscores ( _ ).
/// For integer values they simply must have a minimum character count of 1.
pub (super) fn generate_identifier() -> String {
    use rand::Rng;
    format!("gen_{}", rand::thread_rng()
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
    use crate::common::Interaction;
    use crate::models::admin_api_upstream_requests::UpstreamType;
    use crate::models::requests::InteractionFlow;

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
