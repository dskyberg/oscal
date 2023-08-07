pub use referenced_control_objectives::*;
pub use assessed_controls::*;


pub mod referenced_control_objectives;
pub mod assessed_controls;

/// Reviewed Controls and Control Objectives
/// Identifies the controls being assessed and their control objectives.
/// $id: #assembly_oscal-assessment-common_reviewed-controls
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ReviewedControls {
	pub remarks: Option<Remarks>,
	/// Control Objective Description
	/// A human-readable description of control objectives.
	pub description: Option<String>,
	pub control_selections: Vec<AssessedControls>,
	pub control_objective_selections: Option<Vec<ReferencedControlObjectives>>,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
}
