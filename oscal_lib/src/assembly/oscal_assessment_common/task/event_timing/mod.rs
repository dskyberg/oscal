pub use frequency_condition::*;
pub use on_date_condition::*;
pub use on_date_range_condition::*;

pub mod frequency_condition;
pub mod on_date_condition;
pub mod on_date_range_condition;
pub mod time_unit;

/// Event Timing
/// The timing under which the task is intended to occur.
/// $id: #assembly_oscal-assessment-common_task_event-timing_event-timing
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct EventTiming {
    /// On Date Range Condition
    /// The task is intended to occur within the specified date range.
    pub within_date_range: Option<OnDateRangeCondition>,
    /// On Date Condition
    /// The task is intended to occur on the specified date.
    pub on_date: Option<OnDateCondition>,
    /// Frequency Condition
    /// The task is intended to occur at the specified frequency.
    pub at_frequency: Option<FrequencyCondition>,
}
