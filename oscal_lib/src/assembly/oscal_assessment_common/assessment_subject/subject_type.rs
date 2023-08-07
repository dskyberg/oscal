/// Subject Type
/// Indicates the type of assessment subject, such as a component, inventory, item, location, or party represented by this selection statement.
/// $id: #assembly_oscal-assessment-common_assessment-subject_subject-type_subject-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum SubjectType {
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
}
