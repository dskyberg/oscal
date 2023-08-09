/// Associated Activity
/// Identifies an individual activity to be performed as part of a task.
/// $id: #assembly_oscal-assessment-common_task_associated-activity_associated-activity
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentSubject;
use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::assembly::oscal_metadata::ResponsibleRole;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssociatedActivity {
	pub links: Option<Vec<Link>>,
	pub remarks: Option<Remarks>,
	/// Activity Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to an activity defined in the list of activities.
	pub activity_uuid: UuidDatatype,
	pub props: Option<Vec<Property>>,
	pub responsible_roles: Option<Vec<ResponsibleRole>>,
	pub subjects: Vec<AssessmentSubject>,
}
