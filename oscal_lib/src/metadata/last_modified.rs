use serde::{Deserialize, Serialize};

use crate::{DateTimeWithTimezoneDatatype, Error, SchemaElement};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LastModified(DateTimeWithTimezoneDatatype);

impl SchemaElement for LastModified {
    fn schema_title() -> &'static str {
        "Last Modified Timestamp"
    }
    fn schema_description() -> &'static str {
        "A string used to distinguish the current version of the document from other previous (and future) versions."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_version")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:last-modified"
    }
}

impl LastModified {
    pub fn new() -> Self {
        Self(DateTimeWithTimezoneDatatype::new())
    }
}
impl Default for LastModified {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for LastModified {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(DateTimeWithTimezoneDatatype::try_from(value)?))
    }
}

impl From<&DateTimeWithTimezoneDatatype> for LastModified {
    fn from(value: &DateTimeWithTimezoneDatatype) -> Self {
        Self(value.clone())
    }
}
