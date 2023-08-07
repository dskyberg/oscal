/// On Date Condition
/// The task is intended to occur on the specified date.
/// $id: #assembly_oscal-assessment-common_task_event-timing_event-timing_on-date-condition_on-date-condition
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::DateTimeWithTimezoneDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct OnDateCondition {
	/// On Date Condition
	/// The task must occur on the specified date.
	pub date: DateTimeWithTimezoneDatatype,
}
