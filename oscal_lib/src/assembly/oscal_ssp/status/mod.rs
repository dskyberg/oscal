pub use state::*;


pub mod state;

/// Status
/// Describes the operational status of the system.
/// $id: #assembly_oscal-ssp_status
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Status {
	/// State
	/// The current operating status.
	pub state: State,
	pub remarks: Option<Remarks>,
}
