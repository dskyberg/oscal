/// On Date Range Condition
/// The task is intended to occur within the specified date range.
/// $id: #assembly_oscal-assessment-common_task_event-timing_event-timing_on-date-range-condition_on-date-range-condition
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::DateTimeWithTimezoneDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct OnDateRangeCondition {
	/// End Date Condition
	/// The task must occur on or before the specified date.
	pub end: DateTimeWithTimezoneDatatype,
	/// Start Date Condition
	/// The task must occur on or after the specified date.
	pub start: DateTimeWithTimezoneDatatype,
}
