/// File name: ../oscal_lib/src/oscal_complete_oscal_metadata/document_id.rs
/// pub use oscal_complete_oscal_metadata::*;
///
/// pub mod oscal_complete_oscal_metadata;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{SchemaElement, StringDatatype, URIDatatype};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DocumentId {
    pub identifier: StringDatatype,
    /// enum: [
    ///     "http://www.doi.org/"
    /// ]
    pub scheme: Option<URIDatatype>,
}

impl SchemaElement for DocumentId {
    fn schema_title() -> &'static str {
        "Document Identifier"
    }
    fn schema_description() -> &'static str {
        r#"A document identifier qualified by an identifier scheme. A document identifier provides a globally unique identifier with a cross-instance scope that is used for a group of documents that are to be treated as different versions of the same document. If this element does not appear, or if the value of this element is empty, the value of "document-id" is equal to the value of the "uuid" flag of the top-level root element."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#field_oscal-metadata_document-id")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-metadata:document-id"
    }
}
