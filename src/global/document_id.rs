use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

///  A document identifier qualified by an identifier scheme.
/// A document identifier provides a globally unique identifier
/// with a cross-instance scope that is used for a group of d
/// ocuments that are to be treated as different versions of the
/// same document. If this element does not appear, or if the
/// value of this element is empty, the value of "document-id"
/// is equal to the value of the "uuid" flag of the top-level root element.
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentId {
    /// Qualifies the kind of document identifier using a URI. If the scheme is not provided the value of the element will be interpreted as a string of characters.
    pub scheme: Option<String>,
    pub identifier: Option<String>,
}

pub type DocumentIds = Vec<DocumentId>;
