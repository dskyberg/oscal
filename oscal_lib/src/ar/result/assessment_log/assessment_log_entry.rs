use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{logged_by::LoggedBy, related_task::RelatedTask},
    metadata::{Link, Property, Remarks},
    DateTimeWithTimezoneDatatype, SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentLogEntry {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    pub start: DateTimeWithTimezoneDatatype,
    pub end: Option<DateTimeWithTimezoneDatatype>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub logged_by: Option<LoggedBy>,
    pub related_tasks: Option<Vec<RelatedTask>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for AssessmentLogEntry {
    fn constraint_title() -> &'static str {
        "Assessment Log Entry"
    }
    fn constraint_description() -> &'static str {
        "Identifies the result of an action and/or task that occurred as part of executing an assessment plan or an assessment event that occurred in producing the assessment results."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-ar_result_assessment-log-entry"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-ar:result:assessment-log-entry"
    }
}
