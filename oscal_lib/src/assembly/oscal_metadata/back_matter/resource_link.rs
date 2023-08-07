/// Resource link
/// A pointer to an external resource with an optional hash for verification and change detection.
/// $id: #assembly_oscal-metadata_back-matter_resource_resource-link_resource-link
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::StringDatatype;
use crate::definitions::UriReferenceDatatype;
use crate::field::oscal_metadata::Hash;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ResourceLink {
	/// Hypertext Reference
	/// A resolvable URI reference to a resource.
	pub href: UriReferenceDatatype,
	/// Media Type
	/// Specifies a media type as defined by the Internet Assigned Numbers Authority (IANA) Media Types Registry.
	pub media_type: Option<StringDatatype>,
	pub hashes: Option<Vec<Hash>>,
}
