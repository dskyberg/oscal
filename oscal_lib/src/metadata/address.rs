/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/address.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype};

use super::address_line::AddressLine;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Address {
    /// "enum": [
    ///    "home",
    ///    "work"
    /// ]
    #[serde(rename = "type")]
    pub _type: Option<TokenDatatype>,
    pub addr_lines: Option<Vec<AddressLine>>,
    pub city: Option<StringDatatype>,
    pub state: Option<StringDatatype>,
    pub postal_code: Option<StringDatatype>,
}

impl SchemaConstraint for Address {
    fn constraint_title() -> &'static str {
        "Address"
    }
    fn constraint_description() -> &'static str {
        r#"A postal address for the location."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_address"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:address"
    }
}
