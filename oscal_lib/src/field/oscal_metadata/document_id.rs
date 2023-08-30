/// Document Identifier
/// A document identifier qualified by an identifier scheme. A document identifier provides a globally unique identifier with a cross-instance scope that is used for a group of documents that are to be treated as different versions of the same document. If this element does not appear, or if the value of this element is empty, the value of "document-id" is equal to the value of the "uuid" flag of the top-level root element.
/// $id: #field_oscal-metadata_document-id
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::definitions::UriDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct DocumentId {
	pub identifier: StringDatatype,
	/// Document Identification Scheme
	/// Qualifies the kind of document identifier using a URI. If the scheme is not provided the value of the element will be interpreted as a string of characters.
	pub scheme: Option<UriDatatype>,
}
