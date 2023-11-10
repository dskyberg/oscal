use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TelephoneNumber {
    #[serde(rename = "type")]
    pub type_: Option<StringDatatype>,
    pub number: StringDatatype,
}

impl SchemaConstraint for TelephoneNumber {
    fn constraint_title() -> &'static str {
        "Telephone Number"
    }
    fn constraint_description() -> &'static str {
        "Contact number by telephone."
    }
    fn constraint_id() -> &'static str {
        "#field_oscal-metadata_telephone-number"
    }
    fn schema_path() -> &'static str {
        "telephone-number"
    }
}
