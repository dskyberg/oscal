use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::related_task::RelatedTask,
    metadata::{Link, Property, Remarks},
    SchemaElement, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskResponseReference {
    response_uuid: UUIDDatatype,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub related_tasks: Option<Vec<RelatedTask>>,
    pub remarks: Option<Remarks>,
}

impl SchemaElement for RiskResponseReference {
    fn schema_title() -> &'static str {
        "Risk Response Reference"
    }
    fn schema_description() -> &'static str {
        "Identifies an individual risk response that this log entry is for."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log:risk-log-entry:related-responses"
    }
}
