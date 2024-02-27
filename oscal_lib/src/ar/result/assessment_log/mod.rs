use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use assessment_log_entry::AssessmentLogEntry;

pub mod assessment_log_entry;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentLog {
    pub entries: Vec<AssessmentLogEntry>,
}

impl SchemaConstraint for AssessmentLog {
    fn constraint_title() -> &'static str {
        "Assessment Log"
    }
    fn constraint_description() -> &'static str {
        "A log of all assessment-related actions taken."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_result_assessment-log"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:result:assessment-log"
    }
}
