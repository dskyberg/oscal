use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{DateTimeWithTimezoneDatatype, SchemaConstraint};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct LastModified(DateTimeWithTimezoneDatatype);

impl SchemaConstraint for LastModified {
    fn constraint_title() -> &'static str {
        "Last Modified Timestamp"
    }
    fn constraint_description() -> &'static str {
        "A string used to distinguish the current version of the document from other previous (and future) versions."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_version"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:last-modified"
    }
}

impl Deref for LastModified {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<&str> for LastModified {
    fn from(value: &str) -> Self {
        Self(DateTimeWithTimezoneDatatype::from(value))
    }
}
