/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/hash.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Hash {
    /// enum: [
    ///    "SHA-224",
    ///    "SHA-256",
    ///    "SHA-384",
    ///    "SHA-512",
    ///    "SHA3-224",
    ///    "SHA3-256",
    ///    "SHA3-384",
    ///    "SHA3-512"
    ///]
    pub algorithm: StringDatatype,
    pub value: StringDatatype,
}

impl SchemaElement for Hash {
    fn schema_title() -> &'static str {
        "Hash"
    }
    fn schema_description() -> &'static str {
        r#"A representation of a cryptographic digest generated over a resource using a specified hash algorithm."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_hash")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:hash"
    }
}
