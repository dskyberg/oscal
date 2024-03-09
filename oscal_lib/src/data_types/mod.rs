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
