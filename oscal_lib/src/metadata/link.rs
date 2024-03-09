/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/link.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, TokenDatatype, URIReferenceDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Link {
    pub href: URIReferenceDatatype,
    /// enum: ["reference"]
    pub rel: Option<TokenDatatype>,
    pub media_type: Option<StringDatatype>,
    pub text: Option<String>,
}

impl SchemaElement for Link {
    fn schema_title() -> &'static str {
        "Link"
    }
    fn schema_description() -> &'static str {
        r#"A reference to a local or remote resource"#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-metadata_link")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:link"
    }
}
