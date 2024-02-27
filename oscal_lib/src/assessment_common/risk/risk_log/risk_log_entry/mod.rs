use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{logged_by::LoggedBy, risk_status::RiskStatus},
    metadata::{Link, Property, Remarks},
    DateTimeWithTimezoneDatatype, SchemaConstraint, UUIDDatatype,
};

use self::risk_response_reference::RiskResponseReference;

pub mod risk_response_reference;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiskLogEntry {
    pub uuid: UUIDDatatype,
    pub title: Option<String>,
    pub description: Option<String>,
    pub start: DateTimeWithTimezoneDatatype,
    pub end: Option<DateTimeWithTimezoneDatatype>,
    pub props: Option<Vec<Property>>,
    pub links: Option<Vec<Link>>,
    pub logged_by: Option<LoggedBy>,
    pub status_change: Option<RiskStatus>,
    pub related_responses: Option<Vec<RiskResponseReference>>,
    pub remarks: Option<Remarks>,
}

impl SchemaConstraint for RiskLogEntry {
    fn constraint_title() -> &'static str {
        "Risk Log Entry"
    }
    fn constraint_description() -> &'static str {
        "Identifies an individual risk response that occurred as part of managing an identified risk."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_risk:risk-log:risk-log-entry"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:risk:risk-log:risk-log-entry"
    }
}
