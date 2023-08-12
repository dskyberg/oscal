/// Property Name
/// A textual label that uniquely identifies a specific attribute, characteristic, or quality of the property's containing object.
/// $id: #assembly_oscal-metadata_property_property-name_property-name
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum PropertyName {
	// orig: marking
	Marking,
	// orig: CORE
	#[serde(rename = "CORE")]
	Core,
	// orig: label
	Label,
	// orig: sort-id
	SortId,
	// orig: response-point
	ResponsePoint,
	// orig: method
	Method,
	// orig: type
	Type,
}
