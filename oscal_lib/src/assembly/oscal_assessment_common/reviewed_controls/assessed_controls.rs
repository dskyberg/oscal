/// Assessed Controls
/// Identifies the controls being assessed. In the assessment plan, these are the planned controls. In the assessment results, these are the actual controls, and reflects any changes from the plan.
/// $id: #assembly_oscal-assessment-common_reviewed-controls_assessed-controls_assessed-controls
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::SelectControlById;
use crate::assembly::oscal_catalog_common::IncludeAll;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessedControls {
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
	/// Assessed Controls Description
	/// A human-readable description of in-scope controls specified for assessment.
	pub description: Option<String>,
	pub include_all: Option<IncludeAll>,
	pub include_controls: Option<Vec<SelectControlById>>,
	pub exclude_controls: Option<Vec<SelectControlById>>,
}
