use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{DateTimeWithTimezoneDatatype, SchemaElement};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct OnDateCondition {
    pub date: DateTimeWithTimezoneDatatype,
}

impl SchemaElement for OnDateCondition {
    fn schema_title() -> &'static str {
        "On Date Condition"
    }
    fn schema_description() -> &'static str {
        "The task is intended to occur on the specified date."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing:on-date-condition"
    }
}
