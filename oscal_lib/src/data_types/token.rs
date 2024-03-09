use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::str::FromStr;

use super::nc_name::NCName;
use crate::{Error, SchemaElement};

/// Wrapper for NCName
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TokenDatatype(NCName);

impl TokenDatatype {
    fn new_if_valid(value: &str) -> Result<Self, Error> {
        Ok(Self(NCName::try_from(value)?))
    }
}

impl FromStr for TokenDatatype {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(NCName::try_from(s)?))
    }
}

impl TryFrom<&str> for TokenDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_if_valid(value)
    }
}

impl SchemaElement for TokenDatatype {
    fn schema_title() -> &'static str {
        "Token"
    }
    fn schema_description() -> &'static str {
        r#"A non-colonized name as defined by XML Schema Part 2: Datatypes Second Edition."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "token"
    }
}

impl Deref for TokenDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize() {
        assert!(serde_json::from_str::<TokenDatatype>(r#""_abc""#).is_ok());
    }
}
