/// File name: ../oscal_lib/src/oscal_complete_oscal_poam/plan_of_action_and_milestones.rs
/// pub use oscal_complete_oscal_poam::*;
///
/// pub mod oscal_complete_oscal_poam;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{import_ssp::ImportSsp, observation::Observation, risk::Risk},
    implementation_common::system_id::SystemId,
    metadata::{BackMatter, Metadata},
    SchemaElement, UUIDDatatype,
};

use super::{local_definitions::LocalDefinitions, poam_item::PoamItem};

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlanOfActionAndMilestones {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub import_ssp: Option<ImportSsp>,
    pub system_id: Option<SystemId>,
    pub local_definitions: Option<LocalDefinitions>,
    pub observations: Option<Vec<Observation>>,
    pub risks: Option<Vec<Risk>>,
    pub poam_items: Vec<PoamItem>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaElement for PlanOfActionAndMilestones {
    fn schema_title() -> &'static str {
        "Plan of Action and Milestones (POA&M)"
    }
    fn schema_description() -> &'static str {
        r#"A plan of action and milestones which identifies initial and residual risks, deviations, and disposition, such as those required by FedRAMP."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-poam_plan-of-action-and-milestones")
    }
    fn schema_path() -> &'static str {
        "oscal-complete-oscal-poam:plan-of-action-and-milestones"
    }
}
