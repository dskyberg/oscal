/// Part Name
/// A textual label that uniquely identifies the part's semantic type.
/// $id: #assembly_oscal-assessment-common_assessment-part_part-name_part-name
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum PartName {
	// orig: asset
	Asset,
	// orig: method
	Method,
	// orig: objective
	Objective,
}
