pub use subject_universally_unique_identifier_reference_type::*;


pub mod subject_universally_unique_identifier_reference_type;

/// Identifies the Subject
/// A human-oriented identifier reference to a resource. Use type to indicate whether the identified resource is a component, inventory item, location, user, or something else.
/// $id: #assembly_oscal-assessment-common_subject-reference
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct SubjectReference {
	/// Subject Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a component, inventory-item, location, party, user, or resource using it's UUID.
	pub subject_uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
	/// Subject Universally Unique Identifier Reference Type
	/// Used to indicate the type of object pointed to by the uuid-ref within a subject.
	#[serde(rename = "type")]
	pub _type: SubjectUniversallyUniqueIdentifierReferenceType,
	/// Subject Reference Title
	/// The title or name for the referenced subject.
	pub title: Option<String>,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
}
