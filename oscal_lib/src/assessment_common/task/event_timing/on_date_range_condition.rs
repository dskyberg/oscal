use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DateTimeWithTimezoneDatatype, SchemaConstraint};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnDateRangeCondition {
    pub start: DateTimeWithTimezoneDatatype,
    pub end: DateTimeWithTimezoneDatatype,
}

impl SchemaConstraint for OnDateRangeCondition {
    fn constraint_title() -> &'static str {
        "On Date Range Condition"
    }
    fn constraint_description() -> &'static str {
        "The task is intended to occur within the specified date range."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing:on-date-condition-range"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing:on-date-condition-range"
    }
}
