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
	pub profile: Option<Profile>,
	pub system_security_plan: Option<SystemSecurityPlan>,
	pub catalog: Option<Catalog>,
	pub assessment_results: Option<AssessmentResults>,
	pub assessment_plan: Option<AssessmentPlan>,
	pub component_definition: Option<ComponentDefinition>,
	pub plan_of_action_and_milestones: Option<PlanOfActionAndMilestones>,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog() {
        let json =
            include_str!("../../tests/fedramp-automation-rev4-baselines/FedRAMP_rev4_HIGH-baseline-resolved-profile_catalog.json");
        let _oscal = serde_json::from_str::<Oscal>(json).expect("failed");
        //assert!(oscal.is_ok());
    }
}

