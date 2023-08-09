#![doc = include_str!("../README.md")]
#![allow(ambiguous_glob_reexports)]
pub use definitions::*;
pub use assembly::*;
pub use field::*;


pub mod definitions;
pub mod assembly;
pub mod field;
pub mod error;

/// http://json-schema.org/draft-07/schema#
/// OSCAL Unified Model of Models: JSON Schema
/// $id: #/oscal
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all ="kebab-case")]
pub struct Oscal {
	pub assessment_plan: Option<AssessmentPlan>,
	pub assessment_results: Option<AssessmentResults>,
	pub component_definition: Option<ComponentDefinition>,
	pub profile: Option<Profile>,
	pub system_security_plan: Option<SystemSecurityPlan>,
	pub plan_of_action_and_milestones: Option<PlanOfActionAndMilestones>,
	pub catalog: Option<Catalog>,
}
