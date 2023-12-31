/// Objective Status
/// A determination of if the objective is satisfied or not within a given system.
/// $id: #assembly_oscal-assessment-common_finding-target_objective-status_objective-status
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ObjectiveStatus {
	pub remarks: Option<Remarks>,
	/// Objective Status State
	/// An indication as to whether the objective is satisfied or not.
	pub state: TokenDatatype,
	/// Objective Status Reason
	/// The reason the objective was given it's status.
	pub reason: Option<TokenDatatype>,
}
