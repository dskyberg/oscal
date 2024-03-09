/// File name: ../oscal_lib/src/oscal_complete_oscal_ap/assessment_plan.rs
/// pub use oscal_complete_oscal_ap::*;
///
/// pub mod oscal_complete_oscal_ap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    assessment_common::{
        assessment_assets::AssessmentAssets, assessment_subject::AssessmentSubject,
        import_ssp::ImportSsp, reviewed_controls::ReviewedControls, task::Task,
    },
    metadata::BackMatter,
    metadata::Metadata,
    SchemaElement, UUIDDatatype,
};

use self::{local_definitions::LocalDefinitions, terms_and_conditions::TermsAndConditions};
pub mod local_definitions;
pub mod terms_and_conditions;

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct AssessmentPlan {
    pub uuid: UUIDDatatype,
    pub metadata: Metadata,
    pub import_ssp: ImportSsp,
    pub local_definitions: Option<LocalDefinitions>,
    pub terms_and_conditions: Option<TermsAndConditions>,
    pub reviewed_controls: ReviewedControls,
    pub assessment_subjects: Option<Vec<AssessmentSubject>>,
    pub assessment_assets: Option<AssessmentAssets>,
    pub tasks: Option<Vec<Task>>,
    pub back_matter: Option<BackMatter>,
}

impl SchemaElement for AssessmentPlan {
    fn schema_title() -> &'static str {
        "Security Assessment Plan (SAP)"
    }
    fn schema_description() -> &'static str {
        r#"An assessment plan, such as those provided by a FedRAMP assessor."#
    }
    fn schema_id() -> Option<&'static str> {
        Some("#assembly_oscal-ap_assessment-plan")
    }
    fn schema_path() -> &'static str {
        "#/definitions/oscal-complete-oscal-ap:assessment-plan"
    }
}
