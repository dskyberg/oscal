/// Assessment Subject Source
/// Assessment subjects will be identified while conducting the referenced activity-instance.
/// $id: #assembly_oscal-assessment-common_assessment-subject-placeholder_assessment-subject-source_assessment-subject-source
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::UuidDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct AssessmentSubjectSource {
	/// Task Universally Unique Identifier
	/// A machine-oriented, globally unique identifier with cross-instance scope that can be used to reference (in this or other OSCAL instances) an assessment activity to be performed as part of the event. The locally defined UUID of the task can be used to reference the data item locally or globally (e.g., in an imported OSCAL instance). This UUID should be assigned per-subject, which means it should be consistently used to identify the same subject across revisions of the document.
	pub task_uuid: UuidDatatype,
}
