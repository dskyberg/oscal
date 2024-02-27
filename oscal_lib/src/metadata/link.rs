/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/link.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaConstraint, StringDatatype, TokenDatatype, URIReferenceDatatype};

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

impl SchemaConstraint for Link {
    fn constraint_title() -> &'static str {
        "Link"
    }
    fn constraint_description() -> &'static str {
        r#"A reference to a local or remote resource"#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-metadata_link"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:link"
    }
}
