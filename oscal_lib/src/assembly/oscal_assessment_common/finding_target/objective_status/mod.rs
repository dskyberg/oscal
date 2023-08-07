pub use objective_status_reason::*;
pub use objective_status_state::*;


pub mod objective_status_reason;
pub mod objective_status_state;

/// Objective Status
/// A determination of if the objective is satisfied or not within a given system.
/// $id: #assembly_oscal-assessment-common_finding-target_objective-status_objective-status
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ObjectiveStatus {
	/// Objective Status State
	/// An indication as to whether the objective is satisfied or not.
	pub state: ObjectiveStatusState,
	pub remarks: Option<Remarks>,
	/// Objective Status Reason
	/// The reason the objective was given it's status.
	pub reason: Option<ObjectiveStatusReason>,
}
