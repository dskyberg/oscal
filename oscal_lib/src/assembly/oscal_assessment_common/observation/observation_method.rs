/// Observation Method
/// Identifies how the observation was made.
/// $id: #assembly_oscal-assessment-common_observation_observation-method_observation-method
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ObservationMethod {
	// orig: EXAMINE
	#[serde(rename = "EXAMINE")]
	Examine,
	// orig: INTERVIEW
	#[serde(rename = "INTERVIEW")]
	Interview,
	// orig: TEST
	#[serde(rename = "TEST")]
	Test,
	// orig: UNKNOWN
	#[serde(rename = "UNKNOWN")]
	Unknown,
}
