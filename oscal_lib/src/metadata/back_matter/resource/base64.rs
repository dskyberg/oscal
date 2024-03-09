use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Base64Datatype, SchemaElement, StringDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Base64 {
    filename: Option<URIReferenceDatatype>,
    media_type: Option<StringDatatype>,
    value: Base64Datatype,
}

impl SchemaElement for Base64 {
    fn schema_title() -> &'static str {
        "Base64"
    }

    fn schema_description() -> &'static str {
        "The Base64 alphabet in RFC 2045 - aligned with XSD."
    }

    fn schema_id() -> Option<&'static str> {
        None
    }

    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:back-matter/resources/base64"
    }
}
