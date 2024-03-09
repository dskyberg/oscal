use serde::{Deserialize, Serialize};

use crate::{DateTimeWithTimezoneDatatype, Error, SchemaElement};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Published(DateTimeWithTimezoneDatatype);

impl Published {
    pub fn new() -> Self {
        Self(DateTimeWithTimezoneDatatype::new())
    }
}
impl Default for Published {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Published {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(DateTimeWithTimezoneDatatype::try_from(value)?))
    }
}

impl From<&DateTimeWithTimezoneDatatype> for Published {
    fn from(value: &DateTimeWithTimezoneDatatype) -> Self {
        Self(value.clone())
    }
}

impl SchemaElement for Published {
    fn schema_title() -> &'static str {
        "Publication Timestamp"
    }
    fn schema_description() -> &'static str {
        "The date and time the document was published. The date-time value must be formatted according to RFC 3339 with full time and time zone included."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_published")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:published"
    }
}
