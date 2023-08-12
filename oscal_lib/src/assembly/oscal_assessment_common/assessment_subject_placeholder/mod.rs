pub use assessment_subject_source::*;


pub mod assessment_subject_source;

/// Assessment Subject Placeholder
/// Used when the assessment subjects will be determined as part of one or more other assessment activities. These assessment subjects will be recorded in the assessment results in the assessment log.
/// $id: #assembly_oscal-assessment-common_assessment-subject-placeholder
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::assembly::oscal_metadata::Link;
use crate::assembly::oscal_metadata::Property;
use crate::definitions::UuidDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentSubjectPlaceholder {
	pub links: Option<Vec<Link>>,
	/// Assessment Subject Placeholder Description
	/// A human-readable description of intent of this assessment subject placeholder.
	pub description: Option<String>,
	pub sources: Vec<AssessmentSubjectSource>,
	pub props: Option<Vec<Property>>,
	pub remarks: Option<Remarks>,
	/// Assessment Subject Placeholder Universally Unique Identifier
	/// A machine-oriented, globally unique identifier for a set of assessment subjects that will be identified by a task or an activity that is part of a task. The locally defined UUID of the assessment subject placeholder can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub uuid: UuidDatatype,
}
