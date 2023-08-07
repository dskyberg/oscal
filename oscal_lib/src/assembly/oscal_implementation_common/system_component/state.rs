/// State
/// The operational status.
/// $id: #assembly_oscal-implementation-common_system-component_status_state_state
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum State {
	// orig: under-development
	UnderDevelopment,
	// orig: operational
	Operational,
	// orig: disposition
	Disposition,
	// orig: other
	Other,
}
