/// State
/// The current operating status.
/// $id: #assembly_oscal-ssp_status_state_state
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum State {
	// orig: operational
	Operational,
	// orig: under-development
	UnderDevelopment,
	// orig: under-major-modification
	UnderMajorModification,
	// orig: disposition
	Disposition,
	// orig: other
	Other,
}
