pub use implementation_state::*;


pub mod implementation_state;

/// Implementation Status
/// Indicates the degree to which the a given control is implemented.
/// $id: #assembly_oscal-implementation-common_implementation-status
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct ImplementationStatus {
	pub remarks: Option<Remarks>,
	/// Implementation State
	/// Identifies the implementation status of the control or control objective.
	pub state: ImplementationState,
}
