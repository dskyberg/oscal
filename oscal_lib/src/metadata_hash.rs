use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataHash {
    pub algorhthm: StringDatatype,
    pub value: StringDatatype,
}

/*
"constraint": {
    "bindings" [
        {"pattern": "hash/altorithm"}
    ],
    "allowed-values": [
        "SHA-224",
        "SHA-256",
        "SHA-384",
        "SHA-512",
        "SHA3-224",
        "SHA3-256",
        "SHA3-384",
        "SHA3-512"
    ]
}
*/
impl SchemaConstraint for MetadataHash {
    fn constraint_title() -> &'static str {
        "Hash"
    }
    fn constraint_description() -> &'static str {
        "A prose statement that provides a recommendation for the use of a parameter."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-guideline"
    }
    fn schema_path() -> &'static str {
        "hash"
    }
}
