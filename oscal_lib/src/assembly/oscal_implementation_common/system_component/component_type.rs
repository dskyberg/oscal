/// Component Type
/// A category describing the purpose of the component.
/// $id: #assembly_oscal-implementation-common_system-component_component-type_component-type
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub enum ComponentType {
	// orig: this-system
	ThisSystem,
	// orig: system
	System,
	// orig: interconnection
	Interconnection,
	// orig: software
	Software,
	// orig: hardware
	Hardware,
	// orig: service
	Service,
	// orig: policy
	Policy,
	// orig: physical
	Physical,
	// orig: process-procedure
	ProcessProcedure,
	// orig: plan
	Plan,
	// orig: guidance
	Guidance,
	// orig: standard
	Standard,
	// orig: validation
	Validation,
	// orig: network
	Network,
}
