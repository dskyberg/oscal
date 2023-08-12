pub use time_unit::*;


pub mod time_unit;

/// Frequency Condition
/// The task is intended to occur at the specified frequency.
/// $id: #assembly_oscal-assessment-common_task_event-timing_event-timing_frequency-condition_frequency-condition
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::PositiveIntegerDatatype;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct FrequencyCondition {
	/// Time Unit
	/// The unit of time for the period.
	pub unit: TimeUnit,
	/// Period
	/// The task must occur after the specified period has elapsed.
	pub period: PositiveIntegerDatatype,
}
