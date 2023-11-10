use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataLink {
    pub href: URIReferenceDatatype,
    pub rel: Option<TokenDatatype>,
    pub media_type: Option<StringDatatype>,
    pub text: Option<String>,
}

/*
"constraint": {
    "bindings": [
        {"pattern": "link/rel"}
    ],
    "allowed_values": [
        "reference"
    ]
}
*/
impl SchemaConstraint for MetadataLink {
    fn constraint_title() -> &'static str {
        "Link"
    }
    fn constraint_description() -> &'static str {
        "A reference to a local or remote resource"
    }
    fn constraint_id() -> &'static str {
        "#/definitions/URIReferenceDatatype"
    }
    fn schema_path() -> &'static str {
        "link"
    }
}
