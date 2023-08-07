/// Implementation State
/// Identifies the implementation status of the control or control objective.
/// $id: #assembly_oscal-implementation-common_implementation-status_implementation-state_implementation-state
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ImplementationState {
	// orig: implemented
	Implemented,
	// orig: partial
	Partial,
	// orig: planned
	Planned,
	// orig: alternative
	Alternative,
	// orig: not-applicable
	NotApplicable,
}
