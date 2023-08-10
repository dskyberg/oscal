/// Referenced Control Objectives
/// Identifies the control objectives of the assessment. In the assessment plan, these are the planned objectives. In the assessment results, these are the assessed objectives, and reflects any changes from the plan.
/// $id: #assembly_oscal-assessment-common_reviewed-controls_referenced-control-objectives_referenced-control-objectives
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::SelectObjectiveById;
use crate::assembly::oscal_catalog_common::IncludeAll;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ReferencedControlObjectives {
	/// Control Objectives Description
	/// A human-readable description of this collection of control objectives.
	pub description: Option<String>,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
	pub include_all: Option<IncludeAll>,
	pub exclude_objectives: Option<Vec<SelectObjectiveById>>,
	pub include_objectives: Option<Vec<SelectObjectiveById>>,
}
