/// File name: ../oscal_lib/src/oscal_complete_oscal_poam/plan_of_action_and_milestones.rs
/// pub use oscal_complete_oscal_poam::*;
///
/// pub mod oscal_complete_oscal_poam;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    oscal_complete_oscal_assessment_common::{import_ssp, observation, risk},
    oscal_complete_oscal_implementation_common::system_id,
    oscal_complete_oscal_metadata::{back_matter, metadata},
    SchemaConstraint, UUIDDatatype,
};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlanOfActionAndMilestones {
    pub uuid: UUIDDatatype,
    /// "#assembly_oscal-metadata_metadata"
    pub metadata: metadata::Metadata,
    /// "#assembly_oscal-assessment-common_import-ssp"
    pub import_ssp: Option<import_ssp::ImportSsp>,
    /// "#field_oscal-implementation-common_system-id"
    pub system_id: Option<system_id::SystemId>,
    /// "#assembly_oscal-poam_local-definitions"
    pub local_definitions: Option<super::local_definitions::LocalDefinitions>,
    /// "#assembly_oscal-assessment-common_observation"
    pub observations: Option<Vec<observation::Observation>>,
    /// "#assembly_oscal-assessment-common_risk"
    pub risks: Option<Vec<risk::Risk>>,
    /// "#assembly_oscal-poam_poam-item"
    pub poam_items: Vec<super::poam_item::PoamItem>,
    /// "#assembly_oscal-metadata_back-matter"
    pub back_matter: Option<back_matter::BackMatter>,
}

impl SchemaConstraint for PlanOfActionAndMilestones {
    fn constraint_title() -> &'static str {
        "Plan of Action and Milestones (POA&M)"
    }
    fn constraint_description() -> &'static str {
        r#"A plan of action and milestones which identifies initial and residual risks, deviations, and disposition, such as those required by FedRAMP."#
    }
    fn constraint_id() -> &'static str {
        "#assembly_oscal-poam_plan-of-action-and-milestones"
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:plan-of-action-and-milestones"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let json = include_str!("test/poam.json");
        let result = serde_json::from_str::<PlanOfActionAndMilestones>(json).expect("oops");
        dbg!(result);
    }
}
