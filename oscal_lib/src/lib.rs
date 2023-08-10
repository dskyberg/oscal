#![doc = include_str!("../README.md")]
#![allow(ambiguous_glob_reexports)]
pub use assembly::*;
pub use definitions::*;
pub use field::*;

pub mod assembly;
pub mod definitions;
pub mod error;
pub mod field;

/// http://json-schema.org/draft-07/schema#
/// OSCAL Unified Model of Models: JSON Schema
/// $id: #/oscal
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Oscal {
    pub component_definition: Option<ComponentDefinition>,
    pub catalog: Option<Catalog>,
    pub assessment_plan: Option<AssessmentPlan>,
    pub plan_of_action_and_milestones: Option<PlanOfActionAndMilestones>,
    pub profile: Option<Profile>,
    pub system_security_plan: Option<SystemSecurityPlan>,
    pub assessment_results: Option<AssessmentResults>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog() {
        let json =
            include_str!("../../tests/fedramp-automation-rev5-baselines/FedRAMP_rev5_HIGH-baseline-resolved-profile_catalog.json");
        let oscal = serde_json::from_str::<Oscal>(json);
        assert!(oscal.is_ok());
    }
}
