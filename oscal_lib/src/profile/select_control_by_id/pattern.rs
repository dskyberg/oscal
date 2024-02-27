/// File name: ../oscal_lib/src/oscal_complete_oscal_profile/select_control_by_id.rs
/// pub use oscal_complete_oscal_profile::*;
///
/// pub mod oscal_complete_oscal_profile;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{Error, SchemaConstraint, StringDatatype};

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

impl SchemaConstraint for Pattern {
    fn constraint_title() -> &'static str {
        "Match Controls by Pattern"
    }
    fn constraint_description() -> &'static str {
        "Select controls by (regular expression) match on ID"
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-profile_select-control-by-id_pattern"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-profile:select-control-by-id_pattern"
    }
}
