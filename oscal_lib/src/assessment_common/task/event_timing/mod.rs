use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaConstraint;

use self::{
    frequency_condition::FrequencyCondition, on_date_condition::OnDateCondition,
    on_date_range_condition::OnDateRangeCondition,
};

pub mod frequency_condition;
pub mod on_date_condition;
pub mod on_date_range_condition;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventTiming {
    pub on_date: Option<OnDateCondition>,
    pub within_date_range: Option<OnDateRangeCondition>,
    pub at_frequency: Option<FrequencyCondition>,
}

impl SchemaConstraint for EventTiming {
    fn constraint_title() -> &'static str {
        "Event Timing"
    }
    fn constraint_description() -> &'static str {
        "The timing under which the task is intended to occur."
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-assessment-common_task:event-timing"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing"
    }
}
