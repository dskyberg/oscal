/// Time Unit
/// The unit of time for the period.
/// $id: #assembly_oscal-assessment-common_task_event-timing_frequency-condition_time-unit_time-unit
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum TimeUnit {
	// orig: seconds
	Seconds,
	// orig: minutes
	Minutes,
	// orig: hours
	Hours,
	// orig: days
	Days,
	// orig: months
	Months,
	// orig: years
	Years,
}
