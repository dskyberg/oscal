use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::{SchemaConstraint, StringDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(transparent)]
pub struct EmailAddress(StringDatatype);
impl SchemaConstraint for EmailAddress {
    fn constraint_title() -> &'static str {
        "Email Address"
    }
    fn constraint_description() -> &'static str {
        "The OSCAL model version the document was authored against."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_oscal-version"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:email-address"
    }
}

impl Deref for EmailAddress {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl From<&str> for EmailAddress {
    fn from(value: &str) -> Self {
        Self(StringDatatype::from(value))
    }
}
