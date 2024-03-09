use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::SchemaElement;

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

impl SchemaElement for EventTiming {
    fn schema_title() -> &'static str {
        "Event Timing"
    }
    fn schema_description() -> &'static str {
        "The timing under which the task is intended to occur."
    }
    fn schema_id() -> Option<&'static str> {
        None
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-assessment-common:task:event-timing"
    }
}
