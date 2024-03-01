use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DateTimeWithTimezoneDatatype, SchemaConstraint};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnDateCondition {
    pub date: DateTimeWithTimezoneDatatype,
}

impl SchemaConstraint for OnDateCondition {
    fn constraint_title() -> &'static str {
        "On Date Condition"
    }
    fn constraint_description() -> &'static str {
        "The task is intended to occur on the specified date."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing:on-date-condition"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing:on-date-condition"
    }
}
