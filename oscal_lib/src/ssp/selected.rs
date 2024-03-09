/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/adjustment_justification.rs
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaElement, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Selected(StringDatatype);

impl Deref for Selected {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for Selected {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}

impl SchemaElement for Selected {
    fn schema_title() -> &'static str {
        "Selected Level (Confidentiality, Integrity, or Availability)"
    }
    fn schema_description() -> &'static str {
        "The selected (Confidentiality, Integrity, or Availability) security impact level."
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-ssp_selected")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:selected"
    }
}
