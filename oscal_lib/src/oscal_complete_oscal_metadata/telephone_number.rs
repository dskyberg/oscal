/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/telephone_number.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TelephoneNumber {
    ///  enum: [                           "enum": [
    /// "home",
    /// "office",
    /// "mobile"
    /// ]
    #[serde(rename = "type")]
    pub _type: Option<StringDatatype>,
    pub number: StringDatatype,
}

impl SchemaConstraint for TelephoneNumber {
    fn constraint_title() -> &'static str {
        "Telephone Number"
    }
    fn constraint_description() -> &'static str {
        r#"Contact number by telephone."#
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_telephone-number"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:telephone-number"
    }
}
