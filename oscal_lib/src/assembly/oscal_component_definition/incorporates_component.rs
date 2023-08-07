/// Incorporates Component
/// TBD
/// $id: #assembly_oscal-component-definition_incorporates-component
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct IncorporatesComponent {
	/// Component Description
	/// A description of the component, including information about its function.
	pub description: String,
	/// Component Reference
	/// A machine-oriented identifier reference to a component.
	pub component_uuid: UuidDatatype,
}
