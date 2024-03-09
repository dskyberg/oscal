/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/telephone_number.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TelephoneNumber {
    ///  enum: [
    ///     "home",
    ///     "office",
    ///     "mobile"
    /// ]
    #[serde(rename = "type")]
    pub _type: Option<StringDatatype>,
    pub number: StringDatatype,
}

impl SchemaElement for TelephoneNumber {
    fn schema_title() -> &'static str {
        "Telephone Number"
    }
    fn schema_description() -> &'static str {
        r#"Contact number by telephone."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_telephone-number")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:telephone-number"
    }
}
