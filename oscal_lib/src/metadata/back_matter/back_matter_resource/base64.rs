use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Base64Datatype, SchemaConstraint, StringDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Base64 {
    filename: Option<URIReferenceDatatype>,
    media_type: Option<StringDatatype>,
    value: Base64Datatype,
}

impl SchemaConstraint for Base64 {
    fn constraint_title() -> &'static str {
        "Base64"
    }

    fn constraint_description() -> &'static str {
        "The Base64 alphabet in RFC 2045 - aligned with XSD."
    }

    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_back-matter:resource:bse64"
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/base64"
    }
}
