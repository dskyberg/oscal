//! UUID data type.  Supported by [uuid].
//!
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

use super::StringType;
use crate::{Error, SchemaConstraint};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UUIDDatatype(Uuid);

impl UUIDDatatype {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UUIDDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for UUIDDatatype {
    type Target = Uuid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for UUIDDatatype {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TryFrom<&str> for UUIDDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = Uuid::parse_str(value)?;
        Ok(Self(result))
    }
}

impl StringType for UUIDDatatype {
    fn pattern() -> &'static str {
        r"^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[45][0-9A-Fa-f]{3}-[89ABab][0-9A-Fa-f]{3}-[0-9A-Fa-f]{12}$"
    }
}

impl SchemaConstraint for UUIDDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#"A type 4 ('random' or 'pseudorandom') or type 5 UUID per RFC 4122."#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "UUIDDatatype"
    }
    fn validate(value: &str) -> Result<(), Error> {
        let _ = uuid::Uuid::parse_str(value);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_uuid() {
        let input = "blah";
        assert!(UUIDDatatype::validate(input).is_err());

        let input = UUIDDatatype::new();
        assert!(UUIDDatatype::validate(&input.to_string()).is_ok());
    }
}
