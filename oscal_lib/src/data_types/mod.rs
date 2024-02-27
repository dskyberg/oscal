pub use boolean::*;
pub use dates::*;
pub use numbers::*;
pub use strings::*;
pub use token::*;
pub use uris::*;
pub use uuid::*;

pub mod boolean;
pub mod dates;
pub mod nc_name;
pub mod numbers;
pub mod strings;
pub mod token;
pub mod uris;
pub mod uuid;

/// Default pattern for StringType
#[allow(dead_code)]
const DEFAULT_PATTERN: &str = "^\\S(.*\\S)?$";

pub trait StringType {
    fn pattern() -> &'static str {
        DEFAULT_PATTERN
    }
    fn format() -> Option<&'static str> {
        None
    }
    fn content_encoding() -> Option<&'static str> {
        None
    }
}

pub trait NumberType {
    fn minimum() -> Option<i64> {
        None
    }
    fn maximum() -> Option<i64> {
        None
    }
}

pub trait DecimalType {
    fn minimum() -> Option<f64> {
        None
    }
    fn maximum() -> Option<f64> {
        None
    }
}
