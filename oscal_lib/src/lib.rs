pub mod assembly;
pub mod definitions;
pub mod error;
pub mod field;

use assembly::{
    oscal_ap::AssessmentPlan, oscal_ar::assessment_results::AssessmentResults,
    oscal_catalog::Catalog, oscal_component_definition::ComponentDefinition,
    oscal_poam::PlanOfActionAndMilestones, oscal_profile::Profile, oscal_ssp::SystemSecurityPlan,
};
/// http://json-schema.org/draft-07/schema#
/// OSCAL Unified Model of Models: JSON Schema
/// $id: #/oscal
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Oscal {
    pub profile: Option<Profile>,
    pub system_security_plan: Option<SystemSecurityPlan>,
    pub catalog: Option<Catalog>,
    pub assessment_plan: Option<AssessmentPlan>,
    pub assessment_results: Option<AssessmentResults>,
    pub component_definition: Option<ComponentDefinition>,
    pub plan_of_action_and_milestones: Option<PlanOfActionAndMilestones>,
}
