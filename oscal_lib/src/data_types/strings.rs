use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaElement};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StringDatatype(String);

impl StringDatatype {
    fn new_if_valid(value: &str) -> Result<Self, Error> {
        Ok(Self(value.trim().to_string()))
    }
}

impl Deref for StringDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for StringDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_if_valid(value)
    }
}

impl FromStr for StringDatatype {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_if_valid(s)
    }
}

impl SchemaElement for StringDatatype {
    fn schema_title() -> &'static str {
        ""
    }
    fn schema_description() -> &'static str {
        r#""#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "StringDatatype"
    }
}

/// A string representing arbitrary binary data encoded using the Base 64 algorithm as defined by RFC4648
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Base64Datatype(String);
impl Deref for Base64Datatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for Base64Datatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.to_string()))
    }
}

impl FromStr for Base64Datatype {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl SchemaElement for Base64Datatype {
    fn schema_title() -> &'static str {
        ""
    }
    fn schema_description() -> &'static str {
        r#"A string representing arbitrary binary data encoded using the Base 64 algorithm as defined by RFC4648"#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "base64"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmailAddress(String);

impl SchemaElement for EmailAddress {
    fn schema_title() -> &'static str {
        "Email Address"
    }
    fn schema_description() -> &'static str {
        "An email address string formatted according to RFC6531"
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "email-address"
    }
}

impl Deref for EmailAddress {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<&str> for EmailAddress {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(String::from(value)))
    }
}

impl FromStr for EmailAddress {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_data_type_from_string() {
        let result = StringDatatype::try_from("abc").expect("fail");
        assert_eq!(result, StringDatatype("abc".to_string()));
    }

    #[test]
    fn test_deref() {
        let show = |s: &str| s.to_string();

        let sdt = StringDatatype::try_from("abc").expect("fail");
        // StringDatatype can be passed as &str
        assert_eq!(show(&sdt), "abc");

        // StringDatatype can be deref'd to &str
        assert_eq!(sdt.deref(), "abc");
    }
}
