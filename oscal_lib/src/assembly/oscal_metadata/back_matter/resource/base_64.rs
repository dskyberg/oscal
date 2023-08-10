/// Base64
/// The Base64 alphabet in RFC 2045 - aligned with XSD.
/// $id: #assembly_oscal-metadata_back-matter_resource_resource_base-64_base-64
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::Base64Datatype;
use crate::definitions::StringDatatype;
use crate::definitions::UriReferenceDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Base64 {
	pub value: Base64Datatype,
	/// Media Type
	/// Specifies a media type as defined by the Internet Assigned Numbers Authority (IANA) Media Types Registry.
	pub media_type: Option<StringDatatype>,
	/// File Name
	/// Name of the file before it was encoded as Base64 to be embedded in a resource. This is the name that will be assigned to the file when the file is decoded.
	pub filename: Option<UriReferenceDatatype>,
}
