use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaElement, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Version(StringDatatype);

impl SchemaElement for Version {
    fn schema_title() -> &'static str {
        "Document Version"
    }
    fn schema_description() -> &'static str {
        "A string used to distinguish the current version of the document from other previous (and future) versions."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_version")
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

impl TryFrom<&str> for Version {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}
