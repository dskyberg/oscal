use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{SchemaConstraint, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct OscalVersion(StringDatatype);
impl SchemaConstraint for OscalVersion {
    fn constraint_title() -> &'static str {
        "OSCAL version"
    }
    fn constraint_description() -> &'static str {
        "The OSCAL model version the document was authored against."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_version"
    }
    fn schema_path() -> &'static str {
        "#field_oscal-metadata_oscal-version"
    }
}

impl Deref for OscalVersion {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<&str> for OscalVersion {
    fn from(value: &str) -> Self {
        Self(StringDatatype::from(value))
    }
}
