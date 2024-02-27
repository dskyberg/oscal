use std::ops::Deref;

/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/remarks.rs
use serde::{Deserialize, Serialize};

use crate::{SchemaConstraint, StringType};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Remarks(String);

impl Deref for Remarks {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Remarks {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl StringType for Remarks {}

impl SchemaConstraint for Remarks {
    fn constraint_title() -> &'static str {
        "Remarks"
    }
    fn constraint_description() -> &'static str {
        r#"Additional commentary on the containing object."#
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_remarks"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:remarks"
    }
}
