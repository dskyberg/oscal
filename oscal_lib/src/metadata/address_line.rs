use std::ops::Deref;

/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/document_id.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Error, SchemaConstraint, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AddressLine(StringDatatype);

impl Deref for AddressLine {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for AddressLine {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}

impl SchemaConstraint for AddressLine {
    fn constraint_title() -> &'static str {
        "Document Identifier"
    }
    fn constraint_description() -> &'static str {
        "A single line of an address."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_addr-line"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:addr-line"
    }
}
