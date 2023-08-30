/// Control Implementation
/// Describes how the system satisfies a set of controls.
/// $id: #assembly_oscal-ssp_control-implementation
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_implementation_common::SetParameter;
use crate::assembly::oscal_ssp::ImplementedRequirement;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ControlImplementation {
	pub set_parameters: Option<Vec<SetParameter>>,
	pub implemented_requirements: Vec<ImplementedRequirement>,
	/// Control Implementation Description
	/// A statement describing important things to know about how this set of control satisfaction documentation is approached.
	pub description: String,
}
