pub use relation::*;


pub mod relation;

/// Link
/// A reference to a local or remote resource
/// $id: #assembly_oscal-metadata_link
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::definitions::UriReferenceDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Link {
	/// Relation
	/// Describes the type of relationship provided by the link. This can be an indicator of the link's purpose.
	pub rel: Option<Relation>,
	/// Hypertext Reference
	/// A resolvable URL reference to a resource.
	pub href: UriReferenceDatatype,
	/// Media Type
	/// Specifies a media type as defined by the Internet Assigned Numbers Authority (IANA) Media Types Registry.
	pub media_type: Option<StringDatatype>,
	/// Link Text
	/// A textual label to associate with the link, which may be used for presentation in a tool.
	pub text: Option<String>,
}
