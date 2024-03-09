/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/select_control_by_id.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaElement, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Pattern(StringDatatype);

impl Deref for Pattern {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl TryFrom<&str> for Pattern {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(StringDatatype::try_from(value)?))
    }
}

impl SchemaElement for Pattern {
    fn schema_title() -> &'static str {
        "Match Controls by Pattern"
    }
    fn schema_description() -> &'static str {
        "Select controls by (regular expression) match on ID"
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:select-control-by-id_pattern"
    }
}
