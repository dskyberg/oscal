use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::SchemaConstraint;

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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Base64Datatype(String);
impl Deref for Base64Datatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Base64Datatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for Base64Datatype {
    fn pattern() -> &'static str {
        r"^[0-9A-Za-z+/]+={0,2}$"
    }
}

impl SchemaConstraint for Base64Datatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "Base64Datatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct BooleanDatatype(bool);

impl Deref for BooleanDatatype {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<bool> for BooleanDatatype {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl SchemaConstraint for BooleanDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "BooleanDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateDatatype(String);

impl Deref for DateDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for DateDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for DateDatatype {
    fn pattern() -> &'static str {
        r"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))(Z|[+-][0-9]{2}:[0-9]{2})?$"
    }
}

impl SchemaConstraint for DateDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "DateDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct DateTimeWithTimezoneDatatype(String);

impl Deref for DateTimeWithTimezoneDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for DateTimeWithTimezoneDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for DateTimeWithTimezoneDatatype {
    fn pattern() -> &'static str {
        r"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\.[0-9]*[1-9])?(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))$"
    }

    fn format() -> Option<&'static str> {
        Some("date-time")
    }
}

impl SchemaConstraint for DateTimeWithTimezoneDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "DateTimeWithTimezoneDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct IntegerDatatype(i64);

impl Deref for IntegerDatatype {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for IntegerDatatype {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl NumberType for IntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

impl SchemaConstraint for IntegerDatatype {
    fn constraint_title() -> &'static str {
        "IntegerDatatype"
    }
    fn constraint_description() -> &'static str {
        "IntegerDatatype"
    }
    fn constraint_id() -> &'static str {
        "IntegerDatatype"
    }
    fn schema_path() -> &'static str {
        "IntegerDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct NonNegativeIntegerDatatype(u64);

impl Deref for NonNegativeIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for NonNegativeIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl NumberType for NonNegativeIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

impl SchemaConstraint for NonNegativeIntegerDatatype {
    fn constraint_title() -> &'static str {
        "NonNegativeIntegerDatatype"
    }
    fn constraint_description() -> &'static str {
        "NonNegativeIntegerDatatype"
    }
    fn constraint_id() -> &'static str {
        "NonNegativeIntegerDatatype"
    }
    fn schema_path() -> &'static str {
        "NonNegativeIntegerDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct PositiveIntegerDatatype(u64);

impl Deref for PositiveIntegerDatatype {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for PositiveIntegerDatatype {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl NumberType for PositiveIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(1)
    }
}

impl SchemaConstraint for PositiveIntegerDatatype {
    fn constraint_title() -> &'static str {
        "PositiveIntegerDatatype"
    }
    fn constraint_description() -> &'static str {
        "PositiveIntegerDatatype"
    }
    fn constraint_id() -> &'static str {
        "PositiveIntegerDatatype"
    }
    fn schema_path() -> &'static str {
        "PositiveIntegerDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct StringDatatype(String);

impl Deref for StringDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for StringDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for StringDatatype {
    fn pattern() -> &'static str {
        r"^\S(.*\S)?$"
    }
}

impl SchemaConstraint for StringDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "StringDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TokenDatatype(String);

impl Deref for TokenDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for TokenDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for TokenDatatype {
    fn pattern() -> &'static str {
        r"^(\p{L}|_)(\p{L}|\p{N}|[.\-_])*$"
    }
}

impl SchemaConstraint for TokenDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "TokenDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct URIDatatype(String);

impl Deref for URIDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for URIDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for URIDatatype {
    fn pattern() -> &'static str {
        r"^[a-zA-Z][a-zA-Z0-9+\-.]+:.+$"
    }

    fn format() -> Option<&'static str> {
        Some("uri")
    }
}

impl SchemaConstraint for URIDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "URIDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct URIReferenceDatatype(String);

impl Deref for URIReferenceDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for URIReferenceDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for URIReferenceDatatype {
    fn format() -> Option<&'static str> {
        Some("uri-reference")
    }
}

impl SchemaConstraint for URIReferenceDatatype {
    fn constraint_title() -> &'static str {
        ""
    }
    fn constraint_description() -> &'static str {
        r#""#
    }
    fn constraint_id() -> &'static str {
        ""
    }
    fn schema_path() -> &'static str {
        "URIReferenceDatatype"
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UUIDDatatype(String);

impl Deref for UUIDDatatype {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for UUIDDatatype {
    fn from(value: &str) -> Self {
        Self(value.to_string())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_data_type_from_string() {
        let result = StringDatatype::from("abc");
        assert_eq!(result, StringDatatype("abc".to_string()));
    }

    #[test]
    fn test_deref() {
        let show = |s: &str| format!("{}", s);

        let sdt = StringDatatype::from("abc");
        // StringDatatype can be passed as &str
        assert_eq!(show(&sdt), "abc");

        // StringDatatype can be deref'd to &str
        assert_eq!(sdt.deref(), "abc");
    }
}
