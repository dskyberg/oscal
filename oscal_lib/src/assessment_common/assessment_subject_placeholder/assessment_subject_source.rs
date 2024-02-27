use serde::{Deserialize, Serialize};

use crate::{SchemaConstraint, UUIDDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubjectSource {
    pub task_uuid: UUIDDatatype,
}

impl SchemaConstraint for AssessmentSubjectSource {
    fn constraint_title() -> &'static str {
        "Assessment Subject Source"
    }
    fn constraint_description() -> &'static str {
        "Assessment subjects will be identified while conducting the referenced activity-instance."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_assessment-subject-placeholder:sources"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-subject-placeholder:sources"
    }
}
