use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{DateTimeWithTimezoneDatatype, SchemaConstraint};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Published(DateTimeWithTimezoneDatatype);

impl Deref for Published {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
impl From<&str> for Published {
    fn from(value: &str) -> Self {
        Self(DateTimeWithTimezoneDatatype::from(value))
    }
}

impl SchemaConstraint for Published {
    fn constraint_title() -> &'static str {
        "Publication Timestamp"
    }
    fn constraint_description() -> &'static str {
        "The date and time the document was published. The date-time value must be formatted according to RFC 3339 with full time and time zone included."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_published"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:published"
    }
}
