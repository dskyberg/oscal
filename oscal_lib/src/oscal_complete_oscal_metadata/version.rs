use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{SchemaConstraint, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Version(StringDatatype);

impl SchemaConstraint for Version {
    fn constraint_title() -> &'static str {
        "Document Version"
    }
    fn constraint_description() -> &'static str {
        "A string used to distinguish the current version of the document from other previous (and future) versions."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_version"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:version"
    }
}

impl Deref for Version {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        Self(StringDatatype::from(value))
    }
}
