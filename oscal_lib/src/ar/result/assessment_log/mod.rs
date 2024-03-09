use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

use assessment_log_entry::AssessmentLogEntry;

pub mod assessment_log_entry;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentLog {
    pub entries: Vec<AssessmentLogEntry>,
}

impl SchemaElement for AssessmentLog {
    fn schema_title() -> &'static str {
        "Assessment Log"
    }
    fn schema_description() -> &'static str {
        "A log of all assessment-related actions taken."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:result:assessment-log"
    }
}
