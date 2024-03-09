//! UUID data type.  Supported by [uuid].
//!
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use uuid::Uuid;

use crate::{Error, SchemaElement, Validate};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UUIDDatatype(String);

impl UUIDDatatype {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

impl Default for UUIDDatatype {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for UUIDDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for UUIDDatatype {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<&str> for UUIDDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let result = Uuid::parse_str(value)?;
        Ok(Self(result.to_string()))
    }
}

impl SchemaElement for UUIDDatatype {
    fn schema_title() -> &'static str {
        ""
    }
    fn schema_description() -> &'static str {
        r#"A type 4 ('random' or 'pseudorandom') or type 5 UUID per RFC 4122."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "UUIDDatatype"
    }
}

impl Validate for UUIDDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let _ = uuid::Uuid::parse_str(value)?;
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
