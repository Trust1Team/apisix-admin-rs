use thiserror::Error;

pub type ApisixLibError<T> = Result<T, ApisixClientError>;

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
    InvalidRequest(String),

    /// Session timed out
    #[error("Session time-out exception")]
    SessionTimeoutException,

    /// Authentication Exception
    #[error("Authentication Exception")]
    AuthenticationException
}