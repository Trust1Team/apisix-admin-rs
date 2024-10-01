pub mod ctrl_api_responses;
pub mod admin_api_responses;
mod admin_api_upstream_requests;
mod admin_api_service_requests;
mod plugins;

use std::fmt;
use std::num::NonZeroU32;
pub use admin_api_upstream_requests::*;

use rand::distr::Alphanumeric;
use serde::{Deserialize, Deserializer, Serialize};
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
    use crate::models::admin_api_upstream_requests::UpstreamType;

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
