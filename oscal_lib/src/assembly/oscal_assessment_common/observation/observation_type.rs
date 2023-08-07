/// Observation Type
/// Identifies the nature of the observation. More than one may be used to further qualify and enable filtering.
/// $id: #assembly_oscal-assessment-common_observation_observation-type_observation-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ObservationType {
	// orig: ssp-statement-issue
	SspStatementIssue,
	// orig: control-objective
	ControlObjective,
	// orig: mitigation
	Mitigation,
	// orig: finding
	Finding,
	// orig: historic
	Historic,
}
