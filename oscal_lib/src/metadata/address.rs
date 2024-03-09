/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/address.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, TokenDatatype};

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

impl SchemaElement for Address {
    fn schema_title() -> &'static str {
        "Address"
    }
    fn schema_description() -> &'static str {
        r#"A postal address for the location."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_address")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:address"
    }
}
