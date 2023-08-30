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
	pub plan_of_action_and_milestones: Option<PlanOfActionAndMilestones>,
	pub assessment_results: Option<AssessmentResults>,
	pub system_security_plan: Option<SystemSecurityPlan>,
	pub component_definition: Option<ComponentDefinition>,
	pub assessment_plan: Option<AssessmentPlan>,
	pub profile: Option<Profile>,
	pub catalog: Option<Catalog>,
}



#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_rev4_moderate() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev4/baselines/json/FedRAMP_rev4_MODERATE-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev4_high() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev4/baselines/json/FedRAMP_rev4_HIGH-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}


#[test]
fn test_rev5_moderate() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/baselines/json/FedRAMP_rev5_MODERATE-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_high() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/baselines/json/FedRAMP_rev5_HIGH-baseline-resolved-profile_catalog.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_poam() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/poam/json/FedRAMP-POAM-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_sap() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/sap/json/FedRAMP-SAP-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_sar() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/sar/FedRAMP-SAR-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}

#[test]
fn test_rev5_ssp() {
    let json =
        include_str!("../../tests/fedramp-automation/dist/content/rev5/templates/ssp/json/FedRAMP-SSP-OSCAL-Template.json");
    let oscal = serde_json::from_str::<Oscal>(json);
    assert!(oscal.is_ok());
}
}

