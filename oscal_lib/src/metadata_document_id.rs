use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, URIDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetadataDocumentId {
    pub scheme: Option<URIDatatype>,
    pub identifier: StringDatatype,
}

/*
"constraint": {
    "bindings": [
       {"pattern": "document-id/scheme"}
    ],
    "allowed-values": [
        "http://www.doi.org/",
        "https://www.doi.org/"
    ]
}
 */
impl SchemaConstraint for MetadataDocumentId {
    fn constraint_title() -> &'static str {
        "Guideline"
    }
    fn constraint_description() -> &'static str {
        "A prose statement that provides a recommendation for the use of a parameter."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-catalog-common_parameter-guideline"
    }
    fn schema_path() -> &'static str {
        "document-id"
    }
}
