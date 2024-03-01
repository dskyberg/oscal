/// File name: ../oscal_lib/src/oscal_complete_oscal_ssp/adjustment_justification.rs
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaConstraint, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Base(StringDatatype);

impl Deref for Base {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for Base {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}

impl SchemaConstraint for Base {
    fn constraint_title() -> &'static str {
        "Base Level (Confidentiality, Integrity, or Availability)"
    }
    fn constraint_description() -> &'static str {
        "The prescribed base (Confidentiality, Integrity, or Availability) security impact level."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-ssp_base"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ssp:base"
    }
}
