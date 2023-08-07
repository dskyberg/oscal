/// Identified Subject
/// Used to detail assessment subjects that were identfied by this task.
/// $id: #assembly_oscal-assessment-common_related-task_identified-subject_identified-subject
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_assessment_common::AssessmentSubject;
use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct IdentifiedSubject {
	pub subjects: Vec<AssessmentSubject>,
	/// Assessment Subject Placeholder Universally Unique Identifier Reference
	/// A machine-oriented identifier reference to a unique assessment subject placeholder defined by this task.
	pub subject_placeholder_uuid: UuidDatatype,
}
