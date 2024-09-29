mod reqwest_generic;
pub mod admin_api_client;

//! In Apache APISIX, the control API is used to:
//!
//! Expose the internal state of APISIX.
//! Control the behavior of a single, isolated APISIX data plane.
pub mod control_api_client;
