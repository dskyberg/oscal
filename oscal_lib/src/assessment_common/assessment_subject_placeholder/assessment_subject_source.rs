use serde::{Deserialize, Serialize};

use crate::{SchemaElement, UUIDDatatype};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentSubjectSource {
    pub task_uuid: UUIDDatatype,
}

impl SchemaElement for AssessmentSubjectSource {
    fn schema_title() -> &'static str {
        "Assessment Subject Source"
    }
    fn schema_description() -> &'static str {
        "Assessment subjects will be identified while conducting the referenced activity-instance."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:assessment-subject-placeholder:sources"
    }
}
