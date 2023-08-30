/// Status
/// Describes the operational status of the system component.
/// $id: #assembly_oscal-implementation-common_system-component_status_status
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::definitions::TokenDatatype;
use crate::field::oscal_metadata::Remarks;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Status {
	pub remarks: Option<Remarks>,
	/// State
	/// The operational status.
	pub state: TokenDatatype,
}
