/// Objective Status Reason
/// The reason the objective was given it's status.
/// $id: #assembly_oscal-assessment-common_finding-target_objective-status_objective-status-reason_objective-status-reason
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ObjectiveStatusReason {
	// orig: pass
	Pass,
	// orig: fail
	Fail,
	// orig: other
	Other,
}
