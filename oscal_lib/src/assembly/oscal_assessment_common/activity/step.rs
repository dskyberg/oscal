/// Step
/// Identifies an individual step in a series of steps related to an activity, such as an assessment test or examination procedure.
/// $id: #assembly_oscal-assessment-common_activity_step_step
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::ReviewedControls;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Step {
	pub reviewed_controls: Option<ReviewedControls>,
	/// Step Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this step elsewhere in this or other OSCAL instances. The locally defined UUID of the step (in a series of steps) can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub links: Option<Vec<Link>>,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
	/// Step Description
	/// A human-readable description of this step.
	pub description: String,
	/// Step Title
	/// The title for this step.
	pub title: Option<String>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
}
