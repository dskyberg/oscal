use thiserror::Error;

/// Shorthand for standard Result
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Crate errors
#[derive(Error, Debug, PartialEq, PartialOrd)]
pub enum OscalError {
    #[error("NCName error: {0}")]
    BadNCName(String),
}
