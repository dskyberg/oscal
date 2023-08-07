/// Subject Universally Unique Identifier Reference Type
/// Used to indicate the type of object pointed to by the uuid-ref within a subject.
/// $id: #assembly_oscal-assessment-common_subject-reference_subject-universally-unique-identifier-reference-type_subject-universally-unique-identifier-reference-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum SubjectUniversallyUniqueIdentifierReferenceType {
	// orig: component
	Component,
	// orig: inventory-item
	InventoryItem,
	// orig: location
	Location,
	// orig: party
	Party,
	// orig: user
	User,
	// orig: resource
	Resource,
}
