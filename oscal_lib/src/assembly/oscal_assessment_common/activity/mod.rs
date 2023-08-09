pub use step::*;


pub mod step;

/// Activity
/// Identifies an assessment or related process that can be performed. In the assessment plan, this is an intended activity which may be associated with an assessment task. In the assessment results, this an activity that was actually performed as part of an assessment.
/// $id: #assembly_oscal-assessment-common_activity
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
pub struct Activity {
	pub links: Option<Vec<Link>>,
	pub related_controls: Option<ReviewedControls>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	pub props: Option<Vec<Property>>,
	/// Included Activity Description
	/// A human-readable description of this included activity.
	pub description: String,
	pub steps: Option<Vec<Step>>,
	/// Included Activity Title
	/// The title for this included activity.
	pub title: Option<String>,
	/// Assessment Activity Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference this assessment activity elsewhere in this or other OSCAL instances. The locally defined UUID of the activity can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
	pub remarks: Option<Remarks>,
}
