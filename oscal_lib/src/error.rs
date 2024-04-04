use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum Error {
    #[error("Boolean parsing error")]
    BooleanParse,
    #[error("UUID parsing error")]
    UuidParse(#[from] uuid::Error),
    #[error("Date parsing error")]
    DateParse(#[from] chrono::ParseError),
    #[error("URI parsing error")]
    UriParse(#[from] fluent_uri::ParseError),
    #[error("URI must be absolute")]
    UriAbsolute,
    #[error("Invalid Token")]
    InvaidToken,
    #[error("NCName illegal first char")]
    NCNameIllegalFirstChar,
    #[error("NCName illegal  char")]
    NCNameIllegalChar,
    #[error("Failed downcast {0}")]
    FailedDowncast(&'static str),
    #[error("JSON Parse error: {0}")]
    JsonParse(String),
}
