use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ApisixClientError {
    /// Config Exception
    #[error("Configuration missing: {0}")]
    ConfigMissingException(&'static str),

    /// Generic
    #[error("Admin Exception: {0}")]
    AdminException(&'static str),

    #[error("Invalid request")]
    InvalidRequest,

    /// Session timed out
    #[error("Session time-out exception")]
    SessionTimeoutException,

    /// Authentication Exception
    #[error("Authentication Exception")]
    AuthenticationException
}