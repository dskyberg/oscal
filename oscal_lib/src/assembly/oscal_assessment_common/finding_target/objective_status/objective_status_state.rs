/// Objective Status State
/// An indication as to whether the objective is satisfied or not.
/// $id: #assembly_oscal-assessment-common_finding-target_objective-status_objective-status_objective-status-state_objective-status-state
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ObjectiveStatusState {
	// orig: satisfied
	Satisfied,
	// orig: not-satisfied
	NotSatisfied,
}
