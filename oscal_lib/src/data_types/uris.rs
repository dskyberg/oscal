use fluent_uri::Uri;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::{Error, SchemaElement, Validate};

/// Repesents an absolute URI, with schema.  For relative paths,
/// use [URIReferenceDatatype].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct URIDatatype(Uri<String>);

impl URIDatatype {
    fn new_if_valid(value: &str) -> Result<Self, Error> {
        let uri = value.parse::<Uri<String>>()?;
        match uri.is_absolute() {
            true => Ok(Self(uri)),
            false => Err(Error::UriAbsolute),
        }
    }
}

impl TryFrom<&str> for URIDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_if_valid(value)
    }
}

impl FromStr for URIDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_if_valid(s)
    }
}

impl SchemaElement for URIDatatype {
    fn schema_title() -> &'static str {
        "URI"
    }
    fn schema_description() -> &'static str {
        r#"A universal resource identifier (URI) formatted according to RFC3986."#
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "uri"
    }
}

impl Validate for URIDatatype {
    fn validate(value: &str) -> Result<(), Error> {
        let _ = Self::new_if_valid(value)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct URIReferenceDatatype(Uri<String>);

impl URIReferenceDatatype {
    fn new_if_valid(value: &str) -> Result<Self, Error> {
        Ok(Self(value.parse::<Uri<String>>()?))
    }
}

impl TryFrom<&str> for URIReferenceDatatype {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new_if_valid(value)
    }
}

impl FromStr for URIReferenceDatatype {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_if_valid(s)
    }
}

impl SchemaElement for URIReferenceDatatype {
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
        "URIReferenceDatatype"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uri() {
        let good_uri = "https://fedramp.gov/ns/oscal";
        assert!(good_uri.parse::<URIDatatype>().is_ok());

        let bad_uri = "#a78f7e4c-a27a-4b1e-901b-ebfecf2b0301";
        assert!(bad_uri.parse::<URIDatatype>().is_err());
    }

    #[test]
    fn test_valid_uri_reference() {
        let good_uri = "https://fedramp.gov/ns/oscal";
        assert!(good_uri.parse::<URIReferenceDatatype>().is_ok());

        let bad_uri = "#a78f7e4c-a27a-4b1e-901b-ebfecf2b0301";
        assert!(bad_uri.parse::<URIReferenceDatatype>().is_ok());
    }

    #[test]
    fn test_serde() {
        let json = format!("\"{}\"", uuid::Uuid::new_v4());
        let uri = serde_json::from_str::<URIDatatype>(&json).expect("fail");
        let result = serde_json::to_string(&uri).expect("fail");
        assert_eq!(&json, &result);
    }
}
