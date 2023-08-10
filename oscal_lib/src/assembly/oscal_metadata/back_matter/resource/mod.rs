pub use base_64::*;
pub use citation::*;
pub use resource_link::*;


pub mod base_64;
pub mod citation;
pub mod resource_link;

/// Resource
/// A resource associated with content in the containing document. A resource may be directly included in the document base64 encoded or may point to one or more equivalent internet resources.
/// $id: #assembly_oscal-metadata_back-matter_resource_resource
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::DocumentId;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Resource {
	pub remarks: Option<Remarks>,
	/// Resource Title
	/// A name given to the resource, which may be used by a tool for display and navigation.
	pub title: Option<String>,
	pub rlinks: Option<Vec<ResourceLink>>,
	/// Resource Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this defined resource elsewhere in this or other OSCAL instances. This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	/// Resource Description
	/// A short summary of the resource used to indicate the purpose of the resource.
	pub description: Option<String>,
	/// Citation
	/// A citation consisting of end note text and optional structured bibliographic data.
	pub citation: Option<Citation>,
	/// Base64
	/// The Base64 alphabet in RFC 2045 - aligned with XSD.
	pub base_64: Option<Base64>,
	pub props: Option<Vec<Property>>,
	pub document_ids: Option<Vec<DocumentId>>,
}
