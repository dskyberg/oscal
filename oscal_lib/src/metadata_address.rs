/// 1246
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataAddress {
    #[serde(rename = "type")]
    pub type_: Option<TokenDatatype>,
    pub addr_lines: Option<Vec<StringDatatype>>,
    pub city: Option<StringDatatype>,
    pub state: Option<StringDatatype>,
    pub postal_code: Option<StringDatatype>,
    pub country: Option<StringDatatype>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "address/type"}
    ],
    "allowed-values": [
        "home",
        "work"
    ]
} */
impl SchemaConstraint for MetadataAddress {
    fn constraint_title() -> &'static str {
        "Address"
    }
    fn constraint_description() -> &'static str {
        "A postal address for the location."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_address"
    }
    fn schema_path() -> &'static str {
        "address"
    }
}
