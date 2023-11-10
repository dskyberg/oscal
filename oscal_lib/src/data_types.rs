use serde::{Deserialize, Serialize};

pub trait TransparentType<T> {
    fn inner(&self) -> &T;
}

pub trait StringType {
    fn pattern() -> Option<&'static str> {
        None
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
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct Base64Datatype {
    inner: String,
}
impl TransparentType<String> for Base64Datatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl From<&str> for Base64Datatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

impl StringType for Base64Datatype {
    fn pattern() -> Option<&'static str> {
        Some("^[0-9A-Za-z+/]+={0,2}$")
    }
    fn content_encoding() -> Option<&'static str> {
        Some("base64")
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct BooleanDatatype {
    inner: bool,
}
impl TransparentType<bool> for BooleanDatatype {
    fn inner(&self) -> &bool {
        &self.inner
    }
}

impl From<bool> for BooleanDatatype {
    fn from(value: bool) -> Self {
        Self { inner: value }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct DateDatatype {
    inner: String,
}
impl TransparentType<String> for DateDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}
impl StringType for DateDatatype {
    fn pattern() -> Option<&'static str> {
        Some("^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\\.[0-9]*[1-9])?(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))$")
    }
    fn format() -> Option<&'static str> {
        Some("date-time")
    }
}

impl From<&str> for DateDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct DateTimeWithTimezoneDatatype {
    inner: String,
}
impl TransparentType<String> for DateTimeWithTimezoneDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}
impl StringType for DateTimeWithTimezoneDatatype {
    fn pattern() -> Option<&'static str> {
        Some( "^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))T(2[0-3]|[01][0-9]):([0-5][0-9]):([0-5][0-9])(\\.[0-9]*[1-9])?(Z|(-((0[0-9]|1[0-2]):00|0[39]:30)|\\+((0[0-9]|1[0-4]):00|(0[34569]|10):30|(0[58]|12):45)))$")
    }
    fn format() -> Option<&'static str> {
        Some("date-time")
    }
}

impl From<&str> for DateTimeWithTimezoneDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct EmailAddressDatatype {
    inner: String,
}
impl TransparentType<String> for EmailAddressDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl StringType for EmailAddressDatatype {
    fn pattern() -> Option<&'static str> {
        Some("^.+@.+$")
    }
    fn format() -> Option<&'static str> {
        Some("email")
    }
}

impl From<&str> for EmailAddressDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct IntegerDatatype {
    inner: i64,
}
impl TransparentType<i64> for IntegerDatatype {
    fn inner(&self) -> &i64 {
        &self.inner
    }
}

impl NumberType for IntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(0)
    }
}

impl From<i64> for IntegerDatatype {
    fn from(value: i64) -> Self {
        Self { inner: value }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct PositiveIntegerDatatype {
    inner: i64,
}
impl TransparentType<i64> for PositiveIntegerDatatype {
    fn inner(&self) -> &i64 {
        &self.inner
    }
}

impl NumberType for PositiveIntegerDatatype {
    fn minimum() -> Option<i64> {
        Some(1)
    }
}

impl From<i64> for PositiveIntegerDatatype {
    fn from(value: i64) -> Self {
        Self { inner: value }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct StringDatatype {
    inner: String,
}
impl TransparentType<String> for StringDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl StringType for StringDatatype {
    fn pattern() -> Option<&'static str> {
        Some("^\\S(.*\\S)?$")
    }
}

impl From<&str> for StringDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct TokenDatatype {
    inner: String,
}
impl TransparentType<String> for TokenDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl StringType for TokenDatatype {
    fn pattern() -> Option<&'static str> {
        Some("^(\\p{L}|_)(\\p{L}|\\p{N}|[.\\-_])*$")
    }
}

impl From<&str> for TokenDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct URIDatatype {
    inner: String,
}
impl TransparentType<String> for URIDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl StringType for URIDatatype {
    fn format() -> Option<&'static str> {
        Some("uri")
    }
    fn pattern() -> Option<&'static str> {
        Some("^[a-zA-Z][a-zA-Z0-9+\\-.]+:.+$")
    }
}

impl From<&str> for URIDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct URIReferenceDatatype {
    inner: String,
}
impl TransparentType<String> for URIReferenceDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl StringType for URIReferenceDatatype {
    fn format() -> Option<&'static str> {
        Some("uri-reference")
    }
}

impl From<&str> for URIReferenceDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(transparent)]
pub struct UUIDDatatype {
    inner: String,
}
impl TransparentType<String> for UUIDDatatype {
    fn inner(&self) -> &String {
        &self.inner
    }
}

impl StringType for UUIDDatatype {
    fn pattern() -> Option<&'static str> {
        Some("^[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[45][0-9A-Fa-f]{3}-[89ABab][0-9A-Fa-f]{3}-[0-9A-Fa-f]{12}$")
    }
}

impl From<&str> for UUIDDatatype {
    fn from(value: &str) -> Self {
        Self {
            inner: value.to_string(),
        }
    }
}
